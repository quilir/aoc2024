use std::{collections::BTreeMap, io::{self, BufRead}};

use crate::Day;

fn p1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    list1.into_iter().zip(list2.into_iter()).map(|(i1, i2)| (i1-i2).abs()).sum()
}

fn p2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut counts = BTreeMap::<i32, i32>::new();
    for i in list2 {
        let val = counts.entry(*i).or_default();
        *val += 1;
    }
    list1.into_iter().map(|i| counts.get(i).unwrap_or(&0)*i).sum()
}

#[derive(Default)]
pub struct Day01 {
    data: Option<Vec<String>>,
}

impl Day01 {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            data: None
        })
    }
}

impl Day for Day01 {
    fn number(&self) -> u8 {
        1
    }

    fn load_data(&mut self) {
        let file = self.input_file();

        let data: Vec<String> = io::BufReader::new(file).lines().collect::<Result<Vec<String>, _>>().unwrap();

        self.data = Some(data);
    }

    fn solve(&self) -> (i32, i32) {
        let mut list1 = Vec::<i32>::new();
        let mut list2 = Vec::<i32>::new();

        for line in self.data.as_ref().unwrap() {
            let mut iter= line.split("   ");
            list1.push(iter.next().unwrap().parse().unwrap());
            list2.push(iter.next().unwrap().parse().unwrap());
        }

        list1.sort();
        list2.sort();


        (p1(&list1, &list2), p2(&list1, &list2))
    }
}