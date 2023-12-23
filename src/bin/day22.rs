use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day22");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("5"))],
            Step::Second => vec![("test0.txt", String::from("7"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: Vec<Vec<Vec<usize>>> = input
            .iter()
            .map(|l| {
                let mut brick: Vec<Vec<usize>> = l
                    .split('~')
                    .map(|c| {
                        c.split(',')
                            .map(|e| e.parse().expect("Not a number"))
                            .collect()
                    })
                    .collect();
                brick.sort_by(|a, b| {
                    if a.len() != b.len() {
                        panic!("Unequal array sizes");
                    }
                    // Ensure that the end closest to origin is first
                    for i in 0..a.len() {
                        let res = a[i].cmp(&b[i]);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    Ordering::Equal
                });
                brick
            })
            .collect();
        // Ensure that the brick closest to the ground is first
        data.sort_by(|a, b| a[0][2].cmp(&b[0][2]));
        for b in 0..data.len() {
            let shift = data
                .iter()
                .filter_map(|o| {
                    if data[b][0][2] > o[1][2]
                        && (0..2).all(|i| data[b][0][i] <= o[1][i] && data[b][1][i] >= o[0][i])
                    {
                        Some(data[b][0][2] - o[1][2] - 1)
                    } else {
                        None
                    }
                })
                .min()
                .unwrap_or(data[b][0][2] - 1);
            for i in 0..2 {
                data[b][i][2] -= shift;
            }
        }
        match step {
            Step::First => self.count(&data).to_string(),
            Step::Second => self.count2(&data).to_string(),
        }
    }
}

fn dependencies(data: &[Vec<Vec<usize>>]) -> HashMap<usize, HashSet<usize>> {
    (0..data.len())
        .map(|b| {
            (
                b,
                data.iter()
                    .enumerate()
                    .filter_map(|(i, o)| {
                        if data[b][0][2] == o[1][2] + 1
                            && (0..2).all(|i| data[b][0][i] <= o[1][i] && data[b][1][i] >= o[0][i])
                        {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        })
        .collect()
}

impl Solution {
    fn count(&self, data: &[Vec<Vec<usize>>]) -> usize {
        let ontop = dependencies(data);
        let mut holds = HashMap::<usize, HashSet<usize>>::new();
        for (b, top) in &ontop {
            for o in top {
                holds.entry(*o).or_default().insert(*b);
            }
        }
        (0..data.len())
            .filter(|i| {
                if let Some(v) = holds.get(i) {
                    v.iter().all(|o| ontop.get(o).unwrap().len() > 1)
                } else {
                    true
                }
            })
            .count()
    }
    fn count2(&self, data: &[Vec<Vec<usize>>]) -> usize {
        let ontop = dependencies(data);
        let mut cache = HashMap::<usize, HashSet<usize>>::new();
        (0..data.len())
            .map(|b| falls(&b, &ontop, &mut cache).len())
            .sum()
    }
}

fn falls<'a>(
    b: &usize,
    ontop: &HashMap<usize, HashSet<usize>>,
    cache: &'a mut HashMap<usize, HashSet<usize>>,
) -> &'a HashSet<usize> {
    if cache.get(b).is_none() {
        let on = ontop.get(b).unwrap();
        let res = if on.is_empty() {
            HashSet::new()
        } else {
            let mut it = on.iter();
            let next = it.next().unwrap();
            let mut res = falls(next, ontop, cache).clone();
            if on.len() == 1 {
                res.insert(*next);
            } else {
                for next in it {
                    let next_res = falls(next, ontop, cache);
                    res = res.intersection(next_res).copied().collect();
                    if res.is_empty() {
                        break;
                    }
                }
            }
            res
        };
        cache.insert(*b, res);
    }
    cache.get(b).unwrap()
}
