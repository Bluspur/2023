use anyhow::{anyhow, Context, Result};
use std::convert::TryFrom;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
struct Row {
    springs: Vec<Spring>,
    contigious_groups: Vec<usize>,
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s
            .split_once(' ')
            .context("Row should be seperated into springs and groups by a space")?;
        let springs: Result<Vec<Spring>> = head
            .chars()
            .map(|c| Spring::try_from(c).context("Failed to parse Spring State"))
            .collect();
        let contigious_groups: Result<Vec<usize>> = tail
            .split(',')
            .map(|c| c.parse::<usize>().context("Failed to parse Group Size"))
            .collect();
        Ok(Row {
            springs: springs?,
            contigious_groups: contigious_groups?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Spring::Unknown),
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            c => Err(anyhow!("{} is not a valid Spring state", c)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_row_from_string() {
        let input = "???.### 1,1,3";
        let expected_springs = vec![
            Spring::Unknown,
            Spring::Unknown,
            Spring::Unknown,
            Spring::Operational,
            Spring::Damaged,
            Spring::Damaged,
            Spring::Damaged,
        ];
        let expected_groups = vec![1, 1, 3];

        let actual = Row::from_str(input).unwrap();

        assert_eq!(expected_springs, actual.springs);
        assert_eq!(expected_groups, actual.contigious_groups);
    }

    #[test]
    fn test_spring_from_char_returns_correctly() {
        let operational = '.';
        let damaged = '#';
        let unknown = '?';

        assert_eq!(Spring::Operational, Spring::try_from(operational).unwrap());
        assert_eq!(Spring::Damaged, Spring::try_from(damaged).unwrap());
        assert_eq!(Spring::Unknown, Spring::try_from(unknown).unwrap());
    }

    #[test]
    fn test_spring_from_char_returns_error_for_invalid_character() {
        let invalid_char = '%';

        assert!(Spring::try_from(invalid_char).is_err())
    }
}
