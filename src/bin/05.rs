use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::BTreeMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, almanac) = parse_input(input).ok()?;
    almanac
        .seeds
        .iter()
        .map(|seed| get_seed_location(*seed, &almanac))
        .min_by(|x, y| x.cmp(y))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, PartialEq)]
struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

#[derive(Debug, PartialEq)]
struct MapEntry {
    length: u32,
    destination_start: u32,
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil_map: BTreeMap<u32, MapEntry>,
    soil_to_fertilizer_map: BTreeMap<u32, MapEntry>,
    fertilizer_to_water_map: BTreeMap<u32, MapEntry>,
    water_to_light_map: BTreeMap<u32, MapEntry>,
    light_to_temperature_map: BTreeMap<u32, MapEntry>,
    temperature_to_humidity_map: BTreeMap<u32, MapEntry>,
    humidity_to_location_map: BTreeMap<u32, MapEntry>,
}

fn get_mapping_from_map(needle: u32, map: &BTreeMap<u32, MapEntry>) -> u32 {
    let mut prev: Option<(&u32, &MapEntry)> = None;
    for (key, value) in map.iter() {
        if *key > needle {
            return match prev {
                Some((&k, entry)) if needle - k < entry.length => {
                    entry.destination_start + (needle - k)
                }
                _ => needle,
            };
        }
        prev = Some((key, value));
    }

    match prev {
        Some((&k, entry)) if needle - k < entry.length => entry.destination_start + (needle - k),
        _ => needle,
    }
}

fn get_seed_location(seed: u32, almanac: &Almanac) -> u32 {
    let soil = get_mapping_from_map(seed, &almanac.seed_to_soil_map);
    let fertilizer = get_mapping_from_map(soil, &almanac.soil_to_fertilizer_map);
    let water = get_mapping_from_map(fertilizer, &almanac.fertilizer_to_water_map);
    let light = get_mapping_from_map(water, &almanac.water_to_light_map);
    let temperature = get_mapping_from_map(light, &almanac.light_to_temperature_map);
    let humidity = get_mapping_from_map(temperature, &almanac.temperature_to_humidity_map);
    get_mapping_from_map(humidity, &almanac.humidity_to_location_map)
}

fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, parse_number)(input)?;
    let (input, _) = multispace0(input)?;

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
            seeds,
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

fn parse_map<'a>(name: &str, input: &'a str) -> IResult<&'a str, BTreeMap<u32, MapEntry>> {
    let (input, _) = tag(name)(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    let (input, result) = map_res(parse_mapping_ranges, map_from_mapping_ranges)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, result))
}

fn map_from_mapping_ranges(ranges: Vec<Range>) -> Result<BTreeMap<u32, MapEntry>, &'static str> {
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
    Ok(map)
}

fn parse_mapping_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    separated_list1(newline, parse_mapping_range)(input)
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

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |x: &str| x.parse::<u32>())(input)
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
            Ok(BTreeMap::from([(
                98,
                MapEntry {
                    destination_start: 50,
                    length: 2
                }
            )]))
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
        assert_eq!(result, None);
    }
}
