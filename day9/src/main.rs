use std::collections::HashSet;

struct Coordinates {
    x: i32,
    y: i32
}

struct Head {
    coordinates: Coordinates
}

impl Head {
    fn new() -> Head {
        Head { coordinates: Coordinates{x: 0, y: 0} }
    }

    fn apply_motion(&mut self, direction: char, distance: i32) {
        match direction {
            'R' => self.coordinates.x += distance,
            'L' => self.coordinates.x -= distance,
            'U' => self.coordinates.y += distance,
            'D' => self.coordinates.y -= distance,
            _   => return
        }
    }
}

struct Tail {
    coordinates: Coordinates
}

impl Tail {
    fn new() -> Tail {
        Tail { coordinates: Coordinates{ x: 0, y: 0 } }
    }

    fn follow(&mut self, head: &Head, visited: &mut HashSet<Coordinates>) {

    }
}

struct Rope {
    head: Head,
    tail: Tail,
    visited: HashSet<Coordinates>
}

impl Rope {
    fn new() -> Rope {
        Rope{ head: Head::new(), tail: Tail::new(), visited: HashSet::new() }
    }

    fn apply_motion(&mut self, direction: char, distance: i32) {
        self.head.apply_motion(direction, distance);
        self.tail.follow(&self.head, &mut self.visited);
    }
}

fn main() {
    let mut rope = Rope::new();

    fn reducer(line: &str, rope: &mut Rope) {
        if line.is_empty() {
            return;
        }

        let split: Vec<&str> = line.split(" ").collect();
        if split.len() != 2 || split[0].len() != 1 {
            panic!("Unexpected line format");
        }
        let direction = split[0].chars().nth(0).unwrap();
        let distance = split[1].parse::<i32>().expect("Distance is not an integer");
        
        rope.apply_motion(direction, distance);
    }

    if !aoc::reduce_input(9, &mut rope, reducer) {
        panic!("Failed to read input");
    }
}
