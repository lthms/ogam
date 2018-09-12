use ::generator::Renderer;
use ::typography::Space;
use std::collections::HashSet;

pub struct Stats;

pub struct Digest<'a> {
    pub words_count: usize,
    pub signs_count: usize,
    pub spaces_count: usize,
    pub characters: HashSet<&'a str>
}

pub fn join<'a>(
    set1: HashSet<&'a str>,
    set2: HashSet<&'a str>
) -> HashSet<&'a str> {
    if set2.is_empty() {
        set1
    } else if set1.is_empty() {
        set2
    } else {
        set1.union(&set2).cloned().collect()
    }
}

impl<'a> Renderer<'a, Digest<'a>> for Stats {
    fn append(&self, d1: Digest<'a>, d2: Digest<'a>) -> Digest<'a> {
        Digest {
            words_count: d1.words_count + d2.words_count,
            signs_count: d1.signs_count + d2.signs_count,
            spaces_count: d1.spaces_count + d2.spaces_count,
            characters: join(d1.characters, d2.characters),
        }
    }

    fn empty(&self) -> Digest<'a> {
        Digest {
            words_count: 0,
            signs_count: 0,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_space(&self, space: Space) -> Digest<'a> {
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

    fn render_word(&self, _word: &'a str) -> Digest<'a> {
        Digest {
            words_count: 1,
            signs_count: 0,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_mark(&self, _mark: &'a str) -> Digest<'a> {
        Digest {
            words_count: 0,
            signs_count: 1,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_illformed(&self, _err: &'a str) -> Digest<'a> {
        self.empty()
    }

    fn emph_template(&self, format: Digest<'a>) -> Digest<'a> {
        format
    }

    fn strong_emph_template(&self, format: Digest<'a>) -> Digest<'a> {
        format
    }

    fn reply_template(&self, reply: Digest<'a>) -> Digest<'a> {
        reply
    }

    fn thought_template(&self, reply: Digest<'a>, author: &Option<&'a str>) -> Digest<'a> {
        let mut reply = reply;
        if let Some(author) = author {
            reply.characters.insert(author);
        }

        reply
    }

    fn dialogue_template(&self, reply: Digest<'a>, author: &Option<&'a str>) -> Digest<'a> {
        let mut reply = reply;
        if let Some(author) = author {
            reply.characters.insert(author);
        }

        reply
    }

    fn between_dialogue(&self) -> Digest<'a> {
        self.empty()
    }

    fn illformed_inline_template(&self, err: Digest<'a>) -> Digest<'a> {
        err
    }

    fn paragraph_template(&self, para: Digest<'a>) -> Digest<'a> {
        para
    }

    fn illformed_block_template(&self, err: Digest<'a>) -> Digest<'a> {
        err
    }

    fn story_template(&self, err: Digest<'a>) -> Digest<'a> {
        err
    }

    fn aside_template(&self, _cls: &Option<&'a str>, err: Digest<'a>) -> Digest<'a> {
        err
    }
}
