use crate::Day;

const MAP_SIZE: usize = 130;
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn in_bounds(pos: &(isize, isize), bounds: &(isize, isize)) -> bool {
    pos.0 >= 0 && pos.0 < bounds.0 && pos.1 >= 0 && pos.1 < bounds.1
}

fn in_loop(
    map: &[[bool; MAP_SIZE]; MAP_SIZE],
    pos: &(isize, isize),
    dir_idx: &usize,
    bounds: &(isize, isize),
) -> bool {
    let mut pos = pos.clone();
    let mut dir_idx = dir_idx.clone();

    let mut visited = [[[false; MAP_SIZE]; MAP_SIZE]; DIRS.len()];
    let mut dir = DIRS[dir_idx];

    loop {
        if visited[dir_idx][pos.0 as usize][pos.1 as usize] {
            return true;
        }
        visited[dir_idx][pos.0 as usize][pos.1 as usize] = true;
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if !in_bounds(&next, &bounds) {
            return false;
        }
        if map[next.0 as usize][next.1 as usize] {
            dir_idx = (dir_idx + 1) % DIRS.len();
            dir = DIRS[dir_idx];
        } else {
            pos = next;
        }
    }
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

fn p2(
    mut map: [[bool; MAP_SIZE]; MAP_SIZE],
    mut pos: (isize, isize),
    bounds: (isize, isize),
) -> isize {
    let mut visited = [[false; MAP_SIZE]; MAP_SIZE];

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
                count += in_loop(&map, &pos, &dir_idx, &bounds) as isize;
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
