use crate::utils::parse_isize;
use crate::Day;

fn row_is_safe(row: &[isize]) -> bool {
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

fn p1(list: &[Vec<isize>]) -> isize {
    list.iter()
        .map(|v| row_is_safe(v))
        .map(|b| b as isize)
        .sum()
}

fn p2(list: &[Vec<isize>]) -> isize {
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
    data: Option<Vec<String>>,
}

impl Day02 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day02 {
    fn solve(&self) -> (isize, isize) {
        let mut list: Vec<Vec<isize>> = Vec::new();

        for line in self.data.as_ref().unwrap() {
            list.push(line.split(" ").map(parse_isize).collect());
        }

        (p1(&list), p2(&list))
    }

    fn number(&self) -> u8 {
        2
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
