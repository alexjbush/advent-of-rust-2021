use std::str::Lines;

use crate::Day;

pub struct Day3 {}

impl<'a> Day<'a> for Day3 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        3
    }
}

fn task1() -> String {
    let parsed_input = get_input(INPUT);
    "N/A".to_string()
}

fn task2() -> String {
    let parsed_input = get_input(INPUT);
    "N/A".to_string()
}

fn get_rates(input: &Lines) -> Vec<u32> {
    let y = input.into_iter().map(|l| {
        //Line
        let par = l.chars().map(|c| c.parse::<u32>().unwrap());
    });
}

fn get_input(input: &str) -> Lines {
    input.lines()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day2::{find_pos, find_pos2, get_input};

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part_1() {
        let parsed_input = get_input(TEST_INPUT);
        assert_eq!(find_pos(parsed_input), 150);
    }

    #[test]
    fn part_2() {
        let parsed_input = get_input(TEST_INPUT);
        assert_eq!(find_pos2(parsed_input), 900);
    }
}
