/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

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

use crate::ast::*;
pub use crate::generator::render;
use crate::generator::Output;
use crate::typography::Typography;

use nom::branch::alt;
use nom::bytes::streaming::{tag, take_till1, take_until, take_while};
use nom::character::streaming::{alphanumeric1, anychar, char, one_of};
use nom::combinator::{complete, cond, map_opt, not, opt, peek, rest, success};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded};
use nom::{Err, IResult, InputLength, Parser};

const BARRIER_TOKENS: &str = "!?.\"«»`+*[]<>|_'’,;-—: \n\r\t   ";

/// Call the parser if the condition is met, fails otherwise.
fn cond_reduce<I, O, E, F>(c: bool, p: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: Clone,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    map_opt(cond(c, p), |x| x)
}

/// Synonym to `many1(complete(i))`
fn some<I, O, E, F>(p: F) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: InputLength + Clone,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    many1(complete(p))
}

/// `consume_until!(x)` will consume the input according to the following rules:
/// 1. at least one character
/// 2.a. until it finds one character that belongs to $arr, or
/// 2.b. until it reaches the end of the input
fn consume_until<'a, E>(arr: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    move |i: &'a str| match take_till1(|x: char| arr.contains(x)).parse(i) {
        Err(Err::Incomplete(_)) => {
            if i.input_len() != 0 {
                rest(i)
            } else {
                Err(Err::Error(error_position!(i, ErrorKind::TakeUntil)))
            }
        }
        Ok((i, o)) => Ok((i, o)),
        Err(e) => Err(e),
    }
}

/// `recover_incomplete!(f)` will fail only if `f` fails due to something else
/// than an incomplete stream.
fn recover_incomplete<'a, O, E, F>(mut p: F) -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    move |i: &'a str| match p.parse(i) {
        Err(Err::Incomplete(_)) => Ok(("", ())),
        Ok((rest, _)) => Ok((rest, ())),
        Err(rest) => Err(rest),
    }
}

fn white_spaces<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str>,
{
    recover_incomplete(take_while(|c| "\r\t    ".contains(c)))
}

fn blank<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str>,
{
    move |i: &'a str| {
        let (i, _) = white_spaces()
            .and(opt(char('\n').and(white_spaces())))
            .parse(i)?;
        Ok((i, ()))
    }
}

#[test]
fn test_white_spaces() {
    assert_eq!(white_spaces::<()>().parse(","), Ok((",", ())));
    assert_eq!(white_spaces::<()>().parse("     ,"), Ok((",", ())));
    assert_eq!(white_spaces::<()>().parse("     "), Ok(("", ())));
}

fn atom<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, Atom<'a>, E>
where
    E: ParseError<&'a str>,
{
    move |i: &'a str| {
        let (i, _) = opt(char('\n').and(white_spaces())).parse(i)?;

        let (i, r) = alt((
            consume_until(BARRIER_TOKENS).map(Atom::Word),
            char(';').map(|_| Atom::Punctuation(Mark::Semicolon)),
            char(':').map(|_| Atom::Punctuation(Mark::Colon)),
            char('?').map(|_| Atom::Punctuation(Mark::Question)),
            char('!').map(|_| Atom::Punctuation(Mark::Exclamation)),
            tag("---").map(|_| Atom::Punctuation(Mark::LongDash)),
            char('—').map(|_| Atom::Punctuation(Mark::LongDash)),
            tag("--").map(|_| Atom::Punctuation(Mark::Dash)),
            char('–').map(|_| Atom::Punctuation(Mark::LongDash)),
            char(',').map(|_| Atom::Punctuation(Mark::Comma)),
            char('-').map(|_| Atom::Punctuation(Mark::Hyphen)),
            char('…').map(|_| Atom::Punctuation(Mark::SuspensionPoints)),
            preceded(char('.'), some(char('.'))).map(|_| Atom::Punctuation(Mark::SuspensionPoints)),
            char('.').map(|_| Atom::Punctuation(Mark::Point)),
            char('\'').map(|_| Atom::Punctuation(Mark::Apostrophe)),
            char('’').map(|_| Atom::Punctuation(Mark::Apostrophe)),
            delimited(char('`'), take_until("`"), char('`')).map(Atom::Word),
        ))
        .parse(i)?;

        let (i, _) = white_spaces().parse(i)?;

        Ok((i, r))
    }
}

