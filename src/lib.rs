use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

mod utils;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![
        day01::Day01::new(),
        day02::Day02::new(),
        day03::Day03::new(),
        day04::Day04::new(),
        day05::Day05::new(),
        day06::Day06::new(),
    ]
}

pub trait Day {
    /// Day number
    fn number(&self) -> u8;

    /// Reads from file and saves adequate input data
    ///
    /// Benchmarks call this method before calling `solve`, excluding time spent on reading from file
    /// from execution time
    fn load_input(&mut self) {
        let file = File::open(format!("data/day{:02}.txt", self.number())).unwrap();
        self.cache_input(
            BufReader::new(file)
                .lines()
                .collect::<Result<Vec<String>, _>>()
                .unwrap(),
        );
    }

    /// Solves the task, returning tuple of results: for the 1st part, and  for the 2nd part
    fn solve(&self) -> (isize, isize);

    /// Saves data in a local field
    ///
    /// No data processing should be performed there
    fn cache_input(&mut self, data: Vec<String>);

    /// Solves the task and prints out results
    fn describe(&mut self) {
        self.load_input();
        let (p1, p2) = self.solve();
        println!("Day {:02}", self.number());
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
