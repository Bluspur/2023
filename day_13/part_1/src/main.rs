use std::str::FromStr;

use anyhow::{Context, Result};

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Error reading input.txt");
    let result = calculate_pattern_summary(&input).expect("Error calculating pattern summary");
    println!("Result: {}", result);
}

fn calculate_pattern_summary(input: &str) -> Result<usize> {
    let patterns = parse_input(input).context("Failed To Parse Input")?;
    let summaries = patterns
        .into_iter()
        .map(|pat| {
            let axis = pat.get_reflection_axis().expect("No reflection axis found");
            pat.get_columns_left_or_above(axis)
        })
        .fold((0, 0), |(acc_rows, acc_cols), summary| match summary {
            Summary::Above(rows) => (acc_rows + rows, acc_cols),
            Summary::Left(cols) => (acc_rows, acc_cols + cols),
        });
    Ok(get_pattern_summary(summaries.1, summaries.0))
}

fn parse_input(input: &str) -> Result<Vec<Pattern>> {
    let mut patterns = Vec::new();

    // Normalise line endings to \n
    let input = input.replace("\r\n", "\n");

    for pattern in input.split("\n\n") {
        patterns.push(Pattern::from_str(pattern)?);
    }

    Ok(patterns)
}

struct Pattern {
    cells: Vec<Vec<CellType>>,
    dimensions: (usize, usize),
    encoded_rows: Vec<usize>,
    encoded_cols: Vec<usize>,
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for char in line.chars() {
                row.push(CellType::try_from(char)?);
            }
            cells.push(row);
        }

        Ok(Self::new(cells))
    }
}

impl Pattern {
    fn new(cells: Vec<Vec<CellType>>) -> Self {
        let dimensions = (cells[0].len(), cells.len());
        let mut pattern = Self {
            cells,
            dimensions,
            encoded_rows: Vec::new(),
            encoded_cols: Vec::new(),
        };

        pattern.encode_rows();
        pattern.encode_columns();

        pattern
    }

    fn encode_rows(&mut self) {
        let mut encoded_rows = Vec::new();
        for row in &self.cells {
            encoded_rows.push(encode_cells(row));
        }
        self.encoded_rows = encoded_rows;
    }

    fn encode_columns(&mut self) {
        let mut encoded_cols = Vec::new();
        for col in 0..self.dimensions.0 {
            let mut column = Vec::new();
            for row in &self.cells {
                column.push(row[col]);
            }
            encoded_cols.push(encode_cells(&column));
        }
        self.encoded_cols = encoded_cols;
    }

    fn get_reflection_axis(&self) -> Option<ReflectionAxis> {
        let max_dim = self.dimensions.0.max(self.dimensions.1);

        for index in 1..max_dim {
            if index < self.dimensions.0
                && self
                    .encoded_rows
                    .iter()
                    .all(|&row| is_reflection_axis(row, index, self.dimensions.0))
            {
                return Some(ReflectionAxis::Vertical(index));
            }

            if index < self.dimensions.1
                && self
                    .encoded_cols
                    .iter()
                    .all(|&col| is_reflection_axis(col, index, self.dimensions.1))
            {
                return Some(ReflectionAxis::Horizontal(index));
            }
        }

        None
    }

    fn get_columns_left_or_above(&self, axis: ReflectionAxis) -> Summary {
        match axis {
            ReflectionAxis::Horizontal(index) => Summary::Above(self.dimensions.1 - index),
            ReflectionAxis::Vertical(index) => Summary::Left(self.dimensions.0 - index),
        }
    }
}

fn encode_cells(cells: &[CellType]) -> usize {
    let bits = cells
        .iter()
        .map(|cell| match cell {
            CellType::Ash => false,
            CellType::Rock => true,
        })
        .collect();

    build_binary_number(bits)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ReflectionAxis {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Summary {
    Above(usize),
    Left(usize),
}

#[derive(Debug, Clone, Copy)]
enum CellType {
    Ash,
    Rock,
}

impl TryFrom<char> for CellType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            _ => Err(anyhow::anyhow!("Cell Type can only be of type '.' or '#'")),
        }
    }
}

#[inline]
fn get_pattern_summary(total_columns_to_left: usize, total_rows_above: usize) -> usize {
    total_columns_to_left + (total_rows_above * 100)
}

// Axis index is from the least significant bit to the most significant bit (L<-R)
// Can return false positives, but I cannot think of a way to avoid it at the moment
fn is_reflection_axis(encoded_line: usize, axis_index: usize, line_size: usize) -> bool {
    let (left, right) = split_and_mirror_binary_pattern_at_least_significant_bit_index(
        encoded_line,
        line_size,
        axis_index,
    );

    left == right
}

