use std::{collections::VecDeque, mem, ops::Rem};

use crate::Day;

const MAP_SIZE: usize = 141;
const UNVISITED_VAL: u32 = u32::MAX / 2;

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub struct Day16 {
    data: Option<Vec<String>>,
}

impl Day16 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day16 {
    fn solve(&self) -> (isize, isize) {
        let mut map = [[false; MAP_SIZE]; MAP_SIZE];
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, arr) in self
            .data
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| s.as_bytes())
            .enumerate()
        {
            for (col, c) in arr.iter().enumerate() {
                match c {
                    b'S' => {
                        start = (row as isize, col as isize);
                    }
                    b'E' => {
                        end = (row as isize, col as isize);
                    }
                    b'#' => {
                        map[row][col] = true;
                    }
                    _ => (),
                }
            }
        }

        // x -> y -> dir -> lowest cost to get there from start
        let mut min_costs = [[[UNVISITED_VAL; MAP_SIZE]; MAP_SIZE]; 4];

        // one heap for current number of turns and other for the +1 one
        let mut curr_queue = VecDeque::new();
        let mut next_queue = VecDeque::new();
        curr_queue.push_back((0_u32, start, 1));
        let mut p1_result = UNVISITED_VAL;

        // pseudo-Dijkstra
        while !curr_queue.is_empty() {
            while let Some((mut cost, mut pos, dir)) = curr_queue.pop_front() {
                if cost > p1_result {
                    continue;
                }

                // visit pos and all next ahead that are unvisited
                while !map[pos.0 as usize][pos.1 as usize]
                    && min_costs[dir][pos.0 as usize][pos.1 as usize] == UNVISITED_VAL
                {
                    min_costs[dir][pos.0 as usize][pos.1 as usize] = cost;
                    if pos == end {
                        p1_result = cost;
                        break;
                    }

                    // perform moves to left and right
                    for dir in [(dir + 1).rem(4), (dir - 1).rem(4)] {
                        let next = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
                        if min_costs[dir][pos.0 as usize][pos.1 as usize] == UNVISITED_VAL
                            && !map[next.0 as usize][next.1 as usize]
                        {
                            min_costs[dir][pos.0 as usize][pos.1 as usize] = cost + 1000;
                            next_queue.push_back((cost + 1001, next, dir));
                        }
                    }
                    cost += 1;
                    pos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
                }
            }

            mem::swap(&mut curr_queue, &mut next_queue);
        }

        // Part 2
        let mut dfs_heap = Vec::with_capacity(100);
        for (dir, dir_costs) in min_costs.iter().enumerate() {
            if p1_result == dir_costs[end.0 as usize][end.1 as usize] {
                dfs_heap.push((end, dir));
            }
        }

        let mut vis = [[[false; MAP_SIZE]; MAP_SIZE]; 4];
        while let Some((pos, dir)) = dfs_heap.pop() {
            if vis[dir][pos.0 as usize][pos.1 as usize] {
                continue;
            }
            vis[dir][pos.0 as usize][pos.1 as usize] = true;
            let cost = min_costs[dir][pos.0 as usize][pos.1 as usize];

            let rev_dir = (dir + 2).rem(4);
            let next = (pos.0 + DIRS[rev_dir].0, pos.1 + DIRS[rev_dir].1);
            for (add_cost, dir, next) in [
                (1, dir, next),
                (1000, (dir + 1).rem(4), pos),
                (1000, (dir - 1).rem(4), pos),
            ] {
                if min_costs[dir][next.0 as usize][next.1 as usize] + add_cost == cost {
                    dfs_heap.push((next, dir));
                }
            }
        }

        let mut p2_result = 0;
        for row in 0..MAP_SIZE {
            for col in 0..MAP_SIZE {
                if (0..4).any(|i| vis[i][row][col]) {
                    p2_result += 1;
                }
            }
        }

        (p1_result as isize, p2_result)
    }

    fn number(&self) -> u8 {
        16
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
