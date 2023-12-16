use anyhow::{Context, Result};

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    let map = parse_input(&input).expect("Unable to parse input");
    let culmative_distance = cumalative_distance_for_pairs(map);
    println!("Culmative distance: {}", culmative_distance);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn get_manhattan_distance(&self, other: &Coordinate) -> i32 {
        let x = (self.0 - other.0).abs();
        let y = (self.1 - other.1).abs();
        x + y
    }
}

fn cumalative_distance_for_pairs(points: Vec<Coordinate>) -> i32 {
    let mut total_distance = 0;
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            total_distance += points[i].get_manhattan_distance(&points[j]);
        }
    }
    total_distance
}

// Could be optimised by tracking empty rows and columns during parsing
fn parse_input(input: &str) -> Result<Vec<Coordinate>> {
    let mut galaxies: Vec<Coordinate> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| match c {
                '#' => Some(Coordinate(x as i32, y as i32)),
                _ => None,
            })
        })
        .collect();

    let mut max_x = galaxies.iter().map(|c| c.0).max().context("No max x")?;
    let mut max_y = galaxies.iter().map(|c| c.1).max().context("No max y")?;

    let mut y = 0;
    // Expand the map by one row after each empty row
    while y <= max_y {
        // Check if there are any galaxies on this row
        if !galaxies.iter().any(|c| c.1 == y) {
            // No galaxies on this row, so add one to the y coordinate of all galaxies above this row
            galaxies
                .iter_mut()
                .filter(|c| c.1 > y)
                .for_each(|c| c.1 += 1);
            max_y += 1;
            // Skip the next row to avoid infinite expansion
            y += 2;
        } else {
            y += 1;
        }
    }

    let mut x = 0;
    // Expand the map by one column after each empty column
    while x <= max_x {
        // Check if there are any galaxies in this column
        if !galaxies.iter().any(|c| c.0 == x) {
            // No galaxies in this column, so add one to the x coordinate of all galaxies above this column
            galaxies
                .iter_mut()
                .filter(|c| c.0 > x)
                .for_each(|c| c.0 += 1);
            max_x += 1;
            // Skip the next row to avoid infinite expansion
            x += 2;
        } else {
            x += 1;
        }
    }

    Ok(galaxies)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    #[test]
    fn test_get_manhattan_distance_same_position_should_equal_zero() {
        let start = Coordinate(0, 0);
        let end = Coordinate(0, 0);
        let expected = 0;

        assert_eq!(expected, start.get_manhattan_distance(&end));
    }

    #[test]
    fn test_get_manhattan_distance_positive_start() {
        let start = Coordinate(0, 0);
        let end = Coordinate(1, 1);
        let expected = 2;

        assert_eq!(expected, start.get_manhattan_distance(&end));
    }

    #[test]
    fn test_get_manhattan_distance_negative_start() {
        let start = Coordinate(-1, -1);
        let end = Coordinate(2, 2);
        let expected = 6;

        assert_eq!(expected, start.get_manhattan_distance(&end));
    }

    #[test]
    fn test_cumalative_distance_for_pairs() {
        let points = vec![
            Coordinate(4, 0),
            Coordinate(9, 1),
            Coordinate(0, 2),
            Coordinate(8, 5),
            Coordinate(1, 6),
            Coordinate(12, 7),
            Coordinate(9, 10),
            Coordinate(0, 11),
            Coordinate(5, 11),
        ];
        let expected = 374;

        assert_eq!(expected, cumalative_distance_for_pairs(points));
    }

    #[test]
    fn test_parse_input() {
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

        let expected = vec![
            Coordinate(4, 0),
            Coordinate(9, 1),
            Coordinate(0, 2),
            Coordinate(8, 5),
            Coordinate(1, 6),
            Coordinate(12, 7),
            Coordinate(9, 10),
            Coordinate(0, 11),
            Coordinate(5, 11),
        ];

        assert_eq!(expected, parse_input(input).unwrap());
    }

    #[test]
    fn test_parse_to_culumative_distance() {
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

        let expected = 374;

        let input = parse_input(input).unwrap();
        let actual = cumalative_distance_for_pairs(input);

        assert_eq!(expected, actual);
    }
}
