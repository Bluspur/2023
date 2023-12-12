fn main() {
    let input = std::fs::read_to_string("./data/puzzle_input.txt").unwrap();
    let result: u32 = input.lines().map(parse_line).sum();

    println!("Result: {}", result);
}

fn parse_line(input_line: &str) -> u32 {
    let patterns = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
    ];

    let first_match = patterns
        .iter()
        .filter_map(|(pattern, value)| input_line.find(pattern).map(|index| (index, value)))
        .min_by_key(|&(index, _)| index)
        .unwrap()
        .1;

    let last_match = patterns
        .iter()
        .filter_map(|(pattern, value)| input_line.rfind(pattern).map(|index| (index, value)))
        .max_by_key(|&(index, _)| index)
        .unwrap()
        .1;

    let combined = format!("{}{}", first_match, last_match);

    combined.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_returns_correct_mixed_words_numbers() {
        let test_data = "onetwo3";
        let expected = 13;
        let actual = parse_line(test_data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_line_returns_correct_overlapping_words() {
        let test_data = "onetwone";
        let expected = 11;
        let actual = parse_line(test_data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_line_returns_correct_overlapping_words_with_numbers() {
        let test_data = "onetwone3";
        let expected = 13;
        let actual = parse_line(test_data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_line_returns_correct_for_test_values() {
        let test_data = vec![
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (input, expected) in test_data {
            let actual = parse_line(input);
            assert_eq!(expected, actual);
        }
    }
}
