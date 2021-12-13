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
    "N/A".to_string()
}

#[derive(Debug, PartialEq)]
enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn get_points(&self) -> u32 {
        match self {
            BracketType::Round => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }
}

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

fn step_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| get_first_unbalanced_bracket(l))
        .map(|b| b.get_points())
        .sum()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day10::step_1;

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
        // assert_eq!(step2(TEST_INPUT), 1134);
    }
}
