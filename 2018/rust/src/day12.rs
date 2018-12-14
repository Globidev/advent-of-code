const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day12.txt");

pub fn day12() -> (i64, i64) {
    let (state, rules) = parse_input(RAW_INPUT);

    (part1(&state, &rules), part2(&state, &rules))
}

pub fn part1(initial_state: &[Pot], rules: &Rules) -> i64 {
    let initial_pots = PotRow {
        pots: initial_state.iter().cloned().collect(),
        start_idx: 0
    };

    spread_n(initial_pots, rules, 20)
        .sum_idxs()
}

pub fn part2(initial_state: &[Pot], rules: &Rules) -> i64 {
    let initial_pots = PotRow {
        pots: initial_state.iter().cloned().collect(),
        start_idx: 0
    };

    // Find out cycle properties
    let (cycle_len, cycle_start) = floyd(
        initial_pots.clone(),
        |pot| spread(pot, rules),
        |p1, p2| p1.pots == p2.pots
    );

    // Compute deltas between cycles
    let pots_after_one_cycle = spread_n(initial_pots, rules, cycle_start);
    let pots_after_two_cycles = spread_n(pots_after_one_cycle.clone(), rules, cycle_len);
    let start_delta = pots_after_two_cycles.start_idx - pots_after_one_cycle.start_idx;

    // Fast forward to the last cycle
    let cycles_before_end = (50_000_000_000 - cycle_start) / cycle_len;
    let remaining_steps = (50_000_000_000 - cycle_start) % cycle_len;
    let last_cycled_pots = PotRow {
        pots: pots_after_one_cycle.pots,
        start_idx: pots_after_one_cycle.start_idx + cycles_before_end as i64 * start_delta,
    };

    // Simulate the possible remaining steps
    let final_pots = spread_n(last_cycled_pots, rules, remaining_steps);

    final_pots.sum_idxs()
}

fn floyd<T: Clone, F: Fn(T) -> T, Cmp: Fn(&T, &T) -> bool>(x0: T, f: F, cmp: Cmp)
    -> (usize, usize)
{
    let mut tortoise = f(x0.clone());
    let mut hare = f(f(x0.clone()));

    while !cmp(&tortoise, &hare) {
        tortoise = f(tortoise);
        hare = f(f(hare));
    }

    let mut mu = 0;
    let mut tortoise = x0;
    while !cmp(&tortoise, &hare) {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1
    }

    let mut lam = 1;
    let mut hare = f(tortoise.clone());
    while !cmp(&tortoise, &hare) {
        hare = f(hare);
        lam += 1
    }

    (lam, mu)
}

pub fn parse_input(input: &[u8]) -> (Vec<Pot>, Rules) {
    let mut lines = input.split(|&c| c == b'\n');

    let raw_state = &lines.next().expect("Missing initial state")[15..];
    let initial_state = raw_state.iter().map(|&b| Pot::from(b)).collect();

    let rules = lines.skip(1)
        .map(|line| (
            [
                Pot::from(line[0]),
                Pot::from(line[1]),
                Pot::from(line[2]),
                Pot::from(line[3]),
                Pot::from(line[4]),
            ],
            Pot::from(line[9]),
        ))
        .collect();

    (initial_state, rules)
}

fn spread(mut pot_row: PotRow, rules: &Rules) -> PotRow {
    let fill_left = pot_row.pots.iter()
        .enumerate()
        .find(|(_, p)| **p == Pot::Filled)
        .map(|(i, _)| 4_usize.saturating_sub(i))
        .unwrap_or(0);

    for _ in 0..fill_left {
        pot_row.pots.insert(0, Pot::Empty);
    }
    pot_row.start_idx += fill_left as i64;

    let fill_right = pot_row.pots.iter().rev()
        .enumerate()
        .find(|(_, p)| **p == Pot::Filled)
        .map(|(i, _)| 4_usize.saturating_sub(i))
        .unwrap_or(0);

    for _ in 0..fill_right {
        pot_row.pots.push(Pot::Empty);
    }

    pot_row.pots = (2..pot_row.pots.len() - 2)
        .map(|idx| {
            let pots = &pot_row.pots[idx-2..=idx+2];
            if let Some(result) = rules.get(pots) {
                result.clone()
            } else {
                pot_row.pots[idx].clone()
            }
        })
        .collect();

    pot_row.start_idx -= 2;

    pot_row
}

fn spread_n(pot_row: PotRow, rules: &Rules, n: usize) -> PotRow {
    (0..n)
        .fold(pot_row, |pot, _| spread(pot, rules))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pot {
    Empty,
    Filled,
}

use hashbrown::HashMap;

type Rules = HashMap<PotPattern, Pot>;
type PotPattern = [Pot; 5];

#[derive(Debug, Clone)]
struct PotRow {
    pots: Vec<Pot>,
    start_idx: i64
}

impl PotRow {
    fn sum_idxs(&self) -> i64 {
        self.pots.iter()
            .enumerate()
            .filter_map(|(idx, pot)| match pot {
                Pot::Filled => Some(idx as i64 - self.start_idx),
                Pot::Empty  => None,
            })
            .sum()
    }
}

impl From<u8> for Pot {
    fn from(ch: u8) -> Self {
        match ch {
            b'.' => Pot::Empty,
            b'#' => Pot::Filled,
            c   => panic!("Invalid pot byte: {}", c),
        }
    }
}

impl Into<u8> for Pot {
    fn into(self) -> u8 {
        match self {
            Pot::Empty  => b'.',
            Pot::Filled => b'#',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let (state, rules) = parse_input(RAW_INPUT);

        assert_eq!(part1(&state, &rules), 2045);
    }

    #[test]
    fn p2() {
        let (state, rules) = parse_input(RAW_INPUT);

        assert_eq!(part2(&state, &rules), 2100000000428);
    }
}
