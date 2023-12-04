use std::collections::HashSet;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day04");
}
struct Solution {}

fn parse(line: &str) -> usize {
    let mut parts = line
        .split(": ")
        .last()
        .expect("Nothing in the line")
        .split('|');
    let lucky: HashSet<u32> = parts
        .next()
        .expect("Missing before |")
        .split(' ')
        .filter_map(|l| l.parse().ok())
        .collect();
    let got = parts
        .next()
        .expect("Missing after |")
        .split(' ')
        .filter_map(|l| l.parse().ok())
        .collect();
    lucky.intersection(&got).count()
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("13"))],
            Step::Second => vec![("test0.txt", String::from("30"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<usize> = input.iter().map(|line| parse(line)).collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, data: &[usize]) -> i32 {
        data.iter()
            .map(|card| if *card == 0 { 0 } else { 1 << (*card - 1) })
            .sum()
    }
    fn count2(&self, data: &[usize]) -> usize {
        let mut pile = vec![1; data.len()];
        for (i, card) in data.iter().enumerate() {
            for c in i + 1..i + 1 + *card {
                pile[c] += pile[i];
            }
        }
        pile.iter().sum()
    }
}
