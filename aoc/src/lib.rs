use std::fs;
use std::env;
use std::path;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn cache_target(day: u32) -> Option<path::PathBuf> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        return None;
    }

    let path = path::Path::new(&args[0]);
    let parent = path.parent();

    if parent.is_none() {
        return None;
    }

    let mut cache = parent.unwrap().to_path_buf();
    cache.push(".cache");
    if !cache.is_dir() && std::fs::create_dir(&cache).is_err() {
        return None;
    }

    cache.push(format!("day-{}", day));

    return Some(cache);
}

fn cache_read(day: u32) -> Option<String> {
    let target = cache_target(day);
    if target.is_some() {
        let contents = fs::read_to_string(target.unwrap());
        if contents.is_err() {
            return None;
        }
        return Some(contents.unwrap());
    }   

    return None;
}

fn cache_write(day: u32, input: &String) {
    let target_opt = cache_target(day);

    if target_opt.is_some() {
        let target = target_opt.unwrap();
        let target_str = target.to_str().unwrap();
        let result = fs::write(&target, input);

        if result.is_ok() {
            println!("Cache Write Successful");
        } else {
            println!("Cache Write Failed to {}: {}", target_str, result.unwrap_err());
        }
    }
}

pub fn reduce_input<T>(day: u32, initial: &mut T, reducer: fn(line: &str, accumulator: &mut T)) -> bool {
    let input = get_input(day);

    if input.is_err() {
        return false
    } else {
        let body = input.unwrap();
        let lines: Vec<&str> = body.split("\n").collect();

        for line in lines.iter() {
            reducer(line, initial);
        }

        return true;
    }
}

pub fn get_input(day: u32) -> Result<String, reqwest::Error> {
    let cached = cache_read(day);

    if cached.is_none() {
        let download = download_input(day);

        if download.is_ok() {
            let contents = download.unwrap();

            cache_write(day, &contents);
            return Ok(contents);
        } else {
            return Err(download.unwrap_err());
        }
    } else {
        return Ok(cached.unwrap());
    }
}

pub fn download_input(day: u32) -> Result<String, reqwest::Error> {
    let session = env::var("AOC_SESSION")
        .expect("You must define an AOC_SESSION environment variable for input downloading to work!");

    return download_input_with_session(day, session);
}

pub fn download_input_with_session(day: u32, session: String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    return client.get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("Cookie", format!("session={}", session))
        .send()?
        .text();
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
