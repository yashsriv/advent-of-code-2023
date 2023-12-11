use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input_re = Regex::new(r#"^Card\s+(\d+): ((\s|\d)+) \| ((\s|\d)+)$"#).unwrap();

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

        let _card_num = captures[0].parse::<usize>().ok()?;

        let winning_str = captures[1];
        let hand_str = captures[3];

        let winning_nums = winning_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
        let hand_nums = hand_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();

        let winning_count = winning_nums.intersection(&hand_nums).count();
        if winning_count > 0 {
            sum += 1 << (winning_count - 1);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_re = Regex::new(r#"^Card\s+(\d+): ((\s|\d)+) \| ((\s|\d)+)$"#).unwrap();

    let mut card_counts: HashMap<usize, usize> = HashMap::new();
    for line in input.lines() {
        let captures = input_re.captures(line).map(|captures| {
            captures
                .iter() // All the captured groups
                .skip(1) // Skipping the complete match
                .flatten() // Ignoring all empty optional matches
                .map(|c| c.as_str()) // Grab the original strings
                .collect::<Vec<_>>() // Create a vector
        })?;

        let card_num = captures[0].parse::<usize>().ok()?;
        card_counts.entry(card_num).or_insert(1);
        let card_count = card_counts.get(&card_num).copied().unwrap();

        let winning_str = captures[1];
        let hand_str = captures[3];

        let winning_nums = winning_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
        let hand_nums = hand_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();

        let winning_count = winning_nums.intersection(&hand_nums).count();
        for index in 1..(winning_count + 1) {
            let existing_card_count = card_counts.entry(card_num + index).or_insert(1);
            *existing_card_count += card_count;
        }
    }
    Some(card_counts.values().map(|v| *v as u32).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
