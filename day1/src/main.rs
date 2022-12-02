use std::cmp::Ordering;
use std::env;

struct Elf {
    foods: Vec<u32>
}

impl Elf {
    fn new() -> Elf {
        Elf {
            foods: Vec::new()
        }
    }

    fn add_food(&mut self, calories: u32) {
        self.foods.push(calories);
    }

    fn total_calories(&self) -> u32 {
        let mut sum: u32 = 0;

        for calories in self.foods.iter() {
            sum += calories;
        }

        return sum;
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total_calories() == other.total_calories()
    }
}

impl Eq for Elf {}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut elves = vec![Elf::new()];
    fn reducer(line: &str, mut elves: Vec<Elf>) -> Vec<Elf> {
        if line.is_empty() {
            elves.push(Elf::new());
            return elves;
        }

        let calories = line.parse::<u32>();
        if calories.is_ok() {
            elves.last_mut().unwrap().add_food(calories.unwrap());
        }

        return elves;
    }

    elves = aoc::reduce_file(file_path, elves, reducer);
    elves.sort();
    elves.reverse();

    let mut total = 0;

    for elf in &elves[0..3] {
        total += elf.total_calories();
        println!("Calories: {}", elf.total_calories());
    }

    println!("Total of Top 3: {total}");
}
