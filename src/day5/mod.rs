use crate::Day;
use std::collections::HashMap;
pub struct Day5 {}

impl<'a> Day<'a> for Day5 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &task1), (2, &task2)]
    }

    fn get_day_number(&self) -> usize {
        5
    }
}
#[derive(Debug)]
struct Coordinate {
    p1: Point,
    p2: Point,
}
#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn is_diagonal(&self) -> bool {
        (self.p1.x - self.p2.x).abs() == (self.p1.y - self.p2.y).abs()
    }

    fn generate_points(&self) -> Vec<Point> {
        let (x1, x2, y1, y2) = (self.p1.x, self.p2.x, self.p1.y, self.p2.y);
        if x1 == x2 {
            return get_range(y1, y2)
                .iter()
                .map(move |y| Point { x: x1, y: *y })
                .collect();
        } else if y1 == y2 {
            return get_range(x1, x2)
                .iter()
                .map(move |x| Point { x: *x, y: y1 })
                .collect();
        } else if (x1 - x2).abs() == (y1 - y2).abs() {
            let xr = get_range(x1, x2);
            let yr = get_range(y1, y2);
            xr.iter()
                .zip(yr.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect()
        } else {
            panic!("OP")
        }
    }
}

fn get_range(a: i32, b: i32) -> Vec<i32> {
    if a < b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

fn task1() -> String {
    let coords = parse_input(INPUT);

    run_task(coords, |c| c.is_horizontal_or_vertical()).to_string()
}

fn task2() -> String {
    let coords = parse_input(INPUT);

    run_task(coords, |c| c.is_horizontal_or_vertical() || c.is_diagonal()).to_string()
}

fn run_task<F>(input: Vec<Coordinate>, filter: F) -> usize
where
    F: Fn(&Coordinate) -> bool,
{
    let mut map = HashMap::new();
    input
        .into_iter()
        .filter(|e| filter(e))
        .flat_map(|e| e.generate_points())
        .for_each(|e| {
            let count = map.entry(e).or_insert(0);
            *count += 1;
        });
    map.into_iter().filter(|v| v.1 > 1).count()
}

fn parse_input(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split(|c| c == ',' || c == ' ').collect();
            Coordinate {
                p1: Point {
                    x: s.get(0).unwrap().parse::<i32>().unwrap(),
                    y: s.get(1).unwrap().parse::<i32>().unwrap(),
                },
                p2: Point {
                    x: s.get(3).unwrap().parse::<i32>().unwrap(),
                    y: s.get(4).unwrap().parse::<i32>().unwrap(),
                },
            }
        })
        .collect()
}

const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use crate::day5::{parse_input, run_task};

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part_1() {
        let coords = parse_input(TEST_INPUT);

        assert_eq!(run_task(coords, |c| c.is_horizontal_or_vertical()), 5);
    }

    #[test]
    fn part_2() {
        let coords = parse_input(TEST_INPUT);

        assert_eq!(
            run_task(coords, |c| c.is_horizontal_or_vertical() || c.is_diagonal()),
            12
        );
    }
}
