use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct File {
    size: usize
}

struct Directory {
    path: String,
    files: Vec<File>,
    subdirectories: HashSet<String>
}

impl Directory {
    fn new(path: String) -> Directory {
        Directory{
            path: path,
            files: Vec::new(),
            subdirectories: HashSet::new()
        }
    }

    fn cd(&self, to: &str) -> Option<String> {
        if to == "/" {
            return Some(String::from("/"));
        } else if to == ".." && self.path == "/" {
            return None;
        } else if to == ".." {
            let mut components: Vec<&str> = self.path
                .split("/")
                .filter(|p| !p.is_empty())
                .collect();

            components.pop();
            if components.is_empty() {
                return Some(String::from("/"));
            }

            return Some(format!("/{}/", components.join("/")));
        } else {
            let subdir = self.subdirectories.get(to);
            if subdir.is_none() {
                return None;
            }
            return Some(String::from(format!("{}{}/", self.path, subdir.unwrap())));
        }
    }

    fn add_file(&mut self, size: usize) {
        self.files.push(File { size: size })
    }

    fn size(&self) -> usize {
        return self.files.iter().fold(0, |sum, file| sum + file.size);
    }

    fn size_recursive(&self, fs: &FileSystem) -> usize {
        let mut size = self.size();

        for dir_name in self.subdirectories.iter() {
            let full_path = format!("{}{}/", self.path, dir_name);
            let dir = fs.find(&full_path).expect("SubDirectory not found!");
            size += dir.size_recursive(fs);
        }

        return size;
    }
}

struct FileSystem {
    pwd: String,
    directories: HashMap<String, Directory>
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            pwd: String::from("/"),
            directories: HashMap::new()
        };
        fs.directories.insert(String::from("/"), Directory::new(String::from("/")));
        return fs;
    }

    fn add_dir(&mut self, name: &str) {
        let pwd = self.directories.get_mut(&self.pwd).expect(
            "PWD cannot be found"
        );

        let full_path = format!("{}{}/", self.pwd, name);

        pwd.subdirectories.insert(name.to_string());

        self.directories.insert(full_path.clone(), Directory::new(full_path));
    }

    fn add_file(&mut self, size: usize) {
        let pwd = self.directories.get_mut(&self.pwd).expect(
            "PWD cannot be found"
        );

        pwd.add_file(size);
    }

    fn find(&self, path: &str) -> Option<&Directory> {
        return self.directories.get(path);
    }

    fn cd(&mut self, to: &str) {
        let pwd = self.directories.get(&self.pwd).expect(
            "PWD cannot be found"
        );

        let cd = pwd.cd(to);
        if cd.is_none() {
            panic!("CD target cannot be found");
        }
        self.pwd = cd.unwrap();
    }
}


fn main() {
    let mut filesystem = FileSystem::new();

    fn reducer(line: &str, filesystem: &mut FileSystem) {
        lazy_static! {
            static ref RE_CD: Regex = Regex::new(r"\$ cd ([a-z\./]+)").unwrap();
            static ref RE_DIR: Regex = Regex::new(r"dir ([a-z]+)").unwrap();
            static ref RE_FILE: Regex = Regex::new(r"([0-9]+) ([a-z0-9\.]+)").unwrap();
        }
        if RE_CD.is_match(line) {
            let captures: Vec<regex::Captures> = RE_CD.captures_iter(line).collect();
            let dir_name = &captures[0][1];
            filesystem.cd(dir_name);
        } else if RE_DIR.is_match(line) {
            let captures: Vec<regex::Captures> = RE_DIR.captures_iter(line).collect();
            let dir_name = &captures[0][1];
            filesystem.add_dir(dir_name);
        } else if RE_FILE.is_match(line) {
            let captures: Vec<regex::Captures> = RE_FILE.captures_iter(line).collect();
            let size = captures[0][1].parse::<usize>().expect("File size is not an integer");
            filesystem.add_file(size);
        }
    }

    if !aoc::reduce_input(7, &mut filesystem, reducer) {
        panic!("Failed to read input.");
    }

    let mut sum: usize = 0;
    let mut used_space: usize = 0;

    for (_, directory) in filesystem.directories.iter() {
        let size = directory.size_recursive(&filesystem);
        if size <= 100_000 {
            sum += size;
        }
        if directory.path == "/" {
            used_space = size;
        }
    }

    println!("Sum of total directory sizes: {sum}B");

    let total_space: usize = 70000000;
    let needed_space: usize = 30000000;
    let available_space = total_space - used_space;
    let mut smallest: usize = std::usize::MAX;
    let mut path = String::new();

    for (_, directory) in filesystem.directories.iter() {
        let size = directory.size_recursive(&filesystem);

        if (available_space + size >= needed_space) && size < smallest {
            path = directory.path.clone();
            smallest = size;
        }
    }

    println!("Delete Directory {} to free {}B of space", path, smallest);
}
