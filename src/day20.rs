use std::mem;

use num_complex::Complex;

use crate::Day;

const MAP_SIZE: usize = 141;
const MINIMUM_SAVE: isize = 100;
const MAX_SKIP_P2: isize = 20;

const DIRS: [Complex<isize>; 4] = [
    Complex { re: 1, im: 0 },
    Complex { re: 0, im: 1 },
    Complex { re: -1, im: 0 },
    Complex { re: 0, im: -1 },
];

fn in_bounds(pos: Complex<isize>) -> bool {
    pos.re >= 0 && pos.re < MAP_SIZE as isize && pos.im >= 0 && pos.im < MAP_SIZE as isize
}

fn valid_cheat(
    pos: Complex<isize>,
    target: Complex<isize>,
    map: &[[isize; MAP_SIZE]; MAP_SIZE],
) -> bool {
    let dist = (pos.re - target.re).abs() + (pos.im - target.im).abs();
    let val = map[pos.re as usize][pos.im as usize];
    let target_val = map[target.re as usize][target.im as usize];

    dist <= MAX_SKIP_P2 && target_val >= val + dist + MINIMUM_SAVE
}

pub struct Day20 {
    data: Option<Vec<String>>,
}

impl Day20 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day20 {
    fn solve(&self) -> (isize, isize) {
        let mut map = [[false; MAP_SIZE]; MAP_SIZE];
        let mut start = Complex::ZERO;
        let mut end = Complex::ZERO;

        for (r, str) in self.data.as_ref().unwrap().iter().enumerate() {
            for (c, char) in str.bytes().enumerate() {
                match char {
                    b'#' => map[r][c] = true,
                    b'S' => start = Complex::new(r as isize, c as isize),
                    b'E' => end = Complex::new(r as isize, c as isize),
                    _ => (),
                }
            }
        }

        let mut dist = [[0; MAP_SIZE]; MAP_SIZE];
        let mut curr = start;
        let mut curr_dist = 1;

        let mut track = Vec::with_capacity(10000);
        loop {
            dist[curr.re as usize][curr.im as usize] = curr_dist;
            curr_dist += 1;
            track.push(curr);

            if curr == end {
                break;
            }

            for dir in DIRS {
                let next = curr + dir;
                if in_bounds(next)
                    && !map[next.re as usize][next.im as usize]
                    && dist[next.re as usize][next.im as usize] == 0
                {
                    curr = next;
                    break;
                }
            }
        }

        let mut saves_num = 0;
        for pos in track.iter() {
            let val = dist[pos.re as usize][pos.im as usize];
            for dir in DIRS {
                let next = pos + 2 * dir;
                if in_bounds(next)
                    && dist[next.re as usize][next.im as usize] >= val + 2 + MINIMUM_SAVE
                {
                    saves_num += 1;
                }
            }
        }

        // offsets for new positions that are brought into `MAX_SKIP_P2` range by move in dir_idx direction
        let mut new_nodes_offsets = [[Complex::ZERO; (2 * MAX_SKIP_P2 + 1) as usize]; 4];
        for dir_idx in 0..4 {
            let dir = DIRS[dir_idx];
            let ortho_dir = DIRS[(dir_idx + 1) % 4];
            new_nodes_offsets[dir_idx][0] = dir * MAX_SKIP_P2;
            let mut idx = 1;
            for dir_mult in 0..MAX_SKIP_P2 {
                new_nodes_offsets[dir_idx][idx] =
                    dir * dir_mult + ortho_dir * (MAX_SKIP_P2 - dir_mult);
                idx += 1;
                new_nodes_offsets[dir_idx][idx] =
                    dir * dir_mult - ortho_dir * (MAX_SKIP_P2 - dir_mult);
                idx += 1;
            }
        }

        let mut prev_valid_cheats_set = Vec::with_capacity(500);
        let mut valid_cheats_set = Vec::with_capacity(500);
        for dr in -MAX_SKIP_P2..=MAX_SKIP_P2 {
            let dc_range = MAX_SKIP_P2 - isize::abs(dr);
            for dc in -dc_range..=dc_range {
                let next = Complex::new(start.re + dr, start.im + dc);
                if in_bounds(next) && valid_cheat(start, next, &dist) {
                    prev_valid_cheats_set.push(next);
                }
            }
        }
        let mut saves_num_2 = prev_valid_cheats_set.len();

        let mut prev = start;
        for pos in track[1..(track.len() - MINIMUM_SAVE as usize)]
            .iter()
            .copied()
        {
            for target in prev_valid_cheats_set.drain(..) {
                if valid_cheat(pos, target, &dist) {
                    valid_cheats_set.push(target);
                }
            }

            let idx = match pos - prev {
                Complex { re: 1, im: 0 } => 0,
                Complex { re: 0, im: 1 } => 1,
                Complex { re: -1, im: 0 } => 2,
                Complex { re: 0, im: -1 } => 3,
                _ => panic!("Invalid transition"),
            };
            for target_offset in new_nodes_offsets[idx] {
                let target = pos + target_offset;
                if in_bounds(target) && valid_cheat(pos, target, &dist) {
                    valid_cheats_set.push(target);
                }
            }

            saves_num_2 += valid_cheats_set.len();
            prev = pos;
            mem::swap(&mut prev_valid_cheats_set, &mut valid_cheats_set);
        }

        (saves_num, saves_num_2 as isize)
    }

    fn number(&self) -> u8 {
        20
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
