use std::collections::HashMap;

use crate::Day;
pub struct Day7 {}

impl<'a> Day<'a> for Day7 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        7
    }
}

fn task1() -> String {
    let input = parse_input(INPUT);

    run(input, |v| v).to_string()
}

fn task2() -> String {
    let input = parse_input(INPUT);

    run(input, |v| (1..=v).sum()).to_string()
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

fn run<F>(input: Vec<u32>, calc: F) -> i32
where
    F: Fn(i32) -> i32,
{
    let max = input.iter().max().unwrap();
    //Number sum optimisation
    let mut cache: HashMap<i32, i32> = HashMap::new();
    //Search optimisation, two-way from the middle would be more efficient
    let mut prev: i32 = i32::MAX;
    (0..=*max)
        .map(|i| {
            input.iter().fold(0i32, |z, v| {
                let d = (i as i32 - *v as i32).abs();
                let r = cache.entry(d).or_insert_with(|| calc(d));
                z + *r
            })
        })
        .take_while(|v| {
            let p = prev;
            prev = *v;
            *v < p
        })
        .min()
        .unwrap()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day7::{parse_input, run};

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1() {
        let input = parse_input(TEST_INPUT);

        assert_eq!(run(input, |v| v), 37);
    }

    #[test]
    fn part_2() {
        let input = parse_input(TEST_INPUT);

        assert_eq!(run(input, |v| (1..=v).sum()), 168);
    }
}
