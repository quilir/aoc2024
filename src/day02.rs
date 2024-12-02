use std::io::{self, BufRead};

use crate::Day;

fn row_is_safe(row: &Vec<i32>) -> bool {
    let mut prev = row[0];
    let mut in_range = true;
    let mut increasing = true;
    let mut decreasing = true;

    for i in &row[1..] {
        let diff = *i - prev;
        if diff >= 0 {
            decreasing = false;
        }
        if diff <= 0 {
            increasing = false;
        }
        if diff.abs() < 1 || diff.abs() > 3 {
            in_range = false;
        }
        prev = *i;
    }

    (decreasing || increasing) && in_range
}

fn p1(list: &Vec<Vec<i32>>) -> i32 {
    list.iter().map(row_is_safe).map(|b| b as i32).sum() 
}

fn p2(list: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for row in list {
        for idx in 0..row.len() {
            let mut row = row.clone();
            row.remove(idx);
            if row_is_safe(&row) {
                res += 1;
                break;
            }
        }
    }
    res
}

pub struct Day02 {
    data: Option<Vec<String>>
}

impl Day02 {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            data: None
        })
    }
}

impl Day for Day02 {
    fn number(&self) -> u8 {
        2
    }

    fn load_data(&mut self) {
        let file = self.input_file();

        let data: Vec<String> = io::BufReader::new(file).lines().collect::<Result<Vec<String>, _>>().unwrap();

        self.data = Some(data);
    }

    fn solve(&self) -> (i32, i32) {
        let mut list: Vec<Vec<i32>> = Vec::new();

        for line in self.data.as_ref().unwrap() {
            list.push(line.split(" ").map(|s| s.parse::<i32>()).collect::<Result<Vec<i32>, _>>().unwrap());
        }

        (p1(&list), p2(&list))
    }
}