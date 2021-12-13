use std::collections::HashMap;
use std::collections::HashSet;

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
    part2(INPUT).to_string()
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

fn part2(input: &str) -> u32 {
    input.lines().map(|l| parse_line(l)).sum()
}

fn parse_line(input: &str) -> u32 {
    let (def, nums) = input.split_once("|").unwrap();
    let mut map: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
    def.split_ascii_whitespace().for_each(|d| {
        map.entry(d.len())
            .or_insert_with(|| vec![])
            .push(d.to_string().chars().sorted().collect())
    });
    let mut res: HashMap<char, &HashSet<char>> = HashMap::new();
    //1
    res.insert('1', map.get(&2).unwrap().get(0).unwrap());
    //7
    res.insert('7', map.get(&3).unwrap().get(0).unwrap());
    //4
    res.insert('4', map.get(&4).unwrap().get(0).unwrap());
    //8
    res.insert('8', map.get(&7).unwrap().get(0).unwrap());
    //6
    res.insert(
        '6',
        map.get(&6)
            .unwrap()
            .iter()
            .find(|v| v.union(res.get(&'1').unwrap()).into_iter().count() == 7)
            .unwrap(),
    );
    //0
    res.insert(
        '0',
        map.get(&6)
            .unwrap()
            .iter()
            .filter(|s| *s != *res.get(&'6').unwrap())
            .find(|v| v.union(res.get(&'4').unwrap()).into_iter().count() == 7)
            .unwrap(),
    );
    //9
    res.insert(
        '9',
        map.get(&6)
            .unwrap()
            .iter()
            .find(|s| *s != *res.get(&'6').unwrap() && *s != *res.get(&'0').unwrap())
            .unwrap(),
    );
    //5
    res.insert(
        '5',
        map.get(&5)
            .unwrap()
            .iter()
            .find(|v| {
                v.union(res.get(&'1').unwrap())
                    .map(|v| v.clone())
                    .collect::<HashSet<char>>()
                    == **res.get(&'9').unwrap()
            })
            .unwrap(),
    );
    //2
    res.insert(
        '2',
        map.get(&5)
            .unwrap()
            .iter()
            .find(|v| v.union(res.get(&'4').unwrap()).into_iter().count() == 7)
            .unwrap(),
    );
    //3
    res.insert(
        '3',
        map.get(&5)
            .unwrap()
            .iter()
            .find(|s| *s != *res.get(&'2').unwrap() && *s != *res.get(&'5').unwrap())
            .unwrap(),
    );

    //Result time
    nums.split_ascii_whitespace()
        .map(|n| {
            let r: HashSet<char> = n.chars().into_iter().collect();
            res.iter().find(|(_, v)| ***v == r).unwrap().0
        })
        .join("")
        .parse::<u32>()
        .unwrap()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day8::{part1, part2};

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
        assert_eq!(part2(TEST_INPUT), 61229);
    }
}
