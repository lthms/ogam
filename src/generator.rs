use ::ast::*;
use ::typography::{Space, choose, Typography};

pub trait Renderer<O> {
    fn append(&self, O, O) -> O;
    fn empty(&self) -> O;

    fn render_space(&self, space: Space) -> O;
    fn render_word(&self, word: &str) -> O;
    fn render_mark(&self, mark: &str) -> O;
    fn render_illformed(&self, err: &str) -> O;

    fn emph_template(&self, format: O) -> O;
    fn strong_emph_template(&self, format: O) -> O;

    fn reply_template(&self, reply: O) -> O;

    fn thought_template(&self, reply: O, author: &Option<&str>) -> O;
    fn dialogue_template(&self, reply: O, author: &Option<&str>) -> O;
    fn between_dialogue(&self) -> O;
    fn illformed_inline_template(&self, err: O) -> O;

    fn paragraph_template(&self, para: O) -> O;

    fn illformed_block_template(&self, err: O) -> O;
    fn story_template(&self, err: O) -> O;
    fn aside_template(&self, cls: &Option<&str>, err: O) -> O;

    fn render_atom<'a, T: Typography>(&self, atom: &Atom<'a>, typo: &T) -> O {
        match atom {
            Atom::Punctuation(ref p) => {
                self.render_mark(typo.output(p))
            },
            Atom::Word(w) => {
                self.render_word(w)
            }
            Atom::Void => {
                self.empty()
            }
        }
    }
}

pub trait Renderable<'a> {
    fn render<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, previous: &mut &'b Atom<'a>) -> O;

    fn render_one_shot<O, T: Typography, R: Renderer<O>>(&self, typo: &T, renderer: &R) -> O {
        let start_with = Atom::Void;
        self.render(typo, renderer, &mut &start_with)
    }
}

impl<'a> Renderable<'a> for Atom<'a> {
    fn render<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, previous: &mut &'b Atom<'a>) -> O {
        let space = choose(typo.after_atom(previous), typo.before_atom(self));
        *previous = self;

        renderer.append(renderer.render_space(space), renderer.render_atom(self, typo))
    }
}

impl<'a> Renderable<'a> for Format<'a> {
    fn render<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, previous: &mut &'b Atom<'a>) -> O {
        match self {
            Format::Raw(atoms) => {
                atoms.render(typo, renderer, previous)
            },
            Format::Emph(atoms) => {
                renderer.emph_template(atoms.render(typo, renderer, previous))
            },
            Format::StrongEmph(atoms) => {
                renderer.strong_emph_template(atoms.render(typo, renderer, previous))
            },
            Format::Quote(atoms) => {
                let before = Atom::Punctuation(Mark::OpenQuote).render(typo, renderer, previous);
                let content = atoms.render(typo, renderer, previous);
                let after = Atom::Punctuation(Mark::CloseQuote).render(typo, renderer, previous);

                renderer.append(renderer.append(before, content), after)
            }
        }
    }
}

impl<'a> Reply<'a> {
    fn render_reply<'b, O, T: Typography, R: Renderer<O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        open: Option<&'static Atom<'static>>,
        close: Option<&'static Atom<'static>>,
        previous: &mut &'b Atom<'a>
    ) -> O {
        match self {
            Reply::Simple(atoms) => {
                let o1 = open.map(|x| x.render(typo, renderer, previous)).unwrap_or(renderer.empty());
                let o2 = renderer.reply_template(atoms.render(typo, renderer, previous));
                let o3 = close.map(|x| x.render(typo, renderer, previous)).unwrap_or(renderer.empty());

                renderer.append(o1, renderer.append(o2, o3))
            }
            Reply::WithSay(atoms, insert, None) => {
                let o1 = open.map(|x| x.render(typo, renderer, previous)).unwrap_or(renderer.empty());
                let o2 = renderer.reply_template(atoms.render(typo, renderer, previous));
                let o3 = close.map(|x| x.render(typo, renderer, previous)).unwrap_or(renderer.empty());
                let o4 = insert.render(typo, renderer, previous);

                renderer.append(o1, renderer.append(o2, renderer.append(o3, o4)))
            }
            Reply::WithSay(atoms1, insert, Some(atoms2)) => {
                let o1 = open.map(|x| x.render(typo, renderer, previous)).unwrap_or(renderer.empty());
                let o2 = renderer.reply_template(atoms1.render(typo, renderer, previous));
                let o3 = insert.render(typo, renderer, previous);
                let o4 = renderer.reply_template(atoms2.render(typo, renderer, previous));
                let o5 = close.map(|x| x.render(typo, renderer, previous)).unwrap_or(renderer.empty());

                renderer.append(o1, renderer.append(o2, renderer.append(o3, renderer.append(o4, o5))))
            }
        }
    }
}

