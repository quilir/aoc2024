use std::collections::HashSet;

use rustc_hash::{FxBuildHasher, FxHashSet};

use crate::Day;

const MAP_SIZE: usize = 130;
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(pos: &(isize, isize), bounds: &(isize, isize)) -> bool {
    pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1
}

#[inline]
fn hop(
    pos: &mut (isize, isize),
    dir_idx: &mut usize,
    blanks: &[[[u8; 130]; 130]; DIRS.len()],
    wall_pos: &(isize, isize),
) {
    let blanks_count = blanks[*dir_idx][pos.0 as usize][pos.1 as usize] as isize;
    let blanks_count = match dir_idx {
        0 => {
            if pos.1 == wall_pos.1 && pos.0 > wall_pos.0 && pos.0 - blanks_count <= wall_pos.0 {
                pos.0 - wall_pos.0 - 1
            } else {
                blanks_count
            }
        }
        1 => {
            if pos.0 == wall_pos.0 && pos.1 < wall_pos.1 && pos.1 + blanks_count >= wall_pos.1 {
                wall_pos.1 - pos.1 - 1
            } else {
                blanks_count
            }
        }
        2 => {
            if pos.1 == wall_pos.1 && pos.0 < wall_pos.0 && pos.0 + blanks_count >= wall_pos.0 {
                wall_pos.0 - pos.0 - 1
            } else {
                blanks_count
            }
        }
        3 => {
            if pos.0 == wall_pos.0 && pos.1 > wall_pos.1 && pos.1 - blanks_count <= wall_pos.1 {
                pos.1 - wall_pos.1 - 1
            } else {
                blanks_count
            }
        }
        _ => panic!(),
    };

    if blanks_count != 0 {
        *pos = (
            pos.0 + DIRS[*dir_idx].0 * blanks_count as isize,
            pos.1 + DIRS[*dir_idx].1 * blanks_count as isize,
        );
    }

    *dir_idx = (*dir_idx + 1) % DIRS.len();
}

#[inline]
fn in_loop(
    blanks: &[[[u8; MAP_SIZE]; MAP_SIZE]; DIRS.len()],
    pos: &(isize, isize),
    dir_idx: &usize,
    bounds: &(isize, isize),
    vis_set: &mut HashSet<((isize, isize), usize), FxBuildHasher>,
) -> bool {
    let mut pos = *pos;
    let mut dir_idx = *dir_idx;

    let wall_pos = (pos.0 + DIRS[dir_idx].0, pos.1 + DIRS[dir_idx].1);

    while in_bounds(&pos, bounds) {
        if !vis_set.insert((pos, dir_idx)) {
            return true;
        }
        hop(&mut pos, &mut dir_idx, blanks, &wall_pos);
    }

    false
}

fn p1(
    map: &[[bool; MAP_SIZE]; MAP_SIZE],
    mut pos: (isize, isize),
    bounds: (isize, isize),
) -> isize {
    let mut visited = [[false; MAP_SIZE]; MAP_SIZE];
    let mut dir_idx = 0;
    let mut dir = DIRS[dir_idx];

    let mut count = 0;
    loop {
        if !visited[pos.0 as usize][pos.1 as usize] {
            visited[pos.0 as usize][pos.1 as usize] = true;
            count += 1;
        }
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if !in_bounds(&next, &bounds) {
            break;
        }
        if map[next.0 as usize][next.1 as usize] {
            dir_idx = (dir_idx + 1) % DIRS.len();
            dir = DIRS[dir_idx];
        } else {
            pos = next;
        }
    }

    count
}

