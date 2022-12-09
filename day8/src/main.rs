use std::collections::HashMap;

struct Tree {
    height: i32,
    visibility: usize,
    viewing: Vec<usize>
}

impl Tree {
    fn scenic_score(&self) -> usize {
        return self.viewing.iter().fold(1, |s, v| s * v);
    }

    fn maybe_drop_visibility(&mut self, to_edge: usize, height: i32, heights: &mut HashMap<i32, usize>) -> i32 {
        let mut found = false;
        let mut closest: usize = 0;

        for h in self.height..10 {
            let e = heights.get(&h);
            if e.is_some() && e.unwrap() > &closest {
                closest = *e.unwrap();
                found = true;
            }
        }

        if !found {
            self.viewing.push(to_edge);
        } else {
            self.viewing.push(to_edge - closest);
        }

        heights.insert(self.height, to_edge);

        if height >= self.height {
            self.visibility -= 1;
            return height;
        }
        return self.height;
    }
}

struct TreeGrid {
    rows: usize,
    cols: usize,
    trees: Vec<Vec<Tree>>
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

        let mut row: Vec<Tree> = Vec::new();
        for c in line.chars() {
            row.push(Tree{
                height: c as i32 - '0' as i32,
                viewing: Vec::new(),
                visibility: 4
            });
        }

        self.rows += 1;
        self.trees.push(row);
    }

    fn get_tree(&mut self, i: usize, j: usize) -> &mut Tree {
        return self.trees
                .get_mut(i)
                .expect("row does not exist")
                .get_mut(j)
                .expect("column does not exist");
    }

    fn count_visible(&self) -> usize {
        let mut visible: usize = 0;

        for v in self.trees.iter() {
            for t in v.iter() {
                if t.visibility > 0 {
                    visible += 1;
                }
            }
        }
        return visible;
    }

    fn highest_scenic(&self) -> usize {
        let mut scenic: usize = 0;

        for v in self.trees.iter() {
            for t in v.iter() {
                if t.scenic_score() > scenic {
                    scenic = t.scenic_score();
                }
            }
        }
        return scenic;
    }

    fn mark_visible(&mut self) {
        let rows: usize = self.rows;
        let cols: usize = self.cols;

        for i in 0..rows {
            let mut height = -1;
            let mut heights: HashMap<i32, usize> = HashMap::new();
            for j in 0..cols {
                height = self.get_tree(i, j).maybe_drop_visibility(j, height, &mut heights);
            }

            let mut height = -1;
            let mut heights: HashMap<i32, usize> = HashMap::new();
            for j in (0..cols).rev() {
                height = self.get_tree(i, j).maybe_drop_visibility(cols - 1 - j, height, &mut heights);
            }
        }

        for j in 0..cols {
            let mut height = -1;
            let mut heights: HashMap<i32, usize> = HashMap::new();
            for i in 0..rows {
                height = self.get_tree(i, j).maybe_drop_visibility(i, height, &mut heights);
            }

            let mut height = -1;
            let mut heights: HashMap<i32, usize> = HashMap::new();
            for i in (0..rows).rev() {
                height = self.get_tree(i, j).maybe_drop_visibility(rows - 1 - i, height, &mut heights);
            }
        }
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

    grid.mark_visible();

    println!("{} Visible Trees", grid.count_visible());
    println!("Highest Scenic Score: {}", grid.highest_scenic());
}
