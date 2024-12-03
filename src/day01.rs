use std::collections::BTreeMap;

use crate::{utils::parse_isize, Day};

fn p1(list1: &[isize], list2: &[isize]) -> isize {
    list1
        .iter()
        .zip(list2)
        .map(|(i1, i2)| (i1 - i2).abs())
        .sum()
}

fn p2(list1: &[isize], list2: &[isize]) -> isize {
    let mut counts = BTreeMap::<isize, isize>::new();
    for i in list2 {
        let val = counts.entry(*i).or_default();
        *val += 1;
    }
    list1.iter().map(|i| counts.get(i).unwrap_or(&0) * i).sum()
}

pub struct Day01 {
    data: Option<Vec<String>>,
}

impl Day01 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day01 {
    fn solve(&self) -> (isize, isize) {
        let mut list1 = Vec::<isize>::new();
        let mut list2 = Vec::<isize>::new();

        for line in self.data.as_ref().unwrap() {
            let mut iter = line.split("   ");
            list1.push(parse_isize(iter.next().unwrap()));
            list2.push(parse_isize(iter.next().unwrap()));
        }

        list1.sort();
        list2.sort();

        (p1(&list1, &list2), p2(&list1, &list2))
    }

    fn number(&self) -> u8 {
        1
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
