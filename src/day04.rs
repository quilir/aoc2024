use crate::Day;

const MAP_SIZE: usize = 140 + 6;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 1), (0, 1), (1, 0), (1, 1)];
const DIAGONAL_PAIRS: [[(isize, isize); 2]; 2] = [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]];

fn p1(matrix: &[[u8; MAP_SIZE]; MAP_SIZE]) -> isize {
    let mut found = 0;
    for row in 3..(MAP_SIZE - 3) as isize {
        for col in 3..(MAP_SIZE - 3) as isize {
            for (dr, dc) in DIRECTIONS {
                let mut res: usize = 0;
                for i in 0..4 {
                    res <<= 8;
                    res += matrix[(row + dr * i) as usize][(col + dc * i) as usize] as usize;
                }
                if res == 0x584d4153 || res == 0x53414d58 {
                    found += 1;
                }
            }
        }
    }
    found
}

fn p2(matrix: &[[u8; MAP_SIZE]; MAP_SIZE]) -> isize {
    let mut found = 0;
    for row in 4..(MAP_SIZE - 4) as isize {
        for col in 4..(MAP_SIZE - 4) as isize {
            if matrix[row as usize][col as usize] != b'A' {
                continue;
            }

            let mut ok = true;
            for pair in DIAGONAL_PAIRS {
                let char1 = matrix[(row + pair[0].0) as usize][(col + pair[0].1) as usize] as i8;
                let char2 = matrix[(row + pair[1].0) as usize][(col + pair[1].1) as usize] as i8;

                if (char1 - char2).abs() != 6 {
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
        let mut map = [[b'.'; MAP_SIZE]; MAP_SIZE];
        self.data
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(row, s)| {
                s.as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(col, v)| map[row + 3][col + 3] = *v)
            });

        (p1(&map), p2(&map))
    }

    fn number(&self) -> u8 {
        4
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