impl<'a> Component<'a> {
    fn is_dialog(&self) -> bool {
        match self {
            Component::Dialogue(_, _) => {
                true
            },
            _ => {
                false
            }
        }
    }

    fn render_component<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, was_dialog: bool, will_be_dialog: bool, previous: &mut &'b Atom<'a>) -> O {
        match self {
            Component::Teller(atoms) => {
                atoms.render(typo, renderer, previous)
            }
            Component::IllFormed(err) => {
                renderer.illformed_inline_template(renderer.render_illformed(err))
            }
            Component::Thought(reply, cls) => {
                renderer.thought_template(reply.render_reply(typo, renderer, None, None, previous), cls)
            },
            Component::Dialogue(reply, cls) => {
                let o = typo.open_dialog(was_dialog);
                let e = typo.close_dialog(will_be_dialog);

                let sep = if was_dialog {
                    *previous = &Atom::Void;
                    renderer.between_dialogue()
                } else {
                    renderer.empty()
                };

                let res = renderer.dialogue_template(reply.render_reply(typo, renderer, o, e, previous), cls);

                renderer.append(sep, res)
            },
        }
    }
}

impl<'a, A: Renderable<'a>> Renderable<'a> for Vec<A> {
    fn render<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, previous: &mut &'b Atom<'a>) -> O {
        let mut cursor = self.iter();
        let mut res_str = renderer.empty();

        loop {
            match cursor.next() {
                Some(el) => {
                    let seg = el.render(typo, renderer, previous);
                    res_str = renderer.append(res_str, seg);
                }
                None => {
                    break
                }
            }
        }

        res_str
    }
}

impl<'a> Renderable<'a> for Paragraph<'a> {
    fn render<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, previous: &mut &'b Atom<'a>) -> O {
        let mut was_dialogue = false;
        let mut will_be_dialogue;
        let mut res = renderer.empty();

        match self {
            Paragraph(components) => {
                let mut cursor = components.iter().peekable();

                loop {
                    if let Some(current) = cursor.next() {
                        will_be_dialogue = cursor.peek().map(|x| x.is_dialog()).unwrap_or(false);

                        res = renderer.append(
                            res,
                            current.render_component(typo, renderer, was_dialogue, will_be_dialogue, previous)
                        );

                        was_dialogue = current.is_dialog();
                    } else {
                        break
                    }
                }
            }
        }

        *previous = &Atom::Void;

        renderer.paragraph_template(res)
    }
}

impl<'a> Renderable<'a> for Section<'a> {
    fn render<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, previous: &mut &'b Atom<'a>) -> O {
        *previous = &Atom::Void;

        match self {
            Section::Story(atoms) => {
                renderer.story_template(atoms.render(typo, renderer, previous))
            },
            Section::Aside(cls, atoms) => {
                renderer.aside_template(cls, atoms.render(typo, renderer, previous))
            },
            Section::IllFormed(vec) => {
                renderer.illformed_block_template(
                    vec.iter().fold(renderer.empty(), |res, x| {
                        renderer.append(res, renderer.render_illformed(x))
                    })
                )
            }
        }
    }
}

