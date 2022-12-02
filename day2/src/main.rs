use std::env;

struct RockPaperScissors {
    moves: (char, char),
}

impl RockPaperScissors {
    fn new(theirs: char, mine: char) -> RockPaperScissors {
        RockPaperScissors{
            moves: (theirs, mine)
        }
    }

    fn score_part1(&self) -> u32 {
        match self.moves {
            ('A', 'X')  => 4,
            ('A', 'Y')  => 8,
            ('A', 'Z')  => 3,
            ('B', 'X')  => 1,
            ('B', 'Y')  => 5,
            ('B', 'Z')  => 9,
            ('C', 'X')  => 7,
            ('C', 'Y')  => 2,
            ('C', 'Z')  => 6,
            _           => 0
        }
    }

    fn score_part2(&self) -> u32 {
        match self.moves {
            ('A', 'X')  => 3,
            ('A', 'Y')  => 4,
            ('A', 'Z')  => 8,
            ('B', 'X')  => 1,
            ('B', 'Y')  => 5,
            ('B', 'Z')  => 9,
            ('C', 'X')  => 2,
            ('C', 'Y')  => 6,
            ('C', 'Z')  => 7,
            _           => 0
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut moves: Vec<RockPaperScissors> = Vec::new();

    fn reducer(line: &str, mut moves: Vec<RockPaperScissors>) -> Vec<RockPaperScissors> {
        let components: Vec<&str> = line.split(" ").collect();

        if components.len() == 2 {
            let action = RockPaperScissors::new(components[0].chars().last().unwrap(), components[1].chars().last().unwrap());
            moves.push(action);
        }

        return moves;
    }

    moves = aoc::reduce_file::<Vec<RockPaperScissors>>(file_path, moves, reducer);

    let mut total: u32 = 0;

    for action in moves.iter() {
        total += action.score_part1();
    }

    println!("Total Score (Part 1): {total}");

    total = 0;

    for action in moves.iter() {
        total += action.score_part2();
    }

    println!("Total Score (Part 2): {total}");
}
