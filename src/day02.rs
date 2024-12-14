use crate::utils::parse_isize;
use crate::Day;

const MAX_LEN: usize = 8;

#[inline]
fn calc_diff_repr(a: isize, b: isize) -> i8 {
    match b - a {
        (1..=3) => 1,
        (-3..=-1) => -1,
        _ => 0,
    }
}

#[inline]
fn row_is_safe(row: &[isize], len: usize) -> (i16, i16) {
    let mut res = 0;
    for i in 1..len {
        res += calc_diff_repr(row[i - 1], row[i]);
    }

    if res.abs() == (len - 1) as i8 {
        return (1, 1);
    }

    for i in 0..len {
        let mut res = res;
        if i > 0 {
            res -= calc_diff_repr(row[i - 1], row[i]);
        }
        if i < len - 1 {
            res -= calc_diff_repr(row[i], row[i + 1]);
        }
        if i > 0 && i < len - 1 {
            res += calc_diff_repr(row[i - 1], row[i + 1]);
        }

        if res.abs() == (len - 2) as i8 {
            return (0, 1);
        }
    }

    return (0, 0);
}

fn p1(res: &(i16, i16)) -> isize {
    res.0 as isize
}

fn p2(res: &(i16, i16)) -> isize {
    res.1 as isize
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
        let mut report = [0; MAX_LEN];

        let res = self
            .data
            .as_ref()
            .unwrap()
            .iter()
            .map(|line| {
                let mut len = 0;
                for (idx, v) in line.split(' ').enumerate() {
                    len += 1;
                    report[idx] = parse_isize(v);
                }
                row_is_safe(&report, len)
            })
            .fold((0,0), |mut acc, v| {
                acc.0 += v.0;
                acc.1 += v.1;
                acc
            });

        (p1(&res), p2(&res))
    }

    fn number(&self) -> u8 {
        2
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
