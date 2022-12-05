use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;
use regex::Regex;

trait Crane {
    fn move_crates(&self, instructions: &Vec<MoveInstruction>, hold: &CargoHold) -> CargoHold;
}

struct CrateMover9000 {

}

impl Crane for CrateMover9000 {
    fn move_crates(&self, instructions: &Vec<MoveInstruction>, hold: &CargoHold) -> CargoHold {
        let mut new_hold = (*hold).clone();

        for instruction in instructions.iter() {
            for _ in 0..instruction.count {
                let top = new_hold.stacks.get_mut(&instruction.from).ok_or_else(
                    || format!("Couldn't find 'from' stack {}", instruction.from)
                ).unwrap().crates.pop_front().expect("Not enough crates on stack to satisfy move!");

                new_hold.stacks.get_mut(&instruction.to).ok_or_else(
                    || format!("Couldn't find 'to' stack {}", instruction.to)
                ).unwrap().crates.push_front(top);
            }
        }

        return new_hold;
    }
}

struct CrateMover9001 {

}

impl Crane for CrateMover9001 {
    fn move_crates(&self, instructions: &Vec<MoveInstruction>, hold: &CargoHold) -> CargoHold {
        let mut new_hold = (*hold).clone();

        for instruction in instructions.iter() {
            let mut moved: Vec<char> = Vec::new();

            for _ in 0..instruction.count {
                let top = new_hold.stacks.get_mut(&instruction.from).ok_or_else(
                    || format!("Couldn't find 'from' stack {}", instruction.from)
                ).unwrap().crates.pop_front().expect("Not enough crates on stack to satisfy move!");

                moved.push(top);
            }

            for _ in 0..instruction.count {
                new_hold.stacks.get_mut(&instruction.to).ok_or_else(
                    || format!("Couldn't find 'to' stack {}", instruction.to)
                ).unwrap().crates.push_front(moved.pop().unwrap());
            }
        }

        return new_hold;
    }
}

struct MoveInstruction {
    from:   u32,
    to:     u32,
    count:  u32
}

impl MoveInstruction {
    fn new(from: u32, to: u32, count: u32) -> MoveInstruction {
        MoveInstruction{
            from: from,
            to: to,
            count: count
        }
    }
}

#[derive(Clone)]
struct Stack {
    crates: VecDeque<char>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            crates: VecDeque::new()
        }
    }

    fn add_crate(&mut self, c: char) {
        self.crates.push_back(c);
    }
}

#[derive(Clone)]
struct CargoHold {
    stacks: HashMap<u32, Stack>
}

impl CargoHold {
    fn new() -> CargoHold {
        CargoHold { stacks: HashMap::new() }
    }

    fn add_crate(&mut self, index: u32, c: char) {
        if !self.stacks.contains_key(&index) {
            self.stacks.insert(index, Stack::new());
        }
        self.stacks.get_mut(&index).unwrap().add_crate(c);
    }

    fn top_crates(&self) -> String {
        let mut tops = String::new();

        for index in 1..(self.stacks.len() + 1) {
            let top = self.stacks.get(&(index as u32))
                .expect("Failed to get stack")
                .crates
                .front()
                .expect("Failed to get top of stack");

            tops.push(*top);
        }

        return tops;
    }
}

struct Context {
    hold: CargoHold,
    moves: Vec<MoveInstruction>
}

fn main() {
    let mut context = Context{ hold: CargoHold::new(), moves: Vec::new() };

    fn reducer(line: &str, context: &mut Context) {
        if line.find("move").is_some() {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            }

            for captures in RE.captures_iter(line) {
                let count = captures[1].parse::<u32>().unwrap();
                let from = captures[2].parse::<u32>().unwrap();
                let to = captures[3].parse::<u32>().unwrap();

                context.moves.push(MoveInstruction::new(from, to, count));
            }
        } else if line.find("[").is_some() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '[' | ']' | ' ' => continue,
                    _ => {
                        let index: u32 = ((i / 4) + 1) as u32;
                        context.hold.add_crate(index, c);
                    }
                }
            }
        }
    }

    if !aoc::reduce_input(5, &mut context, reducer) {
        panic!("Failed to read input.");
    }

    let crate_mover_9000 = CrateMover9000{};
    let moved_part1 = crate_mover_9000.move_crates(&context.moves, &context.hold);
    println!("Top Crates (Part 1): {}", moved_part1.top_crates());

    let crate_mover_9001 = CrateMover9001{};
    let moved_part2 = crate_mover_9001.move_crates(&context.moves, &context.hold);
    println!("Top Crates (Part 2): {}", moved_part2.top_crates());
}
