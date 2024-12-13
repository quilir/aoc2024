use num_complex::Complex;

use crate::Day;

const MAP_SIZE: usize = 140 + 2;

#[inline]
fn value_at_pos(pos: &Complex<isize>, map: &[[u8; MAP_SIZE]; MAP_SIZE]) -> u8 {
    map[pos.re as usize][pos.im as usize]
}

#[inline]
fn visit_region(
    value: u8,
    to_vis: &mut Vec<Complex<isize>>,
    map: &[[u8; MAP_SIZE]; MAP_SIZE],
    vis: &mut [[bool; MAP_SIZE]; MAP_SIZE],
) -> (usize, usize, usize) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut corners = 0;

    let dirs = [Complex::I, Complex::ONE, -Complex::I, -Complex::ONE];

    while let Some(pos) = to_vis.pop() {
        if vis[pos.re as usize][pos.im as usize] {
            continue;
        }
        vis[pos.re as usize][pos.im as usize] = true;
        area += 1;

        let mut same_val = [false; 4];

        for dir in 0..4 {
            same_val[dir] = value_at_pos(&(pos + dirs[dir]), map) == value;
            if same_val[dir] {
                let next = pos + dirs[dir];
                if !vis[next.re as usize][next.im as usize] {
                    to_vis.push(next);
                }
            } else {
                perimeter += 1;
            }
        }

        for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
            if !same_val[a] && !same_val[b]
                || same_val[a]
                    && same_val[b]
                    && value_at_pos(&(pos + dirs[a] + dirs[b]), map) != value
            {
                corners += 1;
            }
        }
    }
    (area, perimeter, corners)
}

fn p1(reports: &[(usize, usize, usize)]) -> isize {
    reports
        .iter()
        .map(|(area, perimeter, _)| area * perimeter)
        .sum::<usize>() as isize
}

fn p2(reports: &[(usize, usize, usize)]) -> isize {
    reports
        .iter()
        .map(|(area, _, sides)| area * sides)
        .sum::<usize>() as isize
}

pub struct Day12 {
    data: Option<Vec<String>>,
}

impl Day12 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day12 {
    fn solve(&self) -> (isize, isize) {
        let mut map = [[0; MAP_SIZE]; MAP_SIZE];
        let mut bounds = Complex::ZERO;
        let mut vis = [[false; MAP_SIZE]; MAP_SIZE];
        let mut to_vis = Vec::with_capacity(300);

        self.data
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(row, vec)| {
                vec.as_bytes().iter().enumerate().for_each(|(col, v)| {
                    map[row + 1][col + 1] = *v;
                    bounds = Complex::new((row + 2) as isize, (col + 2) as isize);
                })
            });
        let reports: Vec<_> = (1..bounds.re)
            .flat_map(|row| (1..bounds.im).map(move |col: isize| Complex::new(row, col)))
            .map(|pos| {
                let value: u8 = map[pos.re as usize][pos.im as usize];
                to_vis.push(pos);
                visit_region(value, &mut to_vis, &map, &mut vis)
            })
            .collect();

        (p1(&reports), p2(&reports))
    }

    fn number(&self) -> u8 {
        12
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
