use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Failed to read input file");
    let cards: HashMap<usize, u32> = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .expect("Game should be in format Game x : xxx")
        })
        .map(|(head, tail)| (parse_card_number(head), parse_game_score(tail)))
        .collect();

    let mut card_frequency: HashMap<usize, u32> = cards.keys().map(|k| (*k, 1)).collect();

    for i in 1..=card_frequency.len() {
        let frequency = *card_frequency
            .get(&i)
            .expect("Card should be in card_frequency") as usize;
        let score = *cards.get(&i).expect("Card should be in cards") as usize;
        for _ in 0..frequency {
            for k in i + 1..=i + score {
                let count = card_frequency.entry(k).or_insert(0);
                *count += 1;
            }
        }
    }

    let sum: u32 = card_frequency.values().sum();

    println!("Part 2: {}", sum);
}

fn parse_card_number(input: &str) -> usize {
    input
        .split_whitespace()
        .nth(1)
        .expect("Game head should be two parts long")
        .parse::<usize>()
        .expect("Failed to parse card as u32")
}

fn parse_numbers_list(input: &str) -> HashSet<u32> {
    input
        .split_whitespace()
        .map(|n| {
            n.parse::<u32>()
                .expect("Input numbers should be in a valid number format")
        })
        .collect()
}

fn parse_game_score(input: &str) -> u32 {
    let (winners, plays) = input
        .split_once('|')
        .expect("Game should be in format: x x | x x");
    let (winners, plays) = (parse_numbers_list(winners), parse_numbers_list(plays));
    winners.intersection(&plays).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_card_number_returns_correctly_single_digit() {
        let test_data = "Card 3";
        let expected = 3;
        let actual = parse_card_number(test_data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_card_number_returns_correctly_multi_digit() {
        let test_data = "Card 111";
        let expected = 111;
        let actual = parse_card_number(test_data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_game_score_returns_correct_score() {
        let test_data = "41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = 8;
        let actual = parse_game_score(test_data);

        assert_eq!(expected, actual);
    }
}
