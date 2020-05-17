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

#![cfg_attr(feature = "html", feature(proc_macro_hygiene))]

#[macro_use]
extern crate nom;
#[cfg(feature = "html")]
extern crate maud;

pub mod ast;
pub mod generator;
#[cfg(feature = "html")]
pub mod html;
pub mod stats;
pub mod typography;

use ast::*;
pub use generator::render;
use generator::Output;
use typography::Typography;

use nom::character::streaming::{alphanumeric1, anychar};

const BARRIER_TOKENS: &str = "!?.\"«»`+*[]<>|_'’,;-—: \n\r\t   ";

macro_rules! cond_reduce (
    ($input:expr, $cond:expr, $sub:ident!( $($args:tt)* )) => (
        map_opt!($input, cond!($cond, $sub!($($args)*)), |x| x)
    );
);

/// Synonym to `many1!(complete!(x))`
macro_rules! some (
    ($input:expr, $sub:ident!( $($args:tt)* )) => (
        many1!($input, complete!($sub!($($args)*)))
    );
    ($input:expr, $f:expr) => (
        some!($input, call!($f))
    );
);

/// `consume_until!(x)` will consume the input according to the following rules:
/// 1. at least one character
/// 2.a. until it finds one character that belongs to $arr, or
/// 2.b. until it reaches the end of the input
macro_rules! consume_until (
    ($input:expr, $arr:expr) => (
        {
            use nom::Err;
            use nom::error::ErrorKind;

            match take_till1!($input, |c| $arr.contains(c)) {
                Err(Err::Incomplete(_)) => if $input.len() != 0 {
                    call!($input, nom::combinator::rest)
                } else {
                    Err(Err::Error(error_position!($input, ErrorKind::TakeUntil)))
                },
                Ok((i, o)) => Ok((i, o)),
                Err(e) => Err(e)
            }
        }
    );
);

/// `recover_incomplete!(f)` will fail only if `f` fails due to something else
/// than an incomplete stream.
macro_rules! recover_incomplete (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        {
            use nom::lib::std::result::Result::*;
            use nom::Err;

            match $submac!($i, $($args)*) {
                Err(Err::Incomplete(_)) =>  {
                    Ok(("", ()))
                },
                Ok((rest, _)) => {
                    Ok((rest, ()))
                },
                Err(rest) => Err(rest)
            }
        }
    );
);

named!(white_spaces<&str, ()>,
       recover_incomplete!(take_while!(|c| "\r\t    ".contains(c)))
);

named!(blank<&str, ()>,
       do_parse!(
           white_spaces >>
           opt!(do_parse!(char!('\n') >> white_spaces >> (()))) >>
           (())
       )
);

#[test]
fn test_white_spaces() {
    assert_eq!(white_spaces(","), Ok((",", ())));
    assert_eq!(white_spaces("     ,"), Ok((",", ())));
    assert_eq!(white_spaces("     "), Ok(("", ())));
}

named!(atom<&str, Atom>, do_parse!(
    opt!(do_parse!(char!('\n') >> white_spaces >> (()))) >>
    r: alt!( map!(consume_until!(BARRIER_TOKENS), Atom::Word)
           | do_parse!(char!(';')  >> (Atom::Punctuation(Mark::Semicolon)))
           | do_parse!(char!(':')  >> (Atom::Punctuation(Mark::Colon)))
           | do_parse!(char!('?')  >> (Atom::Punctuation(Mark::Question)))
           | do_parse!(char!('!')  >> (Atom::Punctuation(Mark::Exclamation)))
           | do_parse!(tag!("---") >> (Atom::Punctuation(Mark::LongDash)))
           | do_parse!(char!('—')  >> (Atom::Punctuation(Mark::LongDash)))
           | do_parse!(tag!("--")  >> (Atom::Punctuation(Mark::Dash)))
           | do_parse!(char!('–')  >> (Atom::Punctuation(Mark::LongDash)))
           | do_parse!(char!(',')  >> (Atom::Punctuation(Mark::Comma)))
           | do_parse!(char!('-')  >> (Atom::Punctuation(Mark::Hyphen)))
           | do_parse!(char!('…')  >> (Atom::Punctuation(Mark::SuspensionPoints)))
           | do_parse!(char!('.')  >> some!(char!('.'))
                                   >> (Atom::Punctuation(Mark::SuspensionPoints)))
           | do_parse!(char!('.')  >> (Atom::Punctuation(Mark::Point)))
           | do_parse!(char!('\'') >> (Atom::Punctuation(Mark::Apostrophe)))
           | do_parse!(char!('’')  >> (Atom::Punctuation(Mark::Apostrophe)))
           | do_parse!(char!('`')  >> lw: take_until!("`")
                                   >> char!('`')
                                   >> (Atom::Word(lw)))
           )            >>
    white_spaces >>
    (r)
));

