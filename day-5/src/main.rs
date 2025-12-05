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
}
