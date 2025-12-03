use std::fs;
use std::ops::RangeInclusive;

fn convert_line_into_range(line: &str) -> RangeInclusive<usize> {
    let numbers: Vec<_> = line.split("-").collect();
    let start: usize = numbers[0].parse().expect("Failed to parse start number");
    let end: usize = numbers[1].parse().expect("Failed to parse end number");
    start..=end
}

// Returns all divisors of x greater than 1, used to determine all possible group sizes for checking repeated patterns
fn get_valid_divisors(x: u32) -> Vec<u32> {
    let mut divisors = Vec::new();
    for i in 2..=x {
        if x % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn chunk_string(s: &str, chunk_size: usize) -> Vec<&str> {
    s.as_bytes()
        .chunks(chunk_size)
        .map(|chunk| std::str::from_utf8(chunk).expect("Invalid UTF-8"))
        .collect()
}

// Slices the string into chunks of pattern_size and checks if all chunks are identical
fn check_for_repeated_pattern_of_size(s: &str, pattern_size: usize) -> bool {
    let chunks = chunk_string(s, pattern_size);
    if chunks.len() < 2 {
        return false;
    }
    chunks[1..].iter().any(|chunk| *chunk == chunks[0])
}

fn main() {
    match fs::read_to_string("./day-2/assets/input.txt") {
        Ok(contents) => {
            let output = contents.split(",").for_each(|line| {
                println!("{:?}", line);
            });

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
    fn get_valid_divisors_works() {
        let divisors = super::get_valid_divisors(12);
        assert_eq!(divisors, vec![2, 3, 4, 6, 12]);
    }

    #[test]
    fn chunk_string_works() {
        let s = "123456";
        let chunks = super::chunk_string(s, 2);
        assert_eq!(chunks, vec!["12", "34", "56"]);
    }

    #[test]
    fn chunk_string_not_even() {
        let s = "12345";
        let chunks = super::chunk_string(s, 2);
        assert_eq!(chunks, vec!["12", "34", "5"]);
    }

    #[test]
    fn check_for_repeated_pattern_of_size_works() {
        let s = "ababab";
        assert!(super::check_for_repeated_pattern_of_size(s, 2));
    }

    #[test]
    fn check_for_repeated_pattern_of_size_fails() {
        let s = "abcabc";
        assert!(!super::check_for_repeated_pattern_of_size(s, 2));
    }
}
