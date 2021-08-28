/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::ast::*;
use crate::typography::{choose, previous_dialogue, Space, Typography};

pub trait Output {
    fn empty(input_size: usize) -> Self;

    fn render_space(&mut self, space: Space) -> ();
    fn render_word(&mut self, word: &str) -> ();
    fn render_mark(&mut self, mark: &str) -> ();
    fn render_illformed(&mut self, err: &str) -> ();

    fn emph_template<F>(&mut self, format: F) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn strong_emph_template<F>(&mut self, format: F) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn reply_template<F>(&mut self, reply: F, author: &Option<&str>) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn thought_template<F>(&mut self, reply: F, author: &Option<&str>) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn dialogue_template<F>(&mut self, reply: F, author: &Option<&str>) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn between_dialogue(&mut self) -> ();

    fn illformed_inline_template<F>(&mut self, err: F) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn paragraph_template<F>(&mut self, para: F) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn illformed_block_template<F>(&mut self, err: F) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn story_template<F>(&mut self, err: F) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn aside_template<'input, F>(&mut self, cls: &Option<&'input str>, err: F) -> ()
    where
        F: FnOnce(&mut Self) -> ();

    fn render_atom<T: Typography + ?Sized>(&mut self, atom: &Atom, typo: &T) -> () {
        match atom {
            Atom::Punctuation(ref p) => self.render_mark(typo.output(p)),
            Atom::Word(w) => self.render_word(w),
            Atom::Void => (),
        }
    }
}

struct Memory<'ast, 'input: 'ast> {
    atom: &'ast Atom<'input>,
    was_dialogue: Option<Option<&'input str>>,
    next_paragraph_starts_with_dialogue: bool,
}

impl<'ast, 'input: 'ast> Memory<'ast, 'input> {
    fn init() -> Self {
        Memory {
            atom: &Atom::Void,
            was_dialogue: None,
            next_paragraph_starts_with_dialogue: false,
        }
    }

    fn remember_atom(&mut self, past: &'ast Atom<'input>) {
        self.atom = past;
    }

    fn reset_atom(&mut self) {
        self.atom = &Atom::Void;
    }

    fn remember_dialogue(&mut self, dial: Option<Option<&'input str>>) {
        self.was_dialogue = dial;
    }

    fn reset(&mut self) {
        self.reset_atom();
        self.was_dialogue = None;
        self.next_paragraph_starts_with_dialogue = false;
    }
}

trait Renderable<'ast, 'input: 'ast> {
    fn render<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        mem: &mut Memory<'ast, 'input>,
    ) -> ();

    fn render_one_shot<O: Output, T: Typography + ?Sized>(&'ast self, typo: &T, out: &mut O) -> () {
        self.render(typo, out, &mut Memory::init())
    }
}

impl<'ast, 'input: 'ast, A: Renderable<'ast, 'input>> Renderable<'ast, 'input> for Vec<A> {
    fn render<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        for el in self {
            el.render(typo, out, mem);
        }
    }
}

impl<'ast, 'input: 'ast> Renderable<'ast, 'input> for Atom<'input> {
    fn render<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        let space = choose(typo.after_atom(mem.atom), typo.before_atom(self));

        out.render_space(space);
        out.render_atom(self, typo);

        // TODO: be sure it is not before rendering
        mem.remember_atom(self);
    }
}

impl<'ast, 'input: 'ast> Renderable<'ast, 'input> for Format<'input> {
    fn render<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        match self {
            Format::Raw(atoms) => atoms.render(typo, out, mem),
            Format::Emph(atoms) => out.emph_template(|out| atoms.render(typo, out, mem)),
            Format::StrongEmph(atoms) => {
                out.strong_emph_template(|out| atoms.render(typo, out, mem))
            }
            Format::Quote(atoms) => {
                Atom::Punctuation(Mark::OpenQuote).render(typo, out, mem);
                atoms.render(typo, out, mem);
                Atom::Punctuation(Mark::CloseQuote).render(typo, out, mem);
            }
        }
    }
}

impl<'ast, 'input: 'ast> Reply<'input> {
    fn render_reply<O: Output, T: Typography + ?Sized>(
        &'ast self,
        author: &Option<&'input str>,
        typo: &T,
        out: &mut O,
        open: Option<&'static Atom<'static>>,
        close: Option<&'static Atom<'static>>,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        match self {
            Reply::Simple(atoms) => {
                open.map(|x| x.render(typo, out, mem));
                out.reply_template(|out| atoms.render(typo, out, mem), author);
                close.map(|x| x.render(typo, out, mem));
            }
            Reply::WithSay(atoms, insert, None) => {
                open.map(|x| x.render(typo, out, mem));
                out.reply_template(|out| atoms.render(typo, out, mem), author);
                close.map(|x| x.render(typo, out, mem));
                insert.render(typo, out, mem);
            }
            Reply::WithSay(atoms1, insert, Some(atoms2)) => {
                open.map(|x| x.render(typo, out, mem));
                out.reply_template(|out| atoms1.render(typo, out, mem), author);
                insert.render(typo, out, mem);
                out.reply_template(|out| atoms2.render(typo, out, mem), author);
                close.map(|x| x.render(typo, out, mem));
            }
        }
    }
}

