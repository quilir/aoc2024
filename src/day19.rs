use crate::Day;

const MAX_WORD: usize = 70;
const ALPHABET_SIZE: usize = 26;

#[derive(Default, Debug)]
struct Trie {
    ok: bool,
    children: [Option<Box<Trie>>; ALPHABET_SIZE],
}

impl Trie {
    fn new() -> Box<Self> {
        Box::new(Self::default())
    }

    fn add_word(&mut self, iter: impl IntoIterator<Item = u8>) {
        let mut curr = self;
        for c in iter {
            let idx = (c - b'a') as usize;
            if curr.children[idx].is_none() {
                curr.children[idx] = Some(Trie::new());
            }
            curr = curr.children.get_mut(idx).unwrap().as_mut().unwrap();
        }
        curr.ok = true;
    }
}

pub struct Day19 {
    data: Option<Vec<String>>,
}

impl Day19 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day19 {
    fn solve(&self) -> (isize, isize) {
        let mut trie = Trie::default();
        let mut iter = self.data.as_ref().unwrap().iter();

        iter.next()
            .unwrap()
            .split(", ")
            .for_each(|p| trie.add_word(p.bytes()));

        iter.next();

        let mut p1_res = 0;
        let mut p2_res = 0;
        let mut ways_to_reach = [0; MAX_WORD];
        for word in iter.map(|s| s.as_bytes()) {
            ways_to_reach[0] = 1;

            for mut i in 0..word.len() {
                if ways_to_reach[i] == 0 {
                    continue;
                }
                let ways = ways_to_reach[i];
                let mut curr = Some(&trie);
                while i < word.len() {
                    curr = curr.unwrap().children[(word[i] - b'a') as usize].as_deref();
                    if curr.is_none() {
                        break;
                    }
                    if curr.unwrap().ok {
                        ways_to_reach[i + 1] += ways;
                    }

                    i += 1;
                }
            }
            if ways_to_reach[word.len()] > 0 {
                p1_res += 1;
            }
            p2_res += ways_to_reach[word.len()];
            for i in 0..word.len() + 1 {
                ways_to_reach[i] = 0;
            }
        }

        (p1_res, p2_res)
    }

    fn number(&self) -> u8 {
        19
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
