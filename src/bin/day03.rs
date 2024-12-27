use std::collections::HashMap;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day03");
}
struct Solution {}

fn clockwise(y0: usize, x0: usize, y1: usize, x1: usize) -> impl Iterator<Item = (usize, usize)> {
    (y0..y1)
        .map(move |y| (y, x0))
        .chain((x0..x1).map(move |x| (y1, x)))
        .chain((y0 + 1..=y1).rev().map(move |y| (y, x1)))
        .chain((x0 + 1..=x1).rev().map(move |x| (y0, x)))
}

struct Numb {
    id: u32,
    y: usize,
    x: (usize, usize),
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("4361"))],
            Step::Second => vec![("test0.txt", String::from("467835"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut numbers = vec![];
        let mut parts = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            let mut numb = 0;
            let mut x0 = 0;
            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if numb == 0 {
                        x0 = x + 1;
                    }
                    numb = numb * 10 + c.to_digit(10).expect("Should have been a number");
                } else {
                    if numb != 0 {
                        numbers.push(Numb {
                            id: numb,
                            y: y + 1,
                            x: (x0, x),
                        });
                        numb = 0;
                    }
                    if c != '.' {
                        parts.insert((y + 1, x + 1), c);
                    }
                }
            }
            if numb != 0 {
                numbers.push(Numb {
                    id: numb,
                    y: y + 1,
                    x: (x0, line.len()),
                })
            }
        }
        match step {
            Step::First => self.count(&numbers, &parts).to_string(),
            Step::Second => self.count2(&numbers, &parts).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, numbers: &[Numb], parts: &HashMap<(usize, usize), char>) -> u32 {
        numbers
            .iter()
            .filter_map(|n| {
                if clockwise(n.y - 1, n.x.0 - 1, n.y + 1, n.x.1 + 1)
                    .any(|(y, x)| parts.contains_key(&(y, x)))
                {
                    Some(n.id)
                } else {
                    None
                }
            })
            .sum()
    }

    fn count2(&self, numbers: &[Numb], parts: &HashMap<(usize, usize), char>) -> u32 {
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        for n in numbers {
            for (y, x) in clockwise(n.y - 1, n.x.0 - 1, n.y + 1, n.x.1 + 1) {
                if let Some(c) = parts.get(&(y, x)) {
                    if *c == '*' {
                        gears.entry((y, x)).or_default().push(n.id);
                    }
                }
            }
        }
        gears
            .into_iter()
            .filter_map(|(_, ids)| {
                if ids.len() == 2 {
                    Some(ids[0] * ids[1])
                } else {
                    None
                }
            })
            .sum()
    }
}
