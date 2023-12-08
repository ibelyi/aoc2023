use std::collections::{HashMap, HashSet};

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day08");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![
                ("test0.txt", String::from("6")),
                ("test1.txt", String::from("2")),
            ],
            Step::Second => vec![("test2.txt", String::from("6"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let instr: Vec<char> = input[0].chars().collect();
        let steps = input
            .iter()
            .skip(2)
            .map(|v| {
                let mut it = v.split(" = ");
                let key = it.next().expect("Empty string");
                it = it.next().expect("Nothing after =").split(", ");
                let left = it.next().unwrap().chars().skip(1).collect::<String>();
                let right = it.next().unwrap().chars().take(3).collect::<String>();
                (key.to_owned(), (left, right))
            })
            .collect::<HashMap<String, (String, String)>>();
        match step {
            Step::First => self.count(&instr, &steps).to_string(),
            Step::Second => self.count2(&instr, &steps).to_string(),
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        (a, b) = (b % a, a)
    }
    b
}

fn calc(
    start: &str,
    instr: &[char],
    steps: &HashMap<String, (String, String)>,
    mut f: impl FnMut(&str) -> bool,
) -> u64 {
    let mut result = 0;
    let mut curr = start;
    loop {
        for c in instr {
            result += 1;
            let (left, right) = steps.get(curr).expect("Invalid Step!");
            curr = match c {
                'L' => left,
                'R' => right,
                _ => panic!("Invalid instruction {c}"),
            };
            if f(curr) {
                return result;
            }
        }
    }
}

impl Solution {
    fn count(&self, instr: &[char], steps: &HashMap<String, (String, String)>) -> u64 {
        calc("AAA", instr, steps, |curr| curr == "ZZZ")
    }

    fn count2(&self, instr: &[char], steps: &HashMap<String, (String, String)>) -> u64 {
        let ends = steps
            .keys()
            .filter(|k| k.ends_with('Z'))
            .collect::<HashSet<&String>>();
        steps
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(move |v| calc(v, instr, steps, |curr| ends.contains(&curr.to_string())))
            .fold(1, |a, v| (v / gcd(a, v)) * a)
    }
}
