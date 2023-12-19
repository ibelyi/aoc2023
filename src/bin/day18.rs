use std::collections::HashSet;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day18");
}
struct Solution {}

struct Instr {
    dir: usize,
    len: i32,
    col: u32,
}

impl Instr {
    fn parse(line: &str) -> Instr {
        let mut it = line.split(' ');
        let dir = match it.next().unwrap() {
            "R" => 0,
            "D" => 1,
            "L" => 2,
            "U" => 3,
            _ => panic!("Invalid direction"),
        };
        let len = it.next().unwrap().parse().unwrap();
        let col = it
            .next()
            .unwrap()
            .chars()
            .skip(2)
            .take(6)
            .map(|c| c.to_digit(16).unwrap())
            .fold(0, |a, v| (a << 4) + v);
        Instr { dir, len, col }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("62"))],
            Step::Second => vec![("test0.txt", String::from("952408144115"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Instr> = input.iter().map(|l| Instr::parse(l)).collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self
                .count2(
                    &data
                        .iter()
                        .map(|i| Instr {
                            dir: (i.col & 15) as usize,
                            len: (i.col >> 4) as i32,
                            col: 0,
                        })
                        .collect::<Vec<Instr>>(),
                )
                .to_string(),
        }
    }
}

const DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    fn count(&self, data: &[Instr]) -> usize {
        let mut map = HashSet::<(i32, i32)>::new();
        let mut curr = (0, 0);
        for i in data {
            let (dy, dx) = DIR[i.dir];
            for _ in 0..i.len {
                curr = (curr.0 + dy, curr.1 + dx);
                map.insert(curr);
            }
        }
        if curr != (0, 0) {
            panic!("Not back at the start!");
        }
        let min = (
            map.iter().map(|&v| v.0).min().unwrap(),
            map.iter().map(|&v| v.1).min().unwrap(),
        );
        let max = (
            map.iter().map(|&v| v.0).max().unwrap(),
            map.iter().map(|&v| v.1).max().unwrap(),
        );
        let mut outside: HashSet<(i32, i32)> = (min.0..=max.0)
            .map(|y| (y, min.1))
            .chain((min.0..=max.0).map(|y| (y, max.1)))
            .chain((min.1..=max.1).map(|x| (min.0, x)))
            .chain((min.1..=max.1).map(|x| (max.0, x)))
            .filter(|n| !map.contains(n))
            .collect();
        let mut added = true;
        while added {
            added = false;
            for y in min.0..=max.0 {
                for x in min.1..=max.1 {
                    if !outside.contains(&(y, x))
                        && !map.contains(&(y, x))
                        && DIR
                            .iter()
                            .any(|&(dy, dx)| outside.contains(&(y + dy, x + dx)))
                    {
                        outside.insert((y, x));
                        added = true;
                    }
                }
            }
        }
        ((max.0 - min.0 + 1) * (max.1 - min.1 + 1)) as usize - outside.len()
    }

    fn count2(&self, data: &[Instr]) -> u64 {
        let mut ys = HashSet::new();
        let mut xs = HashSet::new();
        let mut curr = (0, 0);
        for i in data {
            let (dy, dx) = DIR[i.dir];
            curr = (curr.0 + dy * i.len, curr.1 + dx * i.len);
            for d in [-1, 0, 1] {
                ys.insert(curr.0 + d);
                xs.insert(curr.1 + d);
            }
        }
        if curr != (0, 0) {
            panic!("Didn't get back to start: ({},{})!", curr.0, curr.1);
        }
        let mut ys: Vec<i32> = ys.into_iter().collect();
        ys.sort();
        let mut xs: Vec<i32> = xs.into_iter().collect();
        xs.sort();
        let mut map = HashSet::<(usize, usize)>::new();
        let mut from = (
            ys.binary_search(&curr.0).unwrap(),
            xs.binary_search(&curr.1).unwrap(),
        );
        for i in data {
            let (dy, dx) = DIR[i.dir];
            curr = (curr.0 + i.len * dy, curr.1 + i.len * dx);
            let to = (
                ys.binary_search(&curr.0).unwrap(),
                xs.binary_search(&curr.1).unwrap(),
            );
            for y in if from.0 < to.0 {
                from.0..=to.0
            } else {
                to.0..=from.0
            } {
                for x in if from.1 < to.1 {
                    from.1..=to.1
                } else {
                    to.1..=from.1
                } {
                    map.insert((y, x));
                }
            }
            from = to;
        }
        let mut outside: HashSet<(usize, usize)> = (0..ys.len())
            .map(|y| (y, 0))
            .chain((0..ys.len()).map(|y| (y, xs.len() - 1)))
            .chain((0..xs.len()).map(|x| (0, x)))
            .chain((0..xs.len()).map(|x| (ys.len() - 1, x)))
            .collect();
        let mut added = true;
        while added {
            added = false;
            for y in 1..ys.len() - 1 {
                for x in 1..xs.len() - 1 {
                    if outside.contains(&(y, x)) || map.contains(&(y, x)) {
                        continue;
                    }
                    if DIR.iter().any(|&(dy, dx)| {
                        outside.contains(&((y as i32 + dy) as usize, (x as i32 + dx) as usize))
                    }) {
                        outside.insert((y, x));
                        added = true;
                    }
                }
            }
        }
        (1..ys.len() - 1)
            .map(|y| {
                (1..xs.len() - 1)
                    .map(|x| {
                        if outside.contains(&(y, x)) {
                            0
                        } else {
                            (ys[y + 1] - ys[y]) as u64 * (xs[x + 1] - xs[x]) as u64
                        }
                    })
                    .sum::<u64>()
            })
            .sum()
    }
}
