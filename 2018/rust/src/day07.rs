use arrayvec::ArrayVec;
use hashbrown::HashSet;

const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day07.txt");

pub fn day07() -> (String, u32) {
    let relations: Vec<_> = parse_relations(RAW_INPUT).collect();

    let (p1, p2) = (
        part1(relations.iter().cloned()),
        part2(relations.iter().cloned())
    );

    (String::from_utf8(p1.to_vec()).unwrap(), p2)
}

pub fn part1(relations: impl Iterator<Item = Relation>) -> Chain {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let Graph { roots, mut dependencies, dependents } = Graph::from_relations(relations);
    let mut queue: BinaryHeap<_> = roots.into_iter().map(Reverse).collect();
    let mut chain = Chain::new();

    while let Some(Reverse(letter)) = queue.pop() {
        chain.push(letter);

        for dependent in &dependents[(letter - b'A') as usize] {
            let deps_sum = &mut dependencies[(*dependent - b'A') as usize];
            *deps_sum -= letter as u16;
            if *deps_sum == 0 { queue.push(Reverse(*dependent)) }
        }
    }

    chain
}

pub fn part2(relations: impl Iterator<Item = Relation>) -> u32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    struct Worker {
        letter: u8,
        progress: u8
    }

    let Graph { roots, mut dependencies, dependents } = Graph::from_relations(relations);
    let mut queue: BinaryHeap<_> = roots.into_iter().map(Reverse).collect();
    let mut workers = ArrayVec::<[Worker; 8]>::new();
    let mut count = 0;

    loop {
        if workers.is_empty() && queue.is_empty() {
            return count
        }

        for _ in 0 .. (5 - workers.len()).min(queue.len()) {
            let Reverse(letter) = queue.pop().unwrap();
            let progress = 60 + letter - b'A' + 1;
            workers.push(Worker { letter, progress });
        }

        let (idx, &Worker { progress: min_progress, letter }) = workers.iter()
            .enumerate()
            .min_by_key(|(_, w)| w.progress)
            .unwrap();

        workers.remove(idx);

        for worker in &mut workers {
            worker.progress -= min_progress
        }

        for dependent in &dependents[(letter - b'A') as usize] {
            let deps_sum = &mut dependencies[(*dependent - b'A') as usize];
            *deps_sum -= letter as u16;
            if *deps_sum == 0 { queue.push(Reverse(*dependent)) }
        }

        count += min_progress as u32;
    }
}

struct Graph {
    roots: HashSet<u8>,
    dependencies: [u16; STEP_COUNT],
    dependents: [Dependents; STEP_COUNT],
}

impl Graph {
    #[inline(always)]
    fn from_relations(relations: impl Iterator<Item = Relation>) -> Self {
        let mut roots: HashSet<_> = (b'A'..=b'Z').collect();
        let mut dependencies = [0u16; STEP_COUNT];
        let mut dependents: [Dependents; STEP_COUNT] = Default::default();

        for (before, after) in relations {
            roots.remove(&after);
            dependencies[(after - b'A') as usize] += before as u16;
            dependents[(before - b'A') as usize].push(after);
        }

        Self { roots, dependencies, dependents }
    }
}

type Chain = ArrayVec<[u8; STEP_COUNT]>;
type Dependents = ArrayVec<[u8; STEP_COUNT]>;
type Relation = (u8, u8);

const STEP_COUNT: usize = 26;

pub fn parse_relations(input: &[u8]) -> impl Iterator<Item = Relation> + '_ {
    input.split(|&c| c == b'\n')
        .map(|line| (line[5], line[36]))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let relations = parse_relations(RAW_INPUT);

        assert_eq!(part1(relations).as_slice(), &b"JRHSBCKUTVWDQAIGYOPXMFNZEL"[..]);
    }

    #[test]
    fn p2() {
        let relations = parse_relations(RAW_INPUT);

        assert_eq!(part2(relations), 975);
    }
}
