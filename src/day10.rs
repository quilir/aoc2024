use std::mem;

use crate::Day;

use num_complex::Complex;

const MAP_SIZE: usize = 59;
const UNREACHABLE_VAL: u8 = 20;

fn value(pos: &Complex<isize>, map: &[[u8; MAP_SIZE]; MAP_SIZE], bounds: &(usize, usize)) -> u8 {
    if pos.re >= 0 && pos.re < bounds.0 as isize && pos.im >= 0 && pos.im < bounds.1 as isize {
        map[pos.re as usize][pos.im as usize]
    } else {
        UNREACHABLE_VAL
    }
}

fn p1(
    map: &[[u8; MAP_SIZE]; MAP_SIZE],
    zeros: &[Complex<isize>],
    bounds: &(usize, usize),
) -> isize {
    let mut res = 0;
    for zero_pos in zeros {
        let mut curr_poss = vec![*zero_pos];
        let mut next_poss = Vec::new();
        for expected_val in 1..10 {
            for pos in curr_poss.drain(..) {
                for dir in [Complex::i(), -Complex::i(), 1.into(), (-1).into()] {
                    let next_pos = pos + dir;
                    if value(&next_pos, map, bounds) == expected_val {
                        next_poss.push(next_pos);
                    }
                }
            }
            curr_poss.append(&mut next_poss);
            curr_poss.sort_by_key(|c| (c.re, c.im));
            curr_poss.dedup();
        }
        res += curr_poss.len();
    }
    res as isize
}

fn p2(
    map: &[[u8; MAP_SIZE]; MAP_SIZE],
    zeros: &[Complex<isize>],
    bounds: &(usize, usize),
) -> isize {
    let mut visits = [[0_usize; MAP_SIZE]; MAP_SIZE];
    for pos in zeros {
        visits[pos.re as usize][pos.im as usize] = 1;
    }
    let mut curr_poss = zeros.to_vec();
    let mut next_poss = Vec::new();

    for expected_val in 1..10 {
        for pos in curr_poss.drain(..) {
            for dir in [Complex::i(), -Complex::i(), 1.into(), (-1).into()] {
                let next_pos = pos + dir;
                if value(&next_pos, map, bounds) == expected_val {
                    if visits[next_pos.re as usize][next_pos.im as usize] == 0 {
                        next_poss.push(next_pos);
                    }
                    visits[next_pos.re as usize][next_pos.im as usize] +=
                        visits[pos.re as usize][pos.im as usize];
                }
            }
        }
        mem::swap(&mut curr_poss, &mut next_poss);
    }

    let mut res = 0;
    for pos in curr_poss {
        res += visits[pos.re as usize][pos.im as usize];
    }
    res as isize
}

pub struct Day10 {
    data: Option<Vec<String>>,
}

impl Day10 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day10 {
    fn solve(&self) -> (isize, isize) {
        let mut map = [[0_u8; MAP_SIZE]; MAP_SIZE];
        let mut zeros = Vec::new();

        let mut rows = 0;
        let mut cols = 0;

        for (row, s) in self.data.as_ref().unwrap().iter().enumerate() {
            let bytes = s.bytes();
            cols = bytes.len();
            rows += 1;
            for (col, char) in bytes.enumerate() {
                map[row][col] = char - b'0';
                if map[row][col] == 0 {
                    zeros.push(Complex::new(row as isize, col as isize));
                }
            }
        }

        let bounds = (rows, cols);
        (p1(&map, &zeros, &bounds), p2(&map, &zeros, &bounds))
    }

    fn number(&self) -> u8 {
        10
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
