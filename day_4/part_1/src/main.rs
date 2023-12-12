use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Failed to read input file");
    let final_score = input
        .lines()
        .map(|l| {
            l.split_once(':')
                .expect("Game should be in format Game x : xxx")
        })
        .map(|(head, tail)| (parse_card_number(head), parse_game_score(tail)))
        .fold(0, |acc, (_, score)| acc + score);

    println!("Final Score: {}", final_score);
}

fn parse_card_number(input: &str) -> u32 {
    input
        .split_whitespace()
        .nth(1)
        .expect("Game head should be two parts long")
        .parse::<u32>()
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
    let matches_count = winners.intersection(&plays).count() as u32;
    calculate_score(matches_count)
}

fn calculate_score(wins: u32) -> u32 {
    if wins <= 2 {
        return wins;
    }

    2u32.pow(wins - 1)
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
    fn test_calculate_score_returns_true_for_1_win() {
        let expected = 1;
        let actual = calculate_score(1);

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_calculate_score_returns_true_for_2_win() {
        let expected = 2;
        let actual = calculate_score(2);

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_calculate_score_returns_true_for_3_win() {
        let expected = 4;
        let actual = calculate_score(3);

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
