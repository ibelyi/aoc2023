use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day13");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("405"))],
            Step::Second => vec![("test0.txt", String::from("400"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: Vec<Vec<Vec<char>>> = vec![];
        let mut curr: Vec<Vec<char>> = vec![];
        for line in input {
            if line.is_empty() {
                data.push(curr.clone());
                curr.clear();
            } else {
                curr.push(line.chars().collect());
            }
        }
        data.push(curr);
        self.count(&data, step).to_string()
    }
}

fn mirror(lines: Vec<usize>, step: &Step) -> usize {
    for i in 1..lines.len() {
        let max = if i * 2 <= lines.len() {
            i
        } else {
            lines.len() - i
        };
        match step {
            Step::Second => {
                if (0..max)
                    .map(|k| {
                        let diff = lines[i - k - 1] ^ lines[i + k];
                        if diff == 0 {
                            0
                        } else if diff == 1 << diff.ilog2() {
                            1
                        } else {
                            10
                        }
                    })
                    .sum::<usize>()
                    == 1
                {
                    return i;
                }
            }
            Step::First => {
                if (0..max).all(|k| lines[i - k - 1] == lines[i + k]) {
                    return i;
                }
            }
        }
    }
    0
}

fn horizon(area: &[Vec<char>], step: &Step) -> usize {
    mirror(
        area.iter()
            .map(|line| {
                line.iter()
                    .fold(0, |a, &c| if c == '#' { 1 } else { 0 } + (a << 1))
            })
            .collect(),
        step,
    )
}

fn vertical(area: &[Vec<char>], step: &Step) -> usize {
    mirror(
        (0..area[0].len())
            .map(|c| {
                (0..area.len()).fold(0, |a, r| if area[r][c] == '#' { 1 } else { 0 } + (a << 1))
            })
            .collect(),
        step,
    )
}

impl Solution {
    fn count(&self, data: &[Vec<Vec<char>>], step: &Step) -> usize {
        data.iter()
            .map(|area| vertical(area, step) + 100 * horizon(area, step))
            .sum()
    }
}
