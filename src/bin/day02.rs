use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day02");
}
struct Solution {}

struct Game {
    id: u32,
    tries: Vec<[u32; 3]>,
}

impl Game {
    fn parse(line: &str) -> Game {
        let mut part = line.split(": ");
        let id = part
            .next()
            .expect("Empty string")
            .split(' ')
            .last()
            .expect("Missing space")
            .parse::<u32>()
            .expect("Not a number");

        let tries = part
            .next()
            .expect("Missing part after :")
            .split("; ")
            .map(|tr| {
                let mut cont = [0, 0, 0];
                for set in tr.split(", ") {
                    let mut part = set.split(' ');
                    let count = part
                        .next()
                        .expect("No number")
                        .parse::<u32>()
                        .expect("Not a number");
                    cont[match part.next().expect("No color") {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => panic!("unknown color"),
                    }] = count;
                }
                cont
            })
            .collect();
        Game { id, tries }
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("8"))],
            Step::Second => vec![("test0.txt", String::from("2286"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Game> = input.iter().map(|line| Game::parse(line)).collect();
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, data: &[Game]) -> u32 {
        data.iter()
            .filter_map(|g| {
                if g.tries.iter().any(|t| t[0] > 12 || t[1] > 13 || t[2] > 14) {
                    None
                } else {
                    Some(g.id)
                }
            })
            .sum()
    }

    fn count2(&self, data: &[Game]) -> u32 {
        data.iter()
            .map(|g| {
                let mut max = vec![0; 3];
                for t in &g.tries {
                    for i in 0..3 {
                        if max[i] < t[i] {
                            max[i] = t[i];
                        }
                    }
                }
                max.into_iter().product::<u32>()
            })
            .sum()
    }
}
