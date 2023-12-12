use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::ops::{Add, Sub};

use anyhow::{Context, Result};
use bitflags::bitflags;
use colored::*;

fn main() {
    let file =
        std::fs::read_to_string("puzzle_input.txt").expect("Failed to read puzzle_input.txt");
    let map = Map::try_from(file.as_str()).expect("Failed to convert puzzle input to map");
    let loop_tiles = depth_first_search(&map).expect("Failed to depth first search");
    let enclosed_tiles = map
        .get_enclosed_tiles(&loop_tiles)
        .expect("Failed to get enclosed tiles");
    pretty_print_map(map, loop_tiles.clone(), enclosed_tiles.clone());
    //println!("Number of enclosed tiles: {}", enclosed_tiles.len());
}

fn pretty_print_map(
    map: Map,
    loop_coordinate: HashSet<Coordinates>,
    enclosed_coordinates: HashSet<Coordinates>,
) {
    let max_x = map.tiles.keys().map(|Coordinates(x, _)| x).max().unwrap();
    let max_y = map.tiles.keys().map(|Coordinates(_, y)| y).max().unwrap();
    dbg!(map.start);
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            let tile = Coordinates(x, y);
            let is_start = map.start == tile;
            let tile_state = map.tiles.get(&tile).unwrap();
            let is_loop_tile = loop_coordinate.contains(&tile);
            let is_enclosed_tile = enclosed_coordinates.contains(&tile);

            let symbol = tile_state.get_pretty_symbol();
            let colorized = if is_start {
                symbol.bright_green().blink()
            } else if is_loop_tile {
                symbol.bright_red()
            } else if is_enclosed_tile {
                symbol.bright_blue()
            } else {
                symbol.dimmed()
            };

            print!("{}", colorized);
        }
        println!();
    }
}

fn depth_first_search(map: &Map) -> Result<HashSet<Coordinates>> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    let start = map.start;

    stack.push(start);

    while let Some(tile) = stack.pop() {
        if !visited.insert(tile) {
            continue;
        }

        let neighbours = map.get_neighbours(tile)?;

        for neighbour in neighbours {
            stack.push(neighbour);
        }
    }

    Ok(visited)
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct TileState: i64 {
        const NONE = 0b00000000;
        const UP = 0b00000001;
        const DOWN = 0b00000010;
        const LEFT = 0b00000100;
        const RIGHT = 0b00001000;
        const START = 0b00010000;
        const UP_DOWN = Self::UP.bits() | Self::DOWN.bits();
        const LEFT_RIGHT = Self::LEFT.bits() | Self::RIGHT.bits();
        const UP_LEFT = Self::UP.bits() | Self::LEFT.bits();
        const UP_RIGHT = Self::UP.bits() | Self::RIGHT.bits();
        const DOWN_LEFT = Self::DOWN.bits() | Self::LEFT.bits();
        const DOWN_RIGHT = Self::DOWN.bits() | Self::RIGHT.bits();
    }
}

impl TileState {
    fn is_corner(&self) -> bool {
        self.contains(Self::UP_LEFT)
            || self.contains(Self::UP_RIGHT)
            || self.contains(Self::DOWN_LEFT)
            || self.contains(Self::DOWN_RIGHT)
    }

    fn are_opposite_vertical_directions(&self, other: &Self) -> bool {
        let vertical_directions = Self::UP | Self::DOWN;
        let self_vertical_direction = *self & vertical_directions;
        let other_vertical_direction = *other & vertical_directions;

        matches!(
            (self_vertical_direction, other_vertical_direction),
            (TileState::UP, TileState::DOWN) | (TileState::DOWN, TileState::UP)
        )
    }

    fn get_pretty_symbol(&self) -> &'static str {
        let cleaned = *self & !Self::START;
        match cleaned {
            Self::NONE => "░",
            Self::UP_DOWN => "║",
            Self::LEFT_RIGHT => "═",
            Self::UP_LEFT => "╝",
            Self::UP_RIGHT => "╚",
            Self::DOWN_LEFT => "╗",
            Self::DOWN_RIGHT => "╔",
            _ => " ",
        }
    }
}

