use std::collections::HashSet;
use std::{fs, thread};

fn parse_coordinates(line: &str) -> (u64, u64) {
    let parts: Vec<&str> = line.split(',').collect();
    (
        parts[0].parse().expect("Character not a digit"),
        parts[1].parse().expect("Character not a digit"),
    )
}

fn part_1(coordinates: Vec<(u64, u64)>) -> (u64, (u64, u64), (u64, u64)) {
    coordinates.iter().enumerate().fold(
        (0, (0u64, 0u64), (0u64, 0u64)),
        |acc, (i, check_coordinates)| {
            let max_from_this_coordinate = coordinates.iter().enumerate().fold(
                (0, check_coordinates),
                |acc, (j, coordinates)| {
                    let dx = coordinates.0.abs_diff(check_coordinates.0);
                    let dy = coordinates.1.abs_diff(check_coordinates.1);
                    let rect_area = (dx+1) * (dy+1);
                    if rect_area > acc.0 {
                        (rect_area, coordinates)
                    } else {
                        acc
                    }
                },
            );

            if max_from_this_coordinate.0 > acc.0 {
                (
                    max_from_this_coordinate.0,
                    *check_coordinates,
                    *max_from_this_coordinate.1,
                )
            } else {
                acc
            }
        },
    )
}

fn main() {
    match fs::read_to_string("./day-9//assets/input.txt") {
        Ok(contents) => {
            let coordinates = contents
                .lines()
                .map(|line| parse_coordinates(line))
                .collect::<Vec<(u64, u64)>>();

            let result = part_1(coordinates);

            println!(
                "Largest area of {} is between coordinates {},{} and {},{}",
                result.0, result.1 .0, result.1 .1, result.2 .0, result.2 .1
            );
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coordinates_works() {
        assert_eq!(parse_coordinates("123,456"), (123, 456));
    }

    #[test]
    fn part_1_works() {
        let input = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        assert_eq!(part_1(input.clone()), (50, (11,1), (2, 5)));
    }
}
