use lib::almanac::Almanac;

fn main() {
    let file = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    println!(
        "Part 2 - Lowest seed location (ranges): {}",
        calculate(&file)
    );
}

fn calculate(file: &str) -> usize {
    let almanac = Almanac::from_string(file).expect("Unable to parse almanac");
    let seed_locations = almanac.calculate_seed_locations_with_seed_ranges();
    let lowest_seed_location = seed_locations
        .iter()
        .min()
        .expect("No seed locations found");
    *lowest_seed_location
}
