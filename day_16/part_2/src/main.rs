use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Error, Ok, Result};
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    let result = solve_part(&input);
    println!("Result: {}", result);
}

fn solve_part(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Input should be valid");
    // All origins from left edge
    let left_edge = (0..=grid.extents.1)
        .map(|y| ((0, y), Origin::East))
        .collect::<Vec<_>>();
    // All origins from right edge
    let mut right_edge = (0..=grid.extents.1)
        .map(|y| ((grid.extents.0, y), Origin::West))
        .collect::<Vec<_>>();
    // All origins from top edge
    let mut top_edge = (0..=grid.extents.0)
        .map(|x| ((x, grid.extents.1), Origin::South))
        .collect::<Vec<_>>();
    // All origins from bottom edge
    let mut bottom_edge = (0..=grid.extents.0)
        .map(|x| ((x, 0), Origin::North))
        .collect::<Vec<_>>();
    // Combined edges
    let mut edges = left_edge;
    edges.append(&mut right_edge);
    edges.append(&mut top_edge);
    edges.append(&mut bottom_edge);

    edges
        .par_iter()
        .map(|(start, origin)| get_energized_tiles(&grid, *start, *origin).len())
        .max()
        .unwrap()
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Tile::Empty),
            '/' => Ok(Tile::MirrorForward),
            '\\' => Ok(Tile::MirrorBackward),
            '-' => Ok(Tile::SplitterHorizontal),
            '|' => Ok(Tile::SplitterVertical),
            _ => Err(anyhow!("Failed to parse Tile from char")),
        }
    }
}

fn get_energized_tiles(
    grid: &Grid,
    start: (usize, usize),
    origin: Origin,
) -> HashSet<(usize, usize)> {
    let mut energized_tiles = HashSet::new();
    let mut seen_moves = HashSet::new();

    let mut moves = Vec::new();
    moves.push((start, origin));
    seen_moves.insert((start, origin));

    while let Some((current, origin)) = moves.pop() {
        energized_tiles.insert(current);
        let (move_1, move_2) = grid.next_steps(current, origin);
        if let Some((next, origin)) = move_1 {
            if !seen_moves.contains(&(next, origin)) {
                moves.push((next, origin));
                seen_moves.insert((next, origin));
            }
        }
        if let Some((next, origin)) = move_2 {
            if !seen_moves.contains(&(next, origin)) {
                moves.push((next, origin));
                seen_moves.insert((next, origin));
            }
        }
    }
    energized_tiles
}

struct Grid {
    grid: HashMap<(usize, usize), Tile>,
    extents: (usize, usize),
}

impl Grid {
    fn is_in_bounds(
        &self,
        previous: (usize, usize),
        origin: Origin,
    ) -> Option<((usize, usize), Origin)> {
        match origin {
            Origin::North => {
                if previous.1 < self.extents.1 {
                    Some(((previous.0, previous.1 + 1), origin))
                } else {
                    None
                }
            }
            Origin::West => {
                if previous.0 < self.extents.0 {
                    Some(((previous.0 + 1, previous.1), origin))
                } else {
                    None
                }
            }
            Origin::South => previous.1.checked_sub(1).map(|y| ((previous.0, y), origin)),
            Origin::East => previous.0.checked_sub(1).map(|x| ((x, previous.1), origin)),
        }
    }
    fn next_steps(
        &self,
        current: (usize, usize),
        origin: Origin,
    ) -> (
        Option<((usize, usize), Origin)>,
        Option<((usize, usize), Origin)>,
    ) {
        let continue_in_direction = |direction| self.is_in_bounds(current, direction);
        if let Some(tile) = self.grid.get(&current) {
            match tile {
                Tile::Empty => (continue_in_direction(origin), None),
                Tile::MirrorForward => match origin {
                    Origin::North => (continue_in_direction(Origin::East), None),
                    Origin::East => (continue_in_direction(Origin::North), None),
                    Origin::South => (continue_in_direction(Origin::West), None),
                    Origin::West => (continue_in_direction(Origin::South), None),
                },
                Tile::MirrorBackward => match origin {
                    Origin::North => (continue_in_direction(Origin::West), None),
                    Origin::East => (continue_in_direction(Origin::South), None),
                    Origin::South => (continue_in_direction(Origin::East), None),
                    Origin::West => (continue_in_direction(Origin::North), None),
                },
                Tile::SplitterHorizontal => match origin {
                    Origin::North | Origin::South => (
                        continue_in_direction(Origin::East),
                        continue_in_direction(Origin::West),
                    ),
                    Origin::East | Origin::West => (continue_in_direction(origin), None),
                },
                Tile::SplitterVertical => match origin {
                    Origin::East | Origin::West => (
                        continue_in_direction(Origin::North),
                        continue_in_direction(Origin::South),
                    ),
                    Origin::North | Origin::South => (continue_in_direction(origin), None),
                },
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Origin {
    North,
    East,
    South,
    West,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut grid = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, symbol) in line.char_indices() {
                max_x = max_x.max(x);
                let tile = Tile::try_from(symbol)?;
                grid.insert((x, y), tile);
            }
            max_y = max_y.max(y);
        }

        let extents = (max_x, max_y);

        Ok(Grid { grid, extents })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve_part() {
        let input = indoc! {r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
        "#};
        let grid = input.parse::<Grid>().expect("Test Input should be valid");
        assert_eq!(grid.extents, (9, 9));

        assert_eq!(solve_part(input), 51);
    }

    #[test]
    fn test_energized_tiles() {
        // We need to manually escape the backslash in indoc
        let input = indoc! {"
        .|-
        /|/
        /-/
        "};
        let grid = input.parse::<Grid>().expect("Test Input should be valid");
        let energized_tiles = get_energized_tiles(&grid, (0, 0), Origin::West);
        let expected_count = 7;
        assert_eq!(energized_tiles.len(), expected_count);
    }

    #[test]
    fn test_parse_tile_from_char() {
        assert_eq!(Tile::try_from('.').unwrap(), Tile::Empty);
        assert_eq!(Tile::try_from('/').unwrap(), Tile::MirrorForward);
        assert_eq!(Tile::try_from('\\').unwrap(), Tile::MirrorBackward);
        assert_eq!(Tile::try_from('-').unwrap(), Tile::SplitterHorizontal);
        assert_eq!(Tile::try_from('|').unwrap(), Tile::SplitterVertical);
    }

    #[test]
    fn test_parse_tile_from_char_returns_error_invalid_characters() {
        assert!(Tile::try_from('d').is_err());
    }

    #[test]
    fn test_parse_grid_from_string() {
        // We need to manually escape the backslash in indoc
        let input = indoc! {"
        .|/
        -..
        |.\\
        "};
        let actual = input.parse::<Grid>().expect("Test Input should be valid");
        let expected: HashMap<(usize, usize), Tile> = vec![
            ((0, 0), Tile::Empty),
            ((1, 0), Tile::SplitterVertical),
            ((2, 0), Tile::MirrorForward),
            ((0, 1), Tile::SplitterHorizontal),
            ((1, 1), Tile::Empty),
            ((2, 1), Tile::Empty),
            ((0, 2), Tile::SplitterVertical),
            ((1, 2), Tile::Empty),
            ((2, 2), Tile::MirrorBackward),
        ]
        .into_iter()
        .collect();
        let expected_extents = (2, 2);
        assert_eq!(actual.grid.len(), expected.len());
        assert_eq!(actual.extents, expected_extents);
        assert_eq!(actual.grid, expected);
    }
}
