use std::collections::VecDeque;

use crate::Day;

const MAP_SIZE: usize = 71;
const MAX_IDX: usize = MAP_SIZE - 1;
const PART_1_TIME: u16 = 1024;

fn path_len(
    time: u16,
    map: &[[u16; MAP_SIZE]; MAP_SIZE],
    queue: &mut VecDeque<(usize, usize)>,
) -> Option<isize> {
    let mut map = *map;
    let mut dist = 0;
    queue.push_back((0, 0));
    let token = (usize::MAX, usize::MAX);
    queue.push_back(token);

    while queue.len() > 1 {
        match queue.pop_front().unwrap() {
            (usize::MAX, usize::MAX) => {
                dist += 1;
                queue.push_back(token);
            }
            (MAX_IDX, MAX_IDX) => {
                return Some(dist);
            }
            (row, col) => {
                if row > 0 && map[row - 1][col] > time {
                    map[row - 1][col] = 0;
                    queue.push_back((row - 1, col));
                }
                if row < MAX_IDX && map[row + 1][col] > time {
                    map[row + 1][col] = 0;
                    queue.push_back((row + 1, col));
                }
                if col > 0 && map[row][col - 1] > time {
                    map[row][col - 1] = 0;
                    queue.push_back((row, col - 1));
                }
                if col < MAX_IDX && map[row][col + 1] > time {
                    map[row][col + 1] = 0;
                    queue.push_back((row, col + 1));
                }
            }
        }
    }
    None
}

pub struct Day18 {
    data: Option<Vec<String>>,
}

impl Day18 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day18 {
    fn solve(&self) -> (isize, isize) {
        let mut map = [[u16::MAX; MAP_SIZE]; MAP_SIZE];
        let iter = self.data.as_ref().unwrap().iter();
        let bytes = iter.len();

        iter.enumerate().for_each(|(time, str)| {
            let arr = str.as_bytes();
            let mut col = 0;
            let mut row = 0;
            let mut i = 0;
            while arr[i] != b',' {
                col = col * 10 + arr[i] - b'0';
                i += 1;
            }
            i += 1;
            while i < arr.len() {
                row = row * 10 + arr[i] - b'0';
                i += 1;
            }
            map[row as usize][col as usize] = time as u16;
        });

        let mut queue = VecDeque::with_capacity(MAP_SIZE);
        let p1_res = path_len(PART_1_TIME, &map, &mut queue).unwrap();

        let mut low = PART_1_TIME;
        let mut high = bytes as u16;
        while low < high {
            let mid = low + (high - low) / 2;

            queue.clear();
            if path_len(mid, &map, &mut queue).is_some() {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        let mut p2_res = 0;
        for (r, row) in map.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                if *v == low {
                    p2_res = 1000 * c + r;
                    break;
                }
            }
            if p2_res != 0 {
                break;
            }
        }

        (p1_res as isize, p2_res as isize)
    }

    fn number(&self) -> u8 {
        18
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
