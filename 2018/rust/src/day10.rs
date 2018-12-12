const RAW_INPUT_STR: &str = include_str!("../../inputs/day10.txt");
const LETTER_HEIGHT: usize = 10;
// const LETTER_WIDTH: usize = 6;
// const LETTER_SPACING: usize = 2;

pub fn day10() -> (Banner, usize) {
    let points: Vec<_> = parse_input(RAW_INPUT_STR).collect();

    (Banner(part1(&points)), part2(&points))
}

pub fn part1(points: &[SkyPoint]) -> String {
    let steps_required = steps_until_message(points);

    let message_points: Vec<_> = points.iter()
        .map(|p| Position {
            x: p.pos.x + p.vel.dx * steps_required as i32,
            y: p.pos.y + p.vel.dy * steps_required as i32,
        })
        .collect();

    let left = message_points.iter().map(|p| p.x).min().unwrap();
    let right = message_points.iter().map(|p| p.x).max().unwrap();
    let top = message_points.iter().map(|p| p.y).min().unwrap();
    let bottom = message_points.iter().map(|p| p.y).max().unwrap();

    let height = (bottom - top) as usize + 1;
    let width = (right - left) as usize + 1;

    let mut buff = vec![vec!['░'; width]; height];
    message_points.iter().for_each(|p| {
        let y = (p.y - top) as usize;
        let x = (p.x - left) as usize;
        buff[y][x] = '▓';
    });

    buff.into_iter()
        .flat_map(|b| b.into_iter().chain(std::iter::once('\n')))
        .collect()
}

pub fn part2(points: &[SkyPoint]) -> usize {
    steps_until_message(points)
}

fn steps_until_message(points: &[SkyPoint]) -> usize {
    let top0 = points.iter().map(|p| p.pos.y).min().unwrap();
    let bottom0 = points.iter().map(|p| p.pos.y).max().unwrap();

    let top1 = points.iter().map(|p| p.pos.y + p.vel.dy).min().unwrap();
    let bottom1 = points.iter().map(|p| p.pos.y + p.vel.dy).max().unwrap();

    let height0 = (bottom0 - top0) as usize;
    let height1 = (bottom1 - top1) as usize;

    let d_height = height0 - height1;

    ((height0 + 1) - LETTER_HEIGHT) / d_height
}

pub fn parse_input(input: &str) -> impl Iterator<Item = SkyPoint> + '_ {
    input.lines().map(move |line| {
        let mut splitted = line["position=<".len()..line.len() - 1].split("> velocity=<");

        let raw_pos = splitted.next().unwrap();
        let raw_vel = splitted.next().unwrap();

        let mut pos_split = raw_pos[..].split(',');
        let mut vel_split = raw_vel[..].split(',');

        let pos = Position {
            x: pos_split.next().expect("Missing x").trim().parse().expect("Invalid x"),
            y: pos_split.next().expect("Missing y").trim().parse().expect("Invalid y"),
        };

        let vel = Velocity {
            dx: vel_split.next().expect("Missing dx").trim().parse().expect("Invalid dx"),
            dy: vel_split.next().expect("Missing dy").trim().parse().expect("Invalid dy"),
        };

        SkyPoint { pos, vel }
    })
}

#[derive(Debug, Clone)]
pub struct SkyPoint {
    pos: Position,
    vel: Velocity,
}

#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    dx: i32,
    dy: i32,
}

pub struct Banner(String);

use std::fmt;

impl fmt::Debug for Banner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let points: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        // assert_eq!(part1(&points), "PHLGRNFK");
        assert_eq!(part1(&points), "\
▓▓▓▓▓░░░▓░░░░▓░░▓░░░░░░░░▓▓▓▓░░░▓▓▓▓▓░░░▓░░░░▓░░▓▓▓▓▓▓░░▓░░░░▓
▓░░░░▓░░▓░░░░▓░░▓░░░░░░░▓░░░░▓░░▓░░░░▓░░▓▓░░░▓░░▓░░░░░░░▓░░░▓░
▓░░░░▓░░▓░░░░▓░░▓░░░░░░░▓░░░░░░░▓░░░░▓░░▓▓░░░▓░░▓░░░░░░░▓░░▓░░
▓░░░░▓░░▓░░░░▓░░▓░░░░░░░▓░░░░░░░▓░░░░▓░░▓░▓░░▓░░▓░░░░░░░▓░▓░░░
▓▓▓▓▓░░░▓▓▓▓▓▓░░▓░░░░░░░▓░░░░░░░▓▓▓▓▓░░░▓░▓░░▓░░▓▓▓▓▓░░░▓▓░░░░
▓░░░░░░░▓░░░░▓░░▓░░░░░░░▓░░▓▓▓░░▓░░▓░░░░▓░░▓░▓░░▓░░░░░░░▓▓░░░░
▓░░░░░░░▓░░░░▓░░▓░░░░░░░▓░░░░▓░░▓░░░▓░░░▓░░▓░▓░░▓░░░░░░░▓░▓░░░
▓░░░░░░░▓░░░░▓░░▓░░░░░░░▓░░░░▓░░▓░░░▓░░░▓░░░▓▓░░▓░░░░░░░▓░░▓░░
▓░░░░░░░▓░░░░▓░░▓░░░░░░░▓░░░▓▓░░▓░░░░▓░░▓░░░▓▓░░▓░░░░░░░▓░░░▓░
▓░░░░░░░▓░░░░▓░░▓▓▓▓▓▓░░░▓▓▓░▓░░▓░░░░▓░░▓░░░░▓░░▓░░░░░░░▓░░░░▓
");
    }

    #[test]
    fn p2() {
        let points: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&points), 10407);
    }
}
