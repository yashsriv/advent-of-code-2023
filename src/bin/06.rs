use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, races) = parse_input(input).ok()?;
    let mut product = 1;
    for race in races {
        let discriminant = ((race.time * race.time - 4 * race.distance) as f32).sqrt();
        let min_time = (race.time as f32 - discriminant) / 2_f32;
        let max_time = (race.time as f32 + discriminant) / 2_f32;
        let min_time_rounded = min_time.ceil() as u32;
        let max_time_rounded = max_time.floor() as u32;

        let min_included = min_time == min_time.ceil();
        let max_included = max_time == max_time.floor();
        let mut win_count = max_time_rounded - min_time_rounded + 1;
        if min_included {
            win_count -= 1;
        }
        if max_included {
            win_count -= 1;
        }
        product *= win_count;
    }
    Some(product)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, races) = parse_input(input).ok()?;

    let mut actual_time = 0_f64;
    let mut actual_distance = 0_f64;
    for race in races {
        let multiply_by_time = (race.time as f64).log10().ceil();
        let multiply_by_distance = (race.distance as f64).log10().ceil();
        actual_time = actual_time * 10_f64.powf(multiply_by_time) + race.time as f64;
        actual_distance =
            actual_distance * 10_f64.powf(multiply_by_distance) + race.distance as f64;
    }
    let discriminant = (actual_time * actual_time - 4_f64 * actual_distance).sqrt();
    let min_time = (actual_time - discriminant) / 2_f64;
    let max_time = (actual_time + discriminant) / 2_f64;
    let min_time_rounded = min_time.ceil() as u32;
    let max_time_rounded = max_time.floor() as u32;
    let win_count = max_time_rounded - min_time_rounded + 1;
    Some(win_count)
}

#[derive(Debug, PartialEq)]
struct RaceResult {
    time: u32,
    distance: u32,
}

fn parse_input(input: &str) -> IResult<&str, Vec<RaceResult>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, parse_number)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, parse_number)(input)?;
    let (input, _) = line_ending(input)?;

    let mut result = Vec::new();
    for i in 0..times.len() {
        result.push(RaceResult {
            time: times[i],
            distance: distances[i],
        });
    }

    Ok((input, result))
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |x: &str| x.parse::<u32>())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let (input, _) = parse_input(&binding).unwrap();
        // We have parsed the input completely
        assert_eq!(input, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
