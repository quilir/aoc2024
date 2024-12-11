use crate::{utils::parse_usize, Day};

fn rows_mul_sub(data: &[(isize, Vec<isize>)]) -> Vec<bool> {
    data.iter()
        .map(|(res, nums)| {
            let mut heap = vec![(*res, nums.len() - 1)];
            let mut found = false;

            while let Some((prev, pos)) = heap.pop() {
                let num = nums[pos];
                if pos == 0 {
                    if prev == num {
                        found = true;
                        break;
                    }
                    continue;
                }
                if prev % num == 0 {
                    heap.push((prev / num, pos - 1));
                }

                let sub = prev - num;
                if sub >= 0 {
                    heap.push((sub, pos - 1));
                }
            }
            found
        })
        .collect()
}

fn p1(data: &[(isize, Vec<isize>)], rows_passing: &[bool]) -> isize {
    data.iter()
        .zip(rows_passing)
        .map(|((val, _), passes)| *passes as isize * *val)
        .sum()
}

fn p2(data: &[(isize, Vec<isize>)], rows_passing: &[bool]) -> isize {
    data.iter()
        .zip(rows_passing)
        .map(|((res, nums), passes)| {
            if *passes {
                *res
            } else {
                let mut found = false;
                let mut heap = vec![(*res, nums.len() - 1)];
                let decimals = nums
                    .iter()
                    .map(|num| {
                        let mut decimal = 10;
                        while decimal <= *num {
                            decimal *= 10;
                        }
                        decimal
                    })
                    .collect::<Vec<_>>();

                while let Some((prev, pos)) = heap.pop() {
                    let num = nums[pos];
                    if pos == 0 {
                        if prev == num {
                            found = true;
                            break;
                        }
                        continue;
                    }
                    if prev % num == 0 {
                        heap.push((prev / num, pos - 1));
                    }

                    let sub = prev - num;
                    if sub >= 0 {
                        heap.push((sub, pos - 1));
                    }
                    if prev % decimals[pos] == num {
                        heap.push((prev / decimals[pos], pos - 1));
                    }
                }

                found as isize * *res
            }
        })
        .sum()
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

        let rows_passing_p1 = rows_mul_sub(&data);

        (p1(&data, &rows_passing_p1), p2(&data, &rows_passing_p1))
    }

    fn number(&self) -> u8 {
        7
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
