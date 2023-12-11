use std::collections::HashSet;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day11");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("374"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: HashSet<(usize, usize)> = HashSet::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    data.insert((y + 1, x + 1));
                }
            }
        }
        match step {
            Step::First => self.count(&data, 1).to_string(),
            Step::Second => self.count(&data, 999999).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, data: &HashSet<(usize, usize)>, extra: usize) -> usize {
        let min = (
            data.iter().map(|v| v.0).min().unwrap(),
            data.iter().map(|v| v.1).min().unwrap(),
        );
        let max = (
            data.iter().map(|v| v.0).max().unwrap(),
            data.iter().map(|v| v.1).max().unwrap(),
        );
        let empty_row = (min.0..=max.0)
            .filter(|&y| !(min.1..=max.1).any(|x| data.contains(&(y, x))))
            .collect::<Vec<usize>>();
        let empty_col = (min.1..=max.1)
            .filter(|&x| !(min.0..=max.0).any(|y| data.contains(&(y, x))))
            .collect::<Vec<usize>>();
        let data = data
            .iter()
            .map(|&(y, x)| {
                (
                    'y: {
                        if empty_row.is_empty() {
                            y
                        } else if y > *empty_row.last().unwrap() {
                            y + empty_row.len() * extra
                        } else {
                            for (dy, &v) in empty_row.iter().enumerate() {
                                if y < v {
                                    break 'y y + dy * extra;
                                }
                            }
                            y
                        }
                    },
                    'x: {
                        if empty_col.is_empty() {
                            x
                        } else if x > *empty_col.last().unwrap() {
                            x + empty_col.len() * extra
                        } else {
                            for (dx, &v) in empty_col.iter().enumerate() {
                                if x < v {
                                    break 'x x + dx * extra;
                                }
                            }
                            x
                        }
                    },
                )
            })
            .collect::<HashSet<(usize, usize)>>();
        data.iter()
            .map(|&(y0, x0)| {
                data.iter()
                    .map(|&(y1, x1)| if x0 > x1 { x0 - x1 } else { 0 } + if y0 > y1 { y0 - y1 } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}
