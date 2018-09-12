use maud::{Markup, PreEscaped, html};
use ::generator::Renderer;
use ::typography::Space;

pub struct Html;

impl Renderer<Markup> for Html {
    fn append(&self, m1: Markup, m2: Markup) -> Markup {
        html!(
            (m1);
            (m2)
        )
    }

    fn empty(&self) -> Markup {
        html!()
    }

    fn story_template(&self, m: Markup) -> Markup {
        html!(
            div {
                (m)
            }
        )
    }
    fn render_space(&self, space: Space) -> Markup {
        html!(
            (PreEscaped(
                match space {
                    Space::None => "",
                    Space::Normal => " ",
                    Space::Nbsp => "&nbsp;",
                }
            ))
        )
    }

    fn render_word(&self, word: &str) -> Markup {
        html!((word))
    }

    fn render_mark(&self, mark: &str) -> Markup {
        html!((mark))
    }

    fn render_illformed(&self, err: &str) -> Markup {
        html!((err))
    }

    fn emph_template(&self, format: Markup) -> Markup {
        html!(
            em {
                (format)
            }
        )
    }

    fn strong_emph_template(&self, format: Markup) -> Markup {
        html!(
            strong {
                (format)
            }
        )
    }

    fn reply_template(&self, rep: Markup) -> Markup {
        html!(
            span class="reply" {
                (rep)
            }
        )
    }

    fn thought_template(&self, thought: Markup, author: &Option<&str>) -> Markup {
        let author = author.map(|x| format!(" by-{}", x)).unwrap_or("".to_string());

        html!(
            span class={ "thought" (author) } {
                (thought)
            }
        )
    }

    fn dialogue_template(&self, dial: Markup, author: &Option<&str>) -> Markup {
        let author = author.map(|x| format!(" by-{}", x)).unwrap_or("".to_string());

        html!(
            span class={ "dialogue" (author) } {
                (dial)
            }
        )
    }

    fn between_dialogue(&self) -> Markup {
        html!(
            (PreEscaped("</p><p>"))
        )
    }

    fn paragraph_template(&self, para: Markup) -> Markup {
        html!(
            p {
                (para)
            }
        )
    }

    fn aside_template(&self, cls: &Option<&str>, aside: Markup) -> Markup {
        html!(
            div class={ "aside " (cls.unwrap_or("")) } {
                (aside)
            }
        )
    }

    fn illformed_inline_template(&self, ill: Markup) -> Markup {
        html!(
            span class="illformed_inline" {
                (ill)
            }
        )
    }

    fn illformed_block_template(&self, ill: Markup) -> Markup {
        html!(
            div class="illformed" {
                (ill)
            }
        )
    }
}