#[test]
fn test_atom() {
    assert_eq!(
        atom::<()>().parse(","),
        Ok(("", Atom::Punctuation(Mark::Comma)))
    );
    assert_eq!(
        atom::<()>().parse(", "),
        Ok(("", Atom::Punctuation(Mark::Comma)))
    );
    assert_eq!(
        atom::<()>().parse("......."),
        Ok(("", Atom::Punctuation(Mark::SuspensionPoints)))
    );
    assert_eq!(
        atom::<()>().parse("’"),
        Ok(("", Atom::Punctuation(Mark::Apostrophe)))
    );
    assert_eq!(atom::<()>().parse("`@test`"), Ok(("", Atom::Word("@test"))));
    assert_eq!(atom::<()>().parse("test"), Ok(("", Atom::Word("test"))));
}

fn format_aux<'a, E>(
    in_strong: bool,
    in_emph: bool,
    in_quote: bool,
) -> impl FnMut(&'a str) -> IResult<&'a str, Format<'a>, E>
where
    E: ParseError<&'a str>,
{
    alt((
        some(atom()).map(Format::Raw),
        cond_reduce(!in_strong, move |i: &'a str| -> IResult<_, _, E> {
            let (i, _) = opt(char('\n').and(white_spaces())).parse(i)?;
            let (i, _) = char('+').parse(i)?;
            let (i, _) = white_spaces().parse(i)?;
            let (i, st) = some(format_aux(true, in_emph, in_quote)).parse(i)?;
            let (i, _) = blank().parse(i)?;
            let (i, _) = char('+').parse(i)?;
            let (i, _) = white_spaces().parse(i)?;

            Ok((i, Format::StrongEmph(st)))
        }),
        cond_reduce(!in_emph, move |i: &'a str| -> IResult<_, _, E> {
            let (i, _) = opt(char('\n').and(white_spaces())).parse(i)?;
            let (i, _) = char('*').parse(i)?;
            let (i, _) = white_spaces().parse(i)?;
            let (i, em) = some(format_aux(in_strong, true, in_quote)).parse(i)?;
            let (i, _) = blank().parse(i)?;
            let (i, _) = char('*').parse(i)?;
            let (i, _) = white_spaces().parse(i)?;

            Ok((i, Format::Emph(em)))
        }),
        cond_reduce(!in_quote, move |i: &'a str| -> IResult<_, _, E> {
            let (i, _) = opt(char('\n').and(white_spaces())).parse(i)?;
            let (i, _) = alt((char('"'), char('«'))).parse(i)?;
            let (i, _) = white_spaces().parse(i)?;
            let (i, em) = some(format_aux(in_strong, in_emph, in_quote)).parse(i)?;
            let (i, _) = blank().parse(i)?;
            let (i, _) = alt((char('"'), char('»'))).parse(i)?;
            let (i, _) = white_spaces().parse(i)?;

            Ok((i, Format::Quote(em)))
        }),
    ))
}

fn format<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, Format<'a>, E>
where
    E: ParseError<&'a str>,
{
    format_aux(false, false, false)
}

#[test]
fn test_format() {
    assert_eq!(
        format::<()>().parse("Hi stranger, how are you?"),
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
        format::<()>().parse(
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
        format::<()>().parse(
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
        format::<()>().parse("+Hi stranger+, how are you?"),
        Ok((
            ", how are you?",
            Format::StrongEmph(vec![Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger")
            ])])
        ))
    );

    assert_eq!(
        format::<()>().parse("+Hi *stranger*+, how are you?"),
        Ok((
            ", how are you?",
            Format::StrongEmph(vec![
                Format::Raw(vec![Atom::Word("Hi")]),
                Format::Emph(vec![Format::Raw(vec![Atom::Word("stranger")])])
            ])
        ))
    );

    assert_eq!(
        format::<()>()
            .parse("+Hi *+ stranger*, how are you?")
            .is_err(),
        true
    );
}

