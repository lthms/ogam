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
pub enum Atom<'a> {
    Word(&'a str),
    Punctuation(Mark),
    Void,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Format<'a> {
    Raw(Vec<Atom<'a>>),
    Emph(Vec<Format<'a>>),
    StrongEmph(Vec<Format<'a>>),
    Quote(Vec<Format<'a>>)
}

#[derive(PartialEq, Eq, Debug)]
pub enum Reply<'a> {
    Simple(Vec<Format<'a>>),
    WithSay(Vec<Format<'a>>, Vec<Format<'a>>, Option<Vec<Format<'a>>>)
}

#[derive(PartialEq, Eq, Debug)]
pub enum Component<'a> {
    Teller(Vec<Format<'a>>),
    Dialogue(Reply<'a>, Option<&'a str>),
    Thought(Reply<'a>, Option<&'a str>),
    IllFormed(&'a str)
}

#[derive(PartialEq, Eq, Debug)]
pub struct Paragraph<'a>(pub Vec<Component<'a>>);

#[derive(PartialEq, Eq, Debug)]
pub enum Section<'a> {
    Story(Vec<Paragraph<'a>>),
    Aside(Option<&'a str>, Vec<Paragraph<'a>>),
    IllFormed(Vec<&'a str>)
}

#[derive(PartialEq, Eq, Debug)]
pub struct Document<'a>(pub Vec<Section<'a>>);
