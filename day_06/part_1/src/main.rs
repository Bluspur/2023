const SPEED: usize = 1; // mm/ms

fn main() {
    // let input = vec![(54, 239), (70, 1142), (82, 1295), (75, 1253)];
    let input = vec![(54708275, 239114212951253)];
    let mut product_of_winners = 1;

    for (time, record) in input {
        let mut winners = 0;
        for i in 0..=time {
            let distance = get_distance_travelled(i, time);
            if beats_record(distance, record) {
                winners += 1;
            }
        }
        product_of_winners *= winners;
    }

    println!("Product of winners: {}", product_of_winners);
}

fn beats_record(distance: usize, record: usize) -> bool {
    distance > record
}

fn get_distance_travelled(time_held: usize, total_time: usize) -> usize {
    let time_remaining = total_time - time_held;
    let velocity = SPEED * time_held;
    velocity * time_remaining
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distance_travelled() {
        assert_eq!(get_distance_travelled(0, 7), 0);
        assert_eq!(get_distance_travelled(1, 7), 6);
        assert_eq!(get_distance_travelled(2, 7), 10);
        assert_eq!(get_distance_travelled(3, 7), 12);
        assert_eq!(get_distance_travelled(4, 7), 12);
        assert_eq!(get_distance_travelled(5, 7), 10);
        assert_eq!(get_distance_travelled(6, 7), 6);
        assert_eq!(get_distance_travelled(7, 7), 0);
    }
}
