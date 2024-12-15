use num_complex::Complex;

use crate::Day;

const MAP_SIZE: usize = 50;

const DIRS: [Complex<isize>; 4] = [
    Complex { re: 1, im: 0 },
    Complex { re: 0, im: 1 },
    Complex { re: -1, im: 0 },
    Complex { re: 0, im: -1 },
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Box,
    BoxL,
    BoxR,
    Empty,
}

impl Cell {
    fn is_box(&self) -> bool {
        *self == Self::Box || *self == Self::BoxL || *self == Self::BoxR
    }
}

fn cell_at<T: Copy, const COLS: usize>(pos: &Complex<isize>, map: &[[T; COLS]; MAP_SIZE]) -> T {
    map[pos.re as usize][pos.im as usize]
}

fn mut_cell_at<'m, T, const COLS: usize>(
    pos: &Complex<isize>,
    map: &'m mut [[T; COLS]; MAP_SIZE],
) -> &'m mut T {
    &mut map[pos.re as usize][pos.im as usize]
}

fn perform_basic_move<const COLS: usize>(
    pos: &mut Complex<isize>,
    dir_idx: usize,
    map: &mut [[Cell; COLS]; MAP_SIZE],
) {
    let dir = DIRS[dir_idx];
    let mut next = *pos + dir;
    let mut obst = cell_at(&next, map);
    match obst {
        Cell::Wall => (),
        Cell::Empty => *pos = next,
        Cell::Box | Cell::BoxL | Cell::BoxR => {
            loop {
                next += dir;
                obst = cell_at(&next, map);
                if !obst.is_box() {
                    break;
                }
            }
            match obst {
                Cell::Wall => (),
                Cell::Empty => {
                    let mut prev = next;
                    let rev_dir = DIRS[(dir_idx + 2) % 4];
                    next += rev_dir;
                    while next != *pos {
                        *mut_cell_at(&prev, map) = cell_at(&next, map);
                        prev = next;
                        next += rev_dir;
                    }
                    *pos += dir;
                    *mut_cell_at(pos, map) = Cell::Empty;
                }
                Cell::Box | Cell::BoxL | Cell::BoxR => panic!("Cannot be a box"),
            }
        }
    }
}

fn perform_doubles_move(
    pos: &mut Complex<isize>,
    dir_idx: usize,
    time: isize,
    map: &mut [[Cell; 2 * MAP_SIZE]; MAP_SIZE],
    heap: &mut Vec<Complex<isize>>,
    visit_times: &mut [[isize; 2 * MAP_SIZE]; MAP_SIZE],
) {
    let dir = DIRS[dir_idx];
    heap.push(*pos + dir);
    let mut i = 0;
    while i < heap.len() {
        let pos = heap[i];
        i += 1;
        let other_dir = match cell_at(&pos, map) {
            Cell::Wall => {
                heap.clear();
                return;
            }
            Cell::Empty => continue,
            Cell::BoxL => Complex::I,
            Cell::BoxR => -Complex::I,
            Cell::Box => panic!("Don't do wide moves on basic boxes"),
        };
        let next = pos + dir;
        if cell_at(&next, visit_times) != time {
            heap.push(next);
            *mut_cell_at(&next, visit_times) = time;
        }

        let next = pos + dir + other_dir;
        if cell_at(&next, visit_times) != time {
            heap.push(next);
            *mut_cell_at(&next, visit_times) = time;
        }
    }

    for empty_pos in heap.drain(..).rev() {
        *mut_cell_at(&empty_pos, map) = cell_at(&(empty_pos - dir), map);
        *mut_cell_at(&(empty_pos - dir), map) = Cell::Empty
    }
    *pos += dir;
}

fn eval_map<const COLS: usize>(map: &[[Cell; COLS]; MAP_SIZE]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(row, arr)| {
            arr.iter()
                .enumerate()
                .map(move |(col, cell)| (row, col, *cell))
        })
        .filter(|(_, _, c)| *c == Cell::Box || *c == Cell::BoxL)
        .map(|(row, col, _)| 100 * row + col)
        .sum()
}

fn p1(mut pos: Complex<isize>, mut map: [[Cell; MAP_SIZE]; MAP_SIZE], moves: &[usize]) -> isize {
    for dir_idx in moves {
        perform_basic_move(&mut pos, *dir_idx, &mut map);
    }
    eval_map(&map) as isize
}

fn p2(
    mut pos: Complex<isize>,
    mut map: [[Cell; 2 * MAP_SIZE]; MAP_SIZE],
    moves: &[usize],
) -> isize {
    let mut heap = Vec::with_capacity(40);
    let mut visit_times = [[-1; 2 * MAP_SIZE]; MAP_SIZE];
    for (time, dir_idx) in moves.iter().enumerate() {
        match dir_idx {
            1 | 3 => perform_basic_move(&mut pos, *dir_idx, &mut map),
            0 | 2 => perform_doubles_move(
                &mut pos,
                *dir_idx,
                time as isize,
                &mut map,
                &mut heap,
                &mut visit_times,
            ),
            _ => panic!("Invalid direction"),
        }
    }
    eval_map(&map) as isize
}

pub struct Day15 {
    data: Option<Vec<String>>,
}

impl Day15 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day15 {
    fn solve(&self) -> (isize, isize) {
        let mut map_p1 = [[Cell::Empty; MAP_SIZE]; MAP_SIZE];
        let mut map_p2 = [[Cell::Empty; MAP_SIZE * 2]; MAP_SIZE];
        let mut pos = Complex::<isize>::ZERO;
        let mut moves = Vec::<usize>::with_capacity(20000);

        let mut iter = self.data.as_ref().unwrap().iter().map(|s| s.as_bytes());

        for row in 0..MAP_SIZE {
            for (col, c) in iter.next().unwrap().iter().enumerate() {
                match c {
                    b'O' => {
                        map_p1[row][col] = Cell::Box;
                        map_p2[row][2 * col] = Cell::BoxL;
                        map_p2[row][2 * col + 1] = Cell::BoxR;
                    }
                    b'#' => {
                        map_p1[row][col] = Cell::Wall;
                        map_p2[row][2 * col] = Cell::Wall;
                        map_p2[row][2 * col + 1] = Cell::Wall;
                    }
                    b'@' => pos = Complex::new(row as isize, col as isize),
                    _ => (),
                }
            }
        }
        let pos2 = Complex::new(pos.re, 2 * pos.im);

        for r in iter {
            for b in r {
                let dir_idx = match b {
                    b'v' => 0,
                    b'>' => 1,
                    b'^' => 2,
                    b'<' => 3,
                    _ => panic!("Unrecognized move"),
                };
                moves.push(dir_idx);
            }
        }

        (p1(pos, map_p1, &moves), p2(pos2, map_p2, &moves))
    }

    fn number(&self) -> u8 {
        15
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
