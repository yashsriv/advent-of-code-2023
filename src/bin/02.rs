use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::{many0, separated_list1},
    IResult,
};

advent_of_code::solution!(2);

struct Draw<'a> {
    color: &'a str,
    count: u32,
}

struct Game<'a> {
    number: u32,
    draws: Vec<Vec<Draw<'a>>>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, games) = parse_entire_input(input).ok()?;

    let blue_count = 14;
    let green_count = 13;
    let red_count = 12;
    let mut sum = 0;

    for game in games {
        let game_num = game.number;
        let mut is_valid = true;
        for draws in game.draws {
            for draw in draws {
                match (draw.color, draw.count) {
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
    let (_, games) = parse_entire_input(input).ok()?;

    let mut sum = 0;

    for game in games {
        let mut min_blue_count = 0;
        let mut min_green_count = 0;
        let mut min_red_count = 0;

        for draws in game.draws {
            for draw in draws {
                match (draw.color, draw.count) {
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

fn parse_entire_input(input: &str) -> IResult<&str, Vec<Game>> {
    many0(parse_single_line)(input)
}

fn parse_single_line(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, number) = map_res(digit1, from_dec)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, draws) = parse_game(input)?;
    let (input, _) = newline(input)?;

    Ok((input, Game { number, draws }))
}

fn parse_game(input: &str) -> IResult<&str, Vec<Vec<Draw>>> {
    separated_list1(tag("; "), parse_draws)(input)
}

fn parse_draws(input: &str) -> IResult<&str, Vec<Draw>> {
    separated_list1(tag(", "), parse_draw)(input)
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (input, count) = map_res(digit1, from_dec)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((tag("blue"), tag("green"), tag("red")))(input)?;

    Ok((input, Draw { color, count }))
}

fn from_dec(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
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
