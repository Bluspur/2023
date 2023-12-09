use anyhow::{Context, Result};
use rayon::prelude::*;
use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Failed to read input.txt");
    let data = parse(&input).expect("Failed to parse input.txt");
    let result = extrapolate_sequences(data);

    println!("Result: {}", result);
}

// Function that takes a vector of integer sequences, extrapolates each sequence in parallel,
// and returns the sum of the first elements of the extrapolated sequences.
fn extrapolate_sequences(sequences: Vec<Vec<i32>>) -> i32 {
    sequences
        .par_iter()
        .map(|seq| {
            rev_finite_differences(VecDeque::from(seq.to_vec()))
                .expect("Failed to extrapolate sequence")
                .front()
                .expect("Sequence should have more than 0 elements")
                .clone()
        })
        .sum()
}

fn parse(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().context("Failed to parse i32"))
                .collect()
        })
        .collect()
}

// Function that takes a sequence of integers and returns a sequence of the reverse finite differences.
// This function is recursive: it computes the finite differences of the input sequence,
// then calls itself with the differences sequence until it gets a sequence of all zeros.
// At each level of recursion, it prepends the difference between the first element of the input sequence
// and the first element of the differences sequence to the input sequence.
fn rev_finite_differences(mut seq: VecDeque<i32>) -> Option<VecDeque<i32>> {
    let differences: VecDeque<i32> = seq
        .make_contiguous()
        .windows(2)
        .map(|v| v[1] - v[0])
        .collect();

    // If all differences are zero, prepend the difference between the first element of the input sequence
    // and the first element of the differences sequence to the input sequence and return it
    if differences.iter().all(|v| *v == 0) {
        seq.push_front(seq.front()? - differences.front()?);
        return Some(seq);
    } else {
        // Otherwise, recursively call this function with the differences sequence
        let extrapolated_seq = rev_finite_differences(differences)?;
        seq.push_front(seq.front()? - extrapolated_seq.front()?);
    }

    // Return the sequence with the prepended element
    Some(seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";

    #[test]
    fn test_parse_returns_expected_result() {
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        let actual = parse(TEST_INPUT).expect("TEST_INPUT should parse correctly");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_returns_err_for_invalid_u32() {
        let bad_data = "3 f 2 1\n6 f 4 9";

        let actual = parse(bad_data);
        assert!(actual.is_err())
    }

    #[test]
    fn test_finite_differences_zeros() {
        let expected = vec![0, 0, 0, 0];
        let actual = rev_finite_differences(VecDeque::from(vec![0, 0, 0]))
            .expect("Finite Differences should return Some");

        assert_eq!(expected, Vec::from(actual));
    }

    #[test]
    fn test_finite_differences_linear() {
        let expected = vec![0, 1, 2, 3];
        let actual = rev_finite_differences(VecDeque::from(vec![1, 2, 3]))
            .expect("Finite Differences should return Some");

        assert_eq!(expected, Vec::from(actual))
    }

    #[test]
    fn test_finite_differences_curve() {
        let expected = vec![0, 1, 4, 9, 16];
        let actual = rev_finite_differences(VecDeque::from(vec![1, 4, 9, 16]))
            .expect("Finite Differences should return Some");

        assert_eq!(expected, Vec::from(actual))
    }

    #[test]
    fn test_finite_differences_test_data() {
        let expected = vec![5, 10, 13, 16, 21, 30, 45];
        let vdeq = VecDeque::from(vec![10, 13, 16, 21, 30, 45]);
        let actual = rev_finite_differences(vdeq).expect("Finite Differences should return Some");

        assert_eq!(expected, Vec::from(actual))
    }

    #[test]
    fn test_finite_differences_front_values() {
        let expected = vec![-3, 0, 5];
        let mut test_data = vec![
            VecDeque::from(vec![0, 3, 6, 9, 12, 15]),
            VecDeque::from(vec![1, 3, 6, 10, 15, 21]),
            VecDeque::from(vec![10, 13, 16, 21, 30, 45]),
        ];
        let actual: Vec<VecDeque<i32>> = test_data
            .iter()
            .map(|d| rev_finite_differences(d.clone()).expect("Should return a value"))
            .collect();

        assert_eq!(expected[0], actual[0][0]);
        assert_eq!(expected[1], actual[1][0]);
        assert_eq!(expected[2], actual[2][0]);
    }

    #[test]
    fn test_extrapolate_sequences() {
        let mut test_data = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        let expected = 2;
        let actual = extrapolate_sequences(test_data);

        assert_eq!(expected, actual);
    }
}
