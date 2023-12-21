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
            let axis = pat
                .get_smudged_reflection_axis()
                .expect("No reflection axis found");
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

    fn get_smudged_reflection_axis(&self) -> Option<ReflectionAxis> {
        // We can use these local copies to perform some quick iteration
        let mut rows = self.encoded_rows.clone();
        let mut cols = self.encoded_cols.clone();

        let original_axis = Self::try_get_reflection_axis(&rows, &cols, self.dimensions, None);

        let mut col_mask = 1 << self.dimensions.1;

        for y in 0..self.encoded_rows.len() {
            col_mask >>= 1;
            let mut row_mask = 1 << self.dimensions.0;

            for x in 0..self.encoded_cols.len() {
                row_mask >>= 1;

                rows[y] ^= row_mask;
                cols[x] ^= col_mask;

                let result =
                    Self::try_get_reflection_axis(&rows, &cols, self.dimensions, original_axis);

                if result.is_some() && result != original_axis {
                    return result;
                }

                rows[y] ^= row_mask;
                cols[x] ^= col_mask;
            }
        }

        None
    }

    fn try_get_reflection_axis(
        rows: &[usize],
        cols: &[usize],
        dimensions: (usize, usize),
        ignored_axis: Option<ReflectionAxis>,
    ) -> Option<ReflectionAxis> {
        let max_dim = dimensions.0.max(dimensions.1);

        for index in 1..max_dim {
            if index < dimensions.0
                && rows
                    .iter()
                    .all(|&row| is_reflection_axis(row, index, dimensions.0))
            {
                let potential_axis = Some(ReflectionAxis::Vertical(index));
                if potential_axis != ignored_axis {
                    return potential_axis;
                }
            }

            if index < dimensions.1
                && cols
                    .iter()
                    .all(|&col| is_reflection_axis(col, index, dimensions.1))
            {
                let potential_axis = Some(ReflectionAxis::Horizontal(index));
                if potential_axis != ignored_axis {
                    return potential_axis;
                }
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
    let (left, right) = split_and_mirror_pattern(encoded_line, line_size, axis_index);

    left == right
}

fn split_and_mirror_pattern(
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
    fn test_get_axis_ignores_original_axis() {
        let input = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        "};
        let pattern = Pattern::from_str(input).unwrap();
        let actual = pattern.get_smudged_reflection_axis();
        let expected = Some(ReflectionAxis::Horizontal(4));

        assert_eq!(expected, actual);
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
        let expected = 400;
        assert_eq!(expected, actual);
    }
}
