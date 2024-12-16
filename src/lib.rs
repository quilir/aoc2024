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
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

mod utils;

pub fn all_days() -> Vec<Box<dyn Day>> {
    vec![
        day01::Day01::new(),
        day02::Day02::new(),
        day03::Day03::new(),
        day04::Day04::new(),
        day05::Day05::new(),
        day06::Day06::new(),
        day07::Day07::new(),
        day08::Day08::new(),
        day09::Day09::new(),
        day10::Day10::new(),
        day11::Day11::new(),
        day12::Day12::new(),
        day13::Day13::new(),
        day14::Day14::new(),
        day15::Day15::new(),
        day16::Day16::new(),
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
    use std::time;

    use crate::all_days;

    #[test]
    fn test_all() {
        let begin = time::Instant::now();
        for mut day in all_days() {
            day.describe();
        }
        println!("{:?}", time::Instant::now() - begin);
    }
}
