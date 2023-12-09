use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day09");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("114"))],
            Step::Second => vec![("test0.txt", String::from("2"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<i64>> = input
            .iter()
            .map(|line| {
                line.split(' ')
                    .map(|n| n.parse().expect("Not a number"))
                    .collect()
            })
            .collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, data: &[Vec<i64>]) -> i64 {
        data.iter()
            .map(|seq| {
                let mut seq = seq.clone();
                let mut firsts = vec![];
                while seq.iter().any(|v| *v != 0) {
                    let prev = *seq.first().unwrap();
                    firsts.push(prev);
                    seq = seq
                        .into_iter()
                        .skip(1)
                        .fold((prev, vec![]), |(p, mut a), v| {
                            a.push(v - p);
                            (v, a)
                        })
                        .1;
                }
                seq.push(0);
                for first in firsts.into_iter().rev() {
                    seq = seq
                        .into_iter()
                        .fold((first, vec![first]), |(p, mut a), v| {
                            a.push(p + v);
                            (p + v, a)
                        })
                        .1;
                }
                *seq.last().unwrap()
            })
            .sum()
    }

    fn count2(&self, data: &[Vec<i64>]) -> i64 {
        data.iter()
            .map(|seq| {
                let mut seq = seq.clone();
                let mut firsts = vec![];
                while seq.iter().any(|v| *v != 0) {
                    let prev = *seq.first().unwrap();
                    firsts.push(prev);
                    seq = seq
                        .into_iter()
                        .skip(1)
                        .fold((prev, vec![]), |(p, mut a), v| {
                            a.push(v - p);
                            (v, a)
                        })
                        .1;
                }
                firsts.into_iter().rev().fold(0, |a, v| v - a)
            })
            .sum()
    }
}
