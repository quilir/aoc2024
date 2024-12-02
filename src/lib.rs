use std::fs::File;

mod day01;
mod day02;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![day01::Day01::new(), day02::Day02::new()]
}

pub trait Day {
    fn number(&self) -> u8;

    fn input_file(&mut self) -> File {
        File::open(format!("data/day{:02}.txt", self.number())).unwrap()
    }

    fn load_data(&mut self);

    fn solve(&self) -> (i32, i32);

    fn describe(&mut self) {
        self.load_data();
        let (p1, p2) = self.solve();
        println!("Test {:02}", self.number());
        println!("  p1: {p1}");
        println!("  p2: {p2}");
    }
}

#[cfg(test)]
mod tests {
    use crate::all_days;

    #[test]
    fn test_all() {
        for mut day in all_days() {
            day.describe();
        }
    }
}
