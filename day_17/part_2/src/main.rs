use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    ops::Index,
    str::FromStr,
};

use anyhow::Result;
use priority_queue::PriorityQueue;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Failed to read input file");
    let grid = input.parse::<Grid>().expect("Input should be valid grid");
    let end = (grid.width - 1, grid.height - 1).into();
    let search = SearchParameters {
        start: (0, 0).into(),
        end,
        min_movement: 4,
        max_movement: 10,
    };
    let result = grid
        .find_lowest_heatloss(search)
        .expect("There should be a correct path");
    println!("The lowest heatloss is: {}", result);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Move {
    position: Coordinate,
    direction: Option<Direction>,
}

impl Move {
    fn new(position: Coordinate, direction: Option<Direction>) -> Self {
        Self {
            position,
            direction,
        }
    }
}

struct SearchParameters {
    start: Coordinate,
    end: Coordinate,
    min_movement: usize,
    max_movement: usize,
}

struct Grid {
    width: usize,
    height: usize,
    cells: HashMap<Coordinate, usize>,
}

impl Grid {
    // Dijkstras algorithm
    fn find_lowest_heatloss(&self, search: SearchParameters) -> Option<usize> {
        // HashSet of Moves
        let mut explored = HashSet::new();
        // PriorityQueue of Moves that need to be explored
        let mut frontier: PriorityQueue<Move, Reverse<usize>> = PriorityQueue::new();
        // Set up the first move and add it to the frontier
        let first_move = Move::new(search.start, None);
        frontier.push(first_move, Reverse(0));
        // Loop while there are possible moves in the frontier
        while let Some((m, Reverse(c))) = frontier.pop() {
            // Check to see if we have reached the goal
            if m.position == search.end {
                return Some(c);
            }
            // Otherwise we need to add this move to the explored set
            explored.insert(m);
            // Then get all the possible successor moves
            // Automatically excludes already explored moves
            let successors =
                self.successors(m, c, search.min_movement, search.max_movement, &explored);
            // Loop over every successor move
            for (suc_mov, suc_cost) in successors {
                // We try and increase the priority of whatever is already in the frontier
                // If the priority isn't an increase, then we discard the result
                frontier.push_increase(suc_mov, Reverse(suc_cost));
            }
        }

        // If there was no early exit then we can assume that no route was discovered
        // This shouldn't ever happen with our test data
        None
    }

    fn successors(
        &self,
        previous_move: Move,
        previous_move_cost: usize,
        min_movement: usize,
        max_movement: usize,
        explored: &HashSet<Move>,
    ) -> Vec<(Move, usize)> {
        let mut successors = Vec::new();
        // For tracking the increasing costs in each direction E W S N
        let mut culm_costs = (
            previous_move_cost,
            previous_move_cost,
            previous_move_cost,
            previous_move_cost,
        );

        // TODO: Increment the previous move costs but only return valid moves
        // Above the minimum move distance
        // We need to run the loop from 1 to the max movement depth
        for d in 1..=max_movement {
            // Handle Horizontal movements
            if previous_move.direction != Some(Direction::Horizontal) {
                // Positive Direction (East)
                if let Some(pos) =
                    self.bounded_add_coordinates(previous_move.position, Coordinate { x: d, y: 0 })
                {
                    // We need to update the culm score in this direction
                    culm_costs.0 += self[pos];
                    if d >= min_movement {
                        // Create a new move
                        let new_move = Move::new(pos, Some(Direction::Horizontal));
                        // Check if we have already explored from this direction
                        if !explored.contains(&new_move) {
                            successors.push((new_move, culm_costs.0))
                        }
                    }
                }
                // Negative Direction (West)
                if let Some(pos) =
                    self.bounded_sub_coordinates(previous_move.position, Coordinate { x: d, y: 0 })
                {
                    // We need to update the culm score in this direction
                    culm_costs.1 += self[pos];
                    if d >= min_movement {
                        // Create a new move
                        let new_move = Move::new(pos, Some(Direction::Horizontal));
                        // Check if we have already explored from this direction
                        if !explored.contains(&new_move) {
                            successors.push((new_move, culm_costs.1))
                        }
                    }
                }
            }
            if previous_move.direction != Some(Direction::Vertical) {
                // Positive Direction (South)
                if let Some(pos) =
                    self.bounded_add_coordinates(previous_move.position, Coordinate { x: 0, y: d })
                {
                    culm_costs.2 += self[pos];
                    if d >= min_movement {
                        // Create a new move
                        let new_move = Move::new(pos, Some(Direction::Vertical));
                        // Check if we have already explored from this direction
                        if !explored.contains(&new_move) {
                            successors.push((new_move, culm_costs.2))
                        }
                    }
                }
                // Negative Direction (North)
                if let Some(pos) =
                    self.bounded_sub_coordinates(previous_move.position, Coordinate { x: 0, y: d })
                {
                    culm_costs.3 += self[pos];
                    if d >= min_movement {
                        // Create a new move
                        let new_move = Move::new(pos, Some(Direction::Vertical));
                        // Check if we have already explored from this direction
                        if !explored.contains(&new_move) {
                            successors.push((new_move, culm_costs.3))
                        }
                    }
                }
            }
        }

        successors
    }

