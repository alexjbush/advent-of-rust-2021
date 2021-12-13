use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod utils;

pub trait Day<'a> {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)>;
    fn get_day_number(&self) -> usize;
}

fn get_days<'a>() -> Vec<&'a dyn Day<'a>> {
    vec![
        &Day1 {},
        &Day2 {},
        &Day3 {},
        &Day4 {},
        &Day5 {},
        &Day6 {},
        &Day7 {},
        &Day8 {},
        &Day9 {},
    ]
}

pub fn run() {
    get_days().iter().for_each(|d| benchmark(*d))
}

fn benchmark(day: &dyn Day) {
    println!("Day: {}", day.get_day_number());
    day.get_tasks().iter().for_each(|t| {
        let now = Instant::now();
        let res = t.1();
        let elapsed = now.elapsed();
        println!(
            "Finished task: {}, result: {}, elapsed: {:.2?}",
            t.0, res, elapsed
        )
    });
}
