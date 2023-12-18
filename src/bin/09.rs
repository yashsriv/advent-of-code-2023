use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(9);

trait PushReturn<T> {
    fn push_return(self, t: T) -> Self;
}

impl<T> PushReturn<T> for Vec<T> {
    fn push_return(mut self, t: T) -> Self {
        self.push(t);
        self
    }
}

fn get_predictions(sequence: Vec<i64>) -> Option<(i64, i64)> {
    if sequence.iter().all(|val| *val == 0) {
        return Some((0, 0));
    }

    let (first_val, diff_sequence, last_val) =
        sequence
            .into_iter()
            .fold(
                (None, Vec::new(), None),
                |(first, new_seq, prev), value| match prev {
                    None => (Some(value), new_seq, Some(value)),
                    Some(prev_value) => {
                        (first, new_seq.push_return(value - prev_value), Some(value))
                    }
                },
            );

    let (next_first, next_last) = get_predictions(diff_sequence)?;
    Some((first_val? - next_first, last_val? + next_last))
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, sequences) = parse_input(input).ok()?;
    sequences
        .into_iter()
        .map(get_predictions)
        .map(|result| result.map(|(_, next)| next))
        .sum()
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, sequences) = parse_input(input).ok()?;
    sequences
        .into_iter()
        .map(get_predictions)
        .map(|result| result.map(|(prev, _)| prev))
        .sum()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    terminated(
        separated_list1(
            line_ending,
            separated_list1(
                space1,
                map_res(recognize(preceded(opt(tag("-")), digit1)), str::parse),
            ),
        ),
        line_ending,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_prediction() {
        let result = get_predictions(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result, Some((-3, 18)))
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = parse_input(&input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    vec![0, 3, 6, 9, 12, 15,],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45]
                ]
            ))
        )
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
