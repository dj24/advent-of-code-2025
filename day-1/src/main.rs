use std::fs;

struct Output {
    current_position: i32,
    count_of_times_reached_0: i32,
    count_of_times_passed_0: i32,
}

const STARTING_VALUE: Output = Output {
    current_position: 50,
    count_of_times_reached_0: 0,
    count_of_times_passed_0: 0,
};

const MAX_POSITION: i32 = 100;

fn get_movement_value(chars : &[char]) -> i32 {
    let movement: String = chars.iter().collect();
    movement.parse().unwrap_or(0)
}

fn main() {
    match fs::read_to_string("./day-1/assets/input.txt") {
        Ok(contents) => {
            let output = contents.split("\n").fold(STARTING_VALUE, |acc, line| {
                let chars: Vec<char> = line.chars().collect();
                return match &chars[..] {
                    [direction, num @ ..] => {
                        let movement = if *direction == 'L' {
                            -get_movement_value(num)
                        } else {
                            get_movement_value(num)
                        };
                        let total_movement = acc.current_position + movement;
                        let times_passed_0 = total_movement / MAX_POSITION;
                        println!(
                            "Direction: {}, Movement: {}, New Position: {}, Times Passed 0: {}",
                            direction, movement, total_movement, times_passed_0
                        );
                        let new_value = total_movement % MAX_POSITION;
                        Output {
                            current_position: new_value,
                            count_of_times_reached_0: acc.count_of_times_reached_0 + if new_value == 0 { 1 } else { 0 },
                            count_of_times_passed_0: acc.count_of_times_passed_0 + times_passed_0,
                        }
                    }
                    _ => {
                        println!("No movement");
                        acc
                    }
                };
            });
            println!(
                "Final Position: {}, Times Reached 0: {}, Times Passed 0: {}",
                output.current_position, output.count_of_times_reached_0, output.count_of_times_passed_0
            );
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}