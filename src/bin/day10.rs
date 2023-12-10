use std::collections::{HashMap, HashSet};

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day10");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![
                ("test0.txt", String::from("4")),
                ("test1.txt", String::from("8")),
            ],
            Step::Second => vec![
                ("test2.txt", String::from("4")),
                ("test3.txt", String::from("8")),
                ("test4.txt", String::from("10")),
            ],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut map = HashMap::new();
        let mut animal = (0, 0);
        input.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let (y, x) = (y as i32 + 1, x as i32 + 1);
                if c == 'S' {
                    animal = (y, x);
                } else if c != '.' {
                    map.insert(
                        (y, x),
                        match c {
                            '|' => 0b0101, // 0101
                            '-' => 0b1010, // 1010
                            'L' => 0b0011, // 0011
                            'J' => 0b1001, // 1001
                            '7' => 0b1100, // 1100
                            'F' => 0b0110, // 0110
                            _ => panic!("Invalid input"),
                        },
                    );
                }
            })
        });
        match step {
            Step::First => self.count(&map, &animal).to_string(),
            Step::Second => self.count2(&map, &animal).to_string(),
        }
    }
}

const ROUND: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
impl Solution {
    fn count(&self, map: &HashMap<(i32, i32), usize>, an: &(i32, i32)) -> usize {
        let mut curr = (0, 0);
        let mut from = 0;
        for (i, &(dy, dx)) in ROUND.iter().enumerate() {
            if map
                .get(&(an.0 + dy, an.1 + dx))
                .filter(|&v| (*v & (1 << i)) != 0)
                .is_some()
            {
                curr = (an.0 + dy, an.1 + dx);
                from = i;
                break;
            }
        }
        if curr == (0, 0) {
            panic!("No pipe next to the animal!");
        }
        let mut res = 1;
        while curr != *an {
            res += 1;
            (from, curr) = if let Some(&pipe) = map.get(&curr) {
                let to = (pipe ^ (1 << from)).ilog2() as usize;
                (to ^ 2, (curr.0 - ROUND[to].0, curr.1 - ROUND[to].1))
            } else {
                panic!("No connected pipe!");
            };
        }
        res / 2
    }

    fn count2(&self, map: &HashMap<(i32, i32), usize>, an: &(i32, i32)) -> usize {
        let mut curr = (0, 0);
        let mut from = 0;
        let mut an_to = 0;
        for (i, &(dy, dx)) in ROUND.iter().enumerate() {
            if map
                .get(&(an.0 + dy, an.1 + dx))
                .filter(|&v| (*v & (1 << i)) != 0)
                .is_some()
            {
                curr = (an.0 + dy, an.1 + dx);
                from = i;
                an_to = from ^ 2;
                break;
            }
        }
        if curr == (0, 0) {
            panic!("No pipe next to the animal!");
        }
        let mut aloop = HashSet::new();
        let mut right = HashSet::new();
        let mut left = HashSet::new();
        aloop.insert(curr);
        while curr != *an {
            (from, curr) = if let Some(&pipe) = map.get(&curr) {
                let to = (pipe ^ (1 << from)).ilog2() as usize;
                update(&curr, (from, to), &mut left, &mut right);
                (to ^ 2, (curr.0 - ROUND[to].0, curr.1 - ROUND[to].1))
            } else {
                panic!("No connected pipe!");
            };
            aloop.insert(curr);
        }
        update(an, (from, an_to), &mut left, &mut right);
        right.retain(|v| !aloop.contains(v));
        left.retain(|v| !aloop.contains(v));
        let max = (
            aloop.iter().map(|v| v.0).max().unwrap() + 1,
            aloop.iter().map(|v| v.1).max().unwrap() + 1,
        );
        let min = (
            aloop.iter().map(|v| v.0).min().unwrap() - 1,
            aloop.iter().map(|v| v.1).min().unwrap() - 1,
        );
        let total = (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - aloop.len() as i32;
        while (right.len() + left.len()) as i32 != total {
            for y in min.0..=max.0 {
                for x in min.1..=max.1 {
                    let res = (y, x);
                    if aloop.contains(&res) || right.contains(&res) || left.contains(&res) {
                        continue;
                    }
                    for (dy, dx) in ROUND {
                        let next = (y + dy, x + dx);
                        if left.contains(&next) {
                            left.insert(res);
                            break;
                        } else if right.contains(&next) {
                            right.insert(res);
                            break;
                        }
                    }
                }
            }
        }
        if right.contains(&(min.0, min.1)) {
            left.len()
        } else {
            right.len()
        }
    }
}

fn update(
    curr: &(i32, i32),
    from_to: (usize, usize),
    left: &mut HashSet<(i32, i32)>,
    right: &mut HashSet<(i32, i32)>,
) {
    let next = match from_to {
        (0, 1) => [(0, -1, true), (1, 0, true)],
        (1, 2) => [(-1, 0, true), (0, -1, true)],
        (2, 3) => [(0, 1, true), (-1, 0, true)],
        (3, 0) => [(1, 0, true), (0, 1, true)],
        (0, 2) => [(0, -1, true), (0, 1, false)],
        (2, 0) => [(0, -1, false), (0, 1, true)],
        (1, 3) => [(1, 0, false), (-1, 0, true)],
        (3, 1) => [(1, 0, true), (-1, 0, false)],
        (1, 0) => [(0, -1, false), (1, 0, false)],
        (2, 1) => [(-1, 0, false), (0, -1, false)],
        (3, 2) => [(0, 1, false), (-1, 0, false)],
        (0, 3) => [(1, 0, false), (0, 1, false)],
        _ => panic!("Wrong direction"),
    };
    for (dy, dx, pool) in next {
        if pool {
            right.insert((curr.0 + dy, curr.1 + dx));
        } else {
            left.insert((curr.0 + dy, curr.1 + dx));
        }
    }
}
