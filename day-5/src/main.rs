use std::fs;
use std::ops::{Range, RangeInclusive};

fn convert_line_into_range(line: &str) -> RangeInclusive<u64> {
    let numbers: Vec<_> = line.split("-").collect();
    let start: u64 = numbers[0].parse().expect("Failed to parse start number");
    let end: u64 = numbers[1].parse().expect("Failed to parse end number");
    start..=end
}

fn split_into_ranges_and_ids(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let split: Vec<_> = input.split("\n\n").collect();
    (
        split[0]
            .lines()
            .map(convert_line_into_range)
            .collect(),
        split[1].lines()
            .map(|line| line.parse().expect(&*format!("Failed to parse id {}", line).to_string()))
            .collect(),
    )
}

fn is_id_fresh(ranges: Vec<RangeInclusive<u64>>, id: u64) -> bool {
    ranges.iter().find(|range| range.contains(&id)).is_some()
}

fn count_fresh_ids(ranges: Vec<RangeInclusive<u64>>, ids: Vec<u64>) -> u64 {
    ids.into_iter()
        .filter(|id| is_id_fresh(ranges.clone(), *id))
        .count() as u64
}

// Given an index in the ranges, update all other ranges to trim their start and end if they overlap with the range at the given index
fn trim_start_end_of_ranges(ranges: &Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    let mut output_ranges = ranges.clone();
    for (i,range_check) in ranges.iter().enumerate() {
        let (start, end) = {
            (*range_check.start(), *range_check.end())
        };

        let mut min_start = u64::MAX;
        let mut max_end = u64::MIN;

        for (j,range) in output_ranges.iter_mut().enumerate() {
            if i == j {
                continue;
            }
            // Trim start
           if range.start() < &min_start {
                min_start = *range.start();
           }
          if range.end() > &max_end {
             max_end = *range.end();
          }
        }

        let trimmed_start = if start <= min_start { min_start + 1 } else { start };
        let trimmed_end = if end >= max_end { max_end - 1 } else { end };

        output_ranges[i] = trimmed_start..=trimmed_end;

        println!("Trimming {} -> {} with range: {} -> {} = {} -> {}", start, end, min_start, max_end, trimmed_start, trimmed_end);
    }
    output_ranges
}

fn main() {
    match fs::read_to_string("./day-5/assets/input.txt") {
        Ok(contents) => {
            let (ranges, ids) = split_into_ranges_and_ids(&contents);
            println!("Number of fresh IDs: {}", count_fresh_ids(ranges, ids));
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn convert_line_works() {
        let line = "10-20";
        let range = super::convert_line_into_range(line);
        assert_eq!(range, 10..=20);
    }

    #[test]
    fn split_into_ranges_and_ids_works() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let (ranges, ids) = super::split_into_ranges_and_ids(input);
        assert_eq!(ranges, vec![3..=5, 10..=14, 16..=20, 12..=18]);
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn is_id_fresh_works() {
        let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];
        assert_eq!(super::is_id_fresh(ranges.clone(), 1), false);
        assert_eq!(super::is_id_fresh(ranges.clone(), 5), true);
        assert_eq!(super::is_id_fresh(ranges.clone(), 8), false);
        assert_eq!(super::is_id_fresh(ranges.clone(), 11), true);
        assert_eq!(super::is_id_fresh(ranges.clone(), 17), true);
        assert_eq!(super::is_id_fresh(ranges.clone(), 32), false);
    }

    #[test]
    fn count_fresh_ids_works() {
        let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let ids = vec![1, 5, 8, 11, 17, 32];
        let count = super::count_fresh_ids(ranges, ids);
        assert_eq!(count, 3);
    }


    #[test]
    fn trim_start_end_of_ranges_works() {
        let mut ranges = vec![3..=10, 8..=15, 12..=20];
        super::trim_start_end_of_ranges(&mut ranges);
        assert_eq!(ranges, vec![3..=10, 11..=15, 16..=20]);
    }
}
