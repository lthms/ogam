use ::ast::*;
use ::typography::{Space, choose, Typography};

pub trait Renderer<'a, O> {
    fn append(&self, O, O) -> O;
    fn empty(&self) -> O;

    fn render_space(&self, space: Space) -> O;
    fn render_word(&self, word: &'a str) -> O;
    fn render_mark(&self, mark: &'a str) -> O;
    fn render_illformed(&self, err: &'a str) -> O;

    fn emph_template(&self, format: O) -> O;
    fn strong_emph_template(&self, format: O) -> O;

    fn reply_template(&self, reply: O) -> O;

    fn thought_template(&self, reply: O, author: &Option<&'a str>) -> O;
    fn dialogue_template(&self, reply: O, author: &Option<&'a str>) -> O;
    fn between_dialogue(&self) -> O;
    fn illformed_inline_template(&self, err: O) -> O;

    fn paragraph_template(&self, para: O) -> O;

    fn illformed_block_template(&self, err: O) -> O;
    fn story_template(&self, err: O) -> O;
    fn aside_template(&self, cls: &Option<&'a str>, err: O) -> O;

    fn render_atom<T: Typography>(&self, atom: &Atom<'a>, typo: &T) -> O {
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

struct Memory<'b, 'a: 'b> {
    atom: &'b Atom<'a>,
    was_dialogue: bool,
}

impl<'b, 'a: 'b> Memory<'b, 'a> {
    fn init() -> Self {
        Memory {
            atom: &Atom::Void,
            was_dialogue: false,
        }
    }

    fn remember_atom(&mut self, past: &'b Atom<'a>) {
        self.atom = past;
    }

    fn reset_atom(&mut self) {
        self.atom = &Atom::Void;
    }

    fn remember_dialogue(&mut self, dial: bool) {
        self.was_dialogue = dial;
    }

    fn reset(&mut self) {
        self.reset_atom();
        self.was_dialogue = false;
    }
}

trait Renderable<'b, 'a: 'b> {
    fn render<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        mem: &mut Memory<'b, 'a>
    ) -> O;

    fn render_one_shot<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R
    ) -> O {
        let mut start_with = Memory::init();
        self.render(typo, renderer, &mut start_with)
    }
}

impl<'b, 'a: 'b, A: Renderable<'b, 'a>> Renderable<'b, 'a> for Vec<A> {
    fn render<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        let mut res_str = renderer.empty();

        for el in self {
            let seg = el.render(typo, renderer, mem);
            res_str = renderer.append(res_str, seg);
        }

        res_str
    }
}

impl<'b, 'a: 'b> Renderable<'b, 'a> for Atom<'a> {
    fn render<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        let space = choose(typo.after_atom(mem.atom), typo.before_atom(self));

        mem.remember_atom(self);

        renderer.append(renderer.render_space(space), renderer.render_atom(self, typo))
    }
}

impl<'b, 'a: 'b> Renderable<'b, 'a> for Format<'a> {
    fn render<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        match self {
            Format::Raw(atoms) => {
                atoms.render(typo, renderer, mem)
            },
            Format::Emph(atoms) => {
                renderer.emph_template(atoms.render(typo, renderer, mem))
            },
            Format::StrongEmph(atoms) => {
                renderer.strong_emph_template(atoms.render(typo, renderer, mem))
            },
            Format::Quote(atoms) => {
                let before = Atom::Punctuation(Mark::OpenQuote).render(typo, renderer, mem);
                let content = atoms.render(typo, renderer, mem);
                let after = Atom::Punctuation(Mark::CloseQuote).render(typo, renderer, mem);

                renderer.append(renderer.append(before, content), after)
            }
        }
    }
}

impl<'b, 'a: 'b> Reply<'a> {
    fn render_reply<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        open: Option<&'static Atom<'static>>,
        close: Option<&'static Atom<'static>>,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        match self {
            Reply::Simple(atoms) => {
                let o1 = open.map(|x| x.render(typo, renderer, mem)).unwrap_or(renderer.empty());
                let o2 = renderer.reply_template(atoms.render(typo, renderer, mem));
                let o3 = close.map(|x| x.render(typo, renderer, mem)).unwrap_or(renderer.empty());

                renderer.append(o1, renderer.append(o2, o3))
            }
            Reply::WithSay(atoms, insert, None) => {
                let o1 = open.map(|x| x.render(typo, renderer, mem)).unwrap_or(renderer.empty());
                let o2 = renderer.reply_template(atoms.render(typo, renderer, mem));
                let o3 = close.map(|x| x.render(typo, renderer, mem)).unwrap_or(renderer.empty());
                let o4 = insert.render(typo, renderer, mem);

                renderer.append(o1, renderer.append(o2, renderer.append(o3, o4)))
            }
            Reply::WithSay(atoms1, insert, Some(atoms2)) => {
                let o1 = open.map(|x| x.render(typo, renderer, mem)).unwrap_or(renderer.empty());
                let o2 = renderer.reply_template(atoms1.render(typo, renderer, mem));
                let o3 = insert.render(typo, renderer, mem);
                let o4 = renderer.reply_template(atoms2.render(typo, renderer, mem));
                let o5 = close.map(|x| x.render(typo, renderer, mem)).unwrap_or(renderer.empty());

                renderer.append(o1, renderer.append(o2, renderer.append(o3, renderer.append(o4, o5))))
            }
        }
    }
}

