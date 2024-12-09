use crate::Day;

const MAP_SIZE: usize = 130;
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(pos: &(isize, isize), bounds: &(isize, isize)) -> bool {
    pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1
}

fn in_loop(
    blanks: &[[[u8; MAP_SIZE]; MAP_SIZE]; DIRS.len()],
    pos: &(isize, isize),
    dir_idx: &usize,
    bounds: &(isize, isize),
) -> bool {
    let mut pos = *pos;
    let mut dir_idx = *dir_idx;

    let mut visited = [[[false; MAP_SIZE]; MAP_SIZE]; DIRS.len()];
    let mut dir = DIRS[dir_idx];

    while in_bounds(&pos, bounds) {
        if visited[dir_idx][pos.0 as usize][pos.1 as usize] {
            return true;
        }
        visited[dir_idx][pos.0 as usize][pos.1 as usize] = true;
        let blanks_count = blanks[dir_idx][pos.0 as usize][pos.1 as usize];

        if blanks_count != 0 {
            pos = (
                pos.0 + dir.0 * blanks_count as isize,
                pos.1 + dir.1 * blanks_count as isize,
            );
        }

        dir_idx = (dir_idx + 1) % DIRS.len();
        dir = DIRS[dir_idx];
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
    let mut blanks = gen_blanks_table(&map, &bounds);

    let mut count = 0;
    let mut dir_idx = 0;
    let mut dir = DIRS[dir_idx];

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
                calc_blanks_row(next.0, &map, &bounds, &mut blanks);
                calc_blanks_col(next.1, &map, &bounds, &mut blanks);

                count += in_loop(&blanks, &pos, &dir_idx, &bounds) as isize;

                map[next.0 as usize][next.1 as usize] = false;
                calc_blanks_row(next.0, &map, &bounds, &mut blanks);
                calc_blanks_col(next.1, &map, &bounds, &mut blanks);
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
