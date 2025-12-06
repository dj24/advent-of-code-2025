use std::collections::HashMap;
use std::fs;
use std::string::ToString;

fn get_trimmed_column(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(|s| s.trim().to_string())
        .collect()
}

fn rotate_2d_array<T: Clone>(array: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut rotated: Vec<Vec<T>> = Vec::new();
    let num_columns = array[0].len();
    let num_rows = array.len();

    for col_idx in 0..num_columns {
        let mut new_row: Vec<T> = Vec::new();
        for row_idx in 0..num_rows {
            new_row.push(array[row_idx][col_idx].clone());
        }
        rotated.push(new_row);
    }
    rotated
}

fn parse_row(row: Vec<String>) -> Vec<u64> {
    row.into_iter()
        .map(|s| {
            match s.parse::<u64>() {
                Ok(num) => Some(num),
                Err(_) => None, // Using 0 to represent non-numeric entries
            }
        })
        .filter_map(|x| x)
        .collect()
}

fn get_column_total(row: Vec<String>) -> u64 {
    let last_element = row.last().unwrap();
    let parsed_row = parse_row(row.clone());
    match last_element.as_str() {
        "*" => {
            parsed_row.iter().fold(1, |acc, element| acc * element)
        }
        "+" => {
            parsed_row.iter().sum()
        }
        _ => {
            panic!("Unexpected operator: {}", last_element);
        }
    }
}

fn part_1(contents: &String) -> u64 {
    let column_arrays: Vec<Vec<String>> = contents
        .lines()
        .map(get_trimmed_column)
        .collect();

    rotate_2d_array(column_arrays).iter().fold(0, |outer_acc, row| {
        let column_value = get_column_total(row.clone());
        outer_acc + column_value
    })
}

// Creates numbers by taking the nth character from each value
fn create_numbers_from_each_column(row: &Vec<String>) -> Vec<u64> {
    let char_map = row.iter().fold([vec![], vec![], vec![], vec![]], |acc: [Vec<char>; 4], s| {
        let chars: Vec<char> = s.chars().collect();
        let mut new_acc = acc.clone();
        for (i, c) in chars.iter().enumerate() {
            if c.is_whitespace() || *c == '*' || *c == '+' {
                continue;
            }
            new_acc[i].push(*c);
        }
        new_acc
    });

    // Convert the character vectors into numbers
    char_map.iter().filter_map(|char_vec| {
        let num_str: String = char_vec.iter().collect();
        if num_str.is_empty() {
            return None;
        }
        let parsed = num_str.parse::<u64>().expect(format!("Failed to parse number from string: {}", num_str).as_str());
        Some(parsed)
    }).collect()
}

fn get_column_total_part_2(column: Vec<String>) -> u64 {
    let last_element = column.last().unwrap();
    println!("Column: {:?}", column);
    let numbers = create_numbers_from_each_column(&column);
    println!("Numbers: {:?}", numbers);
    match last_element.as_str().trim() {
        "*" => {
            numbers.iter().fold(1, |acc, element| acc * element)
        }
        "+" => {
            numbers.iter().sum()
        }
        _ => {
            panic!("Unexpected operator: {} in column {:?}", last_element, column);
        }
    }
}

fn part_2(contents: &String) -> u64 {
    let rows: Vec<&str> = contents.lines().collect();

    let column_count = rows[0].len();
    let row_count = rows.len() - 1;

    let mut current_column_index = 0;
    let mut last_empty_column_index: i32 = -1;
    let mut total: u64 = 0;

    // Iterate each row in lockstep - when we find an empty column, we stop and go back to collect digits
    while current_column_index <= column_count {
        let is_empty_column = current_column_index == column_count || (0..row_count).all(|row_index| {
            let line = rows[row_index];
            let slice = line.chars().nth(current_column_index).unwrap();
            slice.is_whitespace()
        });

        if is_empty_column {
            let numbers = ((last_empty_column_index + 1) as usize..current_column_index).map(|i| {
                let column: String = rows.iter().map(|line| {
                    line.chars().nth(i).unwrap()
                }).filter(|c| !['*', '+', ' '].contains(c)).collect();
                column.parse::<u64>().unwrap()
            });

            let operator = rows[row_count].chars().nth((last_empty_column_index + 1) as usize).unwrap();

            let result = match operator {
                '*' => {
                    println!("Multiplying numbers: {:?}", numbers.clone().collect::<Vec<u64>>());
                    numbers.fold(1, |acc, element| acc * element)
                }
                '+' => {
                    println!("Adding numbers: {:?}", numbers.clone().collect::<Vec<u64>>());
                    numbers.sum()
                }
                _ => {
                    panic!("Unexpected operator: {}", operator);
                }
            };
            total += result;

            last_empty_column_index = current_column_index as i32;
        }

        current_column_index += 1;
    }
    total
}

