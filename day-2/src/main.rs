use std::fs;

fn split_line_into_numbers(line: &str) -> [&str; 2] {
    let vec: Vec<_> = line.split("-")
        .map(|num_str| num_str.trim())
        .collect();
    assert!(vec.len() == 2);
    [vec[0], vec[1]]
}

fn concat_id_pair(all_ids: Vec<&str>, id_pair: [&str; 2]) -> Vec<String> {
    let mut result = Vec::new();
    for id in all_ids {
        result.push(String::from(id));
    }
    result.push(String::from(id_pair[0]));
    result.push(String::from(id_pair[1]));
    result
}

struct IdSegments {
    extracted: String,
    remaining_concatenated: String,
}

fn splice_segment_from_id(id: String, size: usize, pos: usize) -> IdSegments {
    let extracted_slice_end = pos + size;
    assert!(id.len() >= extracted_slice_end);
    let extracted_slice = &id[pos..extracted_slice_end];
    let remaining_concatenated = format!(
        "{}{}",
        &id[..pos],
        &id[extracted_slice_end..]
    );
    
    IdSegments {
        extracted: String::from(extracted_slice),
        remaining_concatenated,
    }
}

fn id_contains_segment(id: String, segment: String) -> bool {
    id.contains(&segment)
}

fn find_repeating_range_in_string(s: String) -> Option<(usize, usize)> {
    // TODO:
    None
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
    fn splits_line_works() {
        let line = "10 - 20";
        let numbers = super::split_line_into_numbers(line);
        assert_eq!(numbers, ["10", "20"]);
    }

    #[test]
    fn concat_id_pair_works() {
        let all_ids = vec!["1", "2", "3"];
        let id_pair = ["4", "5"];
        let result = super::concat_id_pair(all_ids, id_pair);
        assert_eq!(result, vec!["1", "2", "3", "4", "5"]);
    }
    
    #[test]
    fn splice_id_works() {
        let id = String::from("123456");
        let spliced = super::splice_segment_from_id(id, 2, 2);
        assert_eq!(spliced.extracted, "34");
        assert_eq!(spliced.remaining_concatenated, "1256");
    }
    
    #[test]
    fn id_contains_segment_works() {
        let id = "abcdefg".to_string();
        let segment = "cde".to_string();
        assert!(super::id_contains_segment(id, segment));
    }
}
