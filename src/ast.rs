//  ogmarkup -- a markup language for story writers
//  Copyright (C) <year>  <name of author>
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
