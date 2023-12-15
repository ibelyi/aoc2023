use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day15");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("1320"))],
            Step::Second => vec![("test0.txt", String::from("145"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<&str> = input[0].split(',').collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

fn hash(line: &str) -> usize {
    line.chars().fold(0, |a, c| ((a + c as usize) * 17) & 0xFF)
}

impl Solution {
    fn count(&self, data: &[&str]) -> usize {
        data.iter().map(|&l| hash(l)).sum()
    }

    fn count2(&self, data: &[&str]) -> usize {
        let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
        for &val in data {
            if val.ends_with('-') {
                let label = val.split('-').next().unwrap();
                if let Some(b) = boxes.get_mut(hash(label)) {
                    if let Some(i) = b.iter().position(|(l, _)| *l == label) {
                        b.remove(i);
                    }
                }
            } else {
                let mut it = val.split('=');
                let label = it.next().unwrap();
                let focus = it.next().unwrap().parse::<usize>().unwrap();
                if let Some(b) = boxes.get_mut(hash(label)) {
                    if let Some(i) = b.iter().position(|(l, _)| *l == label) {
                        b[i].1 = focus;
                    } else {
                        b.push((label, focus));
                    }
                }
            }
        }
        boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(k, f)| (i + 1) * (k + 1) * f.1)
                    .sum::<usize>()
            })
            .sum()
    }
}