#[test]
fn test_atom() {
    assert_eq!(atom(","), Ok(("", Atom::Punctuation(Mark::Comma))));
    assert_eq!(atom(", "), Ok(("", Atom::Punctuation(Mark::Comma))));
    assert_eq!(
        atom("......."),
        Ok(("", Atom::Punctuation(Mark::SuspensionPoints)))
    );
    assert_eq!(atom("’"), Ok(("", Atom::Punctuation(Mark::Apostrophe))));
    assert_eq!(atom("`@test`"), Ok(("", Atom::Word("@test"))));
    assert_eq!(atom("test"), Ok(("", Atom::Word("test"))));
}

named_args!(format_rec(in_strong: bool, in_emph: bool, in_quote: bool)<&str, Format>,
    alt!( map!(some!(atom), Format::Raw)
        | cond_reduce!(!in_strong, do_parse!(
            opt!(do_parse!(char!('\n') >> white_spaces >> (()))) >>
            char!('+') >>
            white_spaces >>
            st: some!(call!(format_rec, true, in_emph, in_quote)) >>
            blank >>
            char!('+') >>
            white_spaces >>
            (Format::StrongEmph(st))
          ))
        | cond_reduce!(!in_emph, do_parse!(
            opt!(do_parse!(char!('\n') >> white_spaces >> (()))) >>
            char!('*') >>
            white_spaces >>
            st: some!(call!(format_rec, in_strong, true, in_quote)) >>
            blank >>
            char!('*') >>
            white_spaces >>
            (Format::Emph(st))
          ))
        | cond_reduce!(!in_quote, do_parse!(
            opt!(do_parse!(char!('\n') >> white_spaces >> (()))) >>
            alt!(char!('"') | char!('«')) >>
            white_spaces >>
            st: some!(call!(format_rec, in_strong, in_emph, true)) >>
            white_spaces >>
            alt!(char!('"') | char!('»')) >>
            white_spaces >>
            (Format::Quote(st))
          ))
    )
);

named!(format<&str, Format>, call!(format_rec, false, false, false));

#[test]
fn test_format() {
    assert_eq!(
        format("Hi stranger, how are you?"),
        Ok((
            "",
            Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger"),
                Atom::Punctuation(Mark::Comma),
                Atom::Word("how"),
                Atom::Word("are"),
                Atom::Word("you"),
                Atom::Punctuation(Mark::Question),
            ])
        ))
    );

    assert_eq!(
        format(
            r#"Hi stranger, how
are you?"#
        ),
        Ok((
            "",
            Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger"),
                Atom::Punctuation(Mark::Comma),
                Atom::Word("how"),
                Atom::Word("are"),
                Atom::Word("you"),
                Atom::Punctuation(Mark::Question),
            ])
        ))
    );

    assert_eq!(
        format(
            r#"Hi stranger, how

are you?"#
        ),
        Ok((
            "\n\nare you?",
            Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger"),
                Atom::Punctuation(Mark::Comma),
                Atom::Word("how")
            ])
        ))
    );

    assert_eq!(
        format("+Hi stranger+, how are you?"),
        Ok((
            ", how are you?",
            Format::StrongEmph(vec![Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger")
            ])])
        ))
    );

    assert_eq!(
        format("+Hi *stranger*+, how are you?"),
        Ok((
            ", how are you?",
            Format::StrongEmph(vec![
                Format::Raw(vec![Atom::Word("Hi")]),
                Format::Emph(vec![Format::Raw(vec![Atom::Word("stranger")])])
            ])
        ))
    );

    assert_eq!(format("+Hi *+ stranger*, how are you?").is_err(), true);
}

named_args!(reply(b: char, e: char)<&str, Reply>, do_parse!(
    char!(b)  >>
    call!(white_spaces) >>
    before: some!(format) >>
    x: call!(anychar) >>
    r: alt!( cond_reduce!(x == e, do_parse!((None)))
           | cond_reduce!(x == '|', do_parse!(
               call!(white_spaces) >>
               prep: some!(format) >>
               char!('|') >>
               call!(white_spaces) >>
               after: opt!(some!(format)) >>
               char!(e) >>
               (Some((prep, after)))
           ))
    ) >>
    call!(white_spaces) >>
    (match r {
        None => {
            Reply::Simple(before)
        },
        Some((prep, after)) => {
            Reply::WithSay(before, prep, after)
        }
    })
));

