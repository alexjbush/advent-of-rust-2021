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
    let max = get_rates(&parsed_input, |a, b| a > b);
    let min = get_rates(&parsed_input, |a, b| a < b);
    (max * min).to_string()
}

fn task2() -> String {
    let parsed_input = get_input(INPUT);
    "N/A".to_string()
}

fn get_rates<F>(input: &Vec<&str>, comp: F) -> u32
where
    F: Fn(u32, u32) -> bool,
{
    let counts = input
        //.clone()
        .into_iter()
        .map(|l| {
            //Line
            l.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .reduce(|a, b| {
            let d = a.iter().zip(b.iter()).map(|ae| ae.0 + ae.1);
            d.collect()
        })
        .unwrap();
    let half = input.len() as u32 / 2;
    let radix = counts
        .into_iter()
        .map(|v| if comp(v, half) { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");
    u32::from_str_radix(&radix, 2).unwrap()
}

fn get_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day3::{get_input, get_rates};

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
        assert_eq!(get_rates(&parsed_input, |a, b| a > b), 22);
        assert_eq!(get_rates(&parsed_input, |a, b| a < b), 9);
        //assert_eq!(find_pos(parsed_input), 150);
    }

    // #[test]
    // fn part_2() {
    //     let parsed_input = get_input(TEST_INPUT);
    //     assert_eq!(find_pos2(parsed_input), 900);
    // }
}
