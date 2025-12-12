use std::{fs};

#[derive(Clone, Copy)]
struct PresentShape([[bool; 3]; 3]);

impl PresentShape {
    /// Parses a string in the format:
    /// ..#
    /// .##
    /// ###
    fn new(values: &str) -> Self {
        let lines = values.lines();
        let mut shape = [[false; 3]; 3];
        for (i, line) in lines.enumerate() {
            for (j, ch) in line.chars().enumerate() {
                shape[i][j] = ch == '#';
            }
        }
        PresentShape(shape)
    }
}

impl Default for PresentShape {
    fn default() -> Self {
        PresentShape([[false; 3]; 3])
    }
}

#[derive(Clone, Copy)]
struct PresentRegion {
    width: usize,
    height: usize,
    presents_of_shape_0: usize,
    presents_of_shape_1: usize,
    presents_of_shape_2: usize,
    presents_of_shape_3: usize,
    presents_of_shape_4: usize,
    presents_of_shape_5: usize,
}

impl PresentRegion {
    /// Parses a string in the format "WxH: a b c d e f"
    fn new(values: &str) -> Self {
        let parts = values.split(':').collect::<Vec<&str>>();

        let present_counts: Vec<_> = parts[1].split(" ").filter(|c| *c != "").collect();

        let width_and_height: Vec<_> = parts[0].split('x').collect();

        PresentRegion {
            width: width_and_height[0].parse().expect("Failed to parse width"),
            height: width_and_height[1].parse().expect("Failed to parse height"),
            presents_of_shape_0: present_counts[0].parse().expect("Failed to parse present count 0"),
            presents_of_shape_1: present_counts[1].parse().expect("Failed to parse present count 1"),
            presents_of_shape_2: present_counts[2].parse().expect("Failed to parse present count 2"),
            presents_of_shape_3: present_counts[3].parse().expect("Failed to parse present count 3"),
            presents_of_shape_4: present_counts[4].parse().expect("Failed to parse present count 4"),
            presents_of_shape_5: present_counts[5].parse().expect("Failed to parse present count 5"),
        }
    }
}

impl Default for PresentRegion {
    fn default() -> Self {
        PresentRegion {
            width: 0,
            height: 0,
            presents_of_shape_0: 0,
            presents_of_shape_1: 0,
            presents_of_shape_2: 0,
            presents_of_shape_3: 0,
            presents_of_shape_4: 0,
            presents_of_shape_5: 0,
        }
    }
}

fn starts_with_digit_colon(s: &str) -> bool {
    let mut it = s.chars();
    match (it.next(), it.next()) {
        (Some(c), Some(':')) => c.is_ascii_digit(),
        _ => false,
    }
}

fn parse_input(input: &str) -> ([PresentShape; 5], Vec<PresentRegion>) {
    let mut shapes = [PresentShape::default(); 5];
    let mut regions = Vec::new();
    
    // TODO

    (shapes, regions)
}

fn main() {
    match fs::read_to_string("./day-12/assets/input.txt") {
        Ok(contents) => {}
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_shape_new() {
        let input = "..#
.##
###";
        let shape = PresentShape::new(input);
        assert_eq!(
            shape.0,
            [
                [false, false, true],
                [false, true, true],
                [true, true, true]
            ]
        );
    }

    #[test]
    fn test_present_region_new() {
        let input = "36x24: 1 2 3 4 5 6";
        let region = PresentRegion::new(input);
        assert_eq!(region.width, 36);
        assert_eq!(region.height, 24);
        assert_eq!(region.presents_of_shape_0, 1);
        assert_eq!(region.presents_of_shape_1, 2);
        assert_eq!(region.presents_of_shape_2, 3);
        assert_eq!(region.presents_of_shape_3, 4);
        assert_eq!(region.presents_of_shape_4, 5);
        assert_eq!(region.presents_of_shape_5, 6);
    }

    #[test]
    fn test_parse_input() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let (shapes, regions) = parse_input(input);
        assert_eq!(shapes[0].0, [[true, true, true], [true, true, false], [true, true, false]]);
        assert_eq!(shapes[1].0, [[true, true, true], [true, true, false], [false, true, true]]);
        assert_eq!(shapes[2].0, [[false, true, true], [true, true, true], [true, true, false]]);
        assert_eq!(shapes[3].0, [[true, true, false], [true, true, true], [true, true, false]]);
        assert_eq!(shapes[4].0, [[true, true, true], [true, false, false], [true, true, true]]);
        assert_eq!(regions.len(), 3);
        assert_eq!(regions[0].width, 4);
        assert_eq!(regions[0].height, 4);
        assert_eq!(regions[0].presents_of_shape_4, 2);
        assert_eq!(regions[1].width, 12);
        assert_eq!(regions[1].height, 5);
        assert_eq!(regions[1].presents_of_shape_0, 1);
        assert_eq!(regions[1].presents_of_shape_2, 2);
        assert_eq!(regions[2].width, 12);
        assert_eq!(regions[2].height, 5);
        assert_eq!(regions[2].presents_of_shape_0, 1);
        assert_eq!(regions[2].presents_of_shape_2, 3);
    }
}
