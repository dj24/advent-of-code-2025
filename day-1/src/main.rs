use std::fs;

struct Output {
    current_position: i32,
    count_of_times_reached_0: i32,
    count_of_times_passed_0: i32,
}

impl Default for Output {
    fn default() -> Self {
        Output {
            current_position: 50,
            count_of_times_reached_0: 0,
            count_of_times_passed_0: 0,
        }
    }
}

const MAX_POSITION: i32 = 100;

fn get_number_from_chars(chars: &[char]) -> i32 {
    chars
        .iter()
        .collect::<String>()
        .parse()
        .expect("Failed to parse number")
}

fn fold_line(acc: Output, line: &str) -> Output {
    let chars: Vec<char> = line.chars().collect();
    match &chars[..] {
        [direction, num @ ..] => {
            let movement = match *direction {
                'L' => -get_number_from_chars(num),
                'R' => get_number_from_chars(num),
                _ => panic!("Unknown direction: {}", direction),
            };
            let total_movement = acc.current_position + movement;

            let new_value = total_movement % MAX_POSITION;

            Output {
                current_position: new_value,
                count_of_times_reached_0: acc.count_of_times_reached_0 + if new_value == 0 { 1 } else { 0 },
                count_of_times_passed_0: acc.count_of_times_passed_0,
            }
        }
        _ => acc,
    }
}

fn main() {
    match fs::read_to_string("./day-1/assets/input.txt") {
        Ok(contents) => {
            let output = contents.split("\n").fold(Default::default(), fold_line);
            println!(
                "Final Position: {}, Times Reached 0: {}, Times Passed 0: {}",
                output.current_position,
                output.count_of_times_reached_0,
                output.count_of_times_passed_0
            );
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