impl TryFrom<char> for TileState {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::NONE),
            'S' => Ok(Self::START),
            '|' => Ok(Self::UP | Self::DOWN),
            '-' => Ok(Self::LEFT | Self::RIGHT),
            'L' => Ok(Self::UP | Self::RIGHT),
            'J' => Ok(Self::UP | Self::LEFT),
            '7' => Ok(Self::DOWN | Self::LEFT),
            'F' => Ok(Self::DOWN | Self::RIGHT),
            _ => Err("Invalid character"),
        }
    }
}

// Starting at the top left corner x is zero and counts up as you move right
// Starting at the top left corner y is zero and counts up as you move down
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinates(i64, i64);
impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct Map {
    tiles: HashMap<Coordinates, TileState>,
    start: Coordinates,
}

impl TryFrom<&str> for Map {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();
        let mut start = None;

        for (y, line) in value.lines().enumerate() {
            for (x, symbol) in line.char_indices() {
                let tile_state;
                if symbol == 'S' {
                    start = Some(Coordinates(x as i64, y as i64));
                    tile_state = TileState::START;
                } else {
                    tile_state = TileState::try_from(symbol).map_err(|_| {
                        format!(
                            "Failed to convert {} to TileState at x:{}, y:{}",
                            symbol, x, y
                        )
                    })?;
                }
                tiles.insert(Coordinates(x as i64, y as i64), tile_state);
            }
        }

        let mut map = Map {
            tiles,
            start: match start {
                Some(start) => start,
                None => return Err("No start found".to_string()),
            },
        };

        map.calculate_start_neighbours();

        Ok(map)
    }
}

impl Map {
    fn calculate_start_neighbours(&mut self) {
        let start = self.start;

        let directions = [
            (Coordinates(0, 1), TileState::UP, TileState::DOWN),
            (Coordinates(0, -1), TileState::DOWN, TileState::UP),
            (Coordinates(1, 0), TileState::LEFT, TileState::RIGHT),
            (Coordinates(-1, 0), TileState::RIGHT, TileState::LEFT),
        ];

        for (direction, valid_incoming, outgoing_state) in directions.iter() {
            let neighbour = start + *direction;
            if let Some(neighbour_tile_state) = self.tiles.get(&neighbour) {
                if neighbour_tile_state.contains(*valid_incoming) {
                    if let Some(start_tile_state) = self.tiles.get_mut(&start) {
                        *start_tile_state |= *outgoing_state;
                    }
                }
            }
        }
    }

    fn get_neighbours(&self, tile: Coordinates) -> Result<impl Iterator<Item = Coordinates> + '_> {
        let tile_state = self.tiles.get(&tile).context("Tile not found")?;

        let directions = [
            (TileState::UP, Coordinates(0, -1)),
            (TileState::DOWN, Coordinates(0, 1)),
            (TileState::LEFT, Coordinates(-1, 0)),
            (TileState::RIGHT, Coordinates(1, 0)),
        ];

