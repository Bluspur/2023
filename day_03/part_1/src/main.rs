use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Failed to read input");
    let mut schematic = Schematic::from(input.as_str());
    schematic.calculate_parts();

    // for position in 0..=input.chars().count() {
    //     let Position(x, y) = get_position(schematic.columns, position);

    //     if x == 0 {
    //         println!("|");
    //     }

    //     print!(
    //         "{}",
    //         match schematic.map.get(&Position(x, y)) {
    //             Some(SchematicElement::Number(index)) => {
    //                 if schematic.numbers[*index].is_part {
    //                     "X"
    //                 } else {
    //                     "O"
    //                 }
    //             }
    //             Some(SchematicElement::Symbol) => "S",
    //             None => "^",
    //         }
    //     );
    // }

    let part_sum = schematic.get_part_sum();
    println!("Part 1: {}", part_sum);
}

trait EngineParts {
    fn is_engine_part_symbol(&self) -> bool;
}

impl EngineParts for char {
    #[inline]
    fn is_engine_part_symbol(&self) -> bool {
        matches!(*self, '!'..='-'| '/' | ':'..='@' | '['..='`' | '{'..='~')
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(usize, usize);

impl Position {
    fn get_adjacent_positions(&self) -> Vec<Position> {
        let mut output = Vec::<Position>::new();
        let Position(x, y) = *self;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    output.push(Position(nx as usize, ny as usize));
                }
            }
        }

        output
    }
}

#[derive(Debug, PartialEq)]
enum SchematicElement {
    Number(usize),
    Symbol,
}

#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    position: Position,
    is_part: bool,
}

impl Number {
    fn new(value: u32, position: Position) -> Number {
        Number {
            value,
            position,
            is_part: false,
        }
    }

    fn len(&self) -> usize {
        self.value.to_string().len()
    }
}

struct Schematic {
    numbers: Vec<Number>,
    map: HashMap<Position, SchematicElement>,
}

fn get_numbers(s: &str, cols: usize) -> Vec<Number> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();
    let mut start_index = None;

    for (i, c) in s.char_indices() {
        if c.is_numeric() {
            current_number.push(c);
            if start_index.is_none() {
                start_index = Some(i);
            }
        } else if !current_number.is_empty() {
            let n = Number::new(
                current_number
                    .parse::<u32>()
                    .expect("Failed to Parse Number"),
                get_position(cols, start_index.unwrap()),
            );
            numbers.push(n);
            current_number.clear();
            start_index = None;
        }
    }

    if !current_number.is_empty() {
        let n = Number::new(
            current_number
                .parse::<u32>()
                .expect("Failed to Parse Number"),
            get_position(cols, start_index.unwrap()),
        );
        numbers.push(n);
    }

    numbers
}

fn get_part_symbols(s: &str, cols: usize) -> Vec<Position> {
    let mut output = Vec::<Position>::new();
    for (i, _) in s.char_indices().filter(|&(_, c)| c.is_engine_part_symbol()) {
        let position = get_position(cols, i);
        output.push(position);
    }

    output
}

impl Schematic {
    fn calculate_parts(&mut self) {
        let mut to_update = HashSet::new();

        for s in self
            .map
            .iter()
            .filter(|(_, e)| matches!(e, SchematicElement::Symbol))
        {
            let position = *s.0;
            let get_adjacent_positions = &position.get_adjacent_positions();
            let adjacent_numbers =
                get_adjacent_positions
                    .iter()
                    .filter_map(|p| match self.map.get(p)? {
                        SchematicElement::Number(index) => Some(*index),
                        _ => None,
                    });

            for number in adjacent_numbers {
                to_update.insert(number);
            }
        }

        for number in to_update {
            self.numbers[number].is_part = true;
        }
    }

    fn get_part_sum(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| n.is_part)
            .map(|n| n.value)
            .sum()
    }
}

impl From<&str> for Schematic {
    fn from(s: &str) -> Self {
        let columns = s.lines().next().unwrap().len();
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();

        let numbers = get_numbers(&s, columns);
        let part_symbols = get_part_symbols(&s, columns);
        let mut map = HashMap::<Position, SchematicElement>::new();
        for (i, number) in numbers.iter().enumerate() {
            let x = number.position.0;
            for pos in x..x + number.len() {
                let position = Position(pos, number.position.1);
                map.insert(position, SchematicElement::Number(i));
            }
        }
        for part_symbol in part_symbols.iter() {
            map.insert(*part_symbol, SchematicElement::Symbol);
        }

        Schematic { numbers, map }
    }
}

fn get_position(columns: usize, index: usize) -> Position {
    let x = index % columns;
    let y = index / columns;
    Position(x, y)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_position_returns_correct_position_simple() {
        let cols = 10;
        let index = 1;

        let expected = Position(1, 0);
        let actual = get_position(cols, index);

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_get_position_returns_correct_position_complex() {
        let cols = 10;
        let index = 23;

        let expected = Position(3, 2);
        let actual = get_position(cols, index);

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_get_numbers_returns_expected_result() {
        let test_data = "467..114";

        let expected = vec![
            Number::new(467, Position(0, 0)),
            Number::new(114, Position(1, 1)),
        ];
        let expected_len = 2;

        let actual = get_numbers(test_data, 4);
        let actual_len = actual.len();

        assert_eq!(expected_len, actual_len);
        assert_eq!(expected[0], actual[0]);
        assert_eq!(expected[1], actual[1]);
    }

    #[test]
    fn test_get_part_symbols_returns_expected_result() {
        let test_data = "467./114";

        let expected = [Position(0, 1)];
        let expected_len = 1;

        let actual = get_part_symbols(test_data, 4);
        let actual_len = actual.len();

        assert_eq!(expected_len, actual_len);
        assert_eq!(expected[0], actual[0]);
    }

    #[test]
    fn test_schematic_from_returns_expected_result() {
        let test_data = "467.\n/114";

        let expected_numbers = vec![
            Number::new(467, Position(0, 0)),
            Number::new(114, Position(1, 1)),
        ];
        let expected_numbers_len = 2;
        let expected_map = vec![
            (Position(0, 0), SchematicElement::Number(0)),
            (Position(1, 1), SchematicElement::Number(1)),
            (Position(0, 1), SchematicElement::Symbol),
        ];
        let expected_map_len = 7;

        let actual = Schematic::from(test_data);
        let actual_numbers_len = actual.numbers.len();
        let actual_map_len = actual.map.len();

        assert_eq!(expected_numbers_len, actual_numbers_len);
        assert_eq!(expected_map_len, actual_map_len);
        assert_eq!(expected_numbers[0], actual.numbers[0]);
        assert_eq!(expected_numbers[1], actual.numbers[1]);
        assert!(actual.map.contains_key(&expected_map[0].0));
        assert_eq!(Some(&expected_map[0].1), actual.map.get(&expected_map[0].0));
        assert!(actual.map.contains_key(&expected_map[1].0));
        assert_eq!(Some(&expected_map[1].1), actual.map.get(&expected_map[1].0));
        assert!(actual.map.contains_key(&expected_map[2].0));
        assert_eq!(Some(&expected_map[2].1), actual.map.get(&expected_map[2].0));
    }

    #[test]
    fn test_provided_test_data_returns_expected_result() {
        let test_data = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let expected_sum = 4361;
        let mut schematic = Schematic::from(test_data);
        schematic.calculate_parts();
        let actual_sum = schematic.get_part_sum();

        assert_eq!(expected_sum, actual_sum);
    }
}
