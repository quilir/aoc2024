use crate::{utils::parse_isize, Day};

// x, y, dx, dy
type Robot = (isize, isize, isize, isize);

const SPACE_X: usize = 101;
const SPACE_Y: usize = 103;

#[inline]
fn traverse((x, y, dx, dy): &Robot, moves: isize) -> (isize, isize) {
    (
        (x + moves * (SPACE_X as isize + dx)) % SPACE_X as isize,
        (y + moves * (SPACE_Y as isize + dy)) % SPACE_Y as isize,
    )
}

fn p1(robots: &[Robot]) -> isize {
    let res = robots
        .iter()
        .map(|x| traverse(x, 100))
        .filter(|(x, y)| *x as usize != SPACE_X / 2 && *y as usize != SPACE_Y / 2)
        .fold((0, 0, 0, 0), |mut acc, (x, y)| {
            match ((x as usize) < SPACE_X / 2, (y as usize) < SPACE_Y / 2) {
                (true, true) => acc.0 += 1,
                (true, false) => acc.1 += 1,
                (false, true) => acc.2 += 1,
                (false, false) => acc.3 += 1,
            };
            acc
        });
    res.0 * res.1 * res.2 * res.3
}

fn p2(robots: &[Robot]) -> isize {
    let cong_x = (0..SPACE_X)
        .map(|t| {
            let mut occ = [0; SPACE_X];
            robots
                .iter()
                .for_each(|r| occ[traverse(r, t as isize).0 as usize] += 1);
            occ.into_iter()
                .enumerate()
                .max_by_key(|(_, v)| *v)
                .clone()
                .unwrap()
                .1
        })
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .unwrap()
        .0;
    let cong_y = (0..SPACE_Y)
        .map(|t| {
            let mut occ = [0; SPACE_Y];
            robots
                .iter()
                .for_each(|r| occ[traverse(r, t as isize).1 as usize] += 1);
            occ.into_iter()
                .enumerate()
                .max_by_key(|(_, v)| *v)
                .clone()
                .unwrap()
                .1
        })
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .unwrap()
        .0;

    for i in 0..SPACE_X * SPACE_Y {
        if i % SPACE_X == cong_x && i % SPACE_Y == cong_y {
            return i as isize;
        }
    }

    unreachable!()
}

pub struct Day14 {
    data: Option<Vec<String>>,
}

impl Day14 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day14 {
    fn solve(&self) -> (isize, isize) {
        // p=3,0 v=-2,-2
        let data: Vec<_> = self
            .data
            .as_ref()
            .unwrap()
            .iter()
            .map(|line| {
                let mut split = line.split(['=', ',', ' ']);
                (
                    parse_isize(split.nth(1).unwrap()),
                    parse_isize(split.nth(0).unwrap()),
                    parse_isize(split.nth(1).unwrap()),
                    parse_isize(split.nth(0).unwrap()),
                )
            })
            .collect();

        (p1(&data), p2(&data))
    }

    fn number(&self) -> u8 {
        14
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