fn reply<'a, E>(b: char, e: char) -> impl FnMut(&'a str) -> IResult<&'a str, Reply<'a>, E>
where
    E: ParseError<&'a str>,
{
    move |i: &'a str| {
        let (i, _) = char(b).parse(i)?;
        let (i, _) = white_spaces().parse(i)?;

        let (i, before) = some(format()).parse(i)?;

        let (i, x) = anychar.parse(i)?;

        let (i, r) = alt((
            cond_reduce(x == e, move |i: &'a str| -> IResult<_, _, E> {
                Ok((i, None))
            }),
            cond_reduce(x == '|', move |i: &'a str| -> IResult<_, _, E> {
                let (i, prep) = delimited(
                    white_spaces(),
                    some(format()),
                    char('|').and(white_spaces()),
                )
                .parse(i)?;
                let (i, after) = opt(some(format())).parse(i)?;
                let (i, _) = char(e).parse(i)?;

                Ok((i, Some((prep, after))))
            }),
        ))
        .parse(i)?;

        let (i, _) = white_spaces().parse(i)?;

        match r {
            None => Ok((i, Reply::Simple(before))),
            Some((prep, after)) => Ok((i, Reply::WithSay(before, prep, after))),
        }
    }
}

#[test]
fn test_reply() {
    assert_eq!(
        reply::<()>('[', ']').parse("[Hi stranger]"),
        Ok((
            "",
            Reply::Simple(vec![Format::Raw(vec![
                Atom::Word("Hi"),
                Atom::Word("stranger"),
            ])])
        ))
    );

    assert_eq!(
        reply::<()>('[', ']').parse("[Hi stranger,| they salute.|]"),
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

fn reply_author<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    complete(delimited(
        char('('),
        alphanumeric1,
        char(')').and(white_spaces()),
    ))
}

// Because I have spent a fair amonut of time in a train trying to figure out
// the best way to implement this parser, I consider it is probably a good idea
// to explain why there is a call to `blank` for Dialogue and Thought, but not
// teller. The reason is actually quite simple: the `atom` parser is already
// eating the whitespaces before him, so if we do add blank to Teller as well,
// then this means two newlines are consumed when a Teller component follows a
// Dialogue for instance.
fn component<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, Component<'a>, E>
where
    E: ParseError<&'a str>,
{
    alt((
        some(format()).map(Component::Teller),
        preceded(blank(), reply('[', ']').and(opt(reply_author())))
            .map(|(dial, by)| Component::Dialogue(dial, by)),
        preceded(blank(), reply('<', '>').and(opt(reply_author())))
            .map(|(th, by)| Component::Thought(th, by)),
        consume_until("\n").map(Component::IllFormed),
    ))
}

#[test]
fn test_component() {
    assert_eq!(
        component::<()>().parse("[Hi]"),
        Ok((
            "",
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Hi")])]),
                None
            )
        ))
    );

    assert_eq!(
        component::<()>().parse("Hi stranger,\n*this* is me."),
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
        component::<()>().parse("Hi stranger, this is me."),
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
        component::<()>().parse("[Hi](alice)"),
        Ok((
            "",
            Component::Dialogue(
                Reply::Simple(vec![Format::Raw(vec![Atom::Word("Hi")])]),
                Some("alice")
            )
        ))
    );

    assert_eq!(
        component::<()>().parse("[Hi \ntest\n\n"),
        Ok(("\ntest\n\n", Component::IllFormed("[Hi ")))
    );
}

fn empty_line<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str>,
{
    move |i: &str| {
        let (i, _) = white_spaces().parse(i)?;
        let (i, _) = char('\n').parse(i)?;
        white_spaces().parse(i)
    }
}

fn paragraph<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, Paragraph<'a>, E>
where
    E: ParseError<&'a str>,
{
    delimited(
        not(peek(one_of("=_"))),
        some(component()).map(Paragraph),
        many0(complete(empty_line())),
    )
}

