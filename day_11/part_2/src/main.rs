use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    let mut map = Map::parse_input(&input);
    map.set_expansion_factor(1_000_000);
    let culmative_distance = map.cumulative_distance_for_pairs();
    println!("Culmative distance: {}", culmative_distance);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate(i64, i64);

impl Coordinate {
    fn get_manhattan_distance_with_expansion_factor(
        &self,
        other: &Coordinate,
        empty_rows: &HashSet<i64>,
        empty_cols: &HashSet<i64>,
        expansion_factor: i64,
    ) -> i64 {
        let x = (self.0 - other.0).abs();
        let y = (self.1 - other.1).abs();

        let empty_row_crosses = (self.1.min(other.1)..=self.1.max(other.1))
            .filter(|r| empty_rows.contains(r))
            .count() as i64;
        let empty_col_crosses = (self.0.min(other.0)..=self.0.max(other.0))
            .filter(|c| empty_cols.contains(c))
            .count() as i64;
        let total_crosses = empty_row_crosses + empty_col_crosses;

        // We need to subtract the number of crossings so that they aren't counted twice
        ((x + y) - total_crosses) + expansion_factor * total_crosses
    }
}

struct Map {
    galaxies: Vec<Coordinate>,
    empty_rows: HashSet<i64>,
    empty_columns: HashSet<i64>,
    expansion_factor: i64,
}

impl Map {
    fn parse_input(input: &str) -> Map {
        let mut galaxies: Vec<Coordinate> = Vec::new();
        let mut filled_rows: HashSet<i64> = HashSet::new();
        let mut filled_columns: HashSet<i64> = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in input.lines().enumerate() {
            let y = y as i64;
            for (x, c) in line.char_indices() {
                let x = x as i64;
                if let '#' = c {
                    galaxies.push(Coordinate(x, y));
                    filled_rows.insert(y);
                    filled_columns.insert(x);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }

        let empty_columns = (0..=max_x)
            .filter(|i| !filled_columns.contains(i))
            .collect();

        let empty_rows = (0..=max_y).filter(|i| !filled_rows.contains(i)).collect();

        Map {
            galaxies,
            empty_rows,
            empty_columns,
            expansion_factor: 1,
        }
    }

    fn set_expansion_factor(&mut self, factor: i64) {
        self.expansion_factor = factor;
    }

    // Original Implementation
    // fn cumulative_distance_for_pairs(&self) -> i64 {
    //     let mut total_distance = 0;
    //     for i in 0..self.galaxies.len() - 1 {
    //         for j in i + 1..self.galaxies.len() {
    //             total_distance += self.galaxies[i].get_manhattan_distance_with_expansion_factor(
    //                 &self.galaxies[j],
    //                 &self.empty_rows,
    //                 &self.empty_columns,
    //                 self.expansion_factor,
    //             );
    //         }
    //     }
    //     total_distance
    // }

    // Functional Implementation with Rayon
    fn cumulative_distance_for_pairs(&self) -> i64 {
        let galaxies = &self.galaxies;
        let empty_rows = &self.empty_rows;
        let empty_cols = &self.empty_columns;
        let expansion_factor = self.expansion_factor;

        (0..galaxies.len() - 1)
            .into_par_iter()
            .map_init(
                || galaxies,
                |galaxies, i| {
                    (i + 1..galaxies.len())
                        .map(|j| {
                            galaxies[i].get_manhattan_distance_with_expansion_factor(
                                &galaxies[j],
                                empty_rows,
                                empty_cols,
                                expansion_factor,
                            )
                        })
                        .sum::<i64>()
                },
            )
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_to_map() {
        let input = indoc!(
            "
            #....
            .....
            ..#.#
            .....
            ....#
            "
        );

        let expected_coordinates = vec![
            Coordinate(0, 0),
            Coordinate(2, 2),
            Coordinate(4, 2),
            Coordinate(4, 4),
        ];
        let expected_empty_rows: HashSet<i64> = vec![1, 3].into_iter().collect();
        let expected_empty_columns: HashSet<i64> = vec![1, 3].into_iter().collect();
        let expected_expansion_factor = 1;

        let actual = Map::parse_input(input);
        let actual_coordinates = actual.galaxies;
        let actual_empty_rows = actual.empty_rows;
        let actual_empty_columns = actual.empty_columns;
        let actual_expansion_factor = actual.expansion_factor;

        assert_eq!(actual_coordinates, expected_coordinates);
        assert_eq!(actual_empty_rows, expected_empty_rows);
        assert_eq!(actual_empty_columns, expected_empty_columns);
        assert_eq!(actual_expansion_factor, expected_expansion_factor);
    }

    #[test]
    fn test_get_manhattan_distance_with_expansion_factor_no_crossings() {
        let pair = (Coordinate(0, 0), Coordinate(1, 0));
        let empty_cols: HashSet<i64> = HashSet::new();
        let empty_rows: HashSet<i64> = HashSet::new();
        let expansion_factor = 1;

        let expected = 1;
        let actual = pair.0.get_manhattan_distance_with_expansion_factor(
            &pair.1,
            &empty_cols,
            &empty_rows,
            expansion_factor,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_manhattan_distance_with_triple_expansion_factor_two_crossings() {
        let pair = (Coordinate(0, 0), Coordinate(2, 2));
        let empty_cols: HashSet<i64> = vec![1].into_iter().collect();
        let empty_rows: HashSet<i64> = vec![1].into_iter().collect();
        let expansion_factor = 3;

        let expected = 8;
        let actual = pair.0.get_manhattan_distance_with_expansion_factor(
            &pair.1,
            &empty_rows,
            &empty_cols,
            expansion_factor,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_complex_input() {
        let input = indoc!(
            "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            "
        );

        let expected_coordinates = vec![
            Coordinate(3, 0),
            Coordinate(7, 1),
            Coordinate(0, 2),
            Coordinate(6, 4),
            Coordinate(1, 5),
            Coordinate(9, 6),
            Coordinate(7, 8),
            Coordinate(0, 9),
            Coordinate(4, 9),
        ];
        let expected_empty_rows: HashSet<i64> = vec![3, 7].into_iter().collect();
        let expected_empty_columns: HashSet<i64> = vec![2, 5, 8].into_iter().collect();
        let expected_expansion_factor = 1;

        let actual = Map::parse_input(input);
        let actual_coordinates = actual.galaxies;
        let actual_empty_rows = actual.empty_rows;
        let actual_empty_columns = actual.empty_columns;
        let actual_expansion_factor = actual.expansion_factor;

        assert_eq!(actual_coordinates, expected_coordinates);
        assert_eq!(actual_empty_rows, expected_empty_rows);
        assert_eq!(actual_empty_columns, expected_empty_columns);
        assert_eq!(actual_expansion_factor, expected_expansion_factor);
    }

    #[test]
    fn test_parse_complex_input_and_find_pairs() {
        let input = indoc!(
            "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            "
        );

        let expected = 1030;
        let mut input = Map::parse_input(input);
        input.set_expansion_factor(10);
        let actual = input.cumulative_distance_for_pairs();
        assert_eq!(expected, actual);

        let expected = 8410;
        input.set_expansion_factor(100);
        let actual = input.cumulative_distance_for_pairs();
        assert_eq!(expected, actual);
    }
}
