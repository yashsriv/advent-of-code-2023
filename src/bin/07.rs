use nom::{
    branch::alt,
    character::complete::{char, digit1, newline, space1},
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct HandWithBid {
    hand: Hand,
    bid: u32,
}

fn parse_input(input: &str) -> IResult<&str, Vec<HandWithBid>> {
    separated_list1(
        newline,
        map_res(
            separated_pair(parse_hand, space1, parse_number),
            |(hand, bid): (Hand, u32)| Ok::<HandWithBid, &'static str>(HandWithBid { hand, bid }),
        ),
    )(input)
}

struct Hand(Vec<Card>);

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map_res(count(parse_card, 5), |hand: Vec<Card>| {
        Ok::<Hand, &'static str>(Hand(hand))
    })(input)
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map_res(
        alt((
            char('A'),
            char('K'),
            char('Q'),
            char('J'),
            char('T'),
            char('9'),
            char('8'),
            char('7'),
            char('6'),
            char('5'),
            char('4'),
            char('3'),
            char('2'),
        )),
        |ch| match ch {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err("Invalid card"),
        },
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |x: &str| x.parse::<u32>())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (remaining, _) = parse_input(&input).unwrap();
        assert_eq!(remaining, "\n");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
