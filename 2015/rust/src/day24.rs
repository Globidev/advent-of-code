use std::collections::BinaryHeap;
use std::cmp::Ordering;

type Package = u32;
type Packages = Vec<Package>;
type Entanglement = u64;

trait Node: Ord {
    fn new(Vec<usize>, u32, Entanglement) -> Self;

    fn indices(&self)      -> &Vec<usize>;
    fn weight(&self)       -> &u32;
    fn entanglement(&self) -> &Entanglement;
}

#[derive(Clone, Eq, PartialEq)]
struct NodeWeightFirst {
    indices: Vec<usize>,
    weight: u32,
    entanglement: Entanglement,
}

impl Ord for NodeWeightFirst {
    fn cmp(&self, other: &NodeWeightFirst) -> Ordering {
        self.weight.cmp(&other.weight)
            .then_with(|| other.indices.len().cmp(&self.indices.len()))
            .then_with(|| other.entanglement.cmp(&self.entanglement))
    }
}

impl PartialOrd for NodeWeightFirst {
    fn partial_cmp(&self, other: &NodeWeightFirst) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node for NodeWeightFirst {
    fn new(indices: Vec<usize>, weight: u32, entanglement: Entanglement) -> Self {
        NodeWeightFirst {
            indices: indices,
            weight: weight,
            entanglement: entanglement
        }
    }

    fn indices(&self)      -> &Vec<usize>   { &self.indices }
    fn weight(&self)       -> &u32          { &self.weight }
    fn entanglement(&self) -> &Entanglement { &self.entanglement }
}

#[derive(Clone, Eq, PartialEq)]
struct NodeSizeFirst {
    indices: Vec<usize>,
    weight: u32,
    entanglement: Entanglement,
}

impl Ord for NodeSizeFirst {
    fn cmp(&self, other: &NodeSizeFirst) -> Ordering {
        other.indices.len().cmp(&self.indices.len())
            .then_with(|| self.weight.cmp(&other.weight))
            .then_with(|| other.entanglement.cmp(&self.entanglement))
    }
}

impl PartialOrd for NodeSizeFirst {
    fn partial_cmp(&self, other: &NodeSizeFirst) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node for NodeSizeFirst {
    fn new(indices: Vec<usize>, weight: u32, entanglement: Entanglement) -> Self {
        NodeSizeFirst {
            indices: indices,
            weight: weight,
            entanglement: entanglement
        }
    }

    fn indices(&self)      -> &Vec<usize>   { &self.indices }
    fn weight(&self)       -> &u32          { &self.weight }
    fn entanglement(&self) -> &Entanglement { &self.entanglement }
}

fn quantum_entanglement(packages: &Packages) -> Entanglement {
    packages.iter().fold(1, |acc, x| acc * *x as Entanglement)
}

fn smallest_distrib<N: Node>(pool: &Packages, size: u32) -> Option<Packages> {
    use std::cmp::Ordering::*;

    let mut open_set = BinaryHeap::new();

    for (package, i) in pool.iter().zip(0..) {
        open_set.push(N::new(vec![i], *package, *package as Entanglement))
    }

    loop {
        if let Some(node) = open_set.pop() {
            match node.weight().cmp(&size) {
                Equal   => return Some(node.indices().iter().map(|i| pool[*i]).collect()),
                Greater => (),
                Less    => {
                    for (package, i) in pool.iter().zip(0..) {
                        if !node.indices().contains(&i) {
                            let mut new_indices = node.indices().clone();
                            new_indices.push(i);
                            open_set.push(N::new(
                                new_indices,
                                node.weight() + package,
                                node.entanglement() * *package as Entanglement
                            ));
                        }
                    }
                }
            }
        }
        else {
            break
        }
    }

    None
}

fn entanglement<N: Node>(packages: &Packages, group_count: u32) -> Entanglement {
    let package_weight = packages.iter().fold(0, |a, x| a + x) / group_count;

    smallest_distrib::<N>(&packages, package_weight)
        .map_or(0, |ds| quantum_entanglement(&ds))
}

pub fn p1(input: &str) -> Entanglement {
    // let packages = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
    let packages = input.trim().split('\n')
                               .map(|s| s.parse::<Package>().unwrap())
                               .collect::<Packages>();

    entanglement::<NodeWeightFirst>(&packages, 3)
}

pub fn p2(input: &str) -> Entanglement {
    let packages = input.trim().split('\n')
                               .map(|s| s.parse::<Package>().unwrap())
                               .collect::<Packages>();

    entanglement::<NodeSizeFirst>(&packages, 4)
}
