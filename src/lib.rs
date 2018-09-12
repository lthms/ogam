#[macro_use] extern crate nom;

pub mod ast;
pub mod typography;
pub mod generator;

use ast::*;
use typography::Typography;
use generator::{Renderer, render_document};

const BARRIER_TOKENS: &str = "!?.\"«»`+*[]<>|_'’,;-—: \n\r\t  ";

macro_rules! some (
    ($input:expr, $sub:ident!( $($args:tt)* )) => (
        many1!($input, complete!($sub!($($args)*)))
    );
    ($input:expr, $f:expr) => (
        some!($input, call!($f))
    );
);

macro_rules! consume_until (
  ($input:expr, $arr:expr) => (
    {
      use nom::lib::std::result::Result::*;
      use nom::lib::std::option::Option::*;
      use nom::{Err,IResult,ErrorKind};

      use nom::InputIter;
      use nom::InputTake;
      use nom::FindToken;

      let res: IResult<_,_> = match $input.position(|c| {
        $arr.find_token(c)
      }) {
        Some(0) => Err(Err::Error(error_position!($input, ErrorKind::TakeUntilEither::<u32>))),
        Some(n) => {
          Ok($input.take_split(n))
        },
        None    => {
          Ok($input.take_split($input.len()))
        }
      };
      res
    }
  );
);

