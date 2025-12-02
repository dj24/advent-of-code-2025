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

fn get_times_passed_0(old_position: i32, movement: i32) -> i32 {
    assert!(old_position >= 0, "Position must be greater than zero");
    assert!(
        old_position < MAX_POSITION,
        "Position must be less than MAX_POSITION"
    );
    if movement > 0 {
        (old_position + movement) / MAX_POSITION
    } else {
        let flipped_old_position = (MAX_POSITION - old_position) % MAX_POSITION;
        (flipped_old_position - movement) / MAX_POSITION
    }
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

            let current_position = (acc.current_position + movement).rem_euclid(MAX_POSITION);

            let count_of_times_reached_0 =
                acc.count_of_times_reached_0 + if current_position == 0 { 1 } else { 0 };

            let count_of_times_passed_0 =
                acc.count_of_times_passed_0 + get_times_passed_0(acc.current_position, movement);

            Output {
                current_position,
                count_of_times_reached_0,
                count_of_times_passed_0,
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

#[cfg(test)]
mod tests {
    use crate::get_times_passed_0;

    #[test]
    fn computes_forward_wrap() {
        assert_eq!(get_times_passed_0(10, 250), 2);
    }

    #[test]
    fn computes_backward_wrap() {
        assert_eq!(get_times_passed_0(10, -250), 3);
    }

    #[test]
    fn computes_single_backward_wrap() {
        assert_eq!(get_times_passed_0(30, -50), 1);
    }

    #[test]
    fn computes_no_wrap() {
        assert_eq!(get_times_passed_0(10, 20), 0);
    }

    #[test]
    fn computes_exact_wrap() {
        assert_eq!(get_times_passed_0(10, 90), 1);
    }

    #[test]
    fn computes_exact_multiple_wrap() {
        assert_eq!(get_times_passed_0(10, 190), 2);
    }

    #[test]
    fn computes_exact_backward_wrap() {
        assert_eq!(get_times_passed_0(10, -10), 1);
    }

    #[test]
    fn computes_exact_backward_multiple_wrap() {
        assert_eq!(get_times_passed_0(10, -110), 2);
    }

    #[test]
    fn computes_forwards_from_zero() {
        assert_eq!(get_times_passed_0(0, 150), 1);
    }

    #[test]
    fn computes_backwards_from_zero() {
        assert_eq!(get_times_passed_0(0, -150), 1);
    }

    #[test]
    fn computes_massive_movement() {
        assert_eq!(get_times_passed_0(50, 1000), 10);
    }

    #[test]
    fn computes_massive_negative_movement() {
        assert_eq!(get_times_passed_0(50, -1000), 10);
    }

    #[test]
    fn computes_no_wrap_from_zero_backwards() {
        assert_eq!(get_times_passed_0(0, -50), 0);
    }

    #[test]
    fn computes_no_wrap_from_zero_forwards() {
        assert_eq!(get_times_passed_0(0, 50), 0);
    }
}
