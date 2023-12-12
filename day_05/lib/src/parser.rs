use std::error::Error;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{map, map_res, opt},
    multi::many1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::almanac::Almanac;
use crate::converter::Converter;
use crate::map::Map;

fn parse_integer(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(tag("seeds:"), many1(preceded(space1, parse_integer)))(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let parser = tuple((
        parse_integer,
        preceded(space1, parse_integer),
        preceded(space1, parse_integer),
    ));
    map(
        parser,
        |(destination_range_start, source_range_start, range_length)| {
            Map::new(destination_range_start, source_range_start, range_length)
        },
    )(input)
}

fn parse_converter<'a>(input: &'a str, section_name: &str) -> IResult<&'a str, Converter> {
    let section_header = preceded(
        opt(line_ending),
        separated_pair(tag(section_name), tag(":"), line_ending),
    );
    let parser = preceded(
        section_header,
        many1(terminated(parse_map, opt(line_ending))),
    );
    map(preceded(opt(line_ending), parser), Converter::new)(input)
}

pub fn parse_almanac(input: &str) -> Result<Almanac, Box<dyn Error + '_>> {
    let parser = tuple((
        parse_seeds,
        |i| parse_converter(i, "seed-to-soil map"),
        |i| parse_converter(i, "soil-to-fertilizer map"),
        |i| parse_converter(i, "fertilizer-to-water map"),
        |i| parse_converter(i, "water-to-light map"),
        |i| parse_converter(i, "light-to-temperature map"),
        |i| parse_converter(i, "temperature-to-humidity map"),
        |i| parse_converter(i, "humidity-to-location map"),
    ));
    match map(
        parser,
        |(
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        )| {
            Almanac {
                seeds,
                seed_to_soil_map,
                soil_to_fertilizer_map,
                fertilizer_to_water_map,
                water_to_light_map,
                light_to_temperature_map,
                temperature_to_humidity_map,
                humidity_to_location_map,
            }
        },
    )(input)
    {
        Ok((_, almanac)) => Ok(almanac),
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_parse_seeds() {
        assert_eq!(
            parse_seeds("seeds: 79 14 55 13"),
            Ok(("", vec![79, 14, 55, 13]))
        );
    }

    #[test]
    fn test_parse_seeds_error() {
        assert!(parse_seeds("seeds: abc").is_err());
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_integer("123"), Ok(("", 123)));
        assert_eq!(parse_integer("123abc"), Ok(("abc", 123)));
    }

    #[test]
    fn test_parse_integer_error() {
        assert!(parse_integer("abc").is_err());
    }

    #[test]
    fn test_from_str() {
        let expected = Map::new(50, 98, 2);

        assert_eq!(parse_map("50 98 2"), Ok(("", expected)));
    }

    #[test]
    fn test_from_str_error() {
        assert!(parse_map("abc").is_err());
    }

    #[test]
    fn test_parse_converter() {
        let test_input = indoc! {"
            seed-to-soil map:
            50 98 2
            52 50 48
            "};

        let expected = Converter::new(vec![Map::new(50, 98, 2), Map::new(52, 50, 48)]);

        assert_eq!(
            parse_converter(test_input, "seed-to-soil map"),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_parse_converter_error() {
        let test_input = indoc! {"
            seed-to-soil map:
            50 98 2
            52 50 48
            "};

        assert!(parse_converter(test_input, "abc").is_err());
    }

    #[test]
    fn test_parse_almanac() {
        let expected = Almanac {
            seeds: vec![79, 14, 55, 13],
            seed_to_soil_map: Converter::new(vec![Map::new(50, 98, 2), Map::new(52, 50, 48)]),
            soil_to_fertilizer_map: Converter::new(vec![
                Map::new(0, 15, 37),
                Map::new(37, 52, 2),
                Map::new(39, 0, 15),
            ]),
            fertilizer_to_water_map: Converter::new(vec![
                Map::new(49, 53, 8),
                Map::new(0, 11, 42),
                Map::new(42, 0, 7),
                Map::new(57, 7, 4),
            ]),
            water_to_light_map: Converter::new(vec![Map::new(88, 18, 7), Map::new(18, 25, 70)]),
            light_to_temperature_map: Converter::new(vec![
                Map::new(45, 77, 23),
                Map::new(81, 45, 19),
                Map::new(68, 64, 13),
            ]),
            temperature_to_humidity_map: Converter::new(vec![
                Map::new(0, 69, 1),
                Map::new(1, 0, 69),
            ]),
            humidity_to_location_map: Converter::new(vec![
                Map::new(60, 56, 37),
                Map::new(56, 93, 4),
            ]),
        };

        let actual = parse_almanac(TEST_INPUT).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_almanac_from_file() {
        let expected = Almanac {
            seeds: vec![79, 14, 55, 13],
            seed_to_soil_map: Converter::new(vec![Map::new(50, 98, 2), Map::new(52, 50, 48)]),
            soil_to_fertilizer_map: Converter::new(vec![
                Map::new(0, 15, 37),
                Map::new(37, 52, 2),
                Map::new(39, 0, 15),
            ]),
            fertilizer_to_water_map: Converter::new(vec![
                Map::new(49, 53, 8),
                Map::new(0, 11, 42),
                Map::new(42, 0, 7),
                Map::new(57, 7, 4),
            ]),
            water_to_light_map: Converter::new(vec![Map::new(88, 18, 7), Map::new(18, 25, 70)]),
            light_to_temperature_map: Converter::new(vec![
                Map::new(45, 77, 23),
                Map::new(81, 45, 19),
                Map::new(68, 64, 13),
            ]),
            temperature_to_humidity_map: Converter::new(vec![
                Map::new(0, 69, 1),
                Map::new(1, 0, 69),
            ]),
            humidity_to_location_map: Converter::new(vec![
                Map::new(60, 56, 37),
                Map::new(56, 93, 4),
            ]),
        };
        let file = std::fs::read_to_string("../test_input.txt").expect("Unable to read file");
        let actual = Almanac::from_string(&file);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
