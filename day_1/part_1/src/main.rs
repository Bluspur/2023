use std::io;

use itertools::{Itertools, Position};

fn main() {
    let file_path = "./data/puzzle_input.txt";
    let file_content = parse_file(file_path).expect("Failed to parse file");
    let calibration_values = read_calibration_values(file_content);
    println!("Output: {}", calculate_calibration_sum(calibration_values));
}

struct CalibrationValue(i32);

fn parse_line(input_line: &str) -> CalibrationValue {
    let mut value = input_line
        .chars()
        .filter(|x| x.is_ascii_digit())
        .with_position()
        .filter(|&(i, _)| i != Position::Middle)
        .map(|(_, x)| x)
        .collect::<String>();

    if value.len() == 1 {
        value = value.repeat(2);
    }

    let value = value.parse::<i32>().expect("Failed to parse string");
    CalibrationValue(value)
}

fn parse_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    Ok(std::fs::read_to_string(file_path)?
        .lines()
        .map(|x| x.to_owned())
        .collect_vec())
}

fn read_calibration_values(input: Vec<String>) -> Vec<CalibrationValue> {
    input.iter().map(|x| parse_line(x)).collect_vec()
}

fn calculate_calibration_sum(input: Vec<CalibrationValue>) -> i32 {
    input.iter().map(|x| x.0).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_returns_correct_value() {
        let test_data = "1abc2";
        let expected = CalibrationValue(12);
        let actual = parse_line(test_data);

        assert_eq!(expected.0, actual.0);
    }

    #[test]
    fn parse_file_returns_correct_number_of_lines() {
        let expected = 4;
        let actual = parse_file("./data/test_input.txt")
            .expect("Failed to read file")
            .len();

        assert_eq!(expected, actual);
    }

    #[test]
    fn read_calibration_values_returns_correct_values() {
        let test_data = parse_file("./data/test_input.txt").unwrap();

        let expected = vec![
            CalibrationValue(12),
            CalibrationValue(38),
            CalibrationValue(15),
            CalibrationValue(77),
        ];

        let actual = read_calibration_values(test_data);

        assert_eq!(expected[0].0, actual[0].0);
        assert_eq!(expected[1].0, actual[1].0);
        assert_eq!(expected[2].0, actual[2].0);
        assert_eq!(expected[3].0, actual[3].0);
    }

    #[test]
    fn calculate_calibration_sum_returns_correct_value() {
        let test_data = parse_file("./data/test_input.txt").unwrap();
        let values = read_calibration_values(test_data);
        let actual = calculate_calibration_sum(values);
        let expected = 142;

        assert_eq!(expected, actual);
    }
}
