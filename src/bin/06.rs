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
        let mut win_count = 0;
        for time in 0..race.time {
            let remaining_time = race.time - time;
            let speed = time;
            let distance_covered = speed * remaining_time;
            if distance_covered > race.distance {
                win_count += 1;
            }
        }
        if win_count > 0 {
            product *= win_count;
        }
    }
    Some(product)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
