use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Day;

fn checksum(arr: &[i32]) -> isize {
    let mut res = 0;
    for (idx, v) in arr.iter().enumerate() {
        if *v != -1 {
            res += *v as isize * idx as isize;
        }
    }

    res
}

fn p1(data: &[i32]) -> isize {
    let mut data = data.to_vec();
    let mut i0 = 0;
    let mut i1 = data.len() - 1;

    loop {
        while i0 < data.len() - 1 && data[i0] >= 0 {
            i0 += 1;
        }
        while i1 > 0 && data[i1] < 0 {
            i1 -= 1;
        }

        if i0 < i1 {
            data.swap(i0, i1);
        } else {
            break;
        }
    }
    checksum(&data)
}

fn find_spaces(data: &[i32]) -> Vec<BinaryHeap<Reverse<usize>>> {
    let mut heaps = vec![BinaryHeap::new(); 10];
    let mut space_len = 0;
    for (idx, v) in data.iter().enumerate() {
        if *v == -1 {
            space_len += 1;
        } else if space_len > 0 {
            heaps[space_len].push(Reverse(idx - space_len));
            space_len = 0;
        }
    }

    heaps
}

fn attempt_segment_move(
    size: usize,
    pos: usize,
    spaces: &mut [BinaryHeap<Reverse<usize>>],
    data: &mut [i32],
) {
    if let Some((Reverse(space_pos), space_size)) = (size..10)
        .map(|size| (spaces[size].peek().copied(), size))
        .filter(|(res, _)| res.is_some())
        .map(|(res, size)| (res.unwrap(), size))
        .min_by_key(|(Reverse(pos), _)| *pos)
    {
        if space_pos > pos {
            return;
        }
        spaces[space_size].pop();
        let num = data[pos];
        for j in 0..size {
            data[space_pos + j] = num;
            data[pos + j] = -1;
        }
        if space_size - size > 0 {
            spaces[space_size - size].push(Reverse(space_pos + size));
        }
    }
}

fn p2(data: &[i32]) -> isize {
    let mut data_mut = data.to_vec();
    let mut spaces = find_spaces(data);
    let mut prev = -1;
    let mut size = 0;

    for (idx, v) in data.iter().enumerate().rev() {
        if *v != prev {
            if prev != -1 {
                attempt_segment_move(size, idx + 1, &mut spaces, &mut data_mut);
            }
            size = 0;
        }
        prev = *v;
        size += 1;
    }
    checksum(&data_mut)
}

pub struct Day09 {
    data: Option<Vec<String>>,
}

impl Day09 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day09 {
    fn solve(&self) -> (isize, isize) {
        let mut vec = Vec::new();
        let mut block = true;
        let mut next_val = 0;

        for char in self.data.as_ref().unwrap()[0].as_bytes() {
            let num = char - b'0';
            if block {
                (0..num).for_each(|_| vec.push(next_val));
                next_val += 1;
            } else {
                (0..num).for_each(|_| vec.push(-1));
            }
            block = !block;
        }

        (p1(&vec), p2(&vec))
    }

    fn number(&self) -> u8 {
        9
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
