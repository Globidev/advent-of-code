const RAW_INPUT_STR: &str = include_str!("../../inputs/day09.txt");

pub fn day09() -> (usize, usize) {
    let config = parse_input(RAW_INPUT_STR);

    (part1(&config), part2(&config))
}

use std::collections::VecDeque;

pub fn part1(config: &GameConfig) -> usize {
    let mut points = vec![0; config.player_count];
    let mut current_player = 0;
    let mut marbles = VecDeque::with_capacity(config.marble_count + 1);
    marbles.push_back(0);

    for marble in 1..=config.marble_count {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let popped = marbles.pop_back().unwrap();
                marbles.push_front(popped);
            }

            let new_points = marble + marbles.pop_back().unwrap();
            points[current_player] += new_points;

            let popped = marbles.pop_front().unwrap();
            marbles.push_back(popped);

            current_player = (current_player + 23) % config.player_count;
            continue
        }
        let popped = marbles.pop_front().unwrap();
        marbles.push_back(popped);
        marbles.push_back(marble);
    }

    points.into_iter().max().expect("No points")
}

pub fn part2(config: &GameConfig) -> usize {
    let config = GameConfig {
        marble_count: config.marble_count * 100,
        ..*config
    };

    part1(&config)
}

pub fn parse_input(input: &str) -> GameConfig {
    let mut words = input.split(' ');

    GameConfig {
        player_count: words.nth(0).expect("Missing player count")
            .parse().expect("Invalid player count"),
        marble_count: words.nth(5).expect("Missing marble count")
            .parse().expect("Invalid marble count"),
    }
}

#[derive(Debug)]
pub struct GameConfig {
    player_count: usize,
    marble_count: usize
}

mod tests {
    use super::*;
    #[test]
    fn p1() {
        let config = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(&config), 380705);
    }

    #[test]
    fn p2() {
        let config = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(&config), 3171801582);
    }
}
