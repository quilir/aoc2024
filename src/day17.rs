use std::mem;

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
    fn execute(&mut self) -> Option<usize> {
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
        // println!("{c:?}");
        // println!("{}", c.execute());
        let mut p1_res = 0;
        while let Some(v) = c.execute() {
            p1_res = p1_res * 10 + v;
        }

        let mut valid_registers = vec![0];
        let mut next_registers = vec![];
        for out in instr.into_iter().rev() {
            for reg in valid_registers.drain(..) {
                let reg = reg << 3;
                for offset in 0..8 {
                    c.reset_with_a(reg + offset);
                    if c.execute() == Some(out) {
                        next_registers.push(reg + offset);
                    }
                }
            }
            mem::swap(&mut valid_registers, &mut next_registers);
        }

        (p1_res as isize, valid_registers[0] as isize)
    }

    fn number(&self) -> u8 {
        17
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
