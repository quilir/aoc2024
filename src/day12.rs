use num_complex::Complex;

use crate::Day;

const MAP_SIZE: usize = 140;

fn value_at_pos(
    pos: &Complex<isize>,
    map: &[[u8; MAP_SIZE]; MAP_SIZE],
    bounds: &Complex<isize>,
) -> u8 {
    if pos.re >= 0 && pos.re < bounds.re && pos.im >= 0 && pos.im < bounds.im {
        map[pos.re as usize][pos.im as usize]
    } else {
        0
    }
}

fn visit_region(
    pos: Complex<isize>,
    map: &[[u8; MAP_SIZE]; MAP_SIZE],
    vis: &mut [[bool; MAP_SIZE]; MAP_SIZE],
    bounds: &Complex<isize>,
) -> (usize, usize, usize) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut corners = 0;
    let mut to_vis = vec![pos];
    let value = map[pos.re as usize][pos.im as usize];

    let dirs = [
        Complex::I,
        Complex::<isize>::I + Complex::ONE,
        Complex::ONE,
        Complex::<isize>::ONE - Complex::I,
        -Complex::I,
        -Complex::<isize>::I - Complex::ONE,
        -Complex::ONE,
        -Complex::<isize>::ONE + Complex::I,
        Complex::I,
    ];

    while let Some(pos) = to_vis.pop() {
        if vis[pos.re as usize][pos.im as usize] {
            continue;
        }
        vis[pos.re as usize][pos.im as usize] = true;
        area += 1;

        let mut same_val = [false; 9];
        for i in 0..dirs.len() {
            same_val[i] = value_at_pos(&(pos + dirs[i]), map, bounds) == value;
        }

        for neigh in [0, 2, 4, 6] {
            if same_val[neigh] {
                let next = pos + dirs[neigh];
                if !vis[next.re as usize][next.im as usize] {
                    to_vis.push(next);
                }
            } else {
                perimeter += 1;
            }
        }

        for n in [1, 3, 5, 7] {
            if !same_val[n - 1] && !same_val[n + 1]
                || same_val[n - 1] && same_val[n + 1] && !same_val[n]
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
        let mut bounds = 1.into();
        let mut vis = [[false; MAP_SIZE]; MAP_SIZE];

        self.data
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(row, vec)| {
                vec.as_bytes().iter().enumerate().for_each(|(col, v)| {
                    map[row][col] = *v;
                    bounds = Complex::new((row + 1) as isize, (col + 1) as isize);
                })
            });
        let reports: Vec<_> = (0..bounds.re)
            .into_iter()
            .flat_map(|row| {
                (0..bounds.im)
                    .into_iter()
                    .map(move |col: isize| Complex::new(row, col))
            })
            .map(|pos| visit_region(pos, &map, &mut vis, &bounds))
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
