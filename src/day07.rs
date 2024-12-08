use crate::{utils::parse_usize, Day};

fn p1(data: Vec<(isize, Vec<isize>)>) -> isize {
    let mut sum = 0;
    for (res, nums) in data.into_iter() {
        let mut set = vec![res];
        let mut new_set = Vec::new();
        for num in nums.iter().rev() {
            for prev in set.iter() {
                if prev % num == 0 {
                    new_set.push(prev / num);
                }

                let sub = prev - num;
                if sub >= 0 {
                    new_set.push(sub);
                }
            }
            set.clear();
            set.append(&mut new_set);
        }

        if set.into_iter().any(|v| v == 0) {
            sum += res;
        }
    }
    sum
}

fn p2(data: Vec<(isize, Vec<isize>)>) -> isize {
    let mut sum = 0;
    for (res, nums) in data.into_iter() {
        let mut set = vec![res];
        let mut new_set = Vec::new();
        for num in nums.into_iter().rev() {
            let mut decimal = 10;
            while decimal <= num {
                decimal *= 10;
            }
            for prev in set.iter() {
                if prev % num == 0 {
                    new_set.push(prev / num);
                }

                let sub = prev - num;
                if sub >= 0 {
                    new_set.push(sub);
                }

                if prev % decimal == num {
                    new_set.push(prev / decimal);
                }
            }
            set.clear();
            set.append(&mut new_set);
        }

        if set.into_iter().any(|v| v == 0) {
            sum += res;
        }
    }
    sum
}

pub struct Day07 {
    data: Option<Vec<String>>,
}

impl Day07 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day07 {
    fn solve(&self) -> (isize, isize) {
        let data: Vec<_> = self
            .data
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| {
                let mut split = s.split(": ");
                let res = parse_usize(split.next().unwrap()) as isize;
                let nums: Vec<isize> = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|s| parse_usize(s) as isize)
                    .collect();
                (res, nums)
            })
            .collect();

        (p1(data.clone()), p2(data))
    }

    fn number(&self) -> u8 {
        7
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
