use std::cmp::Ordering;

use crate::{utils::parse_usize, Day};

const MAX_NODES: usize = 100;

fn order_row(graph: &[[bool; MAX_NODES]; MAX_NODES], row: &[usize]) -> Vec<usize> {
    let mut row = row.to_vec();
    row.sort_by(|x, y| {
        if graph[*x][*y] {
            Ordering::Less
        } else if graph[*y][*x] {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    row
}

fn p1(rows: &[Vec<usize>], ordered_rows: &[Vec<usize>]) -> isize {
    rows.iter()
        .zip(ordered_rows)
        .filter(|(r, or)| r == or)
        .map(|(r, _)| r[r.len() / 2])
        .sum::<usize>() as isize
}

fn p2(rows: &[Vec<usize>], ordered_rows: &[Vec<usize>]) -> isize {
    rows.iter()
        .zip(ordered_rows)
        .filter(|(r, or)| r != or)
        .map(|(_, r)| r[r.len() / 2])
        .sum::<usize>() as isize
}
pub struct Day05 {
    data: Option<Vec<String>>,
}

impl Day05 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day05 {
    fn solve(&self) -> (isize, isize) {
        let mut graph = [[false; MAX_NODES]; MAX_NODES];
        let mut rows = Vec::new();
        let mut ordered_rows = Vec::new();

        let mut lines = self.data.as_ref().unwrap().iter();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut pair = line.split('|');
            graph[parse_usize(pair.next().unwrap())][parse_usize(pair.next().unwrap())] = true;
        }

        for line in lines {
            let mut row = Vec::new();
            for s in line.split(',') {
                row.push(parse_usize(s));
            }
            ordered_rows.push(order_row(&graph, &row));
            rows.push(row);
        }

        (p1(&rows, &ordered_rows), p2(&rows, &ordered_rows))
    }

    fn number(&self) -> u8 {
        5
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
