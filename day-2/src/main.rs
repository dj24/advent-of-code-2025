use std::fs;
use std::ops::RangeInclusive;

fn convert_line_into_range(line: &str) -> RangeInclusive<u64> {
    let numbers: Vec<_> = line.split("-").collect();
    let start: u64 = numbers[0].parse().expect("Failed to parse start number");
    let end: u64 = numbers[1].parse().expect("Failed to parse end number");
    start..=end
}

fn is_invalid_id(s: &str) -> bool {
    let (a, b) = s.split_at(s.len() / 2);
    a == b
}

fn find_invalid_ids_in_range(range: RangeInclusive<u64>) -> Vec<u64> {
    range.fold(vec![], |acc, id| {
        if is_invalid_id(&*id.to_string()) {
            [acc, vec![id]].concat()
        } else {
            acc
        }
    })
}

fn sum_total_invalid_ids_in_input(input: String) -> u64 {
    input.split(",").fold(0, |acc, line| {
        let invalid_ids = find_invalid_ids_in_range(convert_line_into_range(line));
        acc + invalid_ids.iter().sum::<u64>()
    })
}

fn main() {
    match fs::read_to_string("./day-2/assets/input.txt") {
        Ok(contents) => {
            println!("{}", sum_total_invalid_ids_in_input(contents));
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
    fn is_invalid_id_works() {
        assert!(super::is_invalid_id("abcabc"));
    }

    #[test]
    fn is_invalid_id_fails() {
        assert!(!super::is_invalid_id("abcab"));
    }

    #[test]
    fn count_invalid_ids_in_range_works() {
        let range = 11..=22;
        let invalid_ids = super::find_invalid_ids_in_range(range);
        assert_eq!(invalid_ids, vec![11, 22]);

        let range = 1188511880..=1188511890;
        let invalid_ids = super::find_invalid_ids_in_range(range);
        assert_eq!(invalid_ids, vec![1188511885]);
    }

    #[test]
    fn sum_total_invalid_ids_in_input_works() {
        let input = String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let total = super::sum_total_invalid_ids_in_input(input);
        assert_eq!(total, 1227775554);
    }
}
