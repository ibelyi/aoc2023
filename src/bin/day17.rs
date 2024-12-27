use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day17");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("102"))],
            Step::Second => vec![("test0.txt", String::from("94"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<u32>> = input
            .iter()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        match step {
            Step::First => self.count(&data, 1, 3).to_string(),
            Step::Second => self.count(&data, 4, 10).to_string(),
        }
    }
}

impl Solution {
    fn count(&self, data: &[Vec<u32>], min: usize, max: usize) -> u32 {
        let mut result = vec![vec![vec![vec![u32::MAX; max]; 4]; data[0].len()]; data.len()];
        result[1][0][2][0] = data[1][0];
        result[0][1][1][0] = data[0][1];
        let mut updated = true;
        while updated {
            updated = false;
            for y in 0..data.len() {
                for x in 0..data[0].len() {
                    for dir in 0..4 {
                        for step in 0..max {
                            if result[y][x][dir][step] != u32::MAX {
                                let mut next = Vec::new();
                                if y > 0
                                    && dir != 2
                                    && ((dir != 0 && step >= min - 1)
                                        || (dir == 0 && step < max - 1))
                                {
                                    next.push((y - 1, x, 0));
                                }
                                if x > 0
                                    && dir != 1
                                    && ((dir != 3 && step >= min - 1)
                                        || (dir == 3 && step < max - 1))
                                {
                                    next.push((y, x - 1, 3));
                                }
                                if y < data.len() - 1
                                    && dir != 0
                                    && ((dir != 2 && step >= min - 1)
                                        || (dir == 2 && step < max - 1))
                                {
                                    next.push((y + 1, x, 2));
                                }
                                if x < data[0].len() - 1
                                    && dir != 3
                                    && ((dir != 1 && step >= min - 1)
                                        || (dir == 1 && step < max - 1))
                                {
                                    next.push((y, x + 1, 1));
                                }
                                for (y0, x0, d0) in next {
                                    let step0 = if dir == d0 { step + 1 } else { 0 };
                                    if result[y0][x0][d0][step0]
                                        > result[y][x][dir][step] + data[y0][x0]
                                    {
                                        updated = true;
                                        result[y0][x0][d0][step0] =
                                            result[y][x][dir][step] + data[y0][x0];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        result[data.len() - 1][data[0].len() - 1]
            .iter()
            .flatten()
            .copied()
            .min()
            .unwrap()
    }
}