impl<'ast, 'input: 'ast> Component<'input> {
    fn is_dialog(&self) -> Option<Option<&'input str>> {
        match self {
            Component::Dialogue(_, author) => Some(*author),
            _ => None,
        }
    }

    fn render_component<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        will_be_dialog: bool,
        is_last: bool,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        match self {
            Component::Teller(atoms) => atoms.render(typo, out, mem),
            Component::IllFormed(err) => {
                // TODO: Is it really what we want to do?
                out.illformed_inline_template(|out| out.render_illformed(err))
            }
            Component::Thought(reply, cls) => out.thought_template(
                |out| reply.render_reply(cls, typo, out, None, None, mem),
                cls,
            ),
            Component::Dialogue(reply, cls) => {
                let was_dialogue = previous_dialogue(mem.was_dialogue, *cls);
                let o = typo.open_dialog(was_dialogue);
                let e = typo.close_dialog(will_be_dialog);

                out.dialogue_template(|out| reply.render_reply(cls, typo, out, o, e, mem), cls);

                if will_be_dialog && !is_last {
                    mem.reset_atom();
                    out.between_dialogue();
                }
            }
        };

        mem.remember_dialogue(self.is_dialog());
    }
}

impl<'ast, 'input: 'ast> Renderable<'ast, 'input> for Paragraph<'input> {
    fn render<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        out.paragraph_template(|out| {
            for i in 0..self.0.len() {
                let is_last = i + 1 >= self.0.len();
                let will_be_dialogue = if !is_last {
                    self.0[i + 1].is_dialog().is_some()
                } else {
                    mem.next_paragraph_starts_with_dialogue
                };

                let comp: &'ast Component<'input> = &self.0[i];
                comp.render_component(typo, out, will_be_dialogue, is_last, mem);
            }
        });

        mem.reset_atom();
    }
}

fn render_paragraphs<'ast, 'input: 'ast, O: Output, T: Typography + ?Sized>(
    ast: &'ast [Paragraph<'input>],
    typo: &T,
    out: &mut O,
    mem: &mut Memory<'ast, 'input>,
) -> () {
    for i in 0..ast.len() {
        mem.next_paragraph_starts_with_dialogue = if i + 1 < ast.len() {
            ast[i + 1].0[0].is_dialog().is_some()
        } else {
            false
        };

        ast[i].render(typo, out, mem);
    }
}

impl<'ast, 'input: 'ast> Renderable<'ast, 'input> for Section<'input> {
    fn render<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        mem.reset();

        match self {
            Section::Story(atoms) => {
                out.story_template(|out| render_paragraphs(atoms, typo, out, mem));
            }
            Section::Aside(cls, atoms) => {
                out.aside_template(cls, |out| render_paragraphs(atoms, typo, out, mem));
            }
            Section::IllFormed(vec) => {
                out.illformed_block_template(|out| {
                    for l in vec {
                        out.render_illformed(l);
                    }
                });
            }
        }
    }
}

impl<'ast, 'input: 'ast> Renderable<'ast, 'input> for Document<'input> {
    fn render<O: Output, T: Typography + ?Sized>(
        &'ast self,
        typo: &T,
        out: &mut O,
        mem: &mut Memory<'ast, 'input>,
    ) -> () {
        match self {
            Document(atoms) => atoms.render(typo, out, mem),
        }
    }
}

pub fn render<'ast, 'input: 'ast, O: Output, T: Typography + ?Sized>(
    doc: &'ast Document<'input>,
    typo: &T,
    out: &mut O,
) -> () {
    doc.render(typo, out, &mut Memory::init())
}

#[cfg(test)]
pub mod test {
    use super::*;

    use crate::typography::FRENCH;

    #[derive(Debug, PartialEq, Eq)]
    struct Html(String);

    impl Html {
        fn push_str(&mut self, s: &str) -> () {
            self.0.push_str(s);
        }
    }

    impl Output for Html {
        fn empty(_: usize) -> Html {
            Html(String::from(""))
        }

        fn render_space(&mut self, space: Space) -> () {
            match space {
                Space::Normal => self.push_str(" "),
                Space::Nbsp => self.push_str("&nbsp;"),
                Space::None => (),
            }
        }

