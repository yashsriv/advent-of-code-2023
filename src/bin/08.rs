use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, line_ending, multispace1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (instructions, DirectionMap(direction_map))) = parse_input(input).ok()?;
    let mut iter = instructions.into_iter().cycle();
    let mut count = 0;
    let mut current_node = direction_map.get("AAA")?;
    while current_node.value != "ZZZ" {
        let direction = iter.next()?;
        match direction {
            Instruction::Left => {
                current_node = direction_map.get(current_node.left)?;
            }
            Instruction::Right => {
                current_node = direction_map.get(current_node.right)?;
            }
        }
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    // This is too slow for the problem input
    let (_, (instructions, DirectionMap(direction_map))) = parse_input(input).ok()?;
    let mut iter = instructions.into_iter().cycle();
    let mut count = 0;
    let mut current_nodes: Vec<&Node> = direction_map
        .values()
        .filter(|Node { value, .. }| value.ends_with("A"))
        .collect();
    while !current_nodes
        .iter()
        .all(|Node { value, .. }| value.ends_with("Z"))
    {
        let direction = iter.next()?;
        current_nodes = current_nodes
            .into_iter()
            .map(|current_node| match direction {
                Instruction::Left => direction_map.get(current_node.left).unwrap(),
                Instruction::Right => direction_map.get(current_node.right).unwrap(),
            })
            .collect();
        count += 1;
    }
    Some(count)
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Node<'a> {
    value: &'a str,
    left: &'a str,
    right: &'a str,
}
struct DirectionMap<'a>(HashMap<&'a str, Node<'a>>);

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map_res(alt((char('L'), char('R'))), |ch: char| match ch {
        'L' => Ok(Instruction::Left),
        'R' => Ok(Instruction::Right),
        _ => Err("impossible char in instruction parse"),
    })(input)
}

fn parse_direction_map_entry(input: &str) -> IResult<&str, (&str, Node)> {
    map_res(
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                char('('),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                char(')'),
            ),
        ),
        |(value, (left, right))| Ok::<_, &'static str>((value, Node { left, right, value })),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, DirectionMap)> {
    let (input, instructions) = terminated(many1(parse_instruction), multispace1)(input)?;
    let (input, direction_map) = terminated(
        separated_list1(line_ending, parse_direction_map_entry),
        multispace1,
    )(input)?;

    Ok((
        input,
        (
            instructions,
            DirectionMap(direction_map.into_iter().collect()),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let (remaining, instruction) = parse_instruction("L").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction, Instruction::Left);

        let (remaining, instruction) = parse_instruction("R").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction, Instruction::Right);

        let result = parse_instruction("Z");
        assert!(result.is_err());

        let (remaining, instructions) =
            terminated(many1(parse_instruction), multispace1)("LR\n\nAAA = (BBB, CCC)\n").unwrap();
        assert_eq!(remaining, "AAA = (BBB, CCC)\n");
        assert_eq!(instructions, vec![Instruction::Left, Instruction::Right]);
    }

    #[test]
    fn test_parse_direction_map_entry() {
        let (remaining, map_entry) = parse_direction_map_entry("AAA = (BBB, CCC)").unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            map_entry,
            (
                "AAA",
                Node {
                    left: "BBB",
                    right: "CCC",
                    value: "AAA",
                }
            )
        );
    }

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, _) = parse_input(&input).unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
