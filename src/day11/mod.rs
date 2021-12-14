use itertools::Itertools;

use crate::Day;
pub struct Day11 {}

impl<'a> Day<'a> for Day11 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        11
    }
}

struct CavernMap {
    map: Vec<Vec<u32>>,
    width: i32,
    height: i32,
}

impl CavernMap {
    fn create(input: &str) -> CavernMap {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect();
        let width = map.get(0).unwrap().len() as i32;
        let height = map.len() as i32;
        CavernMap { map, width, height }
    }

    fn get_coord(&mut self, x: i32, y: i32) -> Option<&mut u32> {
        // static ref WIDTH: usize = self.find_position(|c| c == '\n');
        self.map
            .get_mut(y as usize)
            .and_then(|xs| xs.get_mut(x as usize))
    }
}

fn iterate(map: &mut CavernMap) -> u64 {
    let mut count = 0;
    let list: Vec<(i32, i32)> = (0..map.width)
        .flat_map(|x| (0..map.height).map(move |y| (x, y)))
        .collect();
    for (x, y) in list.iter() {
        let mut to_visit: Vec<(i32, i32)> = vec![(*x, *y)];
        while to_visit.len() > 0 {
            let (t_x, t_y) = to_visit.pop().unwrap();

            map.get_coord(t_x, t_y).iter_mut().for_each(|v| {
                let flash = **v == 9;
                **v += 1;
                if flash {
                    (t_x - 1..=t_x + 1)
                        .flat_map(|n_x| (t_y - 1..=t_y + 1).map(move |n_y| (n_x, n_y)))
                        .for_each(|n| to_visit.push(n));
                }
            })
        }
    }
    for (x, y) in list.iter() {
        let this = map.get_coord(*x, *y).unwrap();
        if *this > 9 {
            *this = 0;
            count += 1;
        }
    }
    count
}

fn task1() -> String {
    step_1(INPUT, 100).to_string()
}

fn task2() -> String {
    step_2(INPUT).to_string()
}

fn step_1(input: &str, steps: usize) -> u64 {
    let mut map = CavernMap::create(input);
    (0..steps).map(|_| iterate(&mut map)).sum()
}

fn step_2(input: &str) -> u64 {
    let mut map = CavernMap::create(input);
    let mut step = 0;
    loop {
        step += 1;
        let flashes = iterate(&mut map);
        if flashes == 100 {
            return step;
        }
    }
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day11::{step_1, step_2};

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part_1() {
        assert_eq!(step_1(TEST_INPUT, 0), 0);
        assert_eq!(step_1(TEST_INPUT, 1), 0);
        assert_eq!(step_1(TEST_INPUT, 2), 35);
        assert_eq!(step_1(TEST_INPUT, 100), 1656);
    }

    #[test]
    fn part_2() {
        assert_eq!(step_2(TEST_INPUT), 195);
    }
}
