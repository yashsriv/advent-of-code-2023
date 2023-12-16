use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::BTreeMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (seeds, almanac)) = parse_input(input).ok()?;
    seeds
        .iter()
        .map(|seed| get_seed_location(*seed, &almanac))
        .min_by(|x, y| x.cmp(y))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, ParsedInput2(seeds, almanac)) = parse_input_2(input).ok()?;
    let soil = map_ranges_to_next_range(seeds, &almanac.seed_to_soil_map);
    let fertilizer = map_ranges_to_next_range(soil, &almanac.soil_to_fertilizer_map);
    let water = map_ranges_to_next_range(fertilizer, &almanac.fertilizer_to_water_map);
    let light = map_ranges_to_next_range(water, &almanac.water_to_light_map);
    let temperature = map_ranges_to_next_range(light, &almanac.light_to_temperature_map);
    let humidity = map_ranges_to_next_range(temperature, &almanac.temperature_to_humidity_map);
    let location = map_ranges_to_next_range(humidity, &almanac.humidity_to_location_map);
    location
        .into_iter()
        .min_by_key(|(start, _)| *start)
        .map(|(start, _)| start)
}

#[derive(Debug, PartialEq)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MapEntry {
    length: u64,
    destination_start: u64,
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seed_to_soil_map: Vec<(u64, MapEntry)>,
    soil_to_fertilizer_map: Vec<(u64, MapEntry)>,
    fertilizer_to_water_map: Vec<(u64, MapEntry)>,
    water_to_light_map: Vec<(u64, MapEntry)>,
    light_to_temperature_map: Vec<(u64, MapEntry)>,
    temperature_to_humidity_map: Vec<(u64, MapEntry)>,
    humidity_to_location_map: Vec<(u64, MapEntry)>,
}

fn map_ranges_to_next_range(
    mut source: Vec<(u64, u64)>,
    mappings: &[(u64, MapEntry)],
) -> Vec<(u64, u64)> {
    let mut new_range = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < source.len() && j < mappings.len() {
        let (source_start, source_end) = source[i];

        let (mapping_start, mapping_entry) = mappings[j];
        let mapping_end = mapping_start + mapping_entry.length - 1;

        if mapping_end < source_start {
            j += 1;
            continue;
        } else if mapping_start > source_start {
            if source_end < mapping_start {
                new_range.push((source_start, source_end));
                i += 1;
            } else {
                new_range.push((source_start, mapping_start - 1));
                source[i] = (mapping_start, source_end);
            }
        } else if mapping_end >= source_end {
            let mapped_start = mapping_entry.destination_start + (source_start - mapping_start);
            let mapped_end = mapped_start + (source_end - source_start);
            new_range.push((mapped_start, mapped_end));
            i += 1;
        } else if mapping_end < source_end {
            let mapped_start = mapping_entry.destination_start + (source_start - mapping_start);
            let mapped_end = mapped_start + (mapping_end - source_start);
            new_range.push((mapped_start, mapped_end));
            source[i] = (mapping_end + 1, source_end);
            j += 1;
        } else {
            panic!("Should not occur");
        }
    }

    while i != source.len() {
        new_range.push(source[i]);
        i += 1;
    }
    new_range.sort_by_key(|(start, _)| *start);
    new_range
}

fn get_seed_location(seed: u64, almanac: &Almanac) -> u64 {
    let soil = map_ranges_to_next_range(vec![(seed, seed)], &almanac.seed_to_soil_map);
    let fertilizer = map_ranges_to_next_range(soil, &almanac.soil_to_fertilizer_map);
    let water = map_ranges_to_next_range(fertilizer, &almanac.fertilizer_to_water_map);
    let light = map_ranges_to_next_range(water, &almanac.water_to_light_map);
    let temperature = map_ranges_to_next_range(light, &almanac.light_to_temperature_map);
    let humidity = map_ranges_to_next_range(temperature, &almanac.temperature_to_humidity_map);
    let location = map_ranges_to_next_range(humidity, &almanac.humidity_to_location_map);
    location[0].0
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Almanac)> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, parse_number)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, almanac) = parse_mappings(input)?;

    Ok((input, (seeds, almanac)))
}

