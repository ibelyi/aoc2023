use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day05");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("35"))],
            Step::Second => vec![("test0.txt", String::from("46"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut lines = input.iter();
        let seeds: Vec<u64> = lines
            .next()
            .expect("Empty File")
            .split(' ')
            .skip(1)
            .map(|line| line.parse().expect("Not a number"))
            .collect();
        let mut maps: Vec<Vec<Vec<u64>>> = vec![];
        let mut map: Vec<Vec<u64>> = vec![];
        for line in lines.skip(2) {
            if line.is_empty() {
                map.sort_by(|a, b| a[1].cmp(&b[1]));
                maps.push(map);
                map = vec![];
            } else if line.starts_with(|c: char| c.is_ascii_digit()) {
                map.push(
                    line.split(' ')
                        .map(|line| line.parse::<u64>().expect("Not a number"))
                        .collect(),
                );
            }
        }
        map.sort_by(|a, b| a[1].cmp(&b[1]));
        maps.push(map);
        match step {
            Step::First => self.count(&seeds, &maps).to_string(),
            Step::Second => self.count2(&seeds, &maps).to_string(),
        }
    }
}

fn src_to_dest(src: u64, map: &Vec<Vec<u64>>) -> u64 {
    for range in map {
        if src >= range[1] && src < range[1] + range[2] {
            return range[0] + src - range[1];
        }
    }
    src
}

fn range_to_dest(src: Vec<Vec<u64>>, map: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut res: Vec<Vec<u64>> = vec![];
    for s in src {
        let mut start = s[0];
        let mut range = s[1];
        for r in map {
            if start >= r[1] + r[2] {
                continue;
            }
            if start + range <= r[1] {
                break;
            }
            if start < r[1] {
                res.push(vec![start, r[1] - start]);
                range -= r[1] - start;
                start = r[1];
            }
            if start + range <= r[1] + r[2] {
                res.push(vec![r[0] + start - r[1], range]);
                range = 0;
                break;
            }
            res.push(vec![r[0] + start - r[1], r[1] + r[2] - start]);
            range -= r[1] + r[2] - start;
            start = r[1] + r[2];
        }
        if range > 0 {
            res.push(vec![start, range]);
        }
    }
    res
}

impl Solution {
    fn count(&self, seeds: &[u64], maps: &[Vec<Vec<u64>>]) -> u64 {
        seeds
            .iter()
            .map(|s| maps.iter().fold(*s, src_to_dest))
            .min()
            .expect("Empty!")
    }

    fn count2(&self, seeds: &[u64], maps: &[Vec<Vec<u64>>]) -> u64 {
        let seeds: Vec<Vec<u64>> = (0..seeds.len())
            .step_by(2)
            .map(|i| vec![seeds[i], seeds[i + 1]])
            .collect();
        let mut res = maps.iter().fold(seeds, range_to_dest);
        res.sort_by(|a, b| a[0].cmp(&b[0]));
        res[0][0]
    }
}
