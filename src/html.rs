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

use generator::Renderer;
use maud::{html, Markup, PreEscaped};
use typography::Space;

pub struct Html;

impl<'input> Renderer<'input, Markup> for Html {
    fn append(&self, m1: Markup, m2: Markup) -> Markup {
        html!((m1)(m2))
    }

    fn empty(&self) -> Markup {
        html!()
    }

    fn story_template(&self, m: Markup) -> Markup {
        html!(
            section class="story" {
                (m)
            }
        )
    }
    fn render_space(&self, space: Space) -> Markup {
        html!(
            (PreEscaped(match space {
                Space::None => "",
                Space::Normal => " ",
                Space::Nbsp => "&nbsp;",
            }))
        )
    }

    fn render_word(&self, word: &'input str) -> Markup {
        html!((word))
    }

    fn render_mark(&self, mark: &'input str) -> Markup {
        html!((mark))
    }

    fn render_illformed(&self, err: &'input str) -> Markup {
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

    fn reply_template(&self, rep: Markup, _author: &Option<&'input str>) -> Markup {
        html!(
            span class="reply" {
                (rep)
            }
        )
    }

    fn thought_template(&self, thought: Markup, author: &Option<&'input str>) -> Markup {
        let author = author
            .map(|x| format!(" by-{}", x))
            .unwrap_or("".to_string());

        html!(
            span class={ "thought" (author) } {
                (thought)
            }
        )
    }

    fn dialogue_template(&self, dial: Markup, author: &Option<&'input str>) -> Markup {
        let author = author
            .map(|x| format!(" by-{}", x))
            .unwrap_or("".to_string());

        html!(
            span class={ "dialogue" (author) } {
                (dial)
            }
        )
    }

    fn between_dialogue(&self) -> Markup {
        html!((PreEscaped("</p><p>")))
    }

    fn paragraph_template(&self, para: Markup) -> Markup {
        html!(
            p {
                (para)
            }
        )
    }

    fn aside_template(&self, cls: &Option<&'input str>, aside: Markup) -> Markup {
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
