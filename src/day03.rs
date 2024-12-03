use core::panic;

use regex::Regex;

use crate::utils::parse_isize;
use crate::Day;

enum Instruction {
    On,
    Off,
    Mul(isize, isize),
}

fn p1(list: &[Instruction]) -> isize {
    list.iter()
        .map(|instr| {
            if let Instruction::Mul(a, b) = instr {
                a * b
            } else {
                0
            }
        })
        .sum()
}

fn p2(list: &[Instruction]) -> isize {
    let mut on = true;
    list.iter()
        .map(move |instr| match instr {
            Instruction::On => {
                on = true;
                0
            }
            Instruction::Off => {
                on = false;
                0
            }
            Instruction::Mul(a, b) if on => a * b,
            _ => 0,
        })
        .sum()
}

pub struct Day03 {
    data: Option<Vec<String>>,
}

impl Day03 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day03 {
    fn solve(&self) -> (isize, isize) {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)()()|don't\(\)()()").unwrap();

        let list: Vec<Instruction> = self
            .data
            .as_ref()
            .unwrap()
            .iter()
            .flat_map(|line| re.captures_iter(line))
            .map(|c| c.extract())
            .map(|(s, [a, b])| match &s[0..3] {
                "mul" => Instruction::Mul(parse_isize(a), parse_isize(b)),
                "do(" => Instruction::On,
                "don" => Instruction::Off,
                _ => panic!("Unknown match"),
            })
            .collect();

        (p1(&list), p2(&list))
    }

    fn number(&self) -> u8 {
        3
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
