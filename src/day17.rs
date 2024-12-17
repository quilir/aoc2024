use std::{collections::VecDeque, mem};

use crate::{utils::parse_usize, Day};

#[derive(Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    i: usize,
    instr: Vec<usize>,
}

impl Computer {
    fn execute_step(&mut self) -> Option<usize> {
        while self.i < self.instr.len() {
            let op = self.i + 1;
            let literal = || self.instr[op];
            let combo = || match self.instr[op] {
                v @ 0..4 => v,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => panic!("Invalid combo operand"),
            };

            match self.instr[self.i] {
                0 => self.a >>= combo(),
                1 => self.b ^= literal(),
                2 => self.b = combo() % 8,
                3 => {
                    if self.a != 0 {
                        self.i = combo();
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    self.i += 2;
                    return Some(combo() % 8);
                }
                6 => self.b = self.a >> combo(),
                7 => self.c = self.a >> combo(),
                _ => panic!("Invalid operation"),
            }
            self.i += 2;
        }
        None
    }

    fn reset_with_a(&mut self, a: usize) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.i = 0;
    }
}

pub struct Day17 {
    data: Option<Vec<String>>,
}

impl Day17 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day17 {
    fn solve(&self) -> (isize, isize) {
        let mut input_iter = self.data.as_ref().unwrap().iter();
        let a = parse_usize(input_iter.next().unwrap().split(' ').nth(2).unwrap());
        let b = parse_usize(input_iter.next().unwrap().split(' ').nth(2).unwrap());
        let c = parse_usize(input_iter.next().unwrap().split(' ').nth(2).unwrap());
        input_iter.next().unwrap();
        let instr: Vec<usize> = input_iter
            .next()
            .unwrap()
            .split([' ', ','])
            .skip(1)
            .map(parse_usize)
            .collect();

        let mut c = Computer {
            a,
            b,
            c,
            i: 0,
            instr: instr.clone(),
        };

        let mut p1_res = 0;
        while let Some(v) = c.execute_step() {
            p1_res = p1_res * 10 + v;
        }

        let mut valid_registers = VecDeque::with_capacity(50);
        valid_registers.push_back((0_usize, instr.len()-1));
        let mut p2_res = -1;
        while let Some((reg, idx)) = valid_registers.pop_front() {
            for offset in 0..8 {
                let reg = (reg << 3) + offset;
                c.reset_with_a(reg);
                if c.execute_step() == Some(instr[idx]) {
                    if idx == 0 {
                        p2_res = reg as isize;
                        break;
                    }
                    valid_registers.push_back((reg, idx - 1));
                }
            }
            if p2_res != -1 {
                break;
            }
        }

        (p1_res as isize, p2_res)
    }

    fn number(&self) -> u8 {
        17
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
