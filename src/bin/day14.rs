use std::collections::HashMap;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day14");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("136"))],
            Step::Second => vec![("test0.txt", String::from("64"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
        match step {
            Step::First => self.count(data).to_string(),
            Step::Second => self.count2(data).to_string(),
        }
    }
}

fn move_n(data: &mut [Vec<char>]) {
    for x in 0..data[0].len() {
        let mut prev = 0;
        for y in 0..data.len() {
            match data[y][x] {
                'O' => {
                    data[y][x] = '.';
                    data[prev][x] = 'O';
                }
                '#' => prev = y,
                _ => continue,
            }
            prev += 1;
        }
    }
}

fn move_s(data: &mut [Vec<char>]) {
    for x in 0..data[0].len() {
        let mut prev = data.len() - 1;
        for y in (0..data.len()).rev() {
            match data[y][x] {
                'O' => {
                    data[y][x] = '.';
                    data[prev][x] = 'O';
                }
                '#' => prev = y,
                _ => continue,
            }
            prev = prev.saturating_sub(1);
        }
    }
}

fn move_w(data: &mut [Vec<char>]) {
    for y in 0..data.len() {
        let mut prev = 0;
        for x in 0..data[0].len() {
            match data[y][x] {
                'O' => {
                    data[y][x] = '.';
                    data[y][prev] = 'O';
                }
                '#' => prev = x,
                _ => continue,
            }
            prev += 1;
        }
    }
}

fn move_e(data: &mut [Vec<char>]) {
    for y in 0..data.len() {
        let mut prev = data[0].len() - 1;
        for x in (0..data[0].len()).rev() {
            match data[y][x] {
                'O' => {
                    data[y][x] = '.';
                    data[y][prev] = 'O';
                }
                '#' => prev = x,

                _ => continue,
            }
            prev = prev.saturating_sub(1);
        }
    }
}

fn cycle(data: &mut [Vec<char>]) {
    move_n(data);
    move_w(data);
    move_s(data);
    move_e(data);
}

fn calc(data: &[Vec<char>]) -> usize {
    data.iter()
        .enumerate()
        .map(|(i, l)| (data.len() - i) * l.iter().filter(|&c| *c == 'O').count())
        .sum()
}

fn cache(data: &[Vec<char>]) -> Vec<u128> {
    data.iter()
        .map(|l| {
            l.iter()
                .fold(0, |a, &c| (a << 1) + if c == 'O' { 1 } else { 0 })
        })
        .collect()
}

impl Solution {
    fn count(&self, mut data: Vec<Vec<char>>) -> usize {
        move_n(&mut data);
        calc(&data)
    }

    fn count2(&self, mut data: Vec<Vec<char>>) -> usize {
        let mut saw: HashMap<Vec<u128>, usize> = HashMap::new();
        let mut jumped = false;
        let total = 1000000000;
        let mut curr = 0;
        while curr < total {
            cycle(&mut data);
            curr += 1;
            if !jumped {
                let val = cache(&data);
                if let Some(p) = saw.get(&val) {
                    curr = total - (total - p) % (curr - p);
                    jumped = true;
                }
                saw.insert(val, curr);
            }
        }
        calc(&data)
    }
}
