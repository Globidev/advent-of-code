use std::collections::BinaryHeap;
use std::cmp::Ordering;

use std::marker::PhantomData;

type Package = u32;
type Packages = Vec<Package>;
type Entanglement = u64;

#[derive(Clone, Eq, PartialEq)]
struct Node<Heuristic> {
    indices: Vec<usize>,
    weight: u32,
    entanglement: Entanglement,
    phantom: PhantomData<Heuristic>
}

impl<T> PartialOrd for Node<T> where Node<T>: Ord {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct WeightFirst {}
#[derive(Eq, PartialEq)]
struct LenFirst {}

impl Ord for Node<WeightFirst> {
    fn cmp(&self, other: &Node<WeightFirst>) -> Ordering {
        self.weight.cmp(&other.weight)
            .then_with(|| other.indices.len().cmp(&self.indices.len()))
            .then_with(|| other.entanglement.cmp(&self.entanglement))
    }
}

impl Ord for Node<LenFirst> {
    fn cmp(&self, other: &Node<LenFirst>) -> Ordering {
        other.indices.len().cmp(&self.indices.len())
            .then_with(|| self.weight.cmp(&other.weight))
            .then_with(|| other.entanglement.cmp(&self.entanglement))
    }
}

fn quantum_entanglement(packages: &Packages) -> Entanglement {
    packages.iter().fold(1, |acc, x| acc * *x as Entanglement)
}

fn smallest_distrib<Heuristic>(pool: &Packages, size: u32) -> Option<Packages>
    where Node<Heuristic>: Ord {
    use std::cmp::Ordering::*;

    let mut open_set = BinaryHeap::new();

    for (package, i) in pool.iter().zip(0..) {
        open_set.push(Node::<Heuristic> {
            indices: vec![i],
            weight: *package,
            entanglement: *package as Entanglement,
            phantom: PhantomData
        })
    }

    loop {
        if let Some(node) = open_set.pop() {
            match node.weight.cmp(&size) {
                Equal   => return Some(node.indices.iter().map(|i| pool[*i]).collect()),
                Greater => (),
                Less    => {
                    for (package, i) in pool.iter().zip(0..) {
                        if !node.indices.contains(&i) {
                            let mut new_indices = node.indices.clone();
                            new_indices.push(i);
                            open_set.push(Node::<Heuristic> {
                                indices: new_indices,
                                weight: node.weight + package,
                                entanglement: node.entanglement * *package as Entanglement,
                                phantom: PhantomData
                            });
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

fn entanglement<Heuristic>(packages: &Packages, group_count: u32) -> Entanglement
    where Node<Heuristic>: Ord {
    let package_weight = packages.iter().fold(0, |a, x| a + x) / group_count;

    smallest_distrib::<Heuristic>(&packages, package_weight)
        .map_or(0, |ds| quantum_entanglement(&ds))
}

pub fn p1(input: &str) -> Entanglement {
    let packages = input.trim().split('\n')
                               .map(|s| s.parse::<Package>().unwrap())
                               .collect::<Packages>();

    entanglement::<WeightFirst>(&packages, 3)
}

pub fn p2(input: &str) -> Entanglement {
    let packages = input.trim().split('\n')
                               .map(|s| s.parse::<Package>().unwrap())
                               .collect::<Packages>();

    entanglement::<LenFirst>(&packages, 4)
}
