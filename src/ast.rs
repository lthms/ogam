/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[derive(PartialEq, Eq, Debug)]
pub enum Mark {
    Semicolon,
    Colon,
    Question,
    Exclamation,
    Dash,
    LongDash,
    Comma,
    Point,
    Hyphen,
    SuspensionPoints,
    Apostrophe,
    OpenQuote,
    CloseQuote,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Atom<'input> {
    Word(&'input str),
    Punctuation(Mark),
    Void,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Format<'input> {
    Raw(Vec<Atom<'input>>),
    Emph(Vec<Format<'input>>),
    StrongEmph(Vec<Format<'input>>),
    Quote(Vec<Format<'input>>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Reply<'input> {
    Simple(Vec<Format<'input>>),
    WithSay(
        Vec<Format<'input>>,
        Vec<Format<'input>>,
        Option<Vec<Format<'input>>>,
    ),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Component<'input> {
    Teller(Vec<Format<'input>>),
    Dialogue(Reply<'input>, Option<&'input str>),
    Thought(Reply<'input>, Option<&'input str>),
    IllFormed(&'input str),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Paragraph<'input>(pub Vec<Component<'input>>);

#[derive(PartialEq, Eq, Debug)]
pub enum Section<'input> {
    Story(Vec<Paragraph<'input>>),
    Aside(Option<&'input str>, Vec<Paragraph<'input>>),
    IllFormed(Vec<&'input str>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Document<'input>(pub Vec<Section<'input>>);
