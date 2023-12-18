use std::collections::HashSet;

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(10);

fn get_next(
    (i, j): (usize, usize),
    grid: &Vec<Vec<TileType>>,
    visited: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    match &grid[i][j] {
        TileType::NorthSouth | TileType::NorthEast | TileType::NorthWest
            if i > 0 && !visited.contains(&(i - 1, j)) =>
        {
            Some((i - 1, j))
        }

        TileType::NorthSouth | TileType::SouthEast | TileType::SouthWest
            if i < grid.len() - 1 && !visited.contains(&(i + 1, j)) =>
        {
            Some((i + 1, j))
        }

        TileType::EastWest | TileType::NorthWest | TileType::SouthWest
            if j > 0 && !visited.contains(&(i, j - 1)) =>
        {
            Some((i, j - 1))
        }

        TileType::EastWest | TileType::NorthEast | TileType::SouthEast
            if j < grid[i].len() - 1 && !visited.contains(&(i, j + 1)) =>
        {
            Some((i, j + 1))
        }

        TileType::Start
            if i > 0
                && matches!(
                    grid[i - 1][j],
                    TileType::NorthSouth | TileType::SouthEast | TileType::SouthWest
                ) =>
        {
            Some((i - 1, j))
        }
        TileType::Start
            if j > 0
                && matches!(
                    grid[i][j - 1],
                    TileType::EastWest | TileType::NorthEast | TileType::SouthEast
                ) =>
        {
            Some((i, j - 1))
        }
        TileType::Start
            if i < grid.len() - 1
                && matches!(
                    grid[i + 1][j],
                    TileType::NorthSouth | TileType::NorthEast | TileType::NorthWest
                ) =>
        {
            Some((i + 1, j))
        }
        TileType::Start
            if j < grid[i].len() - 1
                && matches!(
                    grid[i][j + 1],
                    TileType::EastWest | TileType::NorthWest | TileType::SouthWest
                ) =>
        {
            Some((i, j + 1))
        }
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_input(input).ok()?;
    let (start_i, start_j) = find_start(&grid)?;

    let mut loop_length = 0;

    let mut visited = HashSet::new();
    visited.insert((start_i, start_j));

    let mut result = get_next((start_i, start_j), &grid, &visited);

    while let Some((i, j)) = result {
        visited.insert((i, j));
        result = get_next((i, j), &grid, &visited);
        loop_length += 1;
    }

    Some((loop_length + 1) / 2)
}

fn get_tile_type_without_start(grid: &[Vec<TileType>], (row, column): (usize, usize)) -> &TileType {
    if !matches!(grid[row][column], TileType::Start) {
        &grid[row][column]
    } else {
        let has_north_connection = row > 0
            && matches!(
                grid[row - 1][column],
                TileType::NorthSouth | TileType::SouthEast | TileType::SouthWest
            );
        let has_south_connection = row < grid.len() - 1
            && matches!(
                grid[row + 1][column],
                TileType::NorthSouth | TileType::NorthEast | TileType::NorthWest
            );
        let has_west_connection = column > 0
            && matches!(
                grid[row][column - 1],
                TileType::EastWest | TileType::NorthEast | TileType::SouthEast
            );
        let has_east_connection = column < grid[row].len() - 1
            && matches!(
                grid[row][column + 1],
                TileType::EastWest | TileType::NorthWest | TileType::SouthWest
            );

        match (
            has_north_connection,
            has_south_connection,
            has_east_connection,
            has_west_connection,
        ) {
            (true, true, false, false) => &TileType::NorthSouth,
            (false, false, true, true) => &TileType::EastWest,
            (true, false, true, false) => &TileType::NorthEast,
            (true, false, false, true) => &TileType::NorthWest,
            (false, true, true, false) => &TileType::SouthEast,
            (false, true, false, true) => &TileType::SouthWest,
            _ => panic!("Unexpected scenario"),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, grid) = parse_input(input).ok()?;
    let (start_i, start_j) = find_start(&grid)?;

    let mut visited = HashSet::new();
    visited.insert((start_i, start_j));

    let mut result = get_next((start_i, start_j), &grid, &visited);
    while let Some((i, j)) = result {
        visited.insert((i, j));
        result = get_next((i, j), &grid, &visited);
    }

    let mut area = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let is_visited = visited.contains(&(i, j));
            if is_visited {
                continue;
            }

            let mut is_inside = false;
            let mut location = (i, j);
            while location.0 < grid.len() && location.1 < grid[location.0].len() {
                let tile_type = get_tile_type_without_start(&grid, location);
                if visited.contains(&location)
                    && !matches!(tile_type, TileType::NorthEast | TileType::SouthWest)
                {
                    is_inside = !is_inside;
                }
                location = (location.0 + 1, location.1 + 1);
            }

            if is_inside {
                area += 1;
            }
        }
    }
    Some(area)
}

#[derive(Debug)]
enum TileType {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

fn find_start(grid: &[Vec<TileType>]) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if let TileType::Start = tile {
                return Some((i, j));
            }
        }
    }
    None
}

fn parse_tile(input: &str) -> IResult<&str, TileType> {
    alt((
        map(char('|'), |_| TileType::NorthSouth),
        map(char('-'), |_| TileType::EastWest),
        map(char('L'), |_| TileType::NorthEast),
        map(char('J'), |_| TileType::NorthWest),
        map(char('7'), |_| TileType::SouthWest),
        map(char('F'), |_| TileType::SouthEast),
        map(char('.'), |_| TileType::Ground),
        map(char('S'), |_| TileType::Start),
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<TileType>>> {
    terminated(separated_list1(line_ending, many1(parse_tile)), line_ending)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse_input(&input);
        assert!(result.is_ok());
        let (remaining_input, _) = result.unwrap();
        assert_eq!(remaining_input, "")
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
