use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Could not read file");
    let map = parse(&input);
    let steps = search(&map);
    println!("Steps to destination: {}", steps);
}

fn search(map: &Map) -> usize {
    let starting_nodes: Vec<&String> = map.get_starting_nodes();

    let cul_steps: Vec<usize> = starting_nodes
        .par_iter()
        .map(|&node| {
            let mut current = node;
            let mut steps = 0;
            while map.nodes[current].is_terminal != Some(Terminal::End) {
                current = match map.directions[steps % map.directions.len()] {
                    Direction::Right => map.get_edge(current, &Direction::Right),
                    Direction::Left => map.get_edge(current, &Direction::Left),
                };
                steps += 1;
            }
            steps
        })
        .collect();

    cul_steps
        .iter()
        .cloned()
        .reduce(lcm)
        .expect("No steps found")
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn parse(input: &str) -> Map {
    let input = input.replace("\r\n", "\n");

    let (directions, map) = input
        .split_once("\n\n")
        .expect("Input should be split by \\n\\n");

    let directions = directions.chars().map(Direction::from_char).collect();

    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();

    map.lines().for_each(|l| {
        let (key, value) = l.split_once(" = ").expect("Line must contain ' = '");
        let (left, right) = value
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .expect("Value must contain ', '");

        let node = Node::from_str(key);
        nodes.insert(key.to_string(), node);
        let edge = (left.to_string(), right.to_string());
        edges.insert(key.to_string(), edge);
    });

    Map {
        directions,
        nodes,
        edges,
    }
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

#[derive(Debug, PartialEq)]
struct Node {
    id: String,
    is_terminal: Option<Terminal>,
}

impl Node {
    fn from_str(input: &str) -> Self {
        let id = input.to_string();
        let is_terminal = match input.chars().last() {
            Some('A') => Some(Terminal::Start),
            Some('Z') => Some(Terminal::End),
            _ => None,
        };
        Self { id, is_terminal }
    }
}

#[derive(Debug, PartialEq)]
enum Terminal {
    Start,
    End,
    None,
}

struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, Node>,
    edges: HashMap<String, (String, String)>,
}

impl Map {
    fn get_starting_nodes(&self) -> Vec<&String> {
        self.nodes
            .iter()
            .filter_map(|(k, v)| {
                if v.is_terminal == Some(Terminal::Start) {
                    Some(k)
                } else {
                    None
                }
            })
            .collect()
    }
    fn get_edge(&self, id: &str, direction: &Direction) -> &String {
        let (left, right) = &self.edges[id];
        match direction {
            Direction::Right => right,
            Direction::Left => left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

    #[test]
    fn test_parse_map_directions() {
        let expected_directions = vec![Direction::Left, Direction::Right];

        let map = parse(TEST_INPUT);
        let actual_directions = map.directions;

        assert_eq!(expected_directions, actual_directions);
    }

    #[test]
    fn test_parse_map_nodes() {
        let expected_nodes = vec![
            Node {
                id: "11A".to_string(),
                is_terminal: Some(Terminal::Start),
            },
            Node {
                id: "11B".to_string(),
                is_terminal: None,
            },
            Node {
                id: "11Z".to_string(),
                is_terminal: Some(Terminal::End),
            },
            Node {
                id: "22A".to_string(),
                is_terminal: Some(Terminal::Start),
            },
            Node {
                id: "22B".to_string(),
                is_terminal: None,
            },
            Node {
                id: "22C".to_string(),
                is_terminal: None,
            },
            Node {
                id: "22Z".to_string(),
                is_terminal: Some(Terminal::End),
            },
            Node {
                id: "XXX".to_string(),
                is_terminal: None,
            },
        ];

        let map = parse(TEST_INPUT);
        let actual_nodes = map.nodes;

        assert_eq!(expected_nodes[0], actual_nodes["11A"]);
        assert_eq!(expected_nodes[1], actual_nodes["11B"]);
        assert_eq!(expected_nodes[2], actual_nodes["11Z"]);
        assert_eq!(expected_nodes[3], actual_nodes["22A"]);
        assert_eq!(expected_nodes[4], actual_nodes["22B"]);
        assert_eq!(expected_nodes[5], actual_nodes["22C"]);
        assert_eq!(expected_nodes[6], actual_nodes["22Z"]);
        assert_eq!(expected_nodes[7], actual_nodes["XXX"]);
    }

    #[test]
    fn test_parse_map_edges() {
        let expected_edges = vec![
            ("11A".to_string(), ("11B".to_string(), "XXX".to_string())),
            ("11B".to_string(), ("XXX".to_string(), "11Z".to_string())),
            ("11Z".to_string(), ("11B".to_string(), "XXX".to_string())),
            ("22A".to_string(), ("22B".to_string(), "XXX".to_string())),
            ("22B".to_string(), ("22C".to_string(), "22C".to_string())),
            ("22C".to_string(), ("22Z".to_string(), "22Z".to_string())),
            ("22Z".to_string(), ("22B".to_string(), "22B".to_string())),
            ("XXX".to_string(), ("XXX".to_string(), "XXX".to_string())),
        ]
        .into_iter()
        .collect::<HashMap<String, (String, String)>>();

        let map = parse(TEST_INPUT);
        let actual_edges = map.edges;

        assert_eq!(expected_edges, actual_edges);
    }

    #[test]
    fn test_get_steps_to_destination() {
        let map = parse(TEST_INPUT);
        let expected = 6;

        let actual = search(&map);

        assert_eq!(expected, actual);
    }
}
