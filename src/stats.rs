use generator::Renderer;
use std::collections::HashSet;
use typography::Space;

pub struct Stats;

pub struct Digest {
    pub words_count: usize,
    pub signs_count: usize,
    pub spaces_count: usize,
    pub characters: HashSet<String>,
}

fn join(set1: HashSet<String>, set2: HashSet<String>) -> HashSet<String> {
    if set2.is_empty() {
        set1
    } else if set1.is_empty() {
        set2
    } else {
        set1.union(&set2).cloned().collect()
    }
}

impl Renderer<Digest> for Stats {
    fn append(&self, d1: Digest, d2: Digest) -> Digest {
        Digest {
            words_count: d1.words_count + d2.words_count,
            signs_count: d1.signs_count + d2.signs_count,
            spaces_count: d1.spaces_count + d2.spaces_count,
            characters: join(d1.characters, d2.characters),
        }
    }

    fn empty(&self) -> Digest {
        Digest {
            words_count: 0,
            signs_count: 0,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_space(&self, space: Space) -> Digest {
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

    fn render_word(&self, _word: &str) -> Digest {
        Digest {
            words_count: 1,
            signs_count: 0,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_mark(&self, _mark: &str) -> Digest {
        Digest {
            words_count: 0,
            signs_count: 1,
            spaces_count: 0,
            characters: HashSet::new(),
        }
    }

    fn render_illformed(&self, _err: &str) -> Digest {
        self.empty()
    }

    fn emph_template(&self, format: Digest) -> Digest {
        format
    }

    fn strong_emph_template(&self, format: Digest) -> Digest {
        format
    }

    fn reply_template(&self, reply: Digest, _author: &Option<&str>) -> Digest {
        reply
    }

    fn thought_template(&self, reply: Digest, author: &Option<&str>) -> Digest {
        let mut reply = reply;
        if let Some(author) = author {
            reply.characters.insert(author.to_string());
        }

        reply
    }

    fn dialogue_template(&self, reply: Digest, author: &Option<&str>) -> Digest {
        let mut reply = reply;
        if let Some(author) = author {
            reply.characters.insert(author.to_string());
        }

        reply
    }

    fn between_dialogue(&self) -> Digest {
        self.empty()
    }

    fn illformed_inline_template(&self, err: Digest) -> Digest {
        err
    }

    fn paragraph_template(&self, para: Digest) -> Digest {
        para
    }

    fn illformed_block_template(&self, err: Digest) -> Digest {
        err
    }

    fn story_template(&self, err: Digest) -> Digest {
        err
    }

    fn aside_template(&self, _cls: &Option<&str>, err: Digest) -> Digest {
        err
    }
}
