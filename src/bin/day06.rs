use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day06");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("288"))],
            Step::Second => vec![("test0.txt", String::from("71503"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<u32>> = input
            .iter()
            .map(|line| {
                line.split(':')
                    .last()
                    .expect("Nothing after :")
                    .split(' ')
                    .filter_map(|line| line.parse().ok())
                    .collect()
            })
            .collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

fn value_radix(val: u32) -> u64 {
    let mut res = 1;
    while res < val {
        res *= 10;
    }
    res as u64
}

impl Solution {
    fn count(&self, data: &[Vec<u32>]) -> u32 {
        (0..data[0].len())
            .map(|i| {
                (1..data[0][i])
                    .filter(|p| (data[0][i] - p) * p > data[1][i])
                    .count() as u32
            })
            .product()
    }

    fn count2(&self, data: &[Vec<u32>]) -> u64 {
        let data: Vec<u64> = data
            .iter()
            .map(|v| v.iter().fold(0u64, |a, v| a * value_radix(*v) + *v as u64))
            .collect();
        let guess = ((data[0] * data[0] - 4 * data[1]) as f64).sqrt() as u64;
        let mut min = (data[0] - guess) / 2;
        while (data[0] - min) * min > data[1] {
            min -= 1;
        }
        while (data[0] - min) * min < data[1] {
            min += 1;
        }
        let mut max = (data[0] + guess) / 2;
        while (data[0] - max) * max > data[1] {
            max += 1;
        }
        while (data[0] - max) * max < data[1] {
            max -= 1;
        }
        max - min + 1
    }
}
