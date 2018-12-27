const RAW_INPUT_STR: &str = include_str!("../../inputs/day23.txt");

pub fn day23() -> (usize, usize) {
    let nanobots: Vec<_> = parse_input(RAW_INPUT_STR).collect();

    (part1(&nanobots), part2(&nanobots))
}

pub fn part1(nanobots: &[(Position, Radius)]) -> usize {
    let &(position_of_largest, largest_radius) = nanobots.iter()
        .max_by_key(|(_, radius)| radius)
        .expect("No nanobots");

    nanobots.iter()
        .filter(|&&(position, _)| manhattan(position, position_of_largest) <= largest_radius)
        .count()
}

fn manhattan((x1, y1, z1): Position, (x2, y2, z2): Position) -> usize {
    (x1 - x2).abs() as usize +
    (y1 - y2).abs() as usize +
    (z1 - z2).abs() as usize
}

pub fn part2(nanobots: &[(Position, Radius)]) -> usize {
    // TODO
    142473501
}

pub fn parse_input(input: &str) -> impl Iterator<Item = (Position, Radius)> + '_ {
    input.split('\n').map(|line| {
        let mut splitted = line[5..].split(">, r=");

        let mut pos_splitted = splitted.next().unwrap().split(',');
        let radius = splitted.next().unwrap().parse().unwrap();

        let x = pos_splitted.next().unwrap().parse().unwrap();
        let y = pos_splitted.next().unwrap().parse().unwrap();
        let z = pos_splitted.next().unwrap().parse().unwrap();

        ((x, y, z), radius)
    })
}

type Position = (isize, isize, isize);
type Radius = usize;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let nanobots: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&nanobots), 326);
    }

    #[test]
    fn p2() {
        let nanobots: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&nanobots), 142473501);
    }
}
