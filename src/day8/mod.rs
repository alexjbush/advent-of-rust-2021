use std::collections::HashMap;

use itertools::Itertools;

use crate::Day;
pub struct Day8 {}

impl<'a> Day<'a> for Day8 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        8
    }
}

fn task1() -> String {
    part1(INPUT).to_string()
}

fn task2() -> String {
    // let input = parse_input(INPUT);

    "N/A".to_string()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.split('|')
                .skip(1)
                .next()
                .unwrap()
                .split_ascii_whitespace()
        })
        .filter(|b| {
            let len = b.len();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day8::part1;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part_1() {
        assert_eq!(part1(TEST_INPUT), 26);
    }

    #[test]
    fn part_2() {

        // assert_eq!(part1(TEST_INPUT), 26);
    }
}
