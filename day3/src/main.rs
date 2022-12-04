use aoc;
use std::{collections::HashSet};

fn priority(item: char) -> u32 {
    if item >= 'A' && item <= 'Z' {
        return ((item as u8 - 'A' as u8) + 27) as u32
    } else if item >= 'a' && item <= 'z' {
        return ((item as u8 - 'a' as u8) + 1) as u32
    }
    return 0
}

#[derive(Clone)]
struct RuckSack {
    all: HashSet<char>,
    first: Compartment,
    second: Compartment
}

impl RuckSack {
    fn new(line: String) -> RuckSack {
        let compartments = line.split_at(&line.len() / 2);

        RuckSack{
            all: HashSet::from_iter(line.chars()),
            first: Compartment::new(compartments.0.to_string()),
            second: Compartment::new(compartments.1.to_string())
        }
    }

    fn common_item(&self) -> Option<&char> {
        return self.first.items.intersection(&self.second.items).last();
    }
}

#[derive(Clone)]
struct Compartment {
    items: HashSet<char>
}

impl Compartment {
    fn new(s: String) -> Compartment {
        Compartment { items: s.chars().collect() }
    }
}

fn main() {
    let mut sacks: Vec<RuckSack> = Vec::new();

    fn reducer(line: &str, sacks: &mut Vec<RuckSack>) {
        if line.is_empty() {
            return;
        }

        let sack = RuckSack::new(line.to_string());
        sacks.push(sack);
    }

    if !aoc::reduce_input(3, &mut sacks, reducer) {
        println!("Failed to retrieve input.");
        return;
    }

    let mut total: u32 = 0;

    for sack in &sacks {
        let common_item = sack.common_item()
        .expect("Sack doesn't contain any item common to both compartments!");

        let priority = priority(*common_item);
        total += priority;
    }

    println!("Part 1: Total Priority: {}", total);
    let chunks: Vec<&[RuckSack]> = sacks.chunks(3).collect();

    total = 0;

    for chunk in chunks.iter() {
        let a: HashSet<&char> = chunk[0].all.intersection(&chunk[1].all).collect();
        let b: HashSet<&char> = chunk[1].all.intersection(&chunk[2].all).collect();
        let c = a.intersection(&b);

        let common = c.last()
            .expect("No common items across set of sacks!");

        let priority = priority(**common);
        total += priority;
    }

    println!("Part 2: Total Priority: {}", total);
}