        Ok(directions
            .into_iter()
            .filter_map(move |(state, direction)| {
                if tile_state.contains(state) {
                    Some(tile + direction)
                } else {
                    None
                }
            }))
    }

    fn get_enclosed_tiles(
        &self,
        loop_tiles: &HashSet<Coordinates>,
    ) -> Result<HashSet<Coordinates>> {
        let max_x = self.tiles.keys().map(|Coordinates(x, _)| x).max().unwrap();
        let max_y = self.tiles.keys().map(|Coordinates(_, y)| y).max().unwrap();
        let mut enclosed_tiles = HashSet::new();

        for y in 0..=*max_y {
            let mut edges_crossed = 0;
            let mut last_corner: Option<TileState> = None;
            for x in 0..=*max_x {
                let tile = Coordinates(x, y);
                let is_loop_tile = loop_tiles.contains(&tile);
                let current_tile_state = self
                    .tiles
                    .get(&tile)
                    .context("Tile should be in tiles map")?;
                let current_tile_is_corner = current_tile_state.is_corner();
                let is_inside_loop = edges_crossed % 2 == 1;
                match (
                    is_loop_tile,
                    current_tile_is_corner,
                    last_corner,
                    is_inside_loop,
                ) {
                    // Loop Tile, First Corner
                    (true, true, None, _) => {
                        last_corner = Some(*current_tile_state);
                    }
                    // Loop Tile, Second Corner
                    (true, true, Some(last_corner_state), _) => {
                        if current_tile_state.are_opposite_vertical_directions(&last_corner_state) {
                            edges_crossed += 1;
                        }
                        last_corner = None;
                    }
                    // Loop Tile, Parallel to scan line
                    (true, false, Some(_), _) => {}
                    // Loop Tile, Not parallel to scan line
                    (true, false, None, _) => {
                        edges_crossed += 1;
                    }
                    // Non-Loop Tile inside loop
                    (false, _, _, true) => {
                        enclosed_tiles.insert(Coordinates(x, y));
                    }
                    // Non-Loop Tile outside loop
                    (false, _, _, false) => {}
                }
            }
        }

        Ok(enclosed_tiles)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_tile_state_from_char_valid() {
        assert_eq!(TileState::try_from('.').unwrap(), TileState::NONE);
        assert_eq!(TileState::try_from('S').unwrap(), TileState::START);
        assert_eq!(
            TileState::try_from('|').unwrap(),
            TileState::UP | TileState::DOWN
        );
        assert_eq!(
            TileState::try_from('-').unwrap(),
            TileState::LEFT | TileState::RIGHT
        );
        assert_eq!(
            TileState::try_from('L').unwrap(),
            TileState::UP | TileState::RIGHT
        );
        assert_eq!(
            TileState::try_from('J').unwrap(),
            TileState::UP | TileState::LEFT
        );
        assert_eq!(
            TileState::try_from('7').unwrap(),
            TileState::DOWN | TileState::LEFT
        );
        assert_eq!(
            TileState::try_from('F').unwrap(),
            TileState::DOWN | TileState::RIGHT
        );
    }

    #[test]
    fn test_tile_state_from_char_invalid_returns_error() {
        assert!(TileState::try_from('a').is_err());
    }

    #[test]
    fn test_map_from_str_valid() {
        let map = Map::try_from(indoc!(
            "
            S.|.|
            -LJ-F
            |..|.
            "
        ))
        .unwrap();

        assert_eq!(map.tiles.len(), 15);
        assert!(map.tiles[&Coordinates(0, 0)].contains(TileState::START));
        assert!(map.tiles[&Coordinates(1, 0)].contains(TileState::NONE));
        assert!(map.tiles[&Coordinates(2, 0)].contains(TileState::UP | TileState::DOWN));
        assert!(map.tiles[&Coordinates(3, 0)].contains(TileState::NONE));
        assert!(map.tiles[&Coordinates(4, 0)].contains(TileState::UP | TileState::DOWN));
        assert!(map.tiles[&Coordinates(0, 1)].contains(TileState::LEFT | TileState::RIGHT));
        assert!(map.tiles[&Coordinates(1, 1)].contains(TileState::UP | TileState::RIGHT));
        assert!(map.tiles[&Coordinates(2, 1)].contains(TileState::UP | TileState::LEFT));
        assert!(map.tiles[&Coordinates(3, 1)].contains(TileState::LEFT | TileState::RIGHT));
        assert!(map.tiles[&Coordinates(4, 1)].contains(TileState::DOWN | TileState::RIGHT));
        assert!(map.tiles[&Coordinates(0, 2)].contains(TileState::UP | TileState::DOWN));
        assert!(map.tiles[&Coordinates(1, 2)].contains(TileState::NONE));
        assert!(map.tiles[&Coordinates(2, 2)].contains(TileState::NONE));
        assert!(map.tiles[&Coordinates(3, 2)].contains(TileState::UP | TileState::DOWN));
        assert!(map.tiles[&Coordinates(4, 2)].contains(TileState::NONE));
    }

    #[test]
    fn test_map_from_str_invalid_returns_error() {
        let map = Map::try_from(indoc!(
            "
            S.|.|
            -LJ-F
            |..|.
            "
        ));

        assert!(map.is_ok());
    }

    #[test]
    fn test_calculate_starting_neighbours() {
        let map = Map::try_from(indoc!(
            "
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
            "
        ))
        .unwrap();

        assert!(map.tiles[&Coordinates(0, 2)].contains(TileState::START));
        assert!(map.tiles[&Coordinates(0, 2)].contains(TileState::RIGHT | TileState::DOWN));
    }

    #[test]
    fn test_get_most_distant_tile() {
        let map = Map::try_from(indoc!(
            "
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
            "
        ))
        .unwrap();

        let loop_tiles = dbg!(depth_first_search(&map).unwrap());

        assert_eq!(8, loop_tiles.len() / 2);
    }

    #[test]
    fn test_get_neighbours() {
        let map = Map::try_from(indoc!(
            "
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
            "
        ))
        .unwrap();

        let neighbours = map
            .get_neighbours(Coordinates(0, 2))
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&Coordinates(1, 2)));
        assert!(neighbours.contains(&Coordinates(0, 3)));
    }

    #[test]
    fn test_count_enclosed_tiles() {
        let map = Map::try_from(indoc!(
            "
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
            "
        ))
        .unwrap();

        let loop_tiles = depth_first_search(&map).unwrap();
        let enclosed_tiles = map.get_enclosed_tiles(&loop_tiles).unwrap();

        assert_eq!(4, enclosed_tiles.len());
    }

    #[test]
    fn test_tile_state_is_corner() {
        assert!(TileState::UP_LEFT.is_corner());
        assert!(TileState::UP_RIGHT.is_corner());
        assert!(TileState::DOWN_LEFT.is_corner());
        assert!(TileState::DOWN_RIGHT.is_corner());
        assert!(!TileState::UP.is_corner());
        assert!(!TileState::DOWN.is_corner());
        assert!(!TileState::LEFT.is_corner());
        assert!(!TileState::RIGHT.is_corner());
    }

    #[test]
    fn test_tile_state_are_opposite_vertical_directions() {
        assert!(TileState::UP.are_opposite_vertical_directions(&TileState::DOWN));
        assert!(TileState::DOWN.are_opposite_vertical_directions(&TileState::UP));
        assert!(!TileState::UP.are_opposite_vertical_directions(&TileState::UP));
        assert!(!TileState::DOWN.are_opposite_vertical_directions(&TileState::DOWN));
        assert!(!TileState::UP.are_opposite_vertical_directions(&TileState::LEFT));
        assert!(!TileState::UP.are_opposite_vertical_directions(&TileState::RIGHT));
        assert!(!TileState::DOWN.are_opposite_vertical_directions(&TileState::LEFT));
        assert!(!TileState::DOWN.are_opposite_vertical_directions(&TileState::RIGHT));
    }

    #[test]
    fn test_tile_state_are_opposite_vertical_directions_corners() {
        assert!(TileState::UP_LEFT.are_opposite_vertical_directions(&TileState::DOWN_RIGHT));
        assert!(TileState::UP_RIGHT.are_opposite_vertical_directions(&TileState::DOWN_LEFT));
        assert!(!TileState::DOWN_LEFT.are_opposite_vertical_directions(&TileState::DOWN_RIGHT));
        assert!(!TileState::UP_LEFT.are_opposite_vertical_directions(&TileState::UP_RIGHT));
    }

    #[test]
    fn get_pretty_symbol() {
        let start = TileState::START | TileState::UP | TileState::DOWN;
        assert_eq!(TileState::NONE.get_pretty_symbol(), "░");
        assert_eq!(TileState::UP_DOWN.get_pretty_symbol(), "║");
        assert_eq!(TileState::LEFT_RIGHT.get_pretty_symbol(), "═");
        assert_eq!(TileState::UP_LEFT.get_pretty_symbol(), "╝");
        assert_eq!(TileState::UP_RIGHT.get_pretty_symbol(), "╚");
        assert_eq!(TileState::DOWN_LEFT.get_pretty_symbol(), "╗");
        assert_eq!(TileState::DOWN_RIGHT.get_pretty_symbol(), "╔");
        assert_eq!(start.get_pretty_symbol(), "║");
    }
}