impl<'b, 'a: 'b> Component<'a> {
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

    fn render_component<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        will_be_dialog: bool,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        let res = match self {
            Component::Teller(atoms) => {
                atoms.render(typo, renderer, mem)
            }
            Component::IllFormed(err) => {
                renderer.illformed_inline_template(renderer.render_illformed(err))
            }
            Component::Thought(reply, cls) => {
                renderer.thought_template(
                    reply.render_reply(typo, renderer, None, None, mem),
                    cls
                )
            },
            Component::Dialogue(reply, cls) => {
                let o = typo.open_dialog(mem.was_dialogue);
                let e = typo.close_dialog(will_be_dialog);

                let res = renderer.dialogue_template(
                    reply.render_reply(typo, renderer, o, e, mem),
                    cls
                );

                let sep = if will_be_dialog {
                    mem.reset_atom();
                    renderer.between_dialogue()
                } else {
                    renderer.empty()
                };

                renderer.append(res, sep)
            },
        };

        mem.remember_dialogue(self.is_dialog());

        res
    }
}

impl<'b, 'a: 'b> Renderable<'b, 'a> for Paragraph<'a> {
    fn render<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        let mut will_be_dialogue;
        let mut res = renderer.empty();

                for i in 0..self.0.len() {
                    will_be_dialogue = if i+1 < self.0.len() {
                        self.0[i+1].is_dialog()
                    } else {
                        false
                    };

                    let comp: &'b Component<'a> = &self.0[i];
                    res = renderer.append(
                        res,
                        comp.render_component(
                            typo,
                            renderer,
                            will_be_dialogue,
                            mem
                        )
                    );
        }

        mem.reset_atom();

        renderer.paragraph_template(res)
    }
}

impl<'b, 'a: 'b> Renderable<'b, 'a> for Section<'a> {
    fn render<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        mem.reset();

        match self {
            Section::Story(atoms) => {
                renderer.story_template(atoms.render(typo, renderer, mem))
            },
            Section::Aside(cls, atoms) => {
                renderer.aside_template(cls, atoms.render(typo, renderer, mem))
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

impl<'b, 'a: 'b> Renderable<'b, 'a> for Document<'a> {
    fn render<O, T: Typography, R: Renderer<'a, O>>(
        &'b self,
        typo: &T,
        renderer: &R,
        mem: &mut Memory<'b, 'a>
    ) -> O {
        match self {
            Document(atoms) => {
                atoms.render(typo, renderer, mem)
            }
        }
    }
}

pub fn render_document<'b, 'a: 'b, O, T: Typography, R: Renderer<'a, O>>(
    doc: &'b Document<'a>,
    typo: &T,
    renderer: &R
) -> O {
    let mut start_with = Memory::init();
    doc.render(typo, renderer, &mut start_with)
}

#[cfg(test)]
pub mod test {
    use super::*;

    use ::typography::FRENCH;

    pub struct Html;

    impl<'a> Renderer<'a, String> for Html {
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

        fn render_word(&self, word: &'a str) -> String {
            String::from(word)
        }

        fn render_mark(&self, mark: &'a str) -> String {
            String::from(mark)
        }

        fn render_illformed(&self, err: &'a str) -> String {
            String::from(err)
        }

        fn emph_template(&self, format: String) -> String {
            format!("<em>{}</em>", format)
        }

        fn strong_emph_template(&self, format: String) -> String {
            format!("<strong>{}</strong>", format)
        }

        fn reply_template(&self, reply: String) -> String {
            format!("<span div=\"reply\">{}</span>", reply)
        }

        fn thought_template(&self, reply: String, author: &Option<&'a str>) -> String {
            format!(
                "<span div=\"thought{}\">{}</span>",
                author.map(|x| format!(" by-{}", x)).unwrap_or(String::from("")),
                reply,
            )
        }

        fn dialogue_template(&self, reply: String, author: &Option<&'a str>) -> String {
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

        fn aside_template(&self, cls: &Option<&'a str>, aside: String) -> String {
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
