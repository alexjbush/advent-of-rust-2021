use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use crate::Day;
pub struct Day9 {}

impl<'a> Day<'a> for Day9 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        9
    }
}

fn task1() -> String {
    step1(INPUT).to_string()
}

fn task2() -> String {
    "N/A".to_string()
}

struct CaveMap {
    map: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl CaveMap {
    fn create(input: &str) -> CaveMap {
        let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect_vec()).collect();
        let width = map.get(0).unwrap().len() as i32;
        let height = map.len() as i32;
        CaveMap { map, width, height }
    }

    fn get_coord(&self, x: i32, y: i32) -> Option<u32> {
        // static ref WIDTH: usize = self.find_position(|c| c == '\n');
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        } else {
            return self
                .map
                .get(y as usize)
                .unwrap()
                .get(x as usize)
                .unwrap()
                .to_digit(10);
        }
    }
}

fn step1(input: &str) -> u32 {
    let map = CaveMap::create(input);
    (0..map.width)
        .flat_map(|x| (0..map.height).map(move |y| (x, y)))
        .filter_map(|(x, y)| {
            let val = map.get_coord(x, y).unwrap();
            let is_low = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .map(|(_x, _y)| map.get_coord(*_x, *_y))
                .filter_map(|v| v)
                .all(|_v| _v > val);
            if is_low {
                return Some(val + 1);
            } else {
                return None;
            }
        })
        .sum()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day9::step1;

    use super::CaveMap;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_get() {
        let grid = CaveMap::create(TEST_INPUT);
        assert_eq!(grid.get_coord(-1, 0), None);
        assert_eq!(grid.get_coord(0, -1), None);
        assert_eq!(grid.get_coord(10, 0), None);
        assert_eq!(grid.get_coord(0, 5), None);
        assert_eq!(grid.get_coord(0, 0), Some(2));
        assert_eq!(grid.get_coord(9, 4), Some(8));
    }

    #[test]
    fn part_1() {
        assert_eq!(step1(TEST_INPUT), 15);
    }

    #[test]
    fn part_2() {
        // assert_eq!(part2(TEST_INPUT), 61229);
    }
}
