use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(4);

#[derive(Debug)]
struct ScratchCard {
    number: usize,
    winning_nums: HashSet<usize>,
    hand_nums: HashSet<usize>,
}

fn parse_scratch_card_set(input: &str) -> IResult<&str, HashSet<usize>> {
    let (input, scratch_card_set) =
        separated_list1(space1, map_res(digit1, |x: &str| x.parse::<usize>()))(input)?;
    Ok((input, scratch_card_set.into_iter().collect()))
}
fn parse_scratch_card(input: &str) -> IResult<&str, ScratchCard> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, number) = map_res(digit1, |x: &str| x.parse::<usize>())(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, (winning_nums, hand_nums)) = separated_pair(
        parse_scratch_card_set,
        tuple((space1, tag("|"), space1)),
        parse_scratch_card_set,
    )(input)?;
    let (input, _) = line_ending(input)?;

    Ok((
        input,
        ScratchCard {
            number,
            winning_nums,
            hand_nums,
        },
    ))
}
fn parse_input(input: &str) -> IResult<&str, Vec<ScratchCard>> {
    many1(parse_scratch_card)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, cards) = parse_input(input).ok()?;

    println!("{:?}", cards);
    let mut sum = 0;
    for card in cards {
        let winning_count = card.winning_nums.intersection(&card.hand_nums).count();
        if winning_count > 0 {
            sum += 1 << (winning_count - 1);
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, cards) = parse_input(input).ok()?;

    let mut card_counts: HashMap<usize, usize> = HashMap::new();
    for card in cards {
        card_counts.entry(card.number).or_insert(1);
        let card_count = card_counts.get(&card.number).copied().unwrap();

        let winning_count = card.winning_nums.intersection(&card.hand_nums).count();
        for index in 1..(winning_count + 1) {
            let existing_card_count = card_counts.entry(card.number + index).or_insert(1);
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