#[test]
fn test_reply() {
    assert_eq!(
        reply("[Hi stranger]", '[', ']'),
        Ok((
            "",
            Reply::Simple(vec![Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger"),
            ])])
        ))
    );

    assert_eq!(
        reply("[Hi stranger,| they salute.|]", '[', ']'),
        Ok((
            "",
            Reply::WithSay(
                vec![Format::Raw(vec![
                    Atom::Word("Hi"),
                    Atom::Word("stranger"),
                    Atom::Punctuation(Mark::Comma)
                ])],
                vec![Format::Raw(vec![
                    Atom::Word("they"),
                    Atom::Word("salute"),
                    Atom::Punctuation(Mark::Point)
                ])],
                None
            )
        ))
    );
}

// Because I have spent a fair amonut of time in a train trying to figure out
// the best way to implement this parser, I consider it is probably a good idea
// to explain why there is a call to `blank` for Dialogue and Thought, but not
// teller. The reason is actually quite simple: the `atom` parser is already
// eating the whitespaces before him, so if we do add blank to Teller as well,
// then this means two newlines are consumed when a Teller component follows a
// Dialogue for instance.
named!(component<&str, Component>, alt! (
            do_parse!(
              tel: some!(format) >>
              (Component::Teller(tel)))
         |  do_parse!(
              blank >>
              dial: call!(reply, '[' , ']') >>
              by: opt!(complete!(do_parse!(
                     char!('(') >>
                     name: call!(alphanumeric1) >>
                     char!(')') >>
                     white_spaces >>
                     (name)))) >>
              (Component::Dialogue(dial, by)))
         |  do_parse!(
              blank >>
              th: call!(reply, '<' , '>') >>
              by: opt!(complete!(do_parse!(
                     char!('(') >>
                     name: call!(alphanumeric1) >>
                     char!(')') >>
                     white_spaces >>
                     (name)))) >>
              (Component::Thought(th, by)))
         | map!(consume_until!("\n"), Component::IllFormed)
         )
);

#[test]
fn test_component() {
    assert_eq!(
        component("[Hi]"),
        Ok((
            "",
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Hi")])]),
                None
            )
        ))
    );

    assert_eq!(
        component("Hi stranger,\n*this* is me."),
        Ok((
            "",
            Component::Teller(vec![
                Format::Raw(vec![
                    Atom::Word("Hi"),
                    Atom::Word("stranger"),
                    Atom::Punctuation(Mark::Comma),
                ]),
                Format::Emph(vec![Format::Raw(vec![Atom::Word("this"),])]),
                Format::Raw(vec![
                    Atom::Word("is"),
                    Atom::Word("me"),
                    Atom::Punctuation(Mark::Point)
                ])
            ])
        ))
    );

    assert_eq!(
        component("Hi stranger, this is me."),
        Ok((
            "",
            Component::Teller(vec![Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger"),
                Atom::Punctuation(Mark::Comma),
                Atom::Word("this"),
                Atom::Word("is"),
                Atom::Word("me"),
                Atom::Punctuation(Mark::Point)
            ])])
        ))
    );

    assert_eq!(
        component("[Hi](alice)"),
        Ok((
            "",
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Hi")])]),
                Some("alice")
            )
        ))
    );

    assert_eq!(
        component("[Hi \ntest\n\n"),
        Ok(("\ntest\n\n", Component::IllFormed("[Hi ")))
    );
}

named!(empty_line<&str, ()>, do_parse!(
       white_spaces >>
       char!('\n')  >>
       white_spaces >>
       (())
));

named!(
    paragraph<&str, Paragraph>,
    do_parse!(
        not!(peek!(one_of!("_="))) >>
        p: some!(component) >>
        many0!(complete!(empty_line)) >>
        (Paragraph(p))
    )
);

#[test]
fn test_paragraph() {
    assert_eq!(
        paragraph("+Hi+"),
        Ok((
            "",
            Paragraph(vec![Component::Teller(vec![Format::StrongEmph(vec![
                Format::Raw(vec![Atom::Word("Hi")]),
            ])])])
        ))
    );

    assert_eq!(
        paragraph("[Hi stranger, this is me.] Indeed.\n\n[Hi]"),
        Ok((
            "[Hi]",
            Paragraph(vec![
                Component::Dialogue(
                    Reply::Simple(vec![Format::Raw(vec![
                        Atom::Word("Hi"),
                        Atom::Word("stranger"),
                        Atom::Punctuation(Mark::Comma),
                        Atom::Word("this"),
                        Atom::Word("is"),
                        Atom::Word("me"),
                        Atom::Punctuation(Mark::Point)
                    ])]),
                    None
                ),
                Component::Teller(vec![Format::Raw(vec![
                    Atom::Word("Indeed"),
                    Atom::Punctuation(Mark::Point)
                ])])
            ])
        ))
    );
}

