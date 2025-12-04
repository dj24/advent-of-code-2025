use std::fs;

fn load_lines_into_grid(contents: String) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn count_at_char_in_adjacent_positions(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
) -> u32 {
    let grid_width = grid[0].len() as isize;
    let grid_height = grid.len() as isize;
    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (dr, dc) in directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if new_row >= 0
            && new_row < grid_height
            && new_col >= 0
            && new_col < grid_width
        {
        if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

fn count_valid_rolls_in_grid(grid: &Vec<Vec<char>>) -> u32 {
    let mut valid_positions = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '@' {
                continue;
            }
            let adjacent_at_count = count_at_char_in_adjacent_positions(&grid, row, col);
            if adjacent_at_count < 4 {
                valid_positions += 1;
            }
        }
    }
    valid_positions
}

fn main() {
    match fs::read_to_string("./day-4/assets/input.txt") {
        Ok(contents) => {
            let grid = load_lines_into_grid(contents);
            let valid_positions = count_valid_rolls_in_grid(&grid);
            println!("{}", valid_positions);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn load_lines_into_grid_works() {
        let input = "abc\ndef\nghi";
        let grid = super::load_lines_into_grid(input.to_string());
        assert_eq!(grid, vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
    }

    #[test]
    fn count_at_char_in_adjacent_positions_works() {
        let grid = vec![
            vec!['@', 'a', '@'],
            vec!['b', '@', 'c'],
            vec!['@', 'd', '@'],
        ];
        let count = super::count_at_char_in_adjacent_positions(&grid, 1, 1);
        assert_eq!(count, 4);
    }

    #[test]
    fn count_at_char_in_adjacent_positions_works_at_edge() {
        let grid = vec![
            vec!['@', 'a', '@'],
            vec!['b', '@', 'c'],
            vec!['@', 'd', '@'],
        ];
        let count = super::count_at_char_in_adjacent_positions(&grid, 0, 0);
        assert_eq!(count, 1);
    }

    #[test]
    fn count_valid_rolses_in_grid_works() {
        let str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = super::load_lines_into_grid(str.to_string());
        let count = super::count_valid_rolls_in_grid(&grid);
        assert_eq!(count, 13);
    }
}