#[test]
fn test_paragraph() {
    assert_eq!(
        paragraph::<()>().parse("+Hi+"),
        Ok((
            "",
            Paragraph(vec![Component::Teller(vec![Format::StrongEmph(vec![
                Format::Raw(vec![Atom::Word("Hi")]),
            ])])])
        ))
    );

    assert_eq!(
        paragraph::<()>().parse("[Hi stranger, this is me.] Indeed.\n\n[Hi]"),
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

fn search_recovery_point_aux<'a, E>(acc: &mut Vec<&'a str>, i: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str>,
{
    let mut i_out = i;

    loop {
        match some::<_, _, E, _>(empty_line()).parse(i_out) {
            Ok((i, _)) => {
                i_out = i;
                break;
            }
            _ => {
                let (i, l) = consume_until("\n").parse(i_out)?;
                acc.push(l);
                let (i, _) = char('\n').parse(i)?;
                i_out = i;
            }
        }
    }

    Ok((i_out, ()))
}

fn search_recovery_point<'a, E>(i: &'a str) -> IResult<&'a str, Vec<&'a str>, E>
where
    E: ParseError<&'a str>,
{
    let mut acc = vec![];
    match search_recovery_point_aux::<E>(&mut acc, i) {
        Ok((i, _)) => Ok((i, acc)),
        Err(_) => Ok(("", acc)),
    }
}

#[test]
fn test_recovery() {
    assert_eq!(
        search_recovery_point::<()>.parse(
            r#"We need
to try.

Recover!"#
        ),
        Ok(("Recover!", vec!["We need", "to try."]))
    );
}

fn section<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, Section<'a>, E>
where
    E: ParseError<&'a str>,
{
    delimited(
        success(()),
        alt((
            complete(
                preceded(
                    some(char('_')),
                    opt(delimited(success(()), alphanumeric1, some(char('_')))),
                )
                .and(delimited(
                    some(empty_line()),
                    some(paragraph()),
                    some(char('_')),
                ))
                .map(|(cls, sec)| Section::Aside(cls, sec)),
            ),
            preceded(
                opt(some(char('=')).and(some(empty_line()))),
                some(paragraph()).map(Section::Story),
            ),
            map_opt(search_recovery_point, |x: Vec<_>| {
                if !x.is_empty() {
                    Some(Section::IllFormed(x))
                } else {
                    None
                }
            }),
        )),
        many0(complete(empty_line())),
    )
}

#[test]
fn test_section() {
    assert!(section::<()>().parse("").is_err());

    assert_eq!(
        section::<()>().parse("+\nHi  \n +"),
        Ok((
            "",
            Section::Story(vec![Paragraph(vec![Component::Teller(vec![
                Format::StrongEmph(vec![Format::Raw(vec![Atom::Word("Hi")])])
            ])])])
        ))
    );

    assert_eq!(
        section::<()>().parse("+Hi+"),
        Ok((
            "",
            Section::Story(vec![Paragraph(vec![Component::Teller(vec![
                Format::StrongEmph(vec![Format::Raw(vec![Atom::Word("Hi")])])
            ])])])
        ))
    );

    assert_eq!(
        section::<()>().parse(
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
        section::<()>().parse(
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
        section::<()>().parse(r#"Dear friend."#),
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

fn document<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, Document<'a>, E>
where
    E: ParseError<&'a str>,
{
    preceded(
        opt(complete(blank())).and(many0(complete(empty_line()))),
        many0(section()).map(Document),
    )
}

#[test]
fn test_empty_document() {
    assert_eq!(document::<()>().parse(""), Ok(("", Document(vec![]))));
}

#[test]
fn test_document_with_leading_ws() {
    assert_eq!(
        document::<()>().parse("   \n  \n She opened the letter."),
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
        document::<()>().parse("________________"),
        Ok((
            "",
            Document(vec![Section::IllFormed(vec!["________________"])])
        ))
    );
}

#[test]
fn test_document() {
    assert_eq!(
        document::<()>().parse(
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
        document::<()>().parse(
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
        document::<()>().parse(
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
    match document::<()>().parse(input) {
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
