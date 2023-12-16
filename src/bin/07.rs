use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending, space1},
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug)]
struct HandWithBid {
    hand: Hand,
    bid: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand(Vec<Card>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut counts = HashMap::new();
        let mut has_three_of_a_kind = false;
        let mut has_four_of_a_kind = false;
        for card in &self.0 {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
            if *count == 3 {
                has_three_of_a_kind = true;
            }
            if *count == 4 {
                has_four_of_a_kind = true;
            }
        }

        let base_type = match counts.len() {
            1 => HandType::FiveOfAKind,
            2 if has_four_of_a_kind => HandType::FourOfAKind,
            2 => HandType::FullHouse,
            3 if has_three_of_a_kind => HandType::ThreeOfAKind,
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };

        let joker_count = counts.get(&Card::Joker);
        match joker_count {
            None => base_type,
            Some(c) => match (base_type, c) {
                (HandType::FiveOfAKind, _) => HandType::FiveOfAKind,
                (HandType::FourOfAKind, _) => HandType::FiveOfAKind,
                (HandType::FullHouse, _) => HandType::FiveOfAKind,
                (HandType::ThreeOfAKind, _) => HandType::FourOfAKind,
                (HandType::TwoPair, 2) => HandType::FourOfAKind,
                (HandType::TwoPair, 1) => HandType::FullHouse,
                (HandType::OnePair, _) => HandType::ThreeOfAKind,
                (HandType::HighCard, 1) => HandType::OnePair,
                _ => panic!("Unknown case: {:?}", self),
            },
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        let type_cmp = self.get_type().cmp(&other.get_type());
        if type_cmp != Ordering::Equal {
            return type_cmp;
        }

        for i in 0..5 {
            let card_cmp = self.0[i].cmp(&other.0[i]);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }
        panic!("Should not reach this ever");
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Joker,
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

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut hands_with_bid) = parse_input(input).ok()?;
    hands_with_bid.sort_by_key(|hand_with_bid| hand_with_bid.hand.clone());
    Some(
        hands_with_bid
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, HandWithBid { bid, .. })| {
                acc + bid * (index as u32 + 1)
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut hands_with_bid) = parse_input_2(input).ok()?;
    hands_with_bid.sort_by_key(|hand_with_bid| hand_with_bid.hand.clone());
    Some(
        hands_with_bid
            .into_iter()
            .enumerate()
            .fold(0, |acc, (index, HandWithBid { bid, .. })| {
                acc + bid * (index as u32 + 1)
            }),
    )
}

fn parse_input(input: &str) -> IResult<&str, Vec<HandWithBid>> {
    separated_list1(
        line_ending,
        map_res(
            separated_pair(parse_hand, space1, parse_number),
            |(hand, bid): (Hand, u32)| Ok::<HandWithBid, &'static str>(HandWithBid { hand, bid }),
        ),
    )(input)
}

fn parse_input_2(input: &str) -> IResult<&str, Vec<HandWithBid>> {
    separated_list1(
        line_ending,
        map_res(
            separated_pair(parse_hand_2, space1, parse_number),
            |(hand, bid): (Hand, u32)| Ok::<HandWithBid, &'static str>(HandWithBid { hand, bid }),
        ),
    )(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map_res(count(parse_card(false), 5), |hand: Vec<Card>| {
        Ok::<Hand, &'static str>(Hand(hand))
    })(input)
}

fn parse_hand_2(input: &str) -> IResult<&str, Hand> {
    map_res(count(parse_card(true), 5), |hand: Vec<Card>| {
        Ok::<Hand, &'static str>(Hand(hand))
    })(input)
}

fn parse_card(j_is_joker: bool) -> impl FnMut(&str) -> IResult<&str, Card> {
    move |input| -> IResult<&str, Card> {
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
                'J' if j_is_joker => Ok(Card::Joker),
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
        let (remaining, _) = line_ending::<&str, nom::error::Error<&str>>(remaining).unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
