use crate::Day;

const TYPES_NO: usize = (b'z' - b'0' + 1) as usize;
const MAX_AREA_SIZE: usize = 150;

#[inline]
fn byte_index(b: u8) -> usize {
    (b - b'0') as usize
}

#[inline]
fn in_bounds(pos: &(isize, isize), bounds: &(isize, isize)) -> bool {
    pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1
}

#[inline]
fn calc_basic_antinodes(pos1: &(isize, isize), pos2: &(isize, isize)) -> [(isize, isize); 2] {
    let vec_1_to_2 = (pos2.0 - pos1.0, pos2.1 - pos1.1);

    [
        (pos1.0 - vec_1_to_2.0, pos1.1 - vec_1_to_2.1),
        (pos2.0 + vec_1_to_2.0, pos2.1 + vec_1_to_2.1),
    ]
}

fn p1(antennae: &[Vec<(isize, isize)>], bounds: (isize, isize)) -> isize {
    let mut antinodes = [[false; MAX_AREA_SIZE]; MAX_AREA_SIZE];
    let mut count = 0;
    for antenna_set in antennae {
        for i in 0..antenna_set.len() {
            for j in i + 1..antenna_set.len() {
                for antinode in calc_basic_antinodes(&antenna_set[i], &antenna_set[j]) {
                    if in_bounds(&antinode, &bounds)
                        && !antinodes[antinode.0 as usize][antinode.1 as usize]
                    {
                        count += 1;
                        antinodes[antinode.0 as usize][antinode.1 as usize] = true;
                    }
                }
            }
        }
    }

    count
}

#[inline]
fn mark_all_antinodes(
    pos1: &(isize, isize),
    pos2: &(isize, isize),
    bounds: &(isize, isize),
    antinodes: &mut [[bool; MAX_AREA_SIZE]; MAX_AREA_SIZE],
) -> isize {
    let mut count = 0;
    let vec_1_to_2 = (pos2.0 - pos1.0, pos2.1 - pos1.1);

    let mut p = *pos1;
    while in_bounds(&p, bounds) {
        if !antinodes[p.0 as usize][p.1 as usize] {
            antinodes[p.0 as usize][p.1 as usize] = true;
            count += 1;
        }
        p = (p.0 - vec_1_to_2.0, p.1 - vec_1_to_2.1);
    }

    let mut p: (isize, isize) = (pos1.0 + vec_1_to_2.0, pos1.1 + vec_1_to_2.1);
    while in_bounds(&p, bounds) {
        if !antinodes[p.0 as usize][p.1 as usize] {
            antinodes[p.0 as usize][p.1 as usize] = true;
            count += 1;
        }
        p = (p.0 + vec_1_to_2.0, p.1 + vec_1_to_2.1);
    }
    count
}

fn p2(antennae: &[Vec<(isize, isize)>], bounds: (isize, isize)) -> isize {
    let mut antinodes = [[false; MAX_AREA_SIZE]; MAX_AREA_SIZE];
    let mut count = 0;
    for antenna_set in antennae {
        for i in 0..antenna_set.len() {
            for j in i + 1..antenna_set.len() {
                count +=
                    mark_all_antinodes(&antenna_set[i], &antenna_set[j], &bounds, &mut antinodes);
            }
        }
    }

    count
}

pub struct Day08 {
    data: Option<Vec<String>>,
}

impl Day08 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day08 {
    fn solve(&self) -> (isize, isize) {
        let mut rows = 0;
        let mut cols = 0;
        let mut antennae = vec![Vec::new(); TYPES_NO];

        self.data
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(row, s)| {
                let chars = s.as_bytes().iter();
                rows += 1;
                cols = chars.len() as isize;

                chars.enumerate().for_each(|(col, char)| {
                    if *char != b'.' {
                        antennae[byte_index(*char)].push((row as isize, col as isize));
                    }
                });
            });

        (p1(&antennae, (rows, cols)), p2(&antennae, (rows, cols)))
    }

    fn number(&self) -> u8 {
        8
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
