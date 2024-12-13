use crate::{utils::parse_isize, Day};

// Xa, Ya, Xb, Yb, X, Y
type Input = (isize, isize, isize, isize, isize, isize);

fn p1(configs: &[Input]) -> isize {
    configs
        .iter()
        .map(|(a, c, b, d, x, y)| {
            let inverse_deter = a * d - c * b;

            let x_nom = d * x - b * y;
            let y_nom = -c * x + a * y;

            if x_nom % inverse_deter == 0 && y_nom % inverse_deter == 0 {
                (x_nom * 3 + y_nom) / inverse_deter
            } else {
                0
            }
        })
        .sum()
}

fn p2(configs: &[Input]) -> isize {
    configs
        .iter()
        .map(|(a, c, b, d, x, y)| {
            let x = x + 10000000000000;
            let y = y + 10000000000000;

            let inverse_deter = a * d - c * b;

            let x_nom = d * x - b * y;
            let y_nom = -c * x + a * y;

            if x_nom % inverse_deter == 0 && y_nom % inverse_deter == 0 {
                (x_nom * 3 + y_nom) / inverse_deter
            } else {
                0
            }
        })
        .sum()
}

pub struct Day13 {
    data: Option<Vec<String>>,
}

impl Day13 {
    pub fn new() -> Box<Self> {
        Box::new(Self { data: None })
    }
}

impl Day for Day13 {
    fn solve(&self) -> (isize, isize) {
        let mut configs = Vec::with_capacity(430);

        let vec_data: Vec<_> = self.data.as_ref().unwrap().iter().collect();
        for batch in 0..vec_data.len() / 4 + 1 {
            let xa = parse_isize(&vec_data[batch * 4][12..14]);
            let ya = parse_isize(&vec_data[batch * 4][18..20]);
            let xb = parse_isize(&vec_data[batch * 4 + 1][12..14]);
            let yb = parse_isize(&vec_data[batch * 4 + 1][18..20]);

            let mut split = vec_data[batch * 4 + 2].split([',', '=']);
            let x = parse_isize(split.nth(1).unwrap());
            let y = parse_isize(split.nth(1).unwrap());

            configs.push((xa, ya, xb, yb, x, y));
        }

        (p1(&configs), p2(&configs))
    }

    fn number(&self) -> u8 {
        13
    }

    fn cache_input(&mut self, vec: Vec<String>) {
        self.data = Some(vec);
    }
}