fn calc_blanks_row(
    row: isize,
    map: &[[bool; MAP_SIZE]; MAP_SIZE],
    bounds: &(isize, isize),
    blanks: &mut [[[u8; MAP_SIZE]; MAP_SIZE]; DIRS.len()],
) {
    let mut count = 1;
    for col in (0..bounds.0).rev() {
        blanks[1][row as usize][col as usize] = count;
        if map[row as usize][col as usize] {
            count = 0;
        } else {
            count += 1;
        }
    }

    let mut count = 1;
    for col in 0..bounds.0 {
        blanks[3][row as usize][col as usize] = count;
        if map[row as usize][col as usize] {
            count = 0;
        } else {
            count += 1;
        }
    }
}

fn calc_blanks_col(
    col: isize,
    map: &[[bool; MAP_SIZE]; MAP_SIZE],
    bounds: &(isize, isize),
    blanks: &mut [[[u8; MAP_SIZE]; MAP_SIZE]; DIRS.len()],
) {
    let mut count = 1;
    for row in 0..bounds.0 {
        blanks[0][row as usize][col as usize] = count;
        if map[row as usize][col as usize] {
            count = 0;
        } else {
            count += 1;
        }
    }

    let mut count = 1;
    for row in (0..bounds.0).rev() {
        blanks[2][row as usize][col as usize] = count;
        if map[row as usize][col as usize] {
            count = 0;
        } else {
            count += 1;
        }
    }
}

fn gen_blanks_table(
    map: &[[bool; MAP_SIZE]; MAP_SIZE],
    bounds: &(isize, isize),
) -> [[[u8; MAP_SIZE]; MAP_SIZE]; DIRS.len()] {
    let mut blanks = [[[0_u8; MAP_SIZE]; MAP_SIZE]; DIRS.len()];
    for col in 0..bounds.1 {
        calc_blanks_col(col, map, bounds, &mut blanks);
    }
    for row in 0..bounds.1 {
        calc_blanks_row(row, map, bounds, &mut blanks);
    }
    blanks
}

fn p2(
    mut map: [[bool; MAP_SIZE]; MAP_SIZE],
    mut pos: (isize, isize),
    bounds: (isize, isize),
) -> isize {
    let mut visited = [[false; MAP_SIZE]; MAP_SIZE];
    let blanks = gen_blanks_table(&map, &bounds);

    let mut count = 0;
    let mut dir_idx = 0;
    let mut dir = DIRS[dir_idx];

    let mut vis_set = FxHashSet::with_capacity_and_hasher(1000, FxBuildHasher);

    loop {
        visited[pos.0 as usize][pos.1 as usize] = true;
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if !in_bounds(&next, &bounds) {
            break;
        }
        if map[next.0 as usize][next.1 as usize] {
            dir_idx = (dir_idx + 1) % DIRS.len();
            dir = DIRS[dir_idx];
        } else {
            if !visited[next.0 as usize][next.1 as usize] {
                map[next.0 as usize][next.1 as usize] = true;

                count += in_loop(&blanks, &pos, &dir_idx, &bounds, &mut vis_set) as isize;
                vis_set.clear();

                map[next.0 as usize][next.1 as usize] = false;
            }

            pos = next;
        }
    }

    count
}

pub struct Day06 {
    data: Option<Vec<String>>,
}

impl Day06 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day06 {
    fn solve(&self) -> (isize, isize) {
        let mut map = [[false; MAP_SIZE]; MAP_SIZE];

        let lines = self
            .data
            .as_ref()
            .unwrap()
            .iter()
            .map(String::as_bytes)
            .enumerate();

        let rows = lines.len() as isize;
        let mut cols = 0;

        let mut guard_pos = (0, 0);

        for (row, line) in lines {
            cols = line.len() as isize;

            for (col, char) in line.iter().enumerate() {
                map[row][col] = match char {
                    b'#' => true,
                    b'.' => false,
                    b'^' => {
                        guard_pos = (row as isize, col as isize);
                        false
                    }
                    c => panic!("Unknown char: {c}"),
                }
            }
        }

        (
            p1(&map, guard_pos, (rows, cols)),
            p2(map, guard_pos, (rows, cols)),
        )
    }

    fn number(&self) -> u8 {
        6
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