impl<'a> Renderable<'a> for Document<'a> {
    fn render<'b, O, T: Typography, R: Renderer<O>>(&'b self, typo: &T, renderer: &R, previous: &mut &'b Atom<'a>) -> O {
        match self {
            Document(atoms) => {
                atoms.render(typo, renderer, previous)
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use ::typography::FRENCH;

    pub struct Html;

    impl Renderer<String> for Html {
        fn append(&self, before: String, after: String) -> String {
            format!("{}{}", before, after)
        }

        fn empty(&self) -> String {
            String::from("")
        }

        fn render_space(&self, space: Space) -> String {
            match space {
                Space::Normal => String::from(" "),
                Space::Nbsp => String::from("&nbsp;"),
                Space::None => String::from(""),
            }
        }

        fn render_word(&self, word: &str) -> String {
            String::from(word)
        }

        fn render_mark(&self, mark: &str) -> String {
            String::from(mark)
        }

        fn render_illformed(&self, err: &str) -> String {
            String::from(err)
        }

        fn emph_template<'a>(&self, format: String) -> String {
            format!("<em>{}</em>", format)
        }

        fn strong_emph_template<'a>(&self, format: String) -> String {
            format!("<strong>{}</strong>", format)
        }

        fn reply_template(&self, reply: String) -> String {
            format!("<span div=\"reply\">{}</span>", reply)
        }

        fn thought_template(&self, reply: String, author: &Option<&str>) -> String {
            format!(
                "<span div=\"thought{}\">{}</span>",
                author.map(|x| format!(" by-{}", x)).unwrap_or(String::from("")),
                reply,
            )
        }

        fn dialogue_template(&self, reply: String, author: &Option<&str>) -> String {
            format!(
                "<span div=\"dialogue{}\">{}</span>",
                author.map(|x| format!(" by-{}", x)).unwrap_or(String::from("")),
                reply,
            )
        }

        fn between_dialogue(&self) -> String {
            String::from("</p><p>")
        }

        fn illformed_inline_template(&self, err: String) -> String {
            format!("<span div=\"illformed_inline\">{}</span>", err)
        }

        fn paragraph_template(&self, para: String) -> String {
            format!("<p>{}</p>", para)
        }

        fn illformed_block_template(&self, err: String) -> String {
            format!("<div class=\"illformed_block\">{}</div>", err)
        }

        fn story_template(&self, story: String) -> String {
            format!("<div class=\"story\">{}</div>", story)
        }

        fn aside_template(&self, cls: &Option<&str>, aside: String) -> String {
            format!("<div class=\"aside {}\">{}</div>", cls.unwrap_or(""), aside)
        }
    }

    #[test]
    fn test_generation() {
        let html: Html = Html;

        assert_eq!(vec![Atom::Word("Bonjour"), Atom::Word("toi")].render_one_shot(&FRENCH, &html), "Bonjour toi");

        assert_eq!(
            Format::StrongEmph(vec![Format::Raw(vec![Atom::Word("Bonjour"), Atom::Word("toi")])])
                .render_one_shot(&FRENCH, &html),
            "<strong>Bonjour toi</strong>"
        );

        assert_eq!(
            Format::Quote(vec![Format::Raw(vec![Atom::Word("Bonjour"), Atom::Word("toi")])])
                .render_one_shot(&FRENCH, &html),
            "«&nbsp;Bonjour toi&nbsp;»"
        );

        assert_eq!(
            Paragraph(
                vec![
                    Component::Dialogue(
                        Reply::Simple(
                            vec![
                                Format::Raw(
                                    vec![
                                        Atom::Word("Salut")
                                    ]
                                )
                            ]
                        ),
                        None
                    ),
                    Component::Dialogue(
                        Reply::Simple(
                            vec![
                                Format::Raw(
                                    vec![
                                        Atom::Word("Bonjour")
                                    ]
                                )
                            ]
                        ),
                        None
                    )
                ]
            ).render_one_shot(&FRENCH, &html),
            "<p><span div=\"dialogue\">«<span div=\"reply\">&nbsp;Salut</span></span></p><p><span div=\"dialogue\">—<span div=\"reply\"> Bonjour</span>&nbsp;»</span></p>"
        );
    }
}
