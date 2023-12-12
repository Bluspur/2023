use std::cmp;

fn main() {
    let input = std::fs::read_to_string("./puzzle_input.txt").expect("Failed to read file.");
    let sum = sum_of_game_powers(&input);
    println!("Sum of possible game IDs: {sum}")
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn sum_of_game_powers(input: &str) -> u32 {
    input.lines().map(parse_game).map(calculate_power).sum()
}

fn parse_game(input: &str) -> Game {
    let (head, tail) = input.split_once(':').expect("Invalid Game Format");
    let id = head
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<u32>()
        .expect("Failed to parse index");
    let (max_red, max_green, max_blue) =
        tail.split(';')
            .map(parse_colors)
            .fold((0, 0, 0), |mut acc, (r, g, b)| {
                acc.0 = cmp::max(acc.0, r);
                acc.1 = cmp::max(acc.1, g);
                acc.2 = cmp::max(acc.2, b);
                acc
            });
    Game {
        id,
        max_red,
        max_green,
        max_blue,
    }
}

fn parse_colors(game: &str) -> (u32, u32, u32) {
    let (red, green, blue) = game
        .split(',')
        .map(|x| x.trim().split_once(' ').expect("Failed to parse color"))
        .fold((0, 0, 0), |mut acc, (n, color)| {
            let value = n.parse::<u32>().expect("Failed to Parse Number");
            match color {
                "red" => acc.0 += value,
                "green" => acc.1 += value,
                "blue" => acc.2 += value,
                _ => unreachable!(),
            };
            acc
        });

    (red, green, blue)
}

fn calculate_power(game: Game) -> u32 {
    game.max_red * game.max_green * game.max_blue
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_power_returns_correct_value() {
        let test_input = Game {
            id: 0,
            max_red: 4,
            max_green: 2,
            max_blue: 6,
        };
        let expected = 48;
        let actual = calculate_power(test_input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_game_returns_correct_values() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            max_red: 4,
            max_green: 2,
            max_blue: 6,
        };
        let actual = parse_game(test_data);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_colors_returns_correct_values_all_values() {
        let test_data = "1 red, 2 green, 6 blue";
        let expected = (1, 2, 6);
        let actual = parse_colors(test_data);

        assert_eq!(expected.0, actual.0);
        assert_eq!(expected.1, actual.1);
        assert_eq!(expected.2, actual.2);
    }

    #[test]
    fn test_parse_colors_returns_correct_values_partial_values() {
        let test_data = "3 blue, 4 red";
        let expected = (4, 0, 3);
        let actual = parse_colors(test_data);

        assert_eq!(expected.0, actual.0);
        assert_eq!(expected.1, actual.1);
        assert_eq!(expected.2, actual.2);
    }

    #[test]
    fn test_data_returns_correct_sum() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let expected = 2286;
        let actual = sum_of_game_powers(test_data);

        assert_eq!(expected, actual);
    }
}
