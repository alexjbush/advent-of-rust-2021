use day1::Day1;
use std::time::Instant;

pub mod day1;
pub mod utils;

pub trait Day<'a> {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)>;
    fn get_day_number(&self) -> usize;
}

fn get_days<'a>() -> Vec<&'a dyn Day<'a>> {
    vec![&Day1 {}]
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
