struct TreeGrid {
    rows: usize,
    cols: usize,
    trees: Vec<Vec<u32>>
}

impl TreeGrid {
    fn new() -> TreeGrid {
        TreeGrid { rows: 0, cols: 0, trees: Vec::new() }
    }

    fn add_row(&mut self, line: &str) {
        if line.is_empty() {
            return;
        }

        self.cols = line.len();

        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c as u32 - '0' as u32);
        }

        self.rows += 1;
        self.trees.push(row);
    }

    fn get_tree(&self, i: usize, j: usize) -> u32 {
        return *self.trees
                .get(i)
                .expect("row does not exist")
                .get(j)
                .expect("column does not exist");
    }

    fn is_visible(&self, i: usize, j: usize, height: u32) -> bool {
        if i == 0 || i == self.rows - 1 || j == 0 || j == self.cols - 1 {
            return true;
        }

        let mut visible = true;

        for v in 0..i {
            if self.get_tree(v, j) > height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }

        for v in i+1..self.rows {
            if self.get_tree(v, j) > height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }

        for v in 0..j {
            if self.get_tree(i, v) > height {
                visible = false;
                break;
            }
        }

        if visible {
            return true;
        }

        for v in j+1..self.cols {
            if self.get_tree(i, v) > height {
                visible = false;
                break;
            }
        }

        return visible;
    }

    fn count_visible(&self) -> usize {
        let mut visible: usize = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.is_visible(i, j, self.get_tree(i, j)) {
                    visible += 1;
                }
            }
        }

        return visible;
    }
}

fn main() {
    let mut grid = TreeGrid::new();

    fn reducer(line: &str, grid: &mut TreeGrid) {
        grid.add_row(line);
    }

    if !aoc::reduce_input(8, &mut grid, reducer) {
        panic!("Failed to read input");
    }

    println!("{} Visible Trees", grid.count_visible());
}
