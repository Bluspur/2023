use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

use anyhow::Result;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    let result = solve_part(&input);
    println!("Result: {}", result);
}

fn solve_part(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Input should be valid grid");
    grid.find_lowest_heatloss(Coordinate { x: 0, y: 0 }, Coordinate { x: 2, y: 2 }, 3)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Axis {
    Start,
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Move {
    to: Coordinate,
    cost: usize,
    axis: Axis,
    // Heuristic
    estimated_cost: usize,
}

impl Move {
    fn new(to: Coordinate, cost: usize, axis: Axis, end: Coordinate) -> Self {
        let estimated_cost = cost
            + (end.x as isize - to.x as isize).unsigned_abs()
            + (end.y as isize - to.y as isize).unsigned_abs();
        Self {
            to,
            cost,
            axis,
            estimated_cost,
        }
    }
    fn successors(
        &self,
        grid: &Grid,
        visited: &HashSet<Move>,
        max_movement: usize,
        end: Coordinate,
    ) -> Vec<Move> {
        let mut successors = Vec::new();
        // Get moves in the horizontal axis
        if self.axis == Axis::Vertical || self.axis == Axis::Start {
            let mut culm_pos_cost = 0;
            let mut culm_neg_cost = 0;
            for i in 1..=max_movement {
                // Right
                if self.to.x + i < grid.width {
                    let pos = Coordinate {
                        x: self.to.x + i,
                        y: self.to.y,
                    };
                    let cost = grid.value_at_coordinates(pos);
                    culm_pos_cost += cost;
                    let m = Move::new(pos, self.cost + culm_pos_cost, Axis::Horizontal, end);
                    if !visited.contains(&m) {
                        successors.push(m);
                    }
                }
                // Left
                if let Some(x) = self.to.x.checked_sub(i) {
                    let pos = Coordinate { x, y: self.to.y };
                    let cost = grid.value_at_coordinates(pos);
                    culm_neg_cost += cost;
                    let m = Move::new(pos, self.cost + culm_neg_cost, Axis::Horizontal, end);
                    if !visited.contains(&m) {
                        successors.push(m);
                    }
                }
            }
        }
        // Get moves in the vertical axis
        if self.axis == Axis::Horizontal || self.axis == Axis::Start {
            let mut culm_pos_cost = 0;
            let mut culm_neg_cost = 0;
            for i in 1..=max_movement {
                // Down
                if self.to.y + i < grid.height {
                    let pos = Coordinate {
                        x: self.to.x,
                        y: self.to.y + i,
                    };
                    let cost = grid.value_at_coordinates(pos);
                    culm_pos_cost += cost;
                    let m = Move::new(pos, self.cost + culm_pos_cost, Axis::Vertical, end);
                    if !visited.contains(&m) {
                        successors.push(m);
                    }
                }
                // Up
                if let Some(y) = self.to.y.checked_sub(i) {
                    let pos = Coordinate { x: self.to.x, y };
                    let cost = grid.value_at_coordinates(pos);
                    culm_neg_cost += cost;
                    let m = Move::new(pos, self.cost + culm_neg_cost, Axis::Vertical, end);
                    if !visited.contains(&m) {
                        successors.push(m);
                    }
                }
            }
        }
        successors
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse to make BinaryHeap a min-heap
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<usize>,
}

impl Grid {
    fn value_at_coordinates(&self, coord: Coordinate) -> usize {
        self.cells[self.coordinate_to_index(coord)]
    }

    fn coordinate_to_index(&self, coord: Coordinate) -> usize {
        coord.x + coord.y * self.width
    }

    // A* algorithm
    fn find_lowest_heatloss(
        &self,
        start: Coordinate,
        end: Coordinate,
        max_movement: usize,
    ) -> usize {
        let mut visited = HashSet::new();
        let mut frontier = BinaryHeap::new();
        // We set up the first move manually
        let origin = Move::new(start, self.value_at_coordinates(start), Axis::Start, end);
        frontier.push(origin);
        // Iterate through moves until the frontier is empty
        while let Some(m) = frontier.pop() {
            // First check if we are at the destination
            if m.to == end {
                return m.cost;
            }
            // Add this move to the visited set
            visited.insert(m);
            // Get the successor moves from the current move
            // Duplicate moves are handled during move generation
            for s_m in m.successors(self, &visited, max_movement, end) {
                frontier.push(s_m);
            }
        }
        unreachable!()
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Grid> {
        let mut width = 0;
        let mut height = 0;
        let mut cells = Vec::new();

        for line in s.lines() {
            if width == 0 {
                width = line.len();
            } else if width != line.len() {
                anyhow::bail!("Invalid grid width");
            }
            for c in line.chars() {
                let cell = c
                    .to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Invalid digit in input"))?;
                cells.push(cell as usize);
            }

            height += 1;
        }

        Ok(Grid {
            width,
            height,
            cells,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve_part() {
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

        assert_eq!(solve_part(input), 102);
    }

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
        assert_eq!(grid.cells[0], 2);
        assert_eq!(grid.cells[1], 4);
        assert_eq!(grid.cells[2], 1);
        assert_eq!(grid.cells[3], 3);
        assert_eq!(grid.cells[4], 4);
        assert_eq!(grid.cells[5], 3);
        assert_eq!(grid.cells[6], 2);
        assert_eq!(grid.cells[7], 3);
        assert_eq!(grid.cells[8], 1);
        assert_eq!(grid.cells[9], 1);
        assert_eq!(grid.cells[10], 3);
        assert_eq!(grid.cells[11], 2);
        assert_eq!(grid.cells[12], 3);
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
        let end = Coordinate { x: 12, y: 12 };
        let first_move = Move::new(Coordinate { x: 0, y: 0 }, 2, Axis::Start, end);
        let visited = HashSet::new();
        let successors = first_move.successors(&grid, &visited, 3, end);
        let expected = vec![
            Move {
                to: Coordinate { x: 1, y: 0 },
                cost: 6,
                axis: Axis::Horizontal,
                estimated_cost: 29,
            },
            Move {
                to: Coordinate { x: 2, y: 0 },
                cost: 7,
                axis: Axis::Horizontal,
                estimated_cost: 29,
            },
            Move {
                to: Coordinate { x: 3, y: 0 },
                cost: 10,
                axis: Axis::Horizontal,
                estimated_cost: 31,
            },
            Move {
                to: Coordinate { x: 0, y: 1 },
                cost: 5,
                axis: Axis::Vertical,
                estimated_cost: 28,
            },
            Move {
                to: Coordinate { x: 0, y: 2 },
                cost: 8,
                axis: Axis::Vertical,
                estimated_cost: 30,
            },
            Move {
                to: Coordinate { x: 0, y: 3 },
                cost: 11,
                axis: Axis::Vertical,
                estimated_cost: 32,
            },
        ];

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
        let cost =
            grid.find_lowest_heatloss(Coordinate { x: 0, y: 0 }, Coordinate { x: 12, y: 12 }, 3);
        assert_eq!(cost, 102);
    }

    #[test]
    fn test_find_lowest_heatloss_small() {
        let input = indoc! {"
        241
        321
        325
        "};

        let grid = input.parse::<Grid>().expect("Input should be valid grid");
        let cost =
            grid.find_lowest_heatloss(Coordinate { x: 0, y: 0 }, Coordinate { x: 2, y: 2 }, 3);
        assert_eq!(cost, 13);
    }
}