// Gets each columns value, including whitespace, but with the trailing whitespace removed
fn slice_row(line: &str) -> Vec<String> {
   line.chars().collect::<Vec<char>>()
       .chunks(4)
       .map(|chunk| {
           [chunk[0], chunk[1], chunk[2]].iter().collect::<String>()
       })
       .collect::<Vec<String>>()
}

fn main() {
    match fs::read_to_string("./day-6/assets/input.txt") {
        Ok(contents) => {
            let total = part_1(&contents);
            println!("{}", total);
            let total_part_2 = part_2(&contents);
            println!("{}", total_part_2);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trimmed_column() {
        let line = "  apple   banana   cherry  ";
        let result = get_trimmed_column(line);
        assert_eq!(result, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_rotate_2d_array() {
        let array = vec![
            vec!["a1".to_string(), "a2".to_string(), "a3".to_string()],
            vec!["b1".to_string(), "b2".to_string(), "b3".to_string()],
            vec!["c1".to_string(), "c2".to_string(), "c3".to_string()],
        ];
        let rotated = rotate_2d_array(array);
        let expected = vec![
            vec!["a1".to_string(), "b1".to_string(), "c1".to_string()],
            vec!["a2".to_string(), "b2".to_string(), "c2".to_string()],
            vec!["a3".to_string(), "b3".to_string(), "c3".to_string()],
        ];
        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_parse_row() {
        let row = vec!["123".to_string(), "45".to_string(), "6".to_string(), "*".to_string()];
        let parsed = parse_row(row);
        assert_eq!(parsed, vec![123, 45, 6]);
    }

    #[test]
    fn test_get_row_total_add() {
        let row_add = vec!["1".to_string(), "2".to_string(), "3".to_string(), "+".to_string()];
        let total_add = get_column_total(row_add);
        assert_eq!(total_add, 6);
    }

    #[test]
    fn test_get_row_total_multiply() {
        let row_multiply = vec!["123".to_string(), "45".to_string(), "6".to_string(), "*".to_string()];
        let total_multiply = get_column_total(row_multiply);
        assert_eq!(total_multiply, 33210);
    }

    #[test]
    fn test_part_1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let result = part_1(&input.to_string());
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_slice_row_every_4() {
        let line = "123 328  51 64 ";
        let result = slice_row(line);
        assert_eq!(result, vec!["123", "328"," 51", "64 "]);
    }

    #[test]
    fn slice_row_works_with_smaller_values() {
        let line = "12 32 5  6  ";
        let result = slice_row(line);
        assert_eq!(result, vec!["12", "32"," 5 ", "6 "]);
    }

    #[test]
    fn test_create_numbers_from_each_column() {
        let row = vec!["64 ".to_string(), "23 ".to_string(), "314".to_string(), "*  ".to_string()];
        let numbers = create_numbers_from_each_column(&row);
        assert_eq!(numbers, [623, 431, 4]);
    }

    #[test]
    fn test_get_column_total_part_2_add() {
        let column_add = vec!["64 ".to_string(), "23 ".to_string(), "314".to_string(), "+  ".to_string()];
        let total_add = get_column_total_part_2(column_add);
        assert_eq!(total_add, 1058);
    }

    #[test]
    fn test_get_column_total_part_2_multiply() {
        let column_multiply = vec![" 51".to_string(), "387".to_string(), "215".to_string(), "*  ".to_string()];
        let total_multiply = get_column_total_part_2(column_multiply);
        assert_eq!(total_multiply, 3253600);
    }


    #[test]
    fn test_part_2() {
        let row_1 ="123 328  51 64 ".to_string();
        let row_2 =" 45 64  387 23 ".to_string();
        let row_3 ="  6 98  215 314".to_string();
        let row_4 ="*   +   *   +  ".to_string();
        let input = format!("{}\n{}\n{}\n{}", row_1, row_2, row_3, row_4);
        let result = part_2(&input.to_string());
        assert_eq!(result, 3263827);
    }
}
