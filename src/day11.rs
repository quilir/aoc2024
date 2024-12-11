use std::mem;

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::{utils::parse_usize, Day};

const PREALLOC_SIZE: usize = 4_000;
const POWERS: [usize; 15] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
];

#[inline(always)]
fn calc_stone_split(stone: usize) -> (usize, usize) {
    if stone == 0 {
        (1, usize::MAX)
    } else {
        let mut len = 0;
        while POWERS[len] - 1 < stone {
            len += 1;
        }

        if len % 2 == 0 {
            let half = POWERS[len / 2];
            (stone / half, stone % half)
        } else {
            (stone * 2024, usize::MAX)
        }
    }
}

fn p12(stones_input: &[usize]) -> (isize, isize) {
    // stone number -> index in amounts/next_amounts/stones_split
    let mut indices = FxHashMap::with_capacity_and_hasher(PREALLOC_SIZE, FxBuildHasher);
    // stone idx -> (child 1 idx, child 2 idx)
    let mut stones_split = Vec::with_capacity(PREALLOC_SIZE);
    // stone idx -> count
    let mut amounts = vec![0; stones_input.len()];

    // stones to be split this loop and those due in the next rount
    let mut to_split_now = Vec::new();
    let mut to_split = Vec::new();

    let mut res = (0, 0);

    for s in stones_input {
        let len = to_split.len();
        let idx = indices.entry(*s).or_insert_with(|| {
            to_split.push(*s);
            len
        });
        amounts[*idx] += 1;
    }

    for step in 0..75 {
        mem::swap(&mut to_split_now, &mut to_split);

        let mut get_index = |stone: usize| -> usize {
            let len = indices.len();
            *indices.entry(stone).or_insert_with(|| {
                to_split.push(stone);
                len
            })
        };

        for stone in to_split_now.drain(..) {
            let split = calc_stone_split(stone);
            let idx1 = get_index(split.0);
            let idx2 = if split.1 != usize::MAX {
                get_index(split.1)
            } else {
                usize::MAX
            };
            stones_split.push((idx1, idx2));
        }

        let mut next_amounts = vec![0_usize; indices.len()];

        for ((idx1, idx2), count) in stones_split.iter().zip(amounts) {
            next_amounts[*idx1] += count;
            if *idx2 != usize::MAX {
                next_amounts[*idx2] += count;
            }
        }

        amounts = next_amounts;
        if step == 24 {
            res.0 = amounts.iter().sum::<usize>() as isize
        }
    }

    res.1 = amounts.into_iter().sum::<usize>() as isize;

    res
}

pub struct Day11 {
    data: Option<Vec<String>>,
}

impl Day11 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day11 {
    fn solve(&self) -> (isize, isize) {
        let mut stones = Vec::new();

        for stone in self.data.as_ref().unwrap()[0].split(' ') {
            stones.push(parse_usize(stone));
        }

        p12(&stones)
    }

    fn number(&self) -> u8 {
        11
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
