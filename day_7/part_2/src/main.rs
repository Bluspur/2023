use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Could not read file");
    let mut hands = parse(&input);
    hands.sort();
    let total_winnings = calculate_winnings(&hands);
    println!("Total winnings: {}", total_winnings);
}

fn calculate_winnings(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_hand(cards: &[usize]) -> HandType {
        let mut card_counts = HashMap::new();
        for card in cards {
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }

        let joker_count = card_counts.remove(&1).unwrap_or(0);
        // If there are 5 jokers, we have a five of a kind
        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        let max_key = **card_counts
            .iter()
            .max_by_key(|&(_k, &v)| v)
            .expect("Frequency map should not be empty")
            .0;

        *card_counts
            .get_mut(&max_key)
            .expect("Max key should be in map") += joker_count;

        match card_counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_counts.values().any(|&v| v == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_counts.values().any(|&v| v == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    hand_type: HandType,
    hex_value: usize,
}

impl Hand {
    fn new(cards: Vec<usize>, bid: usize) -> Hand {
        let hex_value = Hand::get_hex_value(&cards);
        let hand_type = HandType::from_hand(&cards);
        Hand {
            cards,
            bid,
            hex_value,
            hand_type,
        }
    }
    fn get_hex_value(cards: &[usize]) -> usize {
        cards.iter().fold(0, |acc, card| acc * 16 + card)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.hex_value.cmp(&other.hex_value),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn from_char(c: char) -> Option<usize> {
    match c {
        'J' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'T' => Some(10),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None,
    }
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(cards, bid)| {
                    let cards = cards
                        .chars()
                        .map(|c| from_char(c).expect("Invalid card"))
                        .collect();
                    let bid = bid.parse::<usize>().expect("Invalid bid");
                    Hand::new(cards, bid)
                })
                .expect("Lines must contain a space")
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_DATA: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    #[test]
    fn test_parse_first_hand() {
        let expected = Hand::new(vec![3, 2, 10, 3, 13], 765);

        let actual = &parse(TEST_DATA)[0];

        assert_eq!(expected, *actual);
    }

    #[test]
    fn test_parse_all_hands() {
        let expected = vec![
            Hand::new(vec![3, 2, 10, 3, 13], 765),
            Hand::new(vec![10, 5, 5, 1, 5], 684),
            Hand::new(vec![13, 13, 6, 7, 7], 28),
            Hand::new(vec![13, 10, 1, 1, 10], 220),
            Hand::new(vec![12, 12, 12, 1, 14], 483),
        ];

        let actual = parse(TEST_DATA);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_hand_hex_value_returns_correctly() {
        let hand = Hand::new(vec![3, 2, 10, 3, 13], 765);

        assert_eq!(hand.hex_value, 0x32A3D);
    }

    #[test]
    fn test_hand_hex_comparison_returns_highest_hand() {
        let hands = vec![Hand::new(vec![14, 2, 2], 0), Hand::new(vec![2, 2, 14], 0)];

        assert!(hands[0] > hands[1]);
        assert!(hands[1] < hands[0]);
    }
    #[test]
    fn test_hand_hex_comparison_second_card_higher() {
        let hands = vec![Hand::new(vec![2, 3, 2], 0), Hand::new(vec![2, 2, 14], 0)];

        assert!(hands[0] > hands[1]);
        assert!(hands[1] < hands[0]);
    }

    #[test]
    fn test_hand_type_ordering() {
        let hands = vec![Hand::new(vec![3, 3, 3], 0), Hand::new(vec![1, 14, 14], 0)];

        assert!(hands[0] > hands[1]);
        assert!(hands[1] < hands[0]);
    }

    #[test]
    fn test_hand_type_from_hand() {
        let hands = vec![
            [2, 2, 2, 2, 2], // Five of a kind
            [2, 2, 2, 2, 3], // Four of a kind
            [2, 2, 2, 3, 3], // Full house
            [4, 2, 3, 3, 3], // Three of a kind
            [4, 4, 2, 2, 3], // Two pair
            [2, 2, 3, 4, 5], // One pair
            [2, 3, 4, 5, 6], // High card
        ];

        assert_eq!(HandType::FiveOfAKind, HandType::from_hand(&hands[0]));
        assert_eq!(HandType::FourOfAKind, HandType::from_hand(&hands[1]));
        assert_eq!(HandType::FullHouse, HandType::from_hand(&hands[2]));
        assert_eq!(HandType::ThreeOfAKind, HandType::from_hand(&hands[3]));
        assert_eq!(HandType::TwoPair, HandType::from_hand(&hands[4]));
        assert_eq!(HandType::OnePair, HandType::from_hand(&hands[5]));
        assert_eq!(HandType::HighCard, HandType::from_hand(&hands[6]));
    }

    #[test]
    fn test_hand_type_from_hand_with_jokers() {
        let hands = [[1, 2, 2, 2, 2], // Five of a kind
            [1, 2, 2, 2, 3], // Four of a kind
            [1, 2, 2, 3, 3], // Full house
            [4, 2, 1, 3, 3], // Three of a kind
            [1, 2, 3, 4, 5]];

        assert_eq!(HandType::FiveOfAKind, HandType::from_hand(&hands[0]));
        assert_eq!(HandType::FourOfAKind, HandType::from_hand(&hands[1]));
        assert_eq!(HandType::FullHouse, HandType::from_hand(&hands[2]));
        assert_eq!(HandType::ThreeOfAKind, HandType::from_hand(&hands[3]));
        assert_eq!(HandType::OnePair, HandType::from_hand(&hands[4]));
    }

    #[test]
    fn test_hand_sort() {
        let mut hands = vec![
            Hand::new(vec![2, 2, 2, 2, 3], 0), // Four of a kind
            Hand::new(vec![2, 2, 2, 2, 2], 0), // Five of a kind
            Hand::new(vec![1, 1, 2, 2, 3], 0), // Four of a kind (2 jokers)
            Hand::new(vec![1, 2, 3, 3, 3], 0), // Four of a kind (1 joker)
            Hand::new(vec![1, 1, 3, 4, 5], 0), // Three of a kind (2 jokers)
            Hand::new(vec![1, 2, 3, 4, 5], 0), // Pair (1 joker)
            Hand::new(vec![1, 1, 2, 2, 3], 0), // Four of a kind (2 jokers)
            Hand::new(vec![2, 2, 2, 3, 3], 0), // Full house
        ];

        let expected = vec![
            Hand::new(vec![1, 2, 3, 4, 5], 0), // Pair (1 joker)
            Hand::new(vec![1, 1, 3, 4, 5], 0), // Three of a kind (2 jokers)
            Hand::new(vec![2, 2, 2, 3, 3], 0), // Full house
            Hand::new(vec![1, 1, 2, 2, 3], 0), // Four of a kind (2 jokers)
            Hand::new(vec![1, 1, 2, 2, 3], 0), // Four of a kind (2 jokers)
            Hand::new(vec![1, 2, 3, 3, 3], 0), // Four of a kind (1 joker)
            Hand::new(vec![2, 2, 2, 2, 3], 0), // Four of a kind
            Hand::new(vec![2, 2, 2, 2, 2], 0), // Five of a kind
        ];

        hands.sort();

        assert_eq!(expected, hands);
    }

    #[test]
    fn test_calculate_total_winnings() {
        let mut hands = parse(TEST_DATA);

        hands.sort();

        for hand in &hands {
            assert!(hand.cards.len() == 5);
        }
        assert_eq!(hands.len(), 5);
        assert_eq!(calculate_winnings(&hands), 5905);
    }

    #[test]
    fn test_data_correct_hands() {
        let mut hands = parse(TEST_DATA);

        let expected = vec![
            Hand::new(vec![3, 2, 10, 3, 13], 765),
            Hand::new(vec![10, 5, 5, 1, 5], 684),
            Hand::new(vec![13, 13, 6, 7, 7], 28),
            Hand::new(vec![13, 10, 1, 1, 10], 220),
            Hand::new(vec![12, 12, 12, 1, 14], 483),
        ];

        assert_eq!(hands.len(), 5);
        assert_eq!(hands, expected);
        assert_eq!(hands[0].hand_type, HandType::OnePair);
        assert_eq!(hands[1].hand_type, HandType::FourOfAKind);
        assert_eq!(hands[2].hand_type, HandType::TwoPair);
        assert_eq!(hands[3].hand_type, HandType::FourOfAKind);
        assert_eq!(hands[4].hand_type, HandType::FourOfAKind);

        hands.sort();
        dbg!(&hands);

        let expected_ordered = vec![
            Hand::new(vec![3, 2, 10, 3, 13], 765),
            Hand::new(vec![13, 13, 6, 7, 7], 28),
            Hand::new(vec![10, 5, 5, 1, 5], 684),
            Hand::new(vec![12, 12, 12, 1, 14], 483),
            Hand::new(vec![13, 10, 1, 1, 10], 220),
        ];

        assert_eq!(hands, expected_ordered);
    }
}
