use crate::Day;

pub struct Day1 {}

impl<'a> Day<'a> for Day1 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        1
    }
}

fn task1() -> String {
    let parsed_input = get_input(INPUT);
    count_increases(parsed_input).to_string()
}

fn task2() -> String {
    let parsed_input = get_input(INPUT);
    count_sum_increases(parsed_input).to_string()
}

fn count_increases(lines: Vec<u32>) -> u32 {
    let mut prev: Option<u32> = None;
    let mut count: u32 = 0;
    for l in lines {
        if prev.map(|p| p < l).get_or_insert(false).to_owned() {
            count += 1;
        };
        prev = Some(l);
    }
    return count;
}

fn count_sum_increases(lines: Vec<u32>) -> u32 {
    let mut prev: Option<u32> = None;
    let mut count: u32 = 0;
    for i in 2..lines.len() {
        let sum = lines.iter().skip(i - 2).take(3).sum();
        if prev.map(|p| p < sum).get_or_insert(false).to_owned() {
            count += 1;
        };
        prev = Some(sum);
    }
    return count;
}

fn get_input(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day1::{count_increases, count_sum_increases, get_input};

    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part_1() {
        let parsed_input = get_input(TEST_INPUT);
        assert_eq!(count_increases(parsed_input), 7);
    }

    #[test]
    fn part_2() {
        let parsed_input = get_input(TEST_INPUT);
        assert_eq!(count_sum_increases(parsed_input), 5);
    }
}
