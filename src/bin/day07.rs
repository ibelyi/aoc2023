use std::{cmp::Ordering, collections::HashMap};

use aoc2023::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day07");
}
struct Solution {}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

struct Card {
    kind: Kind,
    hand: Vec<u32>,
    bid: u32,
}

impl Card {
    fn parse(line: &str, step: &Step) -> Card {
        let mut it = line.split(' ');
        let hand: Vec<u32> = it
            .next()
            .expect("Empty line")
            .chars()
            .map(|c| match c {
                'T' => 10,
                'J' => match step {
                    Step::First => 11,
                    Step::Second => 1,
                },
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                c => c.to_digit(10).expect("Not a number"),
            })
            .collect();
        let bid = it.next().expect("No space").parse().expect("Not a number");
        let kind = kind(&hand);
        Card { kind, hand, bid }
    }
}

fn kind(a: &[u32]) -> Kind {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for c in a {
        *map.entry(*c).or_insert(0) += 1;
    }
    if let Some(j) = map.get(&1) {
        if map.len() == 5 {
            Kind::OnePair
        } else if map.len() == 4 {
            Kind::Three
        } else if map.len() == 3 {
            if *j > 1 || map.iter().any(|(_, v)| *v == 3) {
                Kind::Four
            } else {
                Kind::FullHouse
            }
        } else {
            Kind::Five
        }
    } else if map.len() == 5 {
        Kind::HighCard
    } else if map.len() == 4 {
        Kind::OnePair
    } else if map.len() == 3 {
        if map.iter().any(|(_, v)| *v == 2) {
            Kind::TwoPair
        } else {
            Kind::Three
        }
    } else if map.len() == 2 {
        if map.iter().any(|(_, v)| *v == 3) {
            Kind::FullHouse
        } else {
            Kind::Four
        }
    } else {
        Kind::Five
    }
}

fn compare(a: &Card, b: &Card) -> Ordering {
    match a.kind.cmp(&b.kind) {
        Ordering::Equal => {
            for (i, ac) in a.hand.iter().enumerate() {
                match ac.cmp(&b.hand[i]) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    _ => {}
                }
            }
            Ordering::Equal
        }
        o => o,
    }
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("6440"))],
            Step::Second => vec![("test0.txt", String::from("5905"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: Vec<Card> = input.iter().map(|line| Card::parse(line, step)).collect();
        data.sort_by(compare);
        data.iter()
            .enumerate()
            .map(|(i, c)| (i as u32 + 1) * c.bid)
            .sum::<u32>()
            .to_string()
    }
}
