use crate::Day;

pub struct Day4 {}

impl<'a> Day<'a> for Day4 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        4
    }
}

fn task1() -> String {
    let (calls, boards) = get_input(INPUT);

    run_game_1(calls, boards).to_string()
}

fn task2() -> String {
    let (calls, boards) = get_input(INPUT);

    run_game_2(calls, boards).to_string()
}

struct Board {
    numbers: Vec<Vec<Number>>,
}

#[derive(Debug)]
struct Number {
    number: u32,
    stamped: bool,
}

impl Board {
    fn parse(definition: &str) -> Board {
        let numbers: Vec<Vec<Number>> = definition
            .lines()
            .into_iter()
            .map(|l| {
                l.trim()
                    .split_ascii_whitespace()
                    .map(|n| Number {
                        number: n.parse::<u32>().unwrap(),
                        stamped: false,
                    })
                    .collect::<Vec<Number>>()
            })
            .collect();
        Board { numbers }
    }

    fn mark_and_check(&mut self, number: u32) -> Option<u32> {
        self.mark_number(number).and_then(|v| {
            if self.is_winner(&v) {
                Some(self.calculate_score(number))
            } else {
                None
            }
        })
    }

    fn mark_number(&mut self, number: u32) -> Option<(usize, usize)> {
        for i in 0..5 {
            let line = self.numbers.get_mut(i).unwrap();
            for j in 0..5 {
                let mut n = line.get_mut(j).unwrap();
                if n.number == number {
                    n.stamped = true;
                    return Some((i, j));
                }
            }
        }
        return None;
    }

    fn is_winner(&self, last_marked: &(usize, usize)) -> bool {
        let (i, j) = last_marked;
        self.numbers.get(*i).unwrap().iter().all(|v| v.stamped)
            || (0..5)
                .map(|ii| self.numbers.get(ii).unwrap().get(*j).unwrap().stamped)
                .all(|b| b)
    }

    fn calculate_score(&self, number: u32) -> u32 {
        number
            * self
                .numbers
                .iter()
                .flat_map(|l| {
                    l.into_iter()
                        .filter_map(|n| if n.stamped { None } else { Some(n.number) })
                })
                .sum::<u32>()
    }
}

fn run_game_1(calls_str: &str, boards: Vec<&str>) -> u32 {
    let mut calls = calls_str.split(",").map(|s| s.parse::<u32>().unwrap());
    let mut boards: Vec<Board> = boards.into_iter().map(|b| Board::parse(b)).collect();
    calls
        .find_map(|n| boards.iter_mut().find_map(|b| b.mark_and_check(n)))
        .unwrap()
}

fn run_game_2(calls_str: &str, boards: Vec<&str>) -> u32 {
    let calls = calls_str.split(",").map(|s| s.parse::<u32>().unwrap());
    let mut boards: Vec<Board> = boards.into_iter().map(|b| Board::parse(b)).collect();
    let mut winners: Vec<u32> = vec![];

    for call in calls {
        let mut i = 0;
        while i < boards.len() {
            match boards.get_mut(i).unwrap().mark_and_check(call) {
                Some(b) => {
                    boards.remove(i);
                    winners.push(b);
                }
                None => i += 1,
            }
        }
    }
    winners.into_iter().last().unwrap()
}

fn get_input(input: &str) -> (&str, Vec<&str>) {
    let mut iter = input.split("\n\n").flat_map(|l| l.split("\r\n\r\n"));
    (iter.next().unwrap(), iter.collect())
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day4::{get_input, run_game_1, run_game_2};

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part_1() {
        let (calls, boards) = get_input(TEST_INPUT);

        assert_eq!(run_game_1(calls, boards), 4512);
    }

    #[test]
    fn part_2() {
        let (calls, boards) = get_input(TEST_INPUT);

        assert_eq!(run_game_2(calls, boards), 1924);
    }
}
