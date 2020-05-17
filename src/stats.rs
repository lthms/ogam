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

use generator::Output;
use std::collections::HashSet;
use typography::Space;

pub struct Stats;

pub struct Digest {
    pub words_count: usize,
    pub signs_count: usize,
    pub spaces_count: usize,
    pub characters: HashSet<String>,
}

impl Output for Digest {
    fn empty(_: usize) -> Digest {
        Digest {
            words_count: 0,
            signs_count: 0,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_space(&mut self, space: Space) -> () {
        match space {
            Space::None => (),
            _ => {
                self.spaces_count += 1;
            }
        }
    }

    fn render_word(&mut self, _word: &str) -> () {
        self.words_count += 1;
    }

    fn render_mark(&mut self, _mark: &str) -> () {
        self.signs_count += 1;
    }

    fn render_illformed(&mut self, _err: &str) -> () {}

    fn emph_template<F>(&mut self, format: F) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        format(self)
    }

    fn strong_emph_template<F>(&mut self, format: F) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        format(self)
    }

    fn reply_template<F>(&mut self, reply: F, _author: &Option<&str>) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        reply(self)
    }

    fn thought_template<F>(&mut self, reply: F, author: &Option<&str>) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        // TODO: allocate only if necessary
        author.map(|a| self.characters.insert(a.to_string()));

        reply(self);
    }

    fn dialogue_template<F>(&mut self, reply: F, author: &Option<&str>) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        // TODO: allocate only if necessary
        author.map(|a| self.characters.insert(a.to_string()));

        reply(self);
    }

    fn between_dialogue(&mut self) -> () {}

    fn illformed_inline_template<F>(&mut self, _err: F) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
    }

    fn paragraph_template<F>(&mut self, para: F) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        para(self);
    }

    fn illformed_block_template<F>(&mut self, _err: F) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
    }

    fn story_template<F>(&mut self, story: F) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        story(self);
    }

    fn aside_template<F>(&mut self, _cls: &Option<&str>, aside: F) -> ()
    where
        F: FnOnce(&mut Digest) -> (),
    {
        aside(self)
    }
}
