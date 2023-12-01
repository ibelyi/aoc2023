use aoc2023::{Solver, Step};
use regex::Regex;

pub fn main() {
    let solver = Solution {};
    solver.solve("day01");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("142"))],
            Step::Second => vec![("test1.txt", String::from("281"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<&String> = input.iter().collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

const REG_STR: &str = "1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine";
fn to_digit(l: &str) -> u32 {
    match l {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("Invalid match"),
    }
}

impl Solution {
    fn count(&self, data: &[&String]) -> u32 {
        data.iter()
            .map(|&line| {
                let mut it = line.chars().filter_map(|c| c.to_digit(10));
                if let Some(first) = it.next() {
                    if let Some(last) = it.last() {
                        first * 10 + last
                    } else {
                        first * 11
                    }
                } else {
                    panic!("Wrong input!");
                }
            })
            .sum()
    }

    fn count2(&self, data: &[&String]) -> u32 {
        let fwd_re = Regex::new(REG_STR).unwrap();
        let rev_re = Regex::new(&REG_STR.chars().rev().collect::<String>()).unwrap();
        data.iter()
            .map(|line| {
                10 * to_digit(fwd_re.find(line).expect("Wrong input!").as_str())
                    + to_digit(
                        &rev_re
                            .find(&line.chars().rev().collect::<String>())
                            .expect("Unexpected, can't find in reverse")
                            .as_str()
                            .chars()
                            .rev()
                            .collect::<String>(),
                    )
            })
            .sum()
    }
}
