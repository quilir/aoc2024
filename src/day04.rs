use crate::Day;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIAGONAL_PAIRS: [[(isize, isize); 2]; 2] = [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]];

fn in_bounds(x: isize, y: isize, x_bound: isize, y_bound: isize) -> bool {
    x >= 0 && y >= 0 && x < x_bound && y < y_bound
}

fn p1(matrix: &[Vec<u8>]) -> isize {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let mut found = 0;
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row as usize][col as usize] != b'X' {
                continue;
            }

            for (dr, dc) in DIRECTIONS {
                if !in_bounds(row + 3 * dr, col + 3 * dc, rows, cols) {
                    continue;
                }
                let mut curr_row = row;
                let mut curr_col = col;
                let mut ok = true;
                for char in [b'M', b'A', b'S'] {
                    curr_row += dr;
                    curr_col += dc;
                    if matrix[curr_row as usize][curr_col as usize] != char {
                        ok = false;
                        break;
                    }
                }
                found += ok as isize;
            }
        }
    }
    found
}

fn p2(matrix: &[Vec<u8>]) -> isize {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let mut found = 0;
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if matrix[row as usize][col as usize] != b'A' {
                continue;
            }

            let mut ok = true;
            for pair in DIAGONAL_PAIRS {
                let char1 = matrix[(row + pair[0].0) as usize][(col + pair[0].1) as usize];
                let char2 = matrix[(row + pair[1].0) as usize][(col + pair[1].1) as usize];

                if !(char1 == b'M' && char2 == b'S' || char1 == b'S' && char2 == b'M') {
                    ok = false;
                    break;
                }
            }
            found += ok as isize;
        }
    }
    found
}
pub struct Day04 {
    data: Option<Vec<String>>,
}

impl Day04 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day04 {
    fn solve(&self) -> (isize, isize) {
        let vec: Vec<_> = self
            .data
            .clone()
            .unwrap()
            .into_iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();

        (p1(&vec), p2(&vec))
    }

    fn number(&self) -> u8 {
        4
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
