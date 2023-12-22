use anyhow::Result;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Unable to read file");
    let result = solve_part(&input);
    println!("Result: {}", result);
}

fn solve_part(input: &str) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve_part() {
        let input = indoc! {"
            000
        "};
        assert_eq!(solve_part(input), 0);
    }
}
