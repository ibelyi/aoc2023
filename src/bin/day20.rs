use std::collections::{HashMap, HashSet};

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day20");
}
struct Solution {}

#[derive(Clone)]
enum Module {
    Broadcast(Vec<String>),
    FlipFlop(bool, Vec<String>),
    Conjunct(HashMap<String, bool>, Vec<String>),
}

impl Module {
    fn parse(line: &str) -> (String, Module) {
        let mut split = line.split(" -> ");
        let key = split.next().unwrap();
        let next = split
            .next()
            .unwrap()
            .split(", ")
            .map(|l| l.to_string())
            .collect();
        let first = key.chars().next().unwrap();
        match first {
            '%' => (
                key.chars().skip(1).collect::<String>(),
                Module::FlipFlop(false, next),
            ),
            '&' => (
                key.chars().skip(1).collect::<String>(),
                Module::Conjunct(HashMap::new(), next),
            ),
            _ => (key.to_string(), Module::Broadcast(next)),
        }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => {
                vec![
                    ("test0.txt", String::from("32000000")),
                    ("test1.txt", String::from("11687500")),
                ]
            }
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: HashMap<String, Module> = input.iter().map(|l| Module::parse(l)).collect();
        let mut map = HashMap::<String, Vec<String>>::new();
        for (key, val) in data.iter() {
            for out in match val {
                Module::Broadcast(v) => v,
                Module::Conjunct(_, v) => v,
                Module::FlipFlop(_, v) => v,
            } {
                if let Some(Module::Conjunct(_, _)) = data.get(out) {
                    map.entry(out.to_string())
                        .or_default()
                        .push(key.to_string());
                }
            }
        }
        for (key, val) in map {
            if let Module::Conjunct(inp, _) = data.get_mut(&key).unwrap() {
                for key in val {
                    inp.insert(key, false);
                }
            }
        }
        match step {
            Step::First => self.count(&mut data).to_string(),
            Step::Second => self.count2(&mut data).to_string(),
        }
    }
}

fn make_step(
    data: &mut HashMap<String, Module>,
    start: &str,
    mut func: impl FnMut(&(String, bool)) -> bool,
) -> bool {
    let mut stack = vec![("".to_string(), start.to_string(), false)];
    while let Some((from, to, sig)) = stack.pop() {
        if let Some(module) = data.get_mut(&to) {
            match module {
                Module::Broadcast(out) => {
                    for m in out {
                        stack.insert(0, (to.clone(), m.to_string(), sig));
                    }
                }
                Module::FlipFlop(state, out) => {
                    if !sig {
                        *state = !*state;
                        for m in out {
                            stack.insert(0, (to.clone(), m.to_string(), *state));
                        }
                    }
                }
                Module::Conjunct(inp, out) => {
                    *inp.get_mut(&from).unwrap() = sig;
                    for m in out {
                        stack.insert(0, (to.clone(), m.to_string(), !inp.values().all(|&v| v)));
                    }
                }
            }
        }
        if func(&(to, sig)) {
            return true;
        }
    }
    false
}

fn steps(data: &mut HashMap<String, Module>, start: &str, end: &str) -> u64 {
    let mut step = 0;
    'end: loop {
        step += 1;
        if make_step(data, start, |(to, sig)| to == end && !sig) {
            break 'end step;
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        (a, b) = (b % a, a)
    }
    b
}

fn find_end(data: &HashMap<String, Module>, start: &str) -> String {
    let mut touched = HashSet::<&str>::new();
    let mut stack = vec![start];
    let mut last = start;
    while let Some(curr) = stack.pop() {
        if !touched.contains(curr) {
            for next in match data
                .get(curr)
                .unwrap_or_else(|| panic!("Cant find destination {curr}"))
            {
                Module::Broadcast(v) => v,
                Module::FlipFlop(_, v) => v,
                Module::Conjunct(_, v) => v,
            } {
                if next == "rx" {
                    last = curr;
                } else {
                    stack.push(next);
                }
            }
            touched.insert(curr);
        }
    }
    if let Some(Module::Conjunct(inp, _)) = data.get(last) {
        for prev in inp.keys() {
            if touched.contains(prev as &str) {
                return prev.to_string();
            }
        }
        panic!("Failed to find how we got to {last}");
    } else {
        panic!("Missing {last} or not a Conjunct");
    }
}

impl Solution {
    fn count(&self, data: &mut HashMap<String, Module>) -> u64 {
        let mut beam = (0, 0);
        for _ in 0..1000 {
            _ = make_step(data, "broadcaster", |(_, sig)| {
                if *sig {
                    beam.1 += 1;
                } else {
                    beam.0 += 1;
                };
                false
            });
        }
        beam.0 * beam.1
    }

    fn count2(&self, data: &mut HashMap<String, Module>) -> u64 {
        // Independent streams of signals. First find starts and ends
        let start_ends: Vec<(String, String)> =
            if let Some(Module::Broadcast(v)) = data.get("broadcaster") {
                v.iter().map(|st| (st.to_string(), find_end(data, st)))
            } else {
                panic!("Broadcaster module is missing or not a Broadcast type!");
            }
            .collect(); // Collect to colapse iterator and eliminate reference to data
        start_ends
            .iter()
            // Calculate number of steps to have low signal in each stream
            .map(|(start, end)| steps(data, start, end))
            // Then use smallest common multiplier to known when they do it simultaneously
            .fold(1, |a, v| (v / gcd(a, v)) * a)
    }
}
