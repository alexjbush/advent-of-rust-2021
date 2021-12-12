use crate::Day;
use std::collections::HashMap;
pub struct Day6 {}

impl<'a> Day<'a> for Day6 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        6
    }
}

fn task1() -> String {
    let input = parse_input(INPUT);
    run_sim(input, 80).to_string()
}

fn task2() -> String {
    let input = parse_input(INPUT);
    run_sim(input, 256).to_string()
}

struct FishSim {
    fish: HashMap<u32, u64>,
}

impl FishSim {
    fn advance_day(&mut self) {
        self.fish = self
            .fish
            .iter()
            .flat_map(|(day, count)| {
                if *day == 0 {
                    vec![(6, count), (8, count)]
                } else {
                    vec![(day - 1, count)]
                }
            })
            .fold(HashMap::new(), |mut map, (k, v)| {
                *map.entry(k).or_insert(0) += v;
                map
            });
    }
}

fn run_sim(input: Vec<u32>, days: u32) -> u64 {
    let mut start_day_to_count: HashMap<u32, u64> = HashMap::new();
    input.into_iter().for_each(|d| {
        *start_day_to_count.entry(d).or_insert(0) += 1;
    });

    let mut sim = FishSim {
        fish: start_day_to_count,
    };

    (0..days).for_each(|_| {
        sim.advance_day();
    });

    sim.fish.values().into_iter().fold(0u64, |c, v| c + *v)
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day6::{parse_input, run_sim};

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part_1() {
        let input = parse_input(TEST_INPUT);

        assert_eq!(run_sim(input, 80), 5934);
    }

    #[test]
    fn part_2() {
        let input = parse_input(TEST_INPUT);

        assert_eq!(run_sim(input, 256), 26984457539);
    }
}
