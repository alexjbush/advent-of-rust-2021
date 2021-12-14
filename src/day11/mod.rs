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

fn interate(map: &mut CavernMap) -> u64 {
    let mut count = 0;
    for (x, y) in (0..map.width).flat_map(|x| (0..map.height).map(move |y| (x, y))) {
        let mut to_visit: Vec<(i32, i32)> = vec![(x, y)];
        while to_visit.len() > 0 {
            let (t_x, t_y) = to_visit.pop().unwrap();

            map.get_coord(t_x, t_y).iter_mut().for_each(|v| {
                let flash = **v == 9;
                **v += 1;
                (t_x - 1..=t_x + 1)
                    .flat_map(|n_x| (t_y - 1..=t_y + 1).map(move |n_y| (n_x, n_y)))
                    .filter(|(n_x, n_y)| *n_x != t_x && *n_y != t_y)
                    .for_each(|n| to_visit.push(n));
            })
        }
    }
    count
}

fn task1() -> String {
    "N/A".to_string()
}

fn task2() -> String {
    "N/A".to_string()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {

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
        // assert_eq!(step_1(TEST_INPUT), 26397);
    }

    #[test]
    fn part_2() {
        // assert_eq!(step_2(TEST_INPUT), 288957);
    }
}
