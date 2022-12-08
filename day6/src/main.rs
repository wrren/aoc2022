use std::collections::VecDeque;
use itertools::Itertools;

fn find_sequence_position(datastream: &String, length: usize) -> Option<usize> {
    let mut buffer: VecDeque<char> = VecDeque::new();
    for (i, c) in datastream.char_indices() {
        if buffer.len() == length {
            buffer.pop_back();
        }
        buffer.push_front(c);
        if buffer.iter().unique().count() == length {
            return Some(i + 1);
        }
    }
    return None;
}

fn main() {
    let mut datastream = String::new();

    fn reducer(line: &str, datastream: &mut String) {
        datastream.push_str(line);
    }

    if !aoc::reduce_input(6, &mut datastream, reducer) {
        panic!("Failed to read input");
    }

    println!("Input Length: {}", datastream.len());

    println!("Start-of-Packet after character {}", find_sequence_position(&datastream, 4).expect("Failed to find start-of-packet position"));
    println!("Start-of-Message after character {}", find_sequence_position(&datastream, 14).expect("Failed to find start-of-message position"));
}
