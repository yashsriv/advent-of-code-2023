advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = c.to_digit(10);
                }
                last = c.to_digit(10);
            }
        }
        match (first, last) {
            (Some(first), Some(last)) => {
                sum += first * 10 + last;
            }
            _ => return None,
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;
        let patterns = vec![
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        for (pattern, val) in patterns {
            let mut iter = line.match_indices(pattern);
            let first_match = iter.next();
            let last_match = iter.last();
            let last_match = if last_match.is_none() {
                first_match
            } else {
                last_match
            };
            first = match (first_match, first) {
                (Some((index, _)), None) => Some((index, val)),
                (Some((index, _)), Some((existing_index, _))) if index < existing_index => {
                    Some((index, val))
                }
                _ => first,
            };
            last = match (last_match, last) {
                (Some((index, _)), None) => Some((index, val)),
                (Some((index, _)), Some((existing_index, _))) if index > existing_index => {
                    Some((index, val))
                }
                _ => last,
            };
        }
        match (first, last) {
            (Some((_, first_val)), Some((_, last_val))) => {
                sum += first_val * 10 + last_val;
            }
            _ => return None,
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(141));
    }
}
