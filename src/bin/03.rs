use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
enum Cell {
    Number(i32),
    Symbol(char),
    Empty,
}

fn parse_input(input: &str) -> (Vec<Vec<Cell>>, HashMap<i32, u32>) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut incremental_id = 0;
    let mut part_id_mappings = HashMap::new();
    let mut grid_with_cells = Vec::new();

    for row in &grid {
        let mut is_processing_part = false;
        let mut row_with_cells = Vec::new();
        for cell in row {
            let mapped_cell = match cell {
                '.' => Cell::Empty,
                _ if cell.is_ascii_digit() => Cell::Number(incremental_id),
                &x => Cell::Symbol(x),
            };
            row_with_cells.push(mapped_cell);

            if cell.is_ascii_digit() {
                let part_id = part_id_mappings.entry(incremental_id).or_insert(0);
                *part_id = *part_id * 10 + cell.to_digit(10).unwrap();
                is_processing_part = true;
            } else {
                if is_processing_part {
                    incremental_id += 1;
                }
                is_processing_part = false;
            }
        }
        if is_processing_part {
            incremental_id += 1;
        }
        grid_with_cells.push(row_with_cells);
    }

    (grid_with_cells, part_id_mappings)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid_with_cells, part_id_mappings) = parse_input(input);

    let mut part_id_is_valid = HashSet::new();
    for row in 0..grid_with_cells.len() {
        for col in 0..grid_with_cells[row].len() {
            let cell = &grid_with_cells[row][col];
            if let Cell::Symbol(_) = *cell {
            } else {
                continue;
            }

            // Check top-left
            if row > 0 && col > 0 {
                if let Cell::Number(n) = grid_with_cells[row - 1][col - 1] {
                    part_id_is_valid.insert(n);
                }
            }

            // Check top
            if row > 0 {
                if let Cell::Number(n) = grid_with_cells[row - 1][col] {
                    part_id_is_valid.insert(n);
                }
            }

            // Check top-right
            if row > 0 && col < (grid_with_cells[row - 1].len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row - 1][col + 1] {
                    part_id_is_valid.insert(n);
                }
            }

            // Check left
            if col > 0 {
                if let Cell::Number(n) = grid_with_cells[row][col - 1] {
                    part_id_is_valid.insert(n);
                }
            }

            // Check right
            if col < (grid_with_cells[row].len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row][col + 1] {
                    part_id_is_valid.insert(n);
                }
            }

            // Check bottom-left
            if row < (grid_with_cells.len() - 1) && col > 0 {
                if let Cell::Number(n) = grid_with_cells[row + 1][col - 1] {
                    part_id_is_valid.insert(n);
                }
            }

            // Check bottom
            if row < (grid_with_cells.len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row + 1][col] {
                    part_id_is_valid.insert(n);
                }
            }

            // Check bottom-right
            if row < (grid_with_cells.len() - 1) && col < (grid_with_cells[row + 1].len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row + 1][col + 1] {
                    part_id_is_valid.insert(n);
                }
            }
        }
    }

    Some(
        part_id_mappings
            .iter()
            .filter(|(k, _)| part_id_is_valid.contains(*k))
            .map(|(_, v)| v)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid_with_cells, part_id_mappings) = parse_input(input);

    let mut gear_ratio_sum = 0;
    for row in 0..grid_with_cells.len() {
        for col in 0..grid_with_cells[row].len() {
            let cell = &grid_with_cells[row][col];
            if let Cell::Symbol('*') = *cell {
            } else {
                continue;
            }

            let mut adjacent_part_ids = HashSet::new();

            // Check top-left
            if row > 0 && col > 0 {
                if let Cell::Number(n) = grid_with_cells[row - 1][col - 1] {
                    adjacent_part_ids.insert(n);
                }
            }

            // Check top
            if row > 0 {
                if let Cell::Number(n) = grid_with_cells[row - 1][col] {
                    adjacent_part_ids.insert(n);
                }
            }

            // Check top-right
            if row > 0 && col < (grid_with_cells[row - 1].len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row - 1][col + 1] {
                    adjacent_part_ids.insert(n);
                }
            }

            // Check left
            if col > 0 {
                if let Cell::Number(n) = grid_with_cells[row][col - 1] {
                    adjacent_part_ids.insert(n);
                }
            }

            // Check right
            if col < (grid_with_cells[row].len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row][col + 1] {
                    adjacent_part_ids.insert(n);
                }
            }

            // Check bottom-left
            if row < (grid_with_cells.len() - 1) && col > 0 {
                if let Cell::Number(n) = grid_with_cells[row + 1][col - 1] {
                    adjacent_part_ids.insert(n);
                }
            }

            // Check bottom
            if row < (grid_with_cells.len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row + 1][col] {
                    adjacent_part_ids.insert(n);
                }
            }

            // Check bottom-right
            if row < (grid_with_cells.len() - 1) && col < (grid_with_cells[row + 1].len() - 1) {
                if let Cell::Number(n) = grid_with_cells[row + 1][col + 1] {
                    adjacent_part_ids.insert(n);
                }
            }

            if adjacent_part_ids.len() == 2 {
                // This is a gear
                let gear_ratio: u32 = adjacent_part_ids
                    .iter()
                    .map(|&adjacent_part_id| part_id_mappings.get(&adjacent_part_id).unwrap())
                    .product();
                gear_ratio_sum += gear_ratio;
            }
        }
    }

    Some(gear_ratio_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
