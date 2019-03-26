use generator::Renderer;
use std::collections::HashSet;
use typography::Space;

use std::default::Default;
use std::hash::BuildHasher;

pub struct Stats;

pub struct Digest<'input> {
    pub words_count: usize,
    pub signs_count: usize,
    pub spaces_count: usize,
    pub characters: HashSet<&'input str>,
}

pub fn join<'input, S: BuildHasher + Default>(
    set1: HashSet<&'input str, S>,
    set2: HashSet<&'input str, S>,
) -> HashSet<&'input str, S> {
    if set2.is_empty() {
        set1
    } else if set1.is_empty() {
        set2
    } else {
        set1.union(&set2).cloned().collect()
    }
}

impl<'input> Renderer<'input, Digest<'input>> for Stats {
    fn append(&self, d1: Digest<'input>, d2: Digest<'input>) -> Digest<'input> {
        Digest {
            words_count: d1.words_count + d2.words_count,
            signs_count: d1.signs_count + d2.signs_count,
            spaces_count: d1.spaces_count + d2.spaces_count,
            characters: join(d1.characters, d2.characters),
        }
    }

    fn empty(&self) -> Digest<'input> {
        Digest {
            words_count: 0,
            signs_count: 0,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_space(&self, space: Space) -> Digest<'input> {
        Digest {
            words_count: 0,
            signs_count: 0,
            spaces_count: match space {
                Space::None => 0,
                _ => 1,
            },
            characters: HashSet::new(),
        }
    }

    fn render_word(&self, _word: &'input str) -> Digest<'input> {
        Digest {
            words_count: 1,
            signs_count: 0,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_mark(&self, _mark: &'input str) -> Digest<'input> {
        Digest {
            words_count: 0,
            signs_count: 1,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_illformed(&self, _err: &'input str) -> Digest<'input> {
        self.empty()
    }

    fn emph_template(&self, format: Digest<'input>) -> Digest<'input> {
        format
    }

    fn strong_emph_template(&self, format: Digest<'input>) -> Digest<'input> {
        format
    }

    fn reply_template(
        &self,
        reply: Digest<'input>,
        _author: &Option<&'input str>,
    ) -> Digest<'input> {
        reply
    }

    fn thought_template(
        &self,
        reply: Digest<'input>,
        author: &Option<&'input str>,
    ) -> Digest<'input> {
        let mut reply = reply;
        if let Some(author) = author {
            reply.characters.insert(author);
        }

        reply
    }

    fn dialogue_template(
        &self,
        reply: Digest<'input>,
        author: &Option<&'input str>,
    ) -> Digest<'input> {
        let mut reply = reply;
        if let Some(author) = author {
            reply.characters.insert(author);
        }

        reply
    }

    fn between_dialogue(&self) -> Digest<'input> {
        self.empty()
    }

    fn illformed_inline_template(&self, err: Digest<'input>) -> Digest<'input> {
        err
    }

    fn paragraph_template(&self, para: Digest<'input>) -> Digest<'input> {
        para
    }

    fn illformed_block_template(&self, err: Digest<'input>) -> Digest<'input> {
        err
    }

    fn story_template(&self, err: Digest<'input>) -> Digest<'input> {
        err
    }

    fn aside_template(&self, _cls: &Option<&'input str>, err: Digest<'input>) -> Digest<'input> {
        err
    }
}
