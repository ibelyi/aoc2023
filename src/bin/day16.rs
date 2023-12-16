use std::collections::HashMap;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day16");
}
struct Solution {}

enum Mirror {
    Vert,
    Horz,
    Left,
    Right,
}

impl Mirror {
    fn parse(c: char) -> Option<Mirror> {
        match c {
            '|' => Some(Mirror::Vert),
            '-' => Some(Mirror::Horz),
            '\\' => Some(Mirror::Left),
            '/' => Some(Mirror::Right),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Dir {
    Right = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Up = 0b1000,
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("46"))],
            Step::Second => vec![("test0.txt", String::from("51"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data = HashMap::<(usize, usize), Mirror>::new();
        for (y, l) in input.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if let Some(m) = Mirror::parse(c) {
                    data.insert((y + 1, x + 1), m);
                }
            }
        }
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

fn calc(
    data: &HashMap<(usize, usize), Mirror>,
    max: &(usize, usize),
    start: (usize, usize, Dir),
) -> usize {
    let mut saw = HashMap::<(usize, usize), u8>::new();
    let mut track = vec![start];
    while let Some((y, x, d)) = track.pop() {
        if y == 0 || y == max.0 || x == 0 || x == max.1 {
            continue;
        }
        if let Some(od) = saw.get_mut(&(y, x)) {
            if *od & d as u8 != 0 {
                continue;
            } else {
                *od |= d as u8;
            }
        } else {
            saw.insert((y, x), d as u8);
        }
        if let Some(m) = data.get(&(y, x)) {
            match m {
                Mirror::Right => track.push(match d {
                    Dir::Right => (y - 1, x, Dir::Up),
                    Dir::Down => (y, x - 1, Dir::Left),
                    Dir::Left => (y + 1, x, Dir::Down),
                    Dir::Up => (y, x + 1, Dir::Right),
                }),
                Mirror::Left => track.push(match d {
                    Dir::Right => (y + 1, x, Dir::Down),
                    Dir::Down => (y, x + 1, Dir::Right),
                    Dir::Left => (y - 1, x, Dir::Up),
                    Dir::Up => (y, x - 1, Dir::Left),
                }),
                Mirror::Horz => match d {
                    Dir::Right => track.push((y, x + 1, d)),
                    Dir::Left => track.push((y, x - 1, d)),
                    Dir::Down | Dir::Up => {
                        track.push((y, x - 1, Dir::Left));
                        track.push((y, x + 1, Dir::Right));
                    }
                },
                Mirror::Vert => match d {
                    Dir::Down => track.push((y + 1, x, d)),
                    Dir::Up => track.push((y - 1, x, d)),
                    Dir::Right | Dir::Left => {
                        track.push((y + 1, x, Dir::Down));
                        track.push((y - 1, x, Dir::Up));
                    }
                },
            }
        } else {
            track.push(match d {
                Dir::Right => (y, x + 1, d),
                Dir::Down => (y + 1, x, d),
                Dir::Left => (y, x - 1, d),
                Dir::Up => (y - 1, x, d),
            });
        }
    }
    saw.len()
}

impl Solution {
    fn count(&self, data: &HashMap<(usize, usize), Mirror>) -> usize {
        let max = (
            data.iter().map(|(k, _)| k.1).max().unwrap() + 1,
            data.iter().map(|(k, _)| k.0).max().unwrap() + 1,
        );
        calc(data, &max, (1, 1, Dir::Right))
    }

    fn count2(&self, data: &HashMap<(usize, usize), Mirror>) -> usize {
        let max = (
            data.iter().map(|(k, _)| k.1).max().unwrap() + 1,
            data.iter().map(|(k, _)| k.0).max().unwrap() + 1,
        );
        (1..max.0)
            .map(|y| calc(data, &max, (y, 1, Dir::Right)))
            .chain((1..max.0).map(|y| calc(data, &max, (y, max.0 - 1, Dir::Left))))
            .chain((1..max.1).map(|x| calc(data, &max, (1, x, Dir::Down))))
            .chain((1..max.1).map(|x| calc(data, &max, (max.1 - 1, x, Dir::Up))))
            .max()
            .unwrap()
    }
}
