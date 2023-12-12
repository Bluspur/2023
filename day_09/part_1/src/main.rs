use anyhow::{Context, Result};

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Failed to read input.txt");
    let data = parse(&input).expect("Failed to parse input.txt");
    let result = extrapolate_sequences(data);

    println!("Result: {}", result);
}

fn extrapolate_sequences(sequences: Vec<Vec<i32>>) -> i32 {
    sequences
        .iter()
        .map(|seq| finite_differences(seq.to_vec()).expect("Failed to extrapolate sequence"))
        .fold(0, |acc, seq| {
            acc + seq.last().expect("Sequence have more than 0 elements")
        })
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

fn finite_differences(mut seq: Vec<i32>) -> Option<Vec<i32>> {
    let differences: Vec<i32> = seq.windows(2).map(|v| v[1] - v[0]).collect();

    // Base Case
    if differences.iter().all(|v| *v == 0) {
        seq.push(seq.last()? + differences.last()?);
        return Some(seq);
    }

    // Recursive Case
    let extrapolated_seq = finite_differences(differences)?;

    //Unwind
    seq.push(seq.last()? + extrapolated_seq.last()?);
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
        let actual =
            finite_differences(vec![0, 0, 0]).expect("Finite Differences should return Some");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_finite_differences_linear() {
        let expected = vec![1, 2, 3, 4];
        let actual =
            finite_differences(vec![1, 2, 3]).expect("Finite Differences should return Some");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_finite_differences_curve() {
        let expected = vec![1, 4, 9, 16, 25, 36];
        let actual = finite_differences(vec![1, 4, 9, 16, 25])
            .expect("Finite Differences should return Some");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_finite_differences_test_data_one() {
        let expected = vec![0, 3, 6, 9, 12, 15, 18];
        let actual = finite_differences(vec![0, 3, 6, 9, 12, 15])
            .expect("Finite Differences should return Some");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_finite_differences_test_data_two() {
        let expected = vec![1, 3, 6, 10, 15, 21, 28];
        let actual = finite_differences(vec![1, 3, 6, 10, 15, 21])
            .expect("Finite Differences should return Some");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_finite_differences_test_data_three() {
        let expected = vec![10, 13, 16, 21, 30, 45, 68];
        let actual = finite_differences(vec![10, 13, 16, 21, 30, 45])
            .expect("Finite Differences should return Some");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_extrapolate_sequences() {
        let test_data = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];

        let expected = 114;
        let actual = extrapolate_sequences(test_data);

        assert_eq!(expected, actual);
    }
}
