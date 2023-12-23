use indexmap::IndexMap;
use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    let result = solve_part(&input);
    println!("Result: {}", result.unwrap());
}

fn solve_part(input: &str) -> Result<usize> {
    let commands = parse(input).context("Failed to parse input")?;
    let mut boxes = Boxes::new();
    for command in commands {
        boxes.resolve_command(command);
    }
    let mut total = 0;
    for (box_id, lenses) in boxes.boxes {
        for (i, (_, focal_length)) in lenses.into_iter().enumerate() {
            total += (box_id + 1) * (i + 1) * focal_length;
        }
    }

    Ok(total)
}

struct Boxes {
    boxes: HashMap<usize, IndexMap<String, usize>>,
}

impl Boxes {
    fn new() -> Self {
        Boxes {
            boxes: HashMap::new(),
        }
    }
    fn resolve_command(&mut self, command: Command) {
        match command.command_type {
            CommandType::Insert(focal_length) => {
                let lenses = self.boxes.entry(command.box_id).or_default();
                lenses.insert(command.label, focal_length);
            }
            CommandType::Remove => {
                if let Some(lenses) = self.boxes.get_mut(&command.box_id) {
                    lenses.shift_remove(&command.label);
                }
            }
        }
    }
}

fn hash_sequence(input: &str) -> usize {
    input
        .chars()
        .map(|c| c as usize)
        .fold(0, |acc, ascii_value| ((acc + ascii_value) * 17) % 256)
}

fn parse(input: &str) -> Result<Vec<Command>> {
    input
        .split(',')
        .map(parse_command)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_command(input: &str) -> Result<Command> {
    let parts: Vec<&str> = input.split(|c| matches!(c, '=' | '-')).collect();
    let label = parts[0].to_string();
    let box_id = hash_sequence(parts[0]);

    let command_type = if !parts[1].is_empty() {
        let value = parts[1].parse::<usize>();
        match value {
            Ok(v) => CommandType::Insert(v),
            Err(_) => return Err(anyhow!("Invalid command")),
        }
    } else if input.ends_with('-') {
        CommandType::Remove
    } else {
        return Err(anyhow!("Invalid command"));
    };

    Ok(Command {
        label,
        box_id,
        command_type,
    })
}

#[derive(Debug, PartialEq)]
struct Command {
    label: String,
    box_id: usize,
    command_type: CommandType,
}

#[derive(Debug, PartialEq)]
enum CommandType {
    Insert(usize),
    Remove,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        assert_eq!(solve_part(&input).unwrap(), 145);
    }

    #[test]
    fn test_hash_sequence() {
        let input = "rn";
        let expected = 0;
        let actual = hash_sequence(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_command() {
        let input = "rn=1";
        let expected = Command {
            label: "rn".to_string(),
            box_id: 0,
            command_type: CommandType::Insert(1),
        };
        let actual = parse_command(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let expected = vec![
            Command {
                label: "rn".to_string(),
                box_id: 0,
                command_type: CommandType::Insert(1),
            },
            Command {
                label: "cm".to_string(),
                box_id: 0,
                command_type: CommandType::Remove,
            },
            Command {
                label: "qp".to_string(),
                box_id: 1,
                command_type: CommandType::Insert(3),
            },
            Command {
                label: "cm".to_string(),
                box_id: 0,
                command_type: CommandType::Insert(2),
            },
            Command {
                label: "qp".to_string(),
                box_id: 1,
                command_type: CommandType::Remove,
            },
            Command {
                label: "pc".to_string(),
                box_id: 3,
                command_type: CommandType::Insert(4),
            },
            Command {
                label: "ot".to_string(),
                box_id: 3,
                command_type: CommandType::Insert(9),
            },
            Command {
                label: "ab".to_string(),
                box_id: 3,
                command_type: CommandType::Insert(5),
            },
            Command {
                label: "pc".to_string(),
                box_id: 3,
                command_type: CommandType::Remove,
            },
            Command {
                label: "pc".to_string(),
                box_id: 3,
                command_type: CommandType::Insert(6),
            },
            Command {
                label: "ot".to_string(),
                box_id: 3,
                command_type: CommandType::Insert(7),
            },
        ];
        let actual = parse(input).unwrap();
        assert_eq!(actual, expected);
    }
}
