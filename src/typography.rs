use ast::{Atom, Mark};

pub enum Space {
    Normal,
    Nbsp,
    None,
}

pub fn choose(s1: Space, s2: Space) -> Space {
    match (s1, s2) {
        (Space::None, _) => Space::None,
        (_, Space::None) => Space::None,
        (Space::Nbsp, _) => Space::Nbsp,
        (_, Space::Nbsp) => Space::Nbsp,
        _ => Space::Normal,
    }
}

pub enum PreviousDialogue {
    SameCharacter,
    OtherCharacter,
    NoDialogue,
}

pub fn previous_dialogue(before: Option<Option<&str>>, now: Option<&str>) -> PreviousDialogue {
    match (before, now) {
        (None, _) => PreviousDialogue::NoDialogue,
        (Some(None), _) => PreviousDialogue::OtherCharacter,
        (Some(Some(b)), n) => n
            .map(|n| b == n)
            .map_or(PreviousDialogue::OtherCharacter, |eq| {
                if eq {
                    PreviousDialogue::SameCharacter
                } else {
                    PreviousDialogue::OtherCharacter
                }
            }),
    }
}

pub trait Typography {
    fn decide(&self, &Mark) -> (Space, Space);
    fn output(&self, &Mark) -> &'static str;
    fn open_dialog(&self, PreviousDialogue) -> Option<&'static Atom<'static>>;
    fn close_dialog(&self, bool) -> Option<&'static Atom<'static>>;

    fn before_atom<'a>(&self, atom: &Atom<'a>) -> Space {
        match atom {
            Atom::Punctuation(ref p) => self.decide(p).0,
            Atom::Word(_) => Space::Normal,
            Atom::Void => Space::None,
        }
    }

    fn after_atom<'a>(&self, atom: &Atom<'a>) -> Space {
        match atom {
            Atom::Punctuation(ref p) => self.decide(p).1,
            Atom::Word(_) => Space::Normal,
            Atom::Void => Space::None,
        }
    }
}

pub struct French;

impl Typography for French {
    fn decide(&self, mark: &Mark) -> (Space, Space) {
        match mark {
            Mark::Semicolon => (Space::Nbsp, Space::Normal),
            Mark::Colon => (Space::Nbsp, Space::Normal),
            Mark::OpenQuote => (Space::Normal, Space::Nbsp),
            Mark::CloseQuote => (Space::Nbsp, Space::Normal),
            Mark::Question => (Space::Nbsp, Space::Normal),
            Mark::Exclamation => (Space::Nbsp, Space::Normal),
            Mark::LongDash => (Space::Normal, Space::Normal),
            Mark::Dash => (Space::None, Space::None),
            Mark::Hyphen => (Space::None, Space::None),
            Mark::Comma => (Space::None, Space::Normal),
            Mark::Point => (Space::None, Space::Normal),
            Mark::Apostrophe => (Space::None, Space::None),
            Mark::SuspensionPoints => (Space::None, Space::Normal),
        }
    }

    fn output(&self, mark: &Mark) -> &'static str {
        match mark {
            Mark::Semicolon => ";",
            Mark::Colon => ":",
            Mark::OpenQuote => "«",
            Mark::CloseQuote => "»",
            Mark::Question => "?",
            Mark::Exclamation => "!",
            Mark::LongDash => "—",
            Mark::Dash => "–",
            Mark::Hyphen => "-",
            Mark::Comma => ",",
            Mark::Point => ".",
            Mark::Apostrophe => "’",
            Mark::SuspensionPoints => "…",
        }
    }

    fn open_dialog(&self, before: PreviousDialogue) -> Option<&'static Atom<'static>> {
        match before {
            PreviousDialogue::NoDialogue => Some(&Atom::Punctuation(Mark::OpenQuote)),
            PreviousDialogue::OtherCharacter => Some(&Atom::Punctuation(Mark::LongDash)),
            PreviousDialogue::SameCharacter => Some(&Atom::Punctuation(Mark::CloseQuote)),
        }
    }

    fn close_dialog(&self, after: bool) -> Option<&'static Atom<'static>> {
        if after {
            None
        } else {
            Some(&Atom::Punctuation(Mark::CloseQuote))
        }
    }
}

pub const FRENCH: French = French;

pub struct English;

impl Typography for English {
    fn decide(&self, mark: &Mark) -> (Space, Space) {
        match mark {
            Mark::Semicolon => (Space::None, Space::Normal),
            Mark::Colon => (Space::None, Space::Normal),
            Mark::OpenQuote => (Space::Normal, Space::None),
            Mark::CloseQuote => (Space::None, Space::Normal),
            Mark::Question => (Space::None, Space::Normal),
            Mark::Exclamation => (Space::None, Space::Normal),
            Mark::LongDash => (Space::None, Space::None),
            Mark::Dash => (Space::None, Space::None),
            Mark::Hyphen => (Space::None, Space::None),
            Mark::Comma => (Space::None, Space::Normal),
            Mark::Point => (Space::None, Space::Normal),
            Mark::Apostrophe => (Space::None, Space::None),
            Mark::SuspensionPoints => (Space::None, Space::Normal),
        }
    }

    fn output(&self, mark: &Mark) -> &'static str {
        match mark {
            Mark::Semicolon => ";",
            Mark::Colon => ",",
            Mark::OpenQuote => "“",
            Mark::CloseQuote => "”",
            Mark::Question => "?",
            Mark::Exclamation => "!",
            Mark::LongDash => "—",
            Mark::Dash => "–",
            Mark::Hyphen => "-",
            Mark::Comma => ",",
            Mark::Point => ".",
            Mark::Apostrophe => "’",
            Mark::SuspensionPoints => "…",
        }
    }

    fn open_dialog(&self, before: PreviousDialogue) -> Option<&'static Atom<'static>> {
        match before {
            PreviousDialogue::NoDialogue => Some(&Atom::Punctuation(Mark::OpenQuote)),
            PreviousDialogue::OtherCharacter => Some(&Atom::Punctuation(Mark::OpenQuote)),
            PreviousDialogue::SameCharacter => None,
        }
    }

    fn close_dialog(&self, _after: bool) -> Option<&'static Atom<'static>> {
        Some(&Atom::Punctuation(Mark::CloseQuote))
    }
}

pub const ENGLISH: English = English;
