struct CleaningAssignmentGroup {
    cleaners: Vec<CleaningAssignment>
}

impl CleaningAssignmentGroup {
    fn has_full_overlap(&self) -> bool {
        for (i, a1) in self.cleaners.iter().enumerate() {
            for (j, a2) in self.cleaners.iter().enumerate() {
                if i == j {
                    continue;
                }

                if a1.start <= a2.start && a1.end >= a2.end {
                    return true;
                }
            }
        }
        return false;
    }

    fn has_any_overlap(&self) -> bool {
        for (i, a1) in self.cleaners.iter().enumerate() {
            for (j, a2) in self.cleaners.iter().enumerate() {
                if i == j {
                    continue;
                }

                if (a1.start <= a2.start && a1.end >= a2.start) || (a1.start <= a2.end && a1.end >= a2.end) {
                    return true;
                }
            }
        }
        return false;
    }
}

struct CleaningAssignment {
    start: u32,
    end: u32
}

fn main() {
    let mut groups: Vec<CleaningAssignmentGroup> = Vec::new();

    fn reducer(line: &str, accumulator: &mut Vec<CleaningAssignmentGroup>) {
        if line.is_empty() {
            return;
        }

        let groups = line.split(",");
        let mut assignments: Vec<CleaningAssignment> = Vec::new();

        for group in groups {
            let bounds: Vec<&str> = group.split("-").collect();

            if bounds.len() != 2 {
                panic!("Section bounds {} has unexpected length {}", group, bounds.len());
            }

            let start = bounds[0].parse::<u32>()
                .expect("Section start is not an integer");
            let end = bounds[1].parse::<u32>()
                .expect("Section end is not an integer.");

            assignments.push(CleaningAssignment{
                start: start,
                end: end
            })
        }

        accumulator.push(CleaningAssignmentGroup{cleaners: assignments});
    }

    if !aoc::reduce_input(4, &mut groups, reducer) {
        panic!("Failed to read input.");
    }

    let mut total_full = 0;
    let mut total_partial = 0;

    for group in groups.iter() {
        if group.has_full_overlap() {
            total_full += 1;
        }
        if group.has_any_overlap() {
            total_partial += 1;
        }
    }

    println!("{} groups have a full overlap.", total_full);
    println!("{} groups have a partial overlap.", total_partial);

}
