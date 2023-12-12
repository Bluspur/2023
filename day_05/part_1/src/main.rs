use lib::almanac::Almanac;

fn main() {
    let file = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    println!("Part 1 - Lowest seed location: {}", calculate(&file));
}

fn calculate(file: &str) -> usize {
    let almanac = Almanac::from_string(file).expect("Unable to parse almanac");
    let seed_locations = almanac.calculate_seed_locations();
    let lowest_seed_location = seed_locations
        .iter()
        .min()
        .expect("No seed locations found");
    *lowest_seed_location
}

#[cfg(test)]
mod test {
    use super::*;
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
    fn test_calculate() {
        assert_eq!(35, calculate(TEST_INPUT));
    }
}
