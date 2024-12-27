use aoc2023::{Solver, Step};
use std::collections::HashMap;

pub fn main() {
    let solver = Solution {};
    solver.solve("day12");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("21"))],
            Step::Second => vec![("test0.txt", String::from("525152"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<(Vec<char>, Vec<usize>)> = input
            .iter()
            .map(|line| {
                let split = line.split_once(' ').unwrap();
                let (springs, record) = match step {
                    Step::First => (split.0.to_string(), split.1.to_string()),
                    Step::Second => ([split.0; 5].join("?"), [split.1; 5].join(",")),
                };
                (
                    springs.chars().collect(),
                    record.split(',').map(|l| l.parse().unwrap()).collect(),
                )
            })
            .collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count(&data).to_string(),
        }
    }
}

fn calc(spring: &[char], record: &[usize], result: &mut HashMap<(usize, usize), u64>) -> u64 {
    if record.is_empty() {
        return if spring.iter().any(|&c| c == '#') {
            0
        } else {
            1
        };
    } else if let Some(&res) = result.get(&(spring.len(), record.len())) {
        return res;
    }
    let upto = record.iter().map(|&v| v + 1).sum::<usize>() - 1;
    let mut res = 0;
    if upto <= spring.len() {
        for i in 0..=(spring.len() - upto) {
            if spring[..i].iter().all(|&c| c == '?' || c == '.')
                && (i + record[0] == spring.len()
                    || spring[i + record[0]] == '.'
                    || spring[i + record[0]] == '?')
                && spring[i..(i + record[0])]
                    .iter()
                    .all(|&c| c == '#' || c == '?')
            {
                res += calc(
                    &spring[i + record[0] + if record.len() > 1 { 1 } else { 0 }..],
                    &record[1..],
                    result,
                );
            }
        }
    }
    result.insert((spring.len(), record.len()), res);
    res
}

impl Solution {
    fn count(&self, data: &[(Vec<char>, Vec<usize>)]) -> u64 {
        data.iter()
            .map(|(spring, record)| calc(spring, record, &mut HashMap::new()))
            .sum()
    }
}
