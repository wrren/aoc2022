use std::cmp::Ordering;
use std::env;
use std::fs;

struct Elf {
    foods: Vec<u32>
}

impl Elf {
    fn new() -> Elf {
        Elf {
            foods: Vec::new()
        }
    }

    fn has_food(&self) -> bool {
        !self.foods.is_empty()
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

    println!("Reading input from {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut elves: Vec<Elf> = Vec::new();
    let mut elf: Elf = Elf::new();

    for line in lines.iter() {
        if line.is_empty() {
            if elf.has_food() {
                elves.push(elf);
                elf = Elf::new();
            }
        } else {
            let calories = line.parse::<u32>();
            if calories.is_ok() {
                elf.add_food(calories.unwrap());
            }
        }
    }

    if elf.has_food() {
        elves.push(elf);
    }

    elves.sort();
    elves.reverse();

    let mut total = 0;

    for elf in &elves[0..3] {
        total += elf.total_calories();
        println!("Calories: {}", elf.total_calories());
    }

    println!("Total of Top 3: {total}");
}
