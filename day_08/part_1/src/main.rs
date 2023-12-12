use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Could not read file");
    let (directions, map) = parse(&input);
    let steps = get_steps_to_destination(&directions, &map);
    println!("Steps to destination: {}", steps);
}

fn get_steps_to_destination(
    directions: &[Direction],
    map: &HashMap<String, (String, String)>,
) -> usize {
    let mut steps = 0;
    let mut current = "AAA".to_string();
    while current != "ZZZ" {
        let (left, right) = map.get(&current).expect("Current must be in map");
        match directions[steps % directions.len()] {
            Direction::Right => current = right.to_string(),
            Direction::Left => current = left.to_string(),
        }
        steps += 1;
    }

    steps
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let input = input.replace("\r\n", "\n");

    let (directions, map) = input
        .split_once("\n\n")
        .expect("Input should be split by \\n\\n");

    let directions = directions.chars().map(Direction::from_char).collect();

    let map = map
        .lines()
        .map(|l| {
            let (key, value) = l.split_once(" = ").expect("Line must contain ' = '");
            let (left, right) = value
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .expect("Value must contain ', '");
            (key.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();

    (directions, map)
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parse_map() {
        let expected_directions = vec![Direction::Right, Direction::Left];
        let (actual_directions, actual_map) = parse(TEST_INPUT);

        assert_eq!(expected_directions, actual_directions);
        assert_eq!(
            actual_map.get("AAA"),
            Some(&("BBB".to_string(), "CCC".to_string()))
        );
        assert_eq!(
            actual_map.get("BBB"),
            Some(&("DDD".to_string(), "EEE".to_string()))
        );
        assert_eq!(
            actual_map.get("CCC"),
            Some(&("ZZZ".to_string(), "GGG".to_string()))
        );
        assert_eq!(
            actual_map.get("DDD"),
            Some(&("DDD".to_string(), "DDD".to_string()))
        );
        assert_eq!(
            actual_map.get("EEE"),
            Some(&("EEE".to_string(), "EEE".to_string()))
        );
        assert_eq!(
            actual_map.get("GGG"),
            Some(&("GGG".to_string(), "GGG".to_string()))
        );
        assert_eq!(
            actual_map.get("ZZZ"),
            Some(&("ZZZ".to_string(), "ZZZ".to_string()))
        );
    }

    #[test]
    fn test_get_steps_to_destination() {
        let (directions, map) = parse(TEST_INPUT);
        let expected = 2;

        let actual = get_steps_to_destination(&directions, &map);

        assert_eq!(expected, actual);
    }
}
