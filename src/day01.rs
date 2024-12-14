use crate::{utils::parse_isize, Day};

fn p1(list1: &[isize], list2: &[isize]) -> isize {
    list1
        .iter()
        .zip(list2)
        .map(|(i1, i2)| (i1 - i2).abs())
        .sum()
}

fn p2(list1: &[isize], list2: &[isize]) -> isize {
    let mut l2_idx = 0;
    let mut l2_val = -1;
    let mut l2_count = 0;

    let mut res = 0;
    for item in list1 {
        if *item > l2_val {
            while l2_idx < list2.len() as isize && list2[l2_idx as usize] < *item {
                l2_idx += 1;
            }
            if l2_idx == list2.len() as isize {
                break;
            }
            l2_count = 0;
            l2_val = list2[l2_idx as usize];
            while l2_idx < list2.len() as isize && list2[l2_idx as usize] == l2_val {
                l2_count += 1;
                l2_idx += 1;
            }
        }

        if *item == l2_val {
            res += item * l2_count;
        }
    }
    res
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
        let mut list1 = Vec::<isize>::with_capacity(1000);
        let mut list2 = Vec::<isize>::with_capacity(1000);

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