named_args!(
    search_recovery_point_rec<'a>(acc: &mut Vec<&'a str>)<&'a str, ()>,
    alt!(
        map!(some!(empty_line), |_| ())
      | do_parse!(
          l: consume_until!("\n") >>
          do_parse!(({ acc.push(l) })) >>
          char!('\n') >>
          call!(search_recovery_point_rec, acc) >>
          (())
      )
    )
);

fn search_recovery_point<'input>(
    input: &'input str,
) -> nom::IResult<&'input str, Vec<&'input str>> {
    let mut acc = vec![];
    match search_recovery_point_rec(input, &mut acc) {
        Ok((input, _)) => Ok((input, acc)),
        Err(_) => Ok(("", acc)),
    }
}

#[test]
fn test_recovery() {
    assert_eq!(
        search_recovery_point(
            r#"We need
to try.

Recover!"#
        ),
        Ok(("Recover!", vec!["We need", "to try."]))
    );
}

named!(
    section<&str, Section>, do_parse!(
    res: alt!(
        complete!(do_parse!(
            some!(char!('_')) >>
            cls: opt!(
                    do_parse!(
                        cls: call!(alphanumeric1) >>
                        some!(char!('_')) >>
                        (cls)
                    )
            ) >>
            some!(empty_line) >>
            sec: some!(paragraph) >>
            some!(char!('_')) >>
            (Section::Aside(cls, sec))
        ))
      | do_parse!(
          opt!(do_parse!(some!(char!('=')) >> some!(empty_line) >> (()))) >>
          r: map!(some!(paragraph), Section::Story) >>
          (r)
        )
      | map_opt!(search_recovery_point, |x: Vec<_>| if !x.is_empty() { Some(Section::IllFormed(x)) } else { None } )
    ) >>
    many0!(complete!(empty_line)) >>
    (res)
));