fn split_and_mirror_binary_pattern_at_least_significant_bit_index(
    pattern: usize,
    pattern_length: usize,
    bit_index: usize,
) -> (usize, usize) {
    let len_right = bit_index;
    let len_left = pattern_length - bit_index;
    // Shift the pattern right to get the left side of split
    let mut left_side = pattern >> bit_index;
    // Create a mask so we only get the bits of the right that we want
    let mask = (1 << bit_index) - 1;
    let mut right_side = pattern & mask;

    // We always need to flip the left hand side
    left_side = left_side.reverse_bits();

    // Then we need to determine how far to shift the left hand side, depending on which is longer
    left_side >>= 64 - len_right.min(len_left);

    // If the left is smaller, then we need to realign the right hand side
    if len_left < len_right {
        right_side >>= len_right - len_left;
    }

    (left_side, right_side)
}

fn build_binary_number(bits: Vec<bool>) -> usize {
    let mut number = 0;

    for bit in bits {
        number <<= 1;

        if bit {
            // The zero is inherent after the shift, so we only care if the bit was true
            number |= 1;
        }
    }

    number
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_get_pattern_summary() {
        let columns_to_left = 5;
        let rows_above = 4;

        let expected = 405;
        let actual = get_pattern_summary(columns_to_left, rows_above);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_cell_type_from_char_returns_ok_for_valid_input() {
        assert!(CellType::try_from('.').is_ok());
        assert!(CellType::try_from('#').is_ok());
    }

    #[test]
    fn test_get_cell_type_from_char_returns_err_for_invalid_input() {
        // Whitespace
        assert!(CellType::try_from(' ').is_err());
        // Invalid Character
        assert!(CellType::try_from('a').is_err());
    }

    #[test]
    fn test_is_reflection_axis_returns_true_for_valid_reflection() {
        let axis = 4;
        let row = 0b101100110;

        assert!(is_reflection_axis(row, axis, 9));
    }

    #[test]
    fn test_is_reflection_axis_returns_false_for_invalid_reflection() {
        let axis = 5;
        let row = 0b101100110;

        assert!(!is_reflection_axis(row, axis, 9));
    }

    #[test]
    fn test_build_binary_number() {
        let values = vec![false, true, true, false, false, true, false];
        let expected = 0b0110010;
        let actual = build_binary_number(values);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_encode_cells() {
        let values = vec![
            CellType::Ash,
            CellType::Rock,
            CellType::Rock,
            CellType::Ash,
            CellType::Rock,
        ];
        let expected = 0b01101;
        let actual = encode_cells(&values);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_encode_rows() {
        let cells = vec![
            vec![CellType::Ash, CellType::Rock, CellType::Rock],
            vec![CellType::Rock, CellType::Ash, CellType::Rock],
            vec![CellType::Ash, CellType::Rock, CellType::Ash],
        ];
        let expected = vec![0b011, 0b101, 0b010];
        let pattern = Pattern::new(cells);

        assert_eq!(expected, pattern.encoded_rows);
    }

    #[test]
    fn test_encode_columns() {
        let cells = vec![
            vec![CellType::Ash, CellType::Rock, CellType::Rock],
            vec![CellType::Rock, CellType::Ash, CellType::Rock],
            vec![CellType::Ash, CellType::Rock, CellType::Ash],
        ];
        let expected = vec![0b010, 0b101, 0b110];
        let pattern = Pattern::new(cells);

        assert_eq!(expected, pattern.encoded_cols);
    }

    #[test]
    fn test_get_reflection_axis_returns_some_for_correct_reflection() {
        /*
           #.##..##.
           ..#.##.#.
           ##......#
           ##......#
           ..#.##.#.
           ..##..##.
           #.#.##.#.
        */
        let cells = vec![
            vec![
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
                CellType::Ash,
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
            ],
            vec![
                CellType::Ash,
                CellType::Ash,
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Ash,
            ],
            vec![
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Rock,
            ],
            vec![
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Ash,
                CellType::Rock,
            ],
            vec![
                CellType::Ash,
                CellType::Ash,
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Ash,
            ],
            vec![
                CellType::Ash,
                CellType::Ash,
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
                CellType::Ash,
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
            ],
            vec![
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Rock,
                CellType::Ash,
                CellType::Rock,
                CellType::Ash,
            ],
        ];
        let pattern = Pattern::new(cells);
        let actual = pattern.get_reflection_axis();

        assert!(actual.is_some());
        assert_eq!(ReflectionAxis::Vertical(4), actual.unwrap());
    }

    #[test]
    fn test_calculate_columns_left_or_above() {
        let pattern = Pattern::new(vec![
            vec![CellType::Ash, CellType::Rock, CellType::Rock],
            vec![CellType::Rock, CellType::Ash, CellType::Rock],
            vec![CellType::Ash, CellType::Rock, CellType::Ash],
        ]);

        assert_eq!(
            Summary::Left(2),
            pattern.get_columns_left_or_above(ReflectionAxis::Vertical(1))
        );
        assert_eq!(
            Summary::Above(1),
            pattern.get_columns_left_or_above(ReflectionAxis::Horizontal(2))
        );
        assert_eq!(
            Summary::Left(1),
            pattern.get_columns_left_or_above(ReflectionAxis::Vertical(2))
        );
        assert_eq!(
            Summary::Above(2),
            pattern.get_columns_left_or_above(ReflectionAxis::Horizontal(1))
        );
    }

    #[test]
    fn test_calculate_pattern_summary() {
        let input = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        "};
        let actual = calculate_pattern_summary(input).unwrap();
        let expected = 405;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_string_to_pattern_to_axis_extreme_left() {
        let input = indoc! {"
        ..####.##
        ....#.#..
        ..#......
        ....#..#.
        ####..#..
        ###....#.
        ###..##.#
        ###...#..
        ###.#...#
        ###.#...#
        ###.#.#..
        ###..##.#
        ###....#.
        ####..#..
        ....#..#.
        "};

        let pattern = Pattern::from_str(input).unwrap();
        let actual = pattern.get_reflection_axis();
        let expected = Some(ReflectionAxis::Vertical(8));
        let expected_summary = Summary::Left(1);

        assert_eq!(expected, actual);
        assert_eq!(
            expected_summary,
            pattern.get_columns_left_or_above(actual.unwrap())
        );
    }

    #[test]
    fn test_string_to_pattern_to_axis_extreme_right() {
        let input = indoc! {"
        .##...#..#...##..
        ##..########..###
        ##..#..##..#..###
        ...##.#..#.##....
        .#...##..##...#..
        ..##........##...
        ##.###....###.###
        ..##...##...##...
        ######....##.####
        ...##.#..#.##....
        .....#....#......
        ...##......##....
        .......##........
        "};

        let pattern = Pattern::from_str(input).unwrap();
        let actual = pattern.get_reflection_axis();
        let expected = Some(ReflectionAxis::Vertical(1));
        let expected_summary = Summary::Left(16);
        assert_eq!(expected, actual);
        assert_eq!(
            expected_summary,
            pattern.get_columns_left_or_above(actual.unwrap())
        );
    }

    #[test]
    fn test_string_to_pattern_to_axis_extreme_top() {
        let input = indoc! {"
        #...#.##.
        #...#.##.
        ..#.#....
        ...#.....
        .##.##..#
        ##...####
        #.#.#####
        #.##.#...
        .###.....
        #.#.#####
        ###.##..#
        .#..##..#
        .#....##.
        "};

        let pattern = Pattern::from_str(input).unwrap();
        let actual = pattern.get_reflection_axis();
        let expected = Some(ReflectionAxis::Horizontal(12));
        let expected_summary = Summary::Above(1);

        assert_eq!(expected, actual);
        assert_eq!(
            expected_summary,
            pattern.get_columns_left_or_above(actual.unwrap())
        );
    }

    #[test]
    fn test_string_to_pattern_to_axis_extreme_bottom() {
        let input = indoc! {"
        .##...#.#...#
        #######..#.#.
        #..#.######.#
        ....###..##.#
        ......##.....
        #..##..#...##
        #..####.#....
        ###.#..##....
        ............#
        #..#..#####.#
        #..#..#####.#
        "};

        let pattern = Pattern::from_str(input).unwrap();
        let actual = pattern.get_reflection_axis();
        let expected = Some(ReflectionAxis::Horizontal(1));
        let expected_summary = Summary::Above(10);
        assert_eq!(expected, actual);
        assert_eq!(
            expected_summary,
            pattern.get_columns_left_or_above(actual.unwrap())
        );
    }

    #[test]
    fn test_split_pattern_at_bit_index() {
        let bits = 0b111000100;
        let expected = [
            (0, 0),           // 1
            (0b10, 0),        // 2
            (0, 0b100),       // 3
            (0b0011, 0b0100), // 4
            (0b0111, 0b10),   // 5
            (0b111, 0),       // 6
            (0b11, 0b10),     // 7
            (0b1, 0b1),       // 8
        ];

        for i in 1..9 {
            assert_eq!(
                expected[i - 1],
                split_and_mirror_binary_pattern_at_least_significant_bit_index(bits, 9, i)
            )
        }
    }
}