macro_rules! recover_incomplete (
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        {
            use nom::lib::std::result::Result::*;
            use nom::Err;

            let i_ = $i.clone();
            match $submac!(i_, $($args)*) {
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
    ($i:expr, $f:expr) => (
        complete!($i, call!($f));
    );
);

named!(white_spaces<&str, ()>,
       recover_incomplete!(take_while!(|c| "\r\t   ".contains(c)))
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
    assert_eq!(atom("......."), Ok(("", Atom::Punctuation(Mark::SuspensionPoints))));
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
        Ok(
            (
                "",
                Format::Raw(
                    vec![
                        Atom::Word("Hi"),
                        Atom::Word("stranger"),
                        Atom::Punctuation(Mark::Comma),
                        Atom::Word("how"),
                        Atom::Word("are"),
                        Atom::Word("you"),
                        Atom::Punctuation(Mark::Question),
                    ]
                )
            )
        )
    );

    assert_eq!(
        format(r#"Hi stranger, how
are you?"#),
        Ok(
            (
                "",
                Format::Raw(
                    vec![
                        Atom::Word("Hi"),
                        Atom::Word("stranger"),
                        Atom::Punctuation(Mark::Comma),
                        Atom::Word("how"),
                        Atom::Word("are"),
                        Atom::Word("you"),
                        Atom::Punctuation(Mark::Question),
                    ]
                )
            )
        )
    );

    assert_eq!(
        format(r#"Hi stranger, how

are you?"#),
        Ok(
            (
                "\n\nare you?",
                Format::Raw(
                    vec![
                        Atom::Word("Hi"),
                        Atom::Word("stranger"),
                        Atom::Punctuation(Mark::Comma),
                        Atom::Word("how")
                    ]
                )
            )
        )
    );

    assert_eq!(
        format("+Hi stranger+, how are you?"),
        Ok(
            (
                ", how are you?",
                Format::StrongEmph(
                    vec![
                        Format::Raw(
                            vec![
                                Atom::Word("Hi"),
                                Atom::Word("stranger")
                            ]
                        )
                    ]
                )
            )
        )
    );

    assert_eq!(
        format("+Hi *stranger*+, how are you?"),
        Ok(
            (
                ", how are you?",
                Format::StrongEmph(
                    vec![
                        Format::Raw(vec![Atom::Word("Hi")]),
                        Format::Emph(
                            vec![
                                Format::Raw(vec![Atom::Word("stranger")])
                            ]
                        )
                    ]
                )
            )
        )
    );

    assert_eq!(format("+Hi *+ stranger*, how are you?").is_err(), true);
}

named_args!(reply(b: char, e: char)<&str, Reply>, do_parse!(
    char!(b)  >>
    call!(white_spaces) >>
    before: some!(format) >>
    x: call!(nom::anychar) >>
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
        Ok(
            (
                "",
                Reply::Simple(
                    vec![
                        Format::Raw(
                            vec![
                                Atom::Word("Hi"),
                                Atom::Word("stranger"),

                            ]
                        )
                    ]
                )
            )
        )
    );

    assert_eq!(
        reply("[Hi stranger,| they salute.|]", '[', ']'),
        Ok(
            (
                "",
                Reply::WithSay(
                    vec![
                        Format::Raw(
                            vec![
                                Atom::Word("Hi"),
                                Atom::Word("stranger"),
                                Atom::Punctuation(Mark::Comma)
                            ]
                        )
                    ],
                    vec![
                        Format::Raw(
                            vec![
                                Atom::Word("they"),
                                Atom::Word("salute"),
                                Atom::Punctuation(Mark::Point)
                            ]
                        )
                    ],
                    None
                )
            )
        )
    );
}

// Because I have spent a fair amonut of time in a train trying to figure out
// the best way to implement this parser, I consider it is probably a good idea
// to explain why there is a call to `blank` for Dialogue and Thought, but not
// teller. The reason is actually quite simple: the `atom` parser is already
// eating the whitespaces before him, so if we do add blank to Teller as well,
// then this means two newlines are consumed when a Teller component follows a
// Dialogue for instance.
named!(component<&str, Component>, alt_complete! (
            do_parse!(
              tel: some!(format) >>
              (Component::Teller(tel)))
         |  do_parse!(
              blank >>
              dial: call!(reply, '[' , ']') >>
              by: opt!(complete!(do_parse!(
                     char!('(') >>
                     name: call!(nom::alphanumeric1) >>
                     char!(')') >>
                     white_spaces >>
                     (name)))) >>
              (Component::Dialogue(dial, by)))
         |  do_parse!(
              blank >>
              th: call!(reply, '<' , '>') >>
              by: opt!(complete!(do_parse!(
                     char!('(') >>
                     name: call!(nom::alphanumeric1) >>
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
        Ok(
            (
                "",
                Component::Dialogue(
                    Reply::Simple(
                        vec![
                            Format::Raw(
                                vec![
                                    Atom::Word("Hi")
                                ]
                            )
                        ]
                    ),
                    None
                )
            )
        )
    );

    assert_eq!(
        component("Hi stranger,\n*this* is me."),
        Ok(
            (
                "",
                Component::Teller(
                    vec![
                        Format::Raw(
                            vec![
                                Atom::Word("Hi"),
                                Atom::Word("stranger"),
                                Atom::Punctuation(Mark::Comma),
                            ]
                        ),
                        Format::Emph(
                            vec![
                                Format::Raw(
                                    vec![
                                        Atom::Word("this"),
                                    ]
                                )
                            ]
                        ),
                        Format::Raw(
                            vec![
                                Atom::Word("is"),
                                Atom::Word("me"),
                                Atom::Punctuation(Mark::Point)
                            ]
                        )
                    ]
                )
            )
        )
    );

    assert_eq!(
        component("Hi stranger, this is me."),
        Ok(
            (
                "",
                Component::Teller(
                    vec![
                        Format::Raw(
                            vec![
                                Atom::Word("Hi"),
                                Atom::Word("stranger"),
                                Atom::Punctuation(Mark::Comma),
                                Atom::Word("this"),
                                Atom::Word("is"),
                                Atom::Word("me"),
                                Atom::Punctuation(Mark::Point)
                            ]
                        )
                    ]
                )
            )
        )
    );

    assert_eq!(
        component("[Hi](alice)"),
        Ok(
            (
                "",
                Component::Dialogue(
                    Reply::Simple(
                        vec![
                            Format::Raw(vec![Atom::Word("Hi")])
                        ]
                    ),
                    Some("alice")
                )
            )
        )
    );

    assert_eq!(component("[Hi \ntest"), Ok(("\ntest",Component::IllFormed("[Hi "))));
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
        fst: peek!(nom::anychar) >>
        p: cond_reduce!(
            fst != '_',
            some!(component)
        ) >>
        many0!(complete!(empty_line)) >>
        (Paragraph(p))
    )
);

#[test]
fn test_paragraph() {
    assert_eq!(
        paragraph("[Hi stranger, this is me.] Indeed.\n\n[Hi]"),
        Ok(
            (
                "[Hi]",
                Paragraph(
                    vec![
                        Component::Dialogue(
                            Reply::Simple(
                                vec![
                                    Format::Raw(
                                        vec![
                                            Atom::Word("Hi"),
                                            Atom::Word("stranger"),
                                            Atom::Punctuation(Mark::Comma),
                                            Atom::Word("this"),
                                            Atom::Word("is"),
                                            Atom::Word("me"),
                                            Atom::Punctuation(Mark::Point)
                                        ]
                                    )
                                ]
                            ),
                            None
                        ),
                        Component::Teller(
                            vec![
                                Format::Raw(
                                    vec![
                                        Atom::Word("Indeed"),
                                        Atom::Punctuation(Mark::Point)
                                    ]
                                )
                            ]
                        )
                    ]
                )
            )
        )
    );
}

named_args!(
    search_recovery_point_rec<'a>(acc: &mut Vec<&'a str>)<&'a str, ()>,
    alt!(
        do_parse!(
            alt!(map!(some!(empty_line), |_| ()) | map!(eof!(), |_| ())) >>
            (())
        )
      | do_parse!(
          map!(consume_until!("\n"), |l| acc.push(l)) >>
          char!('\n') >>
          call!(search_recovery_point_rec, acc) >>
          (())
      )
    )
);

fn search_recovery_point<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<&'a str>> {
    let mut acc = vec![];
    match search_recovery_point_rec(input, &mut acc) {
        Ok((input, _)) => {
            Ok((input, acc))
        },
        Err(rest) => {
            Err(rest)
        }
    }
}

#[test]
fn test_recovery() {
    assert_eq!(
        search_recovery_point(r#"We need
to try.

Recover!"#),
        Ok(
            (
                "Recover!",
                vec!["We need", "to try."]
            )
        )
    );
}

named!(
    section<&str, Section>, do_parse!(
    res: alt!(
        do_parse!(
            some!(char!('_')) >>
            cls: opt!(
                    do_parse!(
                        cls: call!(nom::alphanumeric1) >>
                        some!(char!('_')) >>
                        (cls)
                    )
            ) >>
            some!(empty_line) >>
            sec: some!(paragraph) >>
            some!(char!('_')) >>
            (Section::Aside(cls, sec))
        )
      | map!(some!(paragraph), Section::Story)
      | map!(search_recovery_point, Section::IllFormed)
    ) >>
    many0!(complete!(empty_line)) >>
    (res)
));

#[test]
fn test_section() {
    assert_eq!(
        section("+\nHi  \n +"),
        Ok((
            "",
            Section::Story(
                vec![
                    Paragraph(
                        vec![
                            Component::Teller(
                                vec![
                                    Format::StrongEmph(
                                        vec![
                                            Format::Raw(
                                                vec![
                                                    Atom::Word("Hi")
                                                ]
                                            )
                                        ]
                                    )
                                ]
                            )
                        ]
                    )
                ]
            )
        ))
    );

    assert_eq!(
        section("+Hi+"),
        Ok((
            "",
            Section::Story(
                vec![
                    Paragraph(
                        vec![
                            Component::Teller(
                                vec![
                                    Format::StrongEmph(
                                        vec![
                                            Format::Raw(
                                                vec![
                                                    Atom::Word("Hi")
                                                ]
                                            )
                                        ]
                                    )
                                ]
                            )
                        ]
                    )
                ]
            )
        ))
    );

    assert_eq!(
        section(r#"_____letter____
Dear friend.

I love you.
_______________"#),
        Ok(
            (
                "",
                Section::Aside(
                    Some("letter"),
                    vec![
                        Paragraph(
                            vec![
                                Component::Teller(
                                    vec![
                                        Format::Raw(
                                            vec![
                                                Atom::Word("Dear"),
                                                Atom::Word("friend"),
                                                Atom::Punctuation(Mark::Point)
                                            ]
                                        )
                                    ]
                                )
                            ]
                        ),
                        Paragraph(
                            vec![
                                Component::Teller(
                                    vec![
                                        Format::Raw(
                                            vec![
                                                Atom::Word("I"),
                                                Atom::Word("love"),
                                                Atom::Word("you"),
                                                Atom::Punctuation(Mark::Point)
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            )
        )
    );

    assert_eq!(
        section(r#"_____letter____
Dear friend.

I love you."#),
        Ok(
            (
                "I love you.",
                Section::IllFormed(
                    vec!["_____letter____", "Dear friend."]
                )
            )
        )
    );

    assert_eq!(
        section(r#"Dear friend."#),
        Ok(
            (
                "",
                Section::Story(
                    vec![
                        Paragraph(
                            vec![
                                Component::Teller(
                                    vec![
                                        Format::Raw(
                                            vec![
                                                Atom::Word("Dear"),
                                                Atom::Word("friend"),
                                                Atom::Punctuation(Mark::Point)
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            )
        )
    );
}

named!(
    document<&str, Document>, do_parse!(
      blank >>
      many0!(complete!(empty_line)) >>
      x: some!(section) >>
      (Document(x))
    )
);

#[test]
fn test_document() {
    assert_eq!(
        document(r#"She opened the letter, and read it.

_____letter____
Dear friend.

I love you.
_______________"#),
        Ok(
            (
                "",
                Document(
                    vec![
                        Section::Story(
                            vec![
                                Paragraph(
                                    vec![
                                        Component::Teller(
                                            vec![
                                                Format::Raw(
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
                                                )
                                            ]
                                        )
                                    ]
                                )
                            ]
                        ),
                        Section::Aside(
                            Some("letter"),
                            vec![
                                Paragraph(
                                    vec![
                                        Component::Teller(
                                            vec![
                                                Format::Raw(
                                                    vec![
                                                        Atom::Word("Dear"),
                                                        Atom::Word("friend"),
                                                        Atom::Punctuation(Mark::Point)
                                                    ]
                                                )
                                            ]
                                        )
                                    ]
                                ),
                                Paragraph(
                                    vec![
                                        Component::Teller(
                                            vec![
                                                Format::Raw(
                                                    vec![
                                                        Atom::Word("I"),
                                                        Atom::Word("love"),
                                                        Atom::Word("you"),
                                                        Atom::Punctuation(Mark::Point)
                                                    ]
                                                )
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            )
        )
    );
}

#[derive(PartialEq, Eq, Debug)]
pub enum Error<'a> {
    IncompleteParsing(Document<'a>, &'a str),
    ParsingError,
}

pub fn parse_galatian_document<'a>(input: &'a str) -> Result<Document<'a>, Error<'a>> {
    match document(input) {
        Ok(("", res)) => {
            Ok(res)
        },
        Ok((rest, res)) => {
            Err(Error::IncompleteParsing(res, rest))
        },
        _ => {
            Err(Error::ParsingError)
        },
    }
}

pub fn render_galatian_document<'a, O, T: Typography, R: Renderer<O>>(input: &'a str, typo: &T, renderer: &R) -> Result<O, Error<'a>> {
    Ok(render_document(&parse_galatian_document(input)?, typo, renderer))
}

#[test]
fn test_render() {
    use typography::ENGLISH;
    use generator::test::Html;

    assert_eq!(
        render_galatian_document(r#"

    It looks like it is working.

+I+
 smiled *at
*
him. <Oh no.>(me)

but I was like... "Why not?"

And this is great.

______letter________

But why?
____________________

"#, &ENGLISH, &Html).unwrap(),
    "<div class=\"story\"><p>It looks like it is working.</p><p><strong>I</strong> smiled<em> at</em> him.<span div=\"thought by-me\"><span div=\"reply\"> Oh no.</span></span></p><p>but I was like… “Why not?”</p><p>And this is great.</p></div><div class=\"aside letter\"><p>But why?</p></div>");
}