#[test]
fn test_section() {
    assert!(section("").is_err());

    assert_eq!(
        section("+\nHi  \n +"),
        Ok((
            "",
            Section::Story(vec![Paragraph(vec![Component::Teller(vec![
                Format::StrongEmph(vec![Format::Raw(vec![Atom::Word("Hi")])])
            ])])])
        ))
    );

    assert_eq!(
        section("+Hi+"),
        Ok((
            "",
            Section::Story(vec![Paragraph(vec![Component::Teller(vec![
                Format::StrongEmph(vec![Format::Raw(vec![Atom::Word("Hi")])])
            ])])])
        ))
    );

    assert_eq!(
        section(
            r#"_____letter____
Dear friend.

I love you.
_______________"#
        ),
        Ok((
            "",
            Section::Aside(
                Some("letter"),
                vec![
                    Paragraph(vec![Component::Teller(vec![Format::Raw(vec![
                        Atom::Word("Dear"),
                        Atom::Word("friend"),
                        Atom::Punctuation(Mark::Point)
                    ])])]),
                    Paragraph(vec![Component::Teller(vec![Format::Raw(vec![
                        Atom::Word("I"),
                        Atom::Word("love"),
                        Atom::Word("you"),
                        Atom::Punctuation(Mark::Point)
                    ])])])
                ]
            )
        ))
    );

    assert_eq!(
        section(
            r#"_____letter____
Dear friend.

I love you."#
        ),
        Ok((
            "I love you.",
            Section::IllFormed(vec!["_____letter____", "Dear friend."])
        ))
    );

    assert_eq!(
        section(r#"Dear friend."#),
        Ok((
            "",
            Section::Story(vec![Paragraph(vec![Component::Teller(vec![Format::Raw(
                vec![
                    Atom::Word("Dear"),
                    Atom::Word("friend"),
                    Atom::Punctuation(Mark::Point)
                ]
            )])])])
        ))
    );
}

named!(
    document<&str, Document>, do_parse!(
      opt!(complete!(blank)) >>
      many0!(complete!(empty_line)) >>
      x: many0!(section) >>
      (Document(x))
    )
);

#[test]
fn test_empty_document() {
    assert_eq!(document(""), Ok(("", Document(vec![]))));
}

#[test]
fn test_document_with_leading_ws() {
    assert_eq!(
        document("   \n  \n She opened the letter."),
        Ok((
            "",
            Document(vec![Section::Story(vec![Paragraph(vec![
                Component::Teller(vec![Format::Raw(vec![
                    Atom::Word("She"),
                    Atom::Word("opened"),
                    Atom::Word("the"),
                    Atom::Word("letter"),
                    Atom::Punctuation(Mark::Point),
                ])])
            ])])])
        ))
    );
}

#[test]
fn test_incomplete_aside() {
    assert_eq!(
        document("________________"),
        Ok((
            "",
            Document(vec![Section::IllFormed(vec!["________________"])])
        ))
    );
}

#[test]
fn test_document() {
    assert_eq!(
        document(
            r#"She opened the letter.

======

She cry."#
        ),
        Ok((
            "",
            Document(vec![
                Section::Story(vec![Paragraph(vec![Component::Teller(vec![Format::Raw(
                    vec![
                        Atom::Word("She"),
                        Atom::Word("opened"),
                        Atom::Word("the"),
                        Atom::Word("letter"),
                        Atom::Punctuation(Mark::Point),
                    ]
                )])])]),
                Section::Story(vec![Paragraph(vec![Component::Teller(vec![Format::Raw(
                    vec![
                        Atom::Word("She"),
                        Atom::Word("cry"),
                        Atom::Punctuation(Mark::Point),
                    ]
                )])])]),
            ])
        ))
    );
    assert_eq!(
        document(
            r#"She opened the letter.

======She cry."#
        ),
        Ok((
            "",
            Document(vec![
                Section::Story(vec![Paragraph(vec![Component::Teller(vec![Format::Raw(
                    vec![
                        Atom::Word("She"),
                        Atom::Word("opened"),
                        Atom::Word("the"),
                        Atom::Word("letter"),
                        Atom::Punctuation(Mark::Point),
                    ]
                )])])]),
                Section::IllFormed(vec!["======She cry."])
            ])
        ))
    );

    assert_eq!(
        document(
            r#"She opened the letter, and read it.

_____letter____
Dear friend.

I love you.
_______________"#
        ),
        Ok((
            "",
            Document(vec![
                Section::Story(vec![Paragraph(vec![Component::Teller(vec![Format::Raw(
                    vec![
                        Atom::Word("She"),
                        Atom::Word("opened"),
                        Atom::Word("the"),
                        Atom::Word("letter"),
                        Atom::Punctuation(Mark::Comma),
                        Atom::Word("and"),
                        Atom::Word("read"),
                        Atom::Word("it"),
                        Atom::Punctuation(Mark::Point)
                    ]
                )])])]),
                Section::Aside(
                    Some("letter"),
                    vec![
                        Paragraph(vec![Component::Teller(vec![Format::Raw(vec![
                            Atom::Word("Dear"),
                            Atom::Word("friend"),
                            Atom::Punctuation(Mark::Point)
                        ])])]),
                        Paragraph(vec![Component::Teller(vec![Format::Raw(vec![
                            Atom::Word("I"),
                            Atom::Word("love"),
                            Atom::Word("you"),
                            Atom::Punctuation(Mark::Point)
                        ])])])
                    ]
                )
            ])
        ))
    );
}

#[derive(PartialEq, Eq, Debug)]
pub enum Error<'input> {
    IncompleteParsing(Document<'input>, &'input str),
    ParsingError,
}

pub fn parse(input: &str) -> Result<Document, Error> {
    match document(input) {
        Ok(("", res)) => Ok(res),
        Ok((rest, res)) => Err(Error::IncompleteParsing(res, rest)),
        _ => Err(Error::ParsingError),
    }
}

pub fn compile<'input, O: Output, T: Typography + ?Sized>(
    input: &'input str,
    typo: &T,
) -> Result<O, Error<'input>> {
    let mut out = O::empty(input.len());

    render(&parse(input)?, typo, &mut out);

    Ok(out)
}

#[test]
fn test_render() {
    use stats::Digest;
    use typography::ENGLISH;

    let res: Digest = compile(r#"Hi everyone."#, &ENGLISH).unwrap();

    assert_eq!(res.words_count, 2);

    let res: Digest = compile(r#"Hi everyone. +My name is.. Suly+."#, &ENGLISH).unwrap();

    assert_eq!(res.signs_count, 3);

    let res: Digest = compile(
        r#"Hi everyone.

 +My name is.. Suly+.

____test____

What is your name?
____________"#,
        &ENGLISH,
    )
    .unwrap();

    assert_eq!(res.spaces_count, 7);

    let res: Digest = compile(
        r#"Hi everyone.

[+My name is.. Suly+.](john)

[Really?](merida)

[Yay](john)

____test____

What is your name?
____________"#,
        &ENGLISH,
    )
    .unwrap();

    assert_eq!(
        res.characters,
        [String::from("john"), String::from("merida")]
            .iter()
            .cloned()
            .collect()
    );
}
