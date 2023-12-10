advent_of_code::solution!(2);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let input_re = Regex::new(r#"Game (\d+): (.*)"#).unwrap();
    let result_re = Regex::new(r#"(\d+) (blue|green|red)"#).unwrap();

    let blue_count = 14;
    let green_count = 13;
    let red_count = 12;
    let mut sum = 0;

    for line in input.lines() {
        let captures = input_re.captures(line).map(|captures| {
            captures
                .iter() // All the captured groups
                .skip(1) // Skipping the complete match
                .flatten() // Ignoring all empty optional matches
                .map(|c| c.as_str()) // Grab the original strings
                .collect::<Vec<_>>() // Create a vector
        })?;

        let game_num = captures[0].parse::<u32>().ok()?;
        let results_str = captures[1];

        let mut is_valid = true;
        for result in results_str.split(';') {
            for individual_result in result.split(',') {
                let captures = result_re.captures(individual_result).map(|captures| {
                    captures
                        .iter() // All the captured groups
                        .skip(1) // Skipping the complete match
                        .flatten() // Ignoring all empty optional matches
                        .map(|c| c.as_str()) // Grab the original strings
                        .collect::<Vec<_>>() // Create a vector
                })?;

                let count = captures[0].parse::<u32>().ok()?;
                let color = captures[1];

                match (color, count) {
                    ("blue", x) if x > blue_count => {
                        is_valid = false;
                    }
                    ("green", x) if x > green_count => {
                        is_valid = false;
                    }
                    ("red", x) if x > red_count => {
                        is_valid = false;
                    }
                    (_, _) => {}
                }
            }
        }
        if is_valid {
            sum += game_num;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_re = Regex::new(r#"Game (\d+): (.*)"#).unwrap();
    let result_re = Regex::new(r#"(\d+) (blue|green|red)"#).unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let captures = input_re.captures(line).map(|captures| {
            captures
                .iter() // All the captured groups
                .skip(1) // Skipping the complete match
                .flatten() // Ignoring all empty optional matches
                .map(|c| c.as_str()) // Grab the original strings
                .collect::<Vec<_>>() // Create a vector
        })?;

        let results_str = captures[1];

        let mut min_blue_count = 0;
        let mut min_green_count = 0;
        let mut min_red_count = 0;

        for result in results_str.split(';') {
            for individual_result in result.split(',') {
                let captures = result_re.captures(individual_result).map(|captures| {
                    captures
                        .iter() // All the captured groups
                        .skip(1) // Skipping the complete match
                        .flatten() // Ignoring all empty optional matches
                        .map(|c| c.as_str()) // Grab the original strings
                        .collect::<Vec<_>>() // Create a vector
                })?;

                let count = captures[0].parse::<u32>().ok()?;
                let color = captures[1];

                match (color, count) {
                    ("blue", x) if x > min_blue_count => {
                        min_blue_count = x;
                    }
                    ("green", x) if x > min_green_count => {
                        min_green_count = x;
                    }
                    ("red", x) if x > min_red_count => {
                        min_red_count = x;
                    }
                    (_, _) => {}
                }
            }
        }
        sum += min_blue_count * min_green_count * min_red_count;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
