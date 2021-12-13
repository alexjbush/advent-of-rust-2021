use std::collections::HashMap;

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
    step2(INPUT).to_string()
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

fn step2(input: &str) -> u32 {
    let map = CaveMap::create(input);
    let mut count = 0;
    let mut res: HashMap<(i32, i32), i32> = HashMap::new();
    for (x, y) in (0..map.width).flat_map(|x| (0..map.height).map(move |y| (x, y))) {
        if map.get_coord(x, y).map_or_else(|| true, |v| v == 9) || res.contains_key(&(x, y)) {
            continue;
        }
        let mut to_visit: Vec<(i32, i32)> = vec![(x, y)];
        while to_visit.len() > 0 {
            let (t_x, t_y) = to_visit.pop().unwrap();
            if map.get_coord(t_x, t_y).map_or_else(|| true, |v| v == 9)
                || res.contains_key(&(t_x, t_y))
            {
                continue;
            }
            res.insert((t_x, t_y), count);
            vec![
                (t_x - 1, t_y),
                (t_x + 1, t_y),
                (t_x, t_y - 1),
                (t_x, t_y + 1),
            ]
            .into_iter()
            .for_each(|v| to_visit.push(v));
        }
        count += 1;
    }
    res.values()
        .sorted()
        .group_by(|k| *k)
        .into_iter()
        .map(|(_, b)| b.count())
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(3)
        .map(|v| v as u32)
        .product()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day9::{step1, step2};

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
        assert_eq!(step2(TEST_INPUT), 1134);
    }
}
