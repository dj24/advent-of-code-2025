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
            ((Some('|'), _, _), (Some('^'), Some('.'), _)) => '|',
            // Split from the right
            ((_, _, Some('|')), (_, Some('.'), Some('^'))) => '|',
            // Increment split counter when we encounter split character
            ((_, Some('|'), _), (_, Some('^'), _)) => {
                split_indices.insert(i);
                '^'
            }
            // Dont change splitters
            ((_, _, _), (_, Some('^'), _)) => '^',
            _ => '.', // No change
        };
        result_line.push(new_char);
    }
    (split_indices.len() as u64, result_line)
}

fn part_1(input: &str) -> (u64, Vec<String>) {
    let result = input
        .lines()
        .enumerate()
        .fold((0, vec![]), |acc, (i, line)| {
            if i == 0 {
                return (0, vec![line.to_string()]);
            }
            let previous_line = acc.1.last().expect("No previous line");
            let (split_count, line_output) = process_line(previous_line, line);
            println!("Processing line");
            println!("Previous {}", previous_line);
            println!("Current  {}", line);
            println!("Output   {} -> {}", line_output, split_count);
            (acc.0 + split_count, [acc.1, vec![line_output]].concat())
        });
    result
}

// TODO: add up previous rows
fn process_line_part_2(
    previous_line: &str,
    previous_line_with_splits: &str,
    current_line: &str,
) -> String {
    current_line.chars().enumerate().fold(
        String::with_capacity(current_line.len()),
        |mut acc, (i, curr_char)| {
            let previous_left_char = if i > 0 {
                Some(previous_line.chars().nth(i - 1).unwrap())
            } else {
                None
            };
            let previous_right_char = if i <= previous_line.len() {
                Some(previous_line.chars().nth(i + 1).unwrap())
            } else {
                None
            };
            let previous_char = Some(previous_line.chars().nth(i).unwrap());
            let previous_char_with_splits = Some(previous_line_with_splits.chars().nth(i).unwrap());

            match (
                previous_char_with_splits,
                (previous_left_char, previous_char, previous_right_char),
            ) {
                // Previously connected to a splliter, so add up left and right
                ((Some('^'), (Some(a), _, Some(b)))) => {
                    let left_digit = if a.is_digit(10) {
                        a.to_digit(10).unwrap()
                    } else {
                        0
                    };
                    let right_digit = if b.is_digit(10) {
                        b.to_digit(10).unwrap()
                    } else {
                        0
                    };
                    (left_digit + right_digit).to_string()
                }

                _ => curr_char.to_string(),
            }
        },
    )
}

fn main() {
    match fs::read_to_string("./day-7/assets/input.txt") {
        Ok(contents) => {
            println!("Split count {}", part_1(&*contents).0)
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_line_works() {
        let previous_line = ".|.|||.||.||.|.";
        let current_line = ".^.^.^.^.^...^.";
        let expected = "|^|^|^|^|^|||^|";
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
    fn part_1_works() {
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
        let expected = ".......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
|.|.|.|.|.|||.|";
        assert_eq!(
            part_1(input),
            (21, expected.lines().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn process_line_part_2_works() {
        let previous_line = "1.1.1.1.1.111.1";
        let previous_line_with_splits = "|^|^|^|^|^|||^|";
        let current_line = ".|.|||.||.||.|.";
        let expected = ".2.212.21.11.2.";
        assert_eq!(
            process_line_part_2(previous_line, previous_line_with_splits, current_line),
            expected.to_string()
        );
    }
}
