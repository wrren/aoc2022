use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn reduce_file<T>(path: &String, initial: T, reducer: fn(line: &str, accumulator: T) -> T) -> T {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let mut accumulator = initial;

    for line in lines.iter() {
        accumulator = reducer(line, accumulator);
    }

    return accumulator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