        fn render_word(&mut self, word: &str) -> () {
            self.push_str(word);
        }

        fn render_mark(&mut self, mark: &str) -> () {
            self.push_str(mark);
        }

        fn render_illformed(&mut self, err: &str) -> () {
            self.push_str(err);
        }

        fn emph_template<F>(&mut self, format: F) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<em>");
            format(self);
            self.push_str("</em>");
        }

        fn strong_emph_template<F>(&mut self, format: F) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<strong>");
            format(self);
            self.push_str("</strong>");
        }

        fn reply_template<F>(&mut self, reply: F, _author: &Option<&str>) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<span div=\"reply\">");
            reply(self);
            self.push_str("</span>");
        }

        fn thought_template<F>(&mut self, reply: F, author: &Option<&str>) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<span div=\"thought");
            author.map(|a| {
                self.push_str(" by-");
                self.push_str(a)
            });
            self.push_str("\">");
            reply(self);
            self.push_str("</span>");
        }

        fn dialogue_template<F>(&mut self, reply: F, author: &Option<&str>) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<span div=\"dialogue");
            author.map(|a| {
                self.push_str(" by-");
                self.push_str(a)
            });
            self.push_str("\">");
            reply(self);
            self.push_str("</span>");
        }

        fn between_dialogue(&mut self) -> () {
            self.push_str("</p><p>");
        }

        fn illformed_inline_template<F>(&mut self, err: F) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<span div=\"illformed_inline\">");
            err(self);
            self.push_str("</span>");
        }

        fn paragraph_template<F>(&mut self, para: F) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<p>");
            para(self);
            self.push_str("</p>");
        }

        fn illformed_block_template<F>(&mut self, err: F) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<div class=\"illformed_block\">");
            err(self);
            self.push_str("</div>");
        }

        fn story_template<F>(&mut self, story: F) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<div class\"story\">");
            story(self);
            self.push_str("</div>");
        }

        fn aside_template<F>(&mut self, cls: &Option<&str>, aside: F) -> ()
        where
            F: FnOnce(&mut Html) -> (),
        {
            self.push_str("<div class=\"aside");
            cls.map(|x| {
                self.push_str(" ");
                self.push_str(x);
            });
            self.push_str("\">");
            aside(self);
            self.push_str("</div>");
        }
    }

    #[test]
    fn test_generation() {
        let mut out = Html::empty(0);

        vec![Atom::Word("Bonjour"), Atom::Word("toi")].render_one_shot(&FRENCH, &mut out);
        assert_eq!(out.0, "Bonjour toi");

        out = Html::empty(0);

        Format::StrongEmph(vec![Format::Raw(vec![
            Atom::Word("Bonjour"),
            Atom::Word("toi"),
        ])])
        .render_one_shot(&FRENCH, &mut out);

        assert_eq!(out.0, "<strong>Bonjour toi</strong>");

        out = Html::empty(0);

        Format::Quote(vec![Format::Raw(vec![
            Atom::Word("Bonjour"),
            Atom::Word("toi"),
        ])])
        .render_one_shot(&FRENCH, &mut out);

        assert_eq!(out.0, "«&nbsp;Bonjour toi&nbsp;»");

        out = Html::empty(0);

        Paragraph(vec![
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Salut")])]),
                None,
            ),
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Bonjour")])]),
                None,
            ),
        ])
        .render_one_shot(&FRENCH, &mut out);

        assert_eq!(
            out.0,
            "<p><span div=\"dialogue\">«<span div=\"reply\">&nbsp;Salut</span></span></p><p><span div=\"dialogue\">—<span div=\"reply\"> Bonjour</span>&nbsp;»</span></p>"
        );

        out = Html::empty(0);

        Paragraph(vec![
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Salut")])]),
                Some("foo"),
            ),
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Bonjour")])]),
                Some("foo"),
            ),
        ])
        .render_one_shot(&FRENCH, &mut out);

        assert_eq!(
            out.0,
            "<p><span div=\"dialogue by-foo\">«<span div=\"reply\">&nbsp;Salut</span></span></p><p><span div=\"dialogue by-foo\">»<span div=\"reply\"> Bonjour</span>&nbsp;»</span></p>"
        );

        out = Html::empty(0);

        Paragraph(vec![
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Salut")])]),
                None,
            ),
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Bonjour")])]),
                Some("foo"),
            ),
        ])
        .render_one_shot(&FRENCH, &mut out);

        assert_eq!(
            out.0,
            "<p><span div=\"dialogue\">«<span div=\"reply\">&nbsp;Salut</span></span></p><p><span div=\"dialogue by-foo\">—<span div=\"reply\"> Bonjour</span>&nbsp;»</span></p>"
        );
    }
}
