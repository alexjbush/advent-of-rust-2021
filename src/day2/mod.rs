use crate::Day;

pub struct Day2 {}

impl<'a> Day<'a> for Day2 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        2
    }
}

fn task1() -> String {
    let parsed_input = get_input(INPUT);
    find_pos(parsed_input).to_string()
}

fn task2() -> String {
    let parsed_input = get_input(INPUT);
    find_pos2(parsed_input).to_string()
}

fn find_pos(lines: Vec<(i32, i32)>) -> i32 {
    let mut sum = (0, 0);
    for line in lines {
        sum = (sum.0 + line.0, sum.1 + line.1)
    }
    sum.0 * sum.1
}

fn find_pos2(lines: Vec<(i32, i32)>) -> i32 {
    let mut sum = (0, 0);
    let mut aim = 0;
    for line in lines {
        aim = aim + line.1;
        sum = (sum.0 + line.0, sum.1 + line.0 * aim);
    }
    sum.0 * sum.1
}

fn get_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|s| {
            let split: Vec<&str> = s.split(' ').collect();
            let dir = split.get(0).unwrap();
            let distance = split.get(1).unwrap();
            let len = distance.parse::<i32>().unwrap();
            match dir {
                &"forward" => (len, 0),
                &"up" => (0, -1 * len),
                &"down" => (0, len),
                _ => (0, 0),
            }
        })
        .collect()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day2::{find_pos, find_pos2, get_input};

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

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
