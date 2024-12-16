use std::{cmp::Reverse, ops::Rem};

use crate::Day;

const MAP_SIZE: usize = 141;

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
        let mut min_costs = [[[usize::MAX / 2; MAP_SIZE]; MAP_SIZE]; 4];
        // faster alternative to a single (DFS-)heap
        // one (DFS-)heap for each number of turns (cost % 1000)
        let mut tiered_heap = vec![Vec::new(); 300];
        tiered_heap[0].push((Reverse(0), start, 1));
        let mut p1_result = usize::MAX / 2;

        for turn in 0..tiered_heap.len() {
            while let Some((Reverse(cost), pos, dir)) = tiered_heap[turn].pop() {
                if cost > min_costs[dir][pos.0 as usize][pos.1 as usize] || cost > p1_result {
                    continue;
                }
                min_costs[dir][pos.0 as usize][pos.1 as usize] = cost;

                if pos == end {
                    p1_result = cost;
                    continue;
                }

                let next = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);
                for (add_cost, dir, next) in [
                    (1, dir, next),
                    (1000, (dir + 1).rem(4), pos),
                    (1000, (dir - 1).rem(4), pos),
                ] {
                    if !map[next.0 as usize][next.1 as usize]
                        && min_costs[dir][next.0 as usize][next.1 as usize] > add_cost + cost
                    {
                        tiered_heap[(add_cost + cost) / 1000].push((
                            Reverse(add_cost + cost),
                            next,
                            dir,
                        ));
                    }
                }
            }
        }

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
