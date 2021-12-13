use itertools::Itertools;

use crate::Day;
pub struct Day10 {}

impl<'a> Day<'a> for Day10 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        10
    }
}

fn task1() -> String {
    step_1(INPUT).to_string()
}

fn task2() -> String {
    step_2(INPUT).to_string()
}

#[derive(Debug, PartialEq)]
enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn get_corrupt_points(&self) -> u32 {
        match self {
            BracketType::Round => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }
    fn get_incomplete_points(&self) -> u64 {
        match self {
            BracketType::Round => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
        }
    }
}

#[derive(Debug)]
struct Bracket {
    bracket: BracketType,
    open: bool,
}

impl Bracket {
    fn parse(c: char) -> Bracket {
        match c {
            '(' => Bracket {
                bracket: BracketType::Round,
                open: true,
            },
            ')' => Bracket {
                bracket: BracketType::Round,
                open: false,
            },
            '[' => Bracket {
                bracket: BracketType::Square,
                open: true,
            },
            ']' => Bracket {
                bracket: BracketType::Square,
                open: false,
            },
            '{' => Bracket {
                bracket: BracketType::Curly,
                open: true,
            },
            '}' => Bracket {
                bracket: BracketType::Curly,
                open: false,
            },
            '<' => Bracket {
                bracket: BracketType::Angle,
                open: true,
            },
            '>' => Bracket {
                bracket: BracketType::Angle,
                open: false,
            },
            _ => panic!("Not a bracket: {:?}", c),
        }
    }
}

fn get_first_unbalanced_bracket(input: &str) -> Option<BracketType> {
    let mut stack: Vec<Bracket> = Vec::new();
    for b in input.chars().into_iter().map(|c| Bracket::parse(c)) {
        if b.open {
            stack.push(b);
        } else {
            let last = stack.pop();
            if last.map_or_else(|| true, |l| l.bracket != b.bracket) {
                return Some(b.bracket);
            }
        }
    }
    None
}

fn get_completion_score(input: &str) -> u64 {
    let mut stack: Vec<Bracket> = Vec::new();
    for b in input.chars().into_iter().map(|c| Bracket::parse(c)) {
        if b.open {
            stack.push(b);
        } else {
            let last = stack.pop();
            if last.map_or_else(|| true, |l| l.bracket != b.bracket) {
                panic!("Line is corrupt: {:?}", input);
            }
        }
    }
    stack.reverse();
    stack
        .iter()
        .fold(0, |z, b| 5 * z + b.bracket.get_incomplete_points())
}

fn step_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| get_first_unbalanced_bracket(l))
        .map(|b| b.get_corrupt_points())
        .sum()
}

fn step_2(input: &str) -> u64 {
    let r: Vec<u64> = input
        .lines()
        .filter(|l| get_first_unbalanced_bracket(l).is_none())
        .map(|b| get_completion_score(b))
        .sorted()
        .collect();

    *r.get(r.len() / 2).unwrap()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day10::{step_1, step_2};

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part_1() {
        assert_eq!(step_1(TEST_INPUT), 26397);
    }

    #[test]
    fn part_2() {
        assert_eq!(step_2(TEST_INPUT), 288957);
    }
}
