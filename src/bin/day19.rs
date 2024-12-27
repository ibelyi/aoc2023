use std::collections::HashMap;

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day19");
}
struct Solution {}

enum Outcome {
    Reject,
    Accept,
    Next(String),
}

impl Outcome {
    fn parse(line: &str) -> Outcome {
        match line {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            s => Outcome::Next(s.to_string()),
        }
    }
}

enum Rule {
    Less(usize, u64),
    More(usize, u64),
}

impl Rule {
    fn parse(line: &str) -> Rule {
        let mut it = line.chars();
        let index = match it.next().unwrap() {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!("Invalid field"),
        };
        let cmp = it.next().unwrap();
        let val = it.collect::<String>().parse().unwrap();
        match cmp {
            '<' => Rule::Less(index, val),
            '>' => Rule::More(index, val),
            _ => panic!("Unknown comparison"),
        }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("19114"))],
            Step::Second => vec![("test0.txt", String::from("167409079868000"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut rules = HashMap::<String, Vec<(Option<Rule>, Outcome)>>::new();
        let mut it = input.iter();
        for line in it.by_ref() {
            if line.is_empty() {
                break;
            }
            let mut split = line.split('{');
            let key = split.next().unwrap();
            let r = split
                .next()
                .unwrap()
                .split('}')
                .next()
                .unwrap()
                .split(',')
                .map(|l| {
                    if l.contains(':') {
                        let mut sp = l.split(':');
                        (
                            Some(Rule::parse(sp.next().unwrap())),
                            Outcome::parse(sp.next().unwrap()),
                        )
                    } else {
                        (None, Outcome::parse(l))
                    }
                })
                .collect();
            rules.insert(key.to_string(), r);
        }
        let parts = it
            .map(|line| {
                let line = line
                    .chars()
                    .skip(1)
                    .take(line.len() - 2)
                    .collect::<String>();
                line.split(',')
                    .map(|f| f.split('=').nth(1).unwrap().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        match step {
            Step::First => self.count(&rules, &parts).to_string(),
            Step::Second => self.count2(&rules).to_string(),
        }
    }
}

impl Solution {
    fn count(
        &self,
        rules: &HashMap<String, Vec<(Option<Rule>, Outcome)>>,
        parts: &[Vec<u64>],
    ) -> u64 {
        parts
            .iter()
            .filter(|p| {
                let mut curr = "in";
                loop {
                    for rule in rules.get(curr).unwrap() {
                        if let Some(r) = &rule.0 {
                            match r {
                                Rule::Less(c, v) => {
                                    if p[*c] >= *v {
                                        continue;
                                    }
                                }
                                Rule::More(c, v) => {
                                    if p[*c] <= *v {
                                        continue;
                                    }
                                }
                            }
                        };
                        match &rule.1 {
                            Outcome::Accept => return true,
                            Outcome::Reject => return false,
                            Outcome::Next(s) => {
                                curr = s;
                                break;
                            }
                        }
                    }
                }
            })
            .map(|p| p.iter().copied().sum::<u64>())
            .sum()
    }

    fn count2(&self, rules: &HashMap<String, Vec<(Option<Rule>, Outcome)>>) -> u64 {
        let mut stack = vec![("in", vec![(1u64, 4001u64); 4])];
        let mut result = 0;
        while let Some((id, mut curr)) = stack.pop() {
            if let Some(rule) = rules.get(id) {
                for (rul, out) in rule {
                    if let Some(ru) = rul {
                        let next = match ru {
                            Rule::Less(i, v) => {
                                let mut copy = curr.clone();
                                copy[*i] = (curr[*i].0, *v);
                                curr[*i].0 = *v;
                                copy
                            }
                            Rule::More(i, v) => {
                                let mut copy = curr.clone();
                                copy[*i] = (*v + 1, curr[*i].1);
                                curr[*i].1 = *v + 1;
                                copy
                            }
                        };
                        match out {
                            Outcome::Accept => {
                                result += next.iter().map(|&(s, e)| e - s).product::<u64>()
                            }
                            Outcome::Reject => {}
                            Outcome::Next(s) => stack.push((s, next)),
                        }
                    } else {
                        match out {
                            Outcome::Accept => {
                                result += curr.iter().map(|&(s, e)| e - s).product::<u64>()
                            }
                            Outcome::Reject => {}
                            Outcome::Next(s) => stack.push((s, curr)),
                        }
                        break;
                    }
                }
            } else {
                panic!("Invalid rule");
            }
        }
        result
    }
}