    fn bounded_add_coordinates(&self, a: Coordinate, b: Coordinate) -> Option<Coordinate> {
        if a.x + b.x < self.width && a.y + b.y < self.height {
            Some((a.x + b.x, a.y + b.y).into())
        } else {
            None
        }
    }

    fn bounded_sub_coordinates(&self, a: Coordinate, b: Coordinate) -> Option<Coordinate> {
        let x = a.x.checked_sub(b.x);
        let y = a.y.checked_sub(b.y);
        if let (Some(x), Some(y)) = (x, y) {
            Some((x, y).into())
        } else {
            None
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Grid> {
        let mut width = 0;
        let mut height = 0;
        let mut cells = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                // Should be highest on the last character
                width = x;
                let cell = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Invalid digit in input"))?;
                cells.insert(Coordinate { x, y }, cell as usize);
            }
            // Should be highest on the last line
            height = y;
        }

        Ok(Grid {
            width: width + 1,
            height: height + 1,
            cells,
        })
    }
}

impl Index<Coordinate> for Grid {
    type Output = usize;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.cells[&index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_grid() {
        let input = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
        "};

        let grid = input.parse::<Grid>().expect("Input should be valid grid");
        assert_eq!(grid.width, 13);
        assert_eq!(grid.height, 13);
        assert_eq!(grid.cells.len(), 13 * 13);
        // First row
        assert_eq!(grid[(0, 0).into()], 2);
        assert_eq!(grid[(1, 0).into()], 4);
        assert_eq!(grid[(2, 0).into()], 1);
        assert_eq!(grid[(3, 0).into()], 3);
        assert_eq!(grid[(4, 0).into()], 4);
        assert_eq!(grid[(5, 0).into()], 3);
        assert_eq!(grid[(6, 0).into()], 2);
        assert_eq!(grid[(7, 0).into()], 3);
        assert_eq!(grid[(8, 0).into()], 1);
        assert_eq!(grid[(9, 0).into()], 1);
        assert_eq!(grid[(10, 0).into()], 3);
        assert_eq!(grid[(11, 0).into()], 2);
        assert_eq!(grid[(12, 0).into()], 3);
    }

    #[test]
    fn test_successors() {
        let input = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
        "};

        let grid = input.parse::<Grid>().expect("Input should be valid grid");
        let start = (0, 0).into();
        let move_1 = Move::new(start, None);
        let explored = HashSet::new();
        let successors: Vec<(Move, usize)> = grid.successors(move_1, 0, 4, 5, &explored);
        let expected = vec![
            (Move::new((4, 0).into(), Some(Direction::Horizontal)), 12),
            (Move::new((0, 4).into(), Some(Direction::Vertical)), 13),
            (Move::new((5, 0).into(), Some(Direction::Horizontal)), 15),
            (Move::new((0, 5).into(), Some(Direction::Vertical)), 14),
        ];
        assert_eq!(successors.len(), 4);
        assert_eq!(successors, expected);
    }

    #[test]
    fn test_find_lowest_heatloss() {
        let input = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
        "};

        let grid = input.parse::<Grid>().expect("Input should be valid grid");
        let search = SearchParameters {
            start: (0, 0).into(),
            end: (12, 12).into(),
            min_movement: 4,
            max_movement: 10,
        };
        let result = grid
            .find_lowest_heatloss(search)
            .expect("There should be a correct path");
        assert_eq!(result, 94);
    }
}
