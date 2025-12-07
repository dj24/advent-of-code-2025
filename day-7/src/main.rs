use std::collections::HashSet;
use std::{fs, thread};

// Continue beams, or split them if they encounter splitter
fn process_line(previous_line: &str, current_line: &str) -> (u64, String) {
    let line_length = previous_line.len();
    let mut result_line = String::with_capacity(line_length);
    // Use a set for storing the indices of used splits, as we don't want duplicates to be registered by cells either side
    let mut split_indices = HashSet::new();
    for i in 0..line_length {
        let prev_char = previous_line.chars().nth(i);
        let prev_left_adjacent_char = if i > 0 {
            previous_line.chars().nth(i - 1)
        } else {
            None
        };
        let prev_right_adjacent_char = previous_line.chars().nth(i + 1);

        let curr_char = current_line.chars().nth(i);
        let curr_left_adjacent_char = if i > 0 {
            current_line.chars().nth(i - 1)
        } else {
            None
        };
        let curr_right_adjacent_char = current_line.chars().nth(i + 1);

        let new_char = match (
            (prev_left_adjacent_char, prev_char, prev_right_adjacent_char),
            (curr_left_adjacent_char, curr_char, curr_right_adjacent_char),
        ) {
            ((_, Some('|' | 'S'), _), (_, Some('.'), _)) => '|', // Continue beam
            // Split from the left
            ((Some('|'), _, _), (Some('^'), Some('.'), _)) => {
                '|'
            }
            // Split from the right
            ((_, _, Some('|')), (_, Some('.'), Some('^'))) => {
                '|'
            },
            // Increment split counter when we encounter split character
            ((_, Some('|'), _), (_, Some('^'), _)) => {
                split_indices.insert(i);
                '^'
            }, 
            // Dont change splitters
            ((_, _, _), (_, Some('^'), _)) => {
                '^'
            },
            _ => '.',                              // No change
        };
        result_line.push(new_char);
    }
    (split_indices.len() as u64, result_line)
}

fn part_1(input: &str) -> u64 {
    let result = input.lines().enumerate().fold((0, vec![]),|acc, (i, line)| {
        if i == 0 {
            return (0,vec![line.to_string()]);
        }
        let previous_line = acc.1.last().expect("No previous line");
        let (split_count, line_output) = process_line(previous_line, line);
        println!("Processing line");
        println!("Previous {}", previous_line);
        println!("Current  {}", line);
        println!("Output   {} -> {}", line_output, split_count);
        (acc.0 + split_count, [acc.1, vec![line_output]].concat())
    });
    result.0
}

fn main() {
    match fs::read_to_string("./day-7/assets/input.txt") {
        Ok(contents) => {
            println!("Split count {}", part_1(&*contents))
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_line_works() {
        let previous_line = ".|.|||.||.||.|.";
        let current_line =  ".^.^.^.^.^...^.";
        let expected =      "|^|^|^|^|^|||^|";
        assert_eq!(
            process_line(previous_line, current_line),
            (5u64, expected.to_string())
        );
    }

    #[test]
    fn process_long_line_works() {
        let previous_line = "......................................................................|......................................................................";
        let current_line = "......................................................................^......................................................................";
        let expected_line = ".....................................................................|^|.....................................................................";
        assert_eq!(
            process_line(previous_line, current_line),
            (1u64, expected_line.to_string())
        );
    }

    #[test]
    fn part_1_works(){
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(
            part_1(input),
            21
        );
    }
}
