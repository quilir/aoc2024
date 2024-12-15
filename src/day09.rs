use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Day;

const TRIANGLE_NUM: [isize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

#[inline]
fn calc_segment_val(val: isize, pos: isize, len: isize) -> isize {
    val * (pos * len + TRIANGLE_NUM[len as usize])
}

fn p1(data: &[(isize, isize)]) -> isize {
    let mut i0 = 1;
    let mut i1 = data.len() - 1;
    let mut checksum = 0;

    let (mut space_pos, mut space_len) = data[i0];
    let (mut data_pos, mut data_len) = data[i1];

    while i0 < i1 {
        let swap_len = std::cmp::min(space_len, data_len);
        checksum += calc_segment_val((i1 / 2) as isize, space_pos, swap_len);

        space_len -= swap_len;
        data_len -= swap_len;
        space_pos += swap_len;

        if space_len == 0 {
            i0 += 2;
            (space_pos, space_len) = data[i0];
        }
        if data_len == 0 {
            i1 -= 2;
            (data_pos, data_len) = data[i1];
        }
    }
    while i1 > 0 {
        checksum += calc_segment_val((i1 / 2) as isize, data_pos, data_len);
        i1 -= 2;
        (data_pos, data_len) = data[i1];
    }
    checksum
}

#[inline]
fn build_heaps(data: &[(isize, isize)]) -> Vec<BinaryHeap<Reverse<isize>>> {
    let mut i = 1;
    let mut heaps = vec![BinaryHeap::with_capacity(1000); 10];
    while i < data.len() {
        let (pos, len) = data[i];
        heaps[len as usize].push(Reverse(pos));
        i += 2;
    }

    heaps
}

#[inline]
fn attempt_segment_move(
    pos: isize,
    len: isize,
    val: isize,
    spaces: &mut [BinaryHeap<Reverse<isize>>],
    checksum: &mut isize,
) {
    if let Some((space_pos, space_size)) = (len..10)
        .filter_map(|size| (spaces[size as usize].peek().map(|Reverse(v)| (*v, size))))
        .min()
    {
        if space_pos > pos {
            *checksum += calc_segment_val(val, pos, len);
            return;
        }

        *checksum += calc_segment_val(val, space_pos, len);
        spaces[space_size as usize].pop();
        if space_size - len > 0 {
            spaces[(space_size - len) as usize].push(Reverse(space_pos + len));
        }
    } else {
        *checksum += calc_segment_val(val, pos, len);
    }
}

fn p2(data: &[(isize, isize)]) -> isize {
    let mut spaces = build_heaps(data);
    let mut checksum = 0;
    let mut i = data.len() - 1;

    while i > 0 {
        let (pos, len) = data[i];
        attempt_segment_move(pos, len, (i / 2) as isize, &mut spaces, &mut checksum);
        i -= 2;
    }
    checksum
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
        let mut pos = 0;
        let data = self.data.as_ref().unwrap()[0]
            .as_bytes()
            .iter()
            .map(|char| {
                let len = (char - b'0') as isize;
                pos += len;
                (pos - len, len)
            })
            .collect::<Vec<_>>();

        (p1(&data), p2(&data))
    }

    fn number(&self) -> u8 {
        9
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
