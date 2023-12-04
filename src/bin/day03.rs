use std::collections::HashMap;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day03");
}
struct Solution {}

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
                if (n.x.0 - 1..=n.x.1 + 1)
                    .any(|x| (n.y - 1..=n.y + 1).any(|y| parts.contains_key(&(y, x))))
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
            for x in n.x.0 - 1..=n.x.1 + 1 {
                for y in n.y - 1..=n.y + 1 {
                    if let Some(c) = parts.get(&(y, x)) {
                        if *c == '*' {
                            gears
                                .entry((y, x))
                                .or_insert_with(std::vec::Vec::new)
                                .push(n.id);
                        }
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