struct ParsedInput2(Vec<(u64, u64)>, Almanac);
fn parse_input_2(input: &str) -> IResult<&str, ParsedInput2> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seed_ranges) = separated_list1(
        space1,
        map_res(
            separated_pair(parse_number, space1, parse_number),
            |(start, length)| Ok::<(u64, u64), &'static str>((start, start + length - 1)),
        ),
    )(input)?;
    let (input, _) = multispace0(input)?;

    let (input, almanac) = parse_mappings(input)?;

    Ok((input, ParsedInput2(seed_ranges, almanac)))
}

fn parse_mappings(input: &str) -> IResult<&str, Almanac> {
    let (input, seed_to_soil_map) = parse_map("seed-to-soil", input)?;
    let (input, soil_to_fertilizer_map) = parse_map("soil-to-fertilizer", input)?;
    let (input, fertilizer_to_water_map) = parse_map("fertilizer-to-water", input)?;
    let (input, water_to_light_map) = parse_map("water-to-light", input)?;
    let (input, light_to_temperature_map) = parse_map("light-to-temperature", input)?;
    let (input, temperature_to_humidity_map) = parse_map("temperature-to-humidity", input)?;
    let (input, humidity_to_location_map) = parse_map("humidity-to-location", input)?;

    Ok((
        input,
        Almanac {
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        },
    ))
}

fn parse_map<'a>(name: &str, input: &'a str) -> IResult<&'a str, Vec<(u64, MapEntry)>> {
    let (input, _) = tag(name)(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, result) = map_res(parse_mapping_ranges, map_from_mapping_ranges)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, result))
}

fn map_from_mapping_ranges(ranges: Vec<Range>) -> Result<Vec<(u64, MapEntry)>, &'static str> {
    let mut map = BTreeMap::new();
    for range in ranges {
        map.insert(
            range.source_start,
            MapEntry {
                destination_start: range.destination_start,
                length: range.length,
            },
        );
    }
    Ok(map.into_iter().collect())
}

fn parse_mapping_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    separated_list1(line_ending, parse_mapping_range)(input)
}

fn parse_mapping_range(input: &str) -> IResult<&str, Range> {
    let (input, (destination_start, _, source_start, _, length)) =
        tuple((parse_number, space1, parse_number, space1, parse_number))(input)?;
    Ok((
        input,
        Range {
            destination_start,
            source_start,
            length,
        },
    ))
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |x: &str| x.parse::<u64>())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let result = parse_number("23");
        assert_eq!(result, Ok(("", 23)));
    }

    #[test]
    fn test_parse_mapping_range() {
        let result = parse_mapping_range("50 98 2");
        assert_eq!(
            result,
            Ok((
                "",
                Range {
                    destination_start: 50,
                    source_start: 98,
                    length: 2
                }
            ))
        );
    }

    #[test]
    fn test_parse_mapping_ranges() {
        let result = parse_mapping_ranges("50 98 2\n52 50 48\n");
        assert_eq!(
            result,
            Ok((
                "\n",
                vec![
                    Range {
                        destination_start: 50,
                        source_start: 98,
                        length: 2
                    },
                    Range {
                        destination_start: 52,
                        source_start: 50,
                        length: 48
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_map_from_mapping_ranges() {
        let (_, ranges) = parse_mapping_ranges("50 98 2").unwrap();
        let result = map_from_mapping_ranges(ranges);
        assert_eq!(
            result,
            Ok(vec![(
                98,
                MapEntry {
                    destination_start: 50,
                    length: 2
                }
            )])
        );
    }

    #[test]
    fn test_parse_input() {
        let binding = advent_of_code::template::read_file("examples", DAY);
        let (result, _) = parse_input(&binding).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
