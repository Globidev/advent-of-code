const RAW_INPUT_STR: &str = include_str!("../../inputs/day25.txt");

pub fn day25() -> u32 {
    let points: Vec<_> = parse_input(RAW_INPUT_STR).collect();

    part1(&points)
}

pub fn part1(points: &[Point]) -> u32 {
    let mut points = points.to_vec();
    let mut count = 0;

    while !points.is_empty() {
        let mut constellation = vec![points.remove(0)];

        loop {
            let (mut in_range, remaining): (Vec<_>, _) = points.iter()
                .partition(|&&candidate| {
                    constellation.iter()
                        .any(|&point| manhattan(point, candidate) <= 3)
                });

            points = remaining;

            if in_range.is_empty() {
                break
            }

            constellation.append(&mut in_range);
        }

        count += 1;
    }

    count
}

fn manhattan((x1, y1, z1, t1): Point, (x2, y2, z2, t2): Point) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs() + (t1 - t2).abs()
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Point> + '_ {
    input.split('\n').map(|line| {
        let mut coords = line.split(',');

        (
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        )
    })
}

type Point = (i32, i32, i32, i32);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let points: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&points), 363);
    }
}
