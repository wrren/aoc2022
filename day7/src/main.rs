struct File {
    name: String,
    size: usize
}

struct Directory {
    name: String,
    files: Vec<File>,
    parent: *const Directory,
    subdirectories: Vec<Directory>
}

impl Directory {
    fn new(name: String, parent: *const Directory) -> Directory {
        Directory{
            name: name,
            parent: parent,
            files: Vec::new(),
            subdirectories: Vec::new()
        }
    }

    fn size(&self) -> usize {
        let mut size: usize = self.files.iter().fold(0, |sum, file| sum + file.size);
        
        for directory in self.subdirectories.iter() {
            size += directory.size();
        }

        return size;
    }

    fn root(&self) -> &Directory {
        if !self.parent.is_null() {
            return unsafe { self.parent.as_ref().unwrap().root() };
        }
        return &self;
    }

    fn root_mut(&mut self) -> &mut Directory {
        if !self.parent.is_null() {
            return unsafe { self.parent.cast_mut().as_mut().unwrap().root_mut() };
        }
        return self;
    }
}

fn main() {
    let mut root = Directory::new(String::from("/"), std::ptr::null());

    fn reducer(line: &str, directory: &mut Directory) {

    }

    if !aoc::reduce_input(7, &mut root, reducer) {
        panic!("Failed to read input.");
    }

    println!("Full Size: {}", root.root().size());
}
