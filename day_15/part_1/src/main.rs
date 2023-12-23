fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    let result = solve_part(&input);
    println!("Result: {}", result);
}

fn solve_part(input: &str) -> usize {
    input.split(',').map(hash_sequence).sum()
}

fn hash_sequence(input: &str) -> usize {
    input
        .chars()
        .map(|c| c as usize)
        .fold(0, |acc, ascii_value| ((acc + ascii_value) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve_part(input), 1320);
    }

    #[test]
    fn test_hash_sequence() {
        let input = "HASH";
        let expected = 52;
        let actual = hash_sequence(input);
        assert_eq!(actual, expected);
    }
}
