#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

macro_rules! set {
    () => {
        {
            std::collections::HashSet::new()
        }
    };

    ($($es:expr),+) => {
        {
            let mut set = std::collections::HashSet::new();
            set!(@ set | $($es),+);
            set
        }
    };

    (@ $set:ident |) => {
    };

    (@ $set:ident | $e:expr) => {
        $set.insert($e);
    };

    (@ $set:ident | $e:expr, $($es:expr),+) => {
        $set.insert($e);
        set!(@ $set | $($es),+)
    };
}

#[derive(Debug, PartialEq, Eq)]
struct NeighborEdge<'a>(Vertex<'a>, usize);

impl PartialOrd for NeighborEdge<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NeighborEdge<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vertex<'a>(Vec<&'a str>);

impl<'a> Vertex<'a> {
    fn new(node: &'a str) -> Self {
        Self(vec![node])
    }

    fn add(&mut self, mut node: Self) {
        self.0.append(&mut node.0);
        self.0.sort_unstable();
    }

    fn weight(&self, nodes: &HashSet<&'a str>) -> usize {
        nodes.iter().filter(|node| self.0.contains(node)).count()
    }
}

fn calculate_neighbors<'a>(
    edges: &HashMap<&'a str, HashSet<&'a str>>,
    a: &Vertex<'a>,
    b: &HashSet<Vertex<'a>>,
) -> BinaryHeap<NeighborEdge<'a>> {
    b.iter()
        .filter_map(|b| {
            let weight = b.0.iter().map(|end_node| a.weight(&edges[end_node])).sum();
            if weight > 0 {
                Some(NeighborEdge(b.clone(), weight))
            } else {
                None
            }
        })
        .collect::<BinaryHeap<_>>()
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> usize {
    let mut edges: HashMap<_, HashSet<_>> = HashMap::with_capacity(2_048);
    for line in input.lines() {
        let (start_node, neighbors) = line.split_once(": ").expect("invalid connection");
        for node in neighbors.split_ascii_whitespace() {
            edges
                .entry(start_node)
                .and_modify(|e| {
                    e.insert(node);
                })
                .or_insert_with(|| set! {node});
            edges
                .entry(node)
                .and_modify(|e| {
                    e.insert(start_node);
                })
                .or_insert_with(|| set! {start_node});
        }
    }

    let mut graph = edges
        .keys()
        .copied()
        .map(Vertex::new)
        .collect::<HashSet<_>>();

    let (mut min_a, mut min_b, mut min_cut) = (
        Vertex::new("<invalid>"),
        Vertex::new("<invalid>"),
        usize::MAX,
    );

    loop {
        let mut nodes = graph.iter();

        let start_node = nodes.next().unwrap();

        let mut a = start_node.clone();
        let mut b = nodes.cloned().collect::<HashSet<_>>();

        if b.is_empty() {
            break;
        }

        let mut s = start_node.clone();

        let mut neighbors = calculate_neighbors(&edges, &a, &b);

        loop {
            if b.len() == 1 {
                let t = b.into_iter().next().unwrap();

                let cut = neighbors
                    .into_iter()
                    .map(|NeighborEdge(_, weight)| weight)
                    .sum();
                if min_cut > cut {
                    min_a = a.clone();
                    min_b = t.clone();
                    min_cut = cut;
                }

                graph.remove(&s);
                graph.remove(&t);

                s.add(t);

                graph.insert(s.clone());

                break;
            }

            let NeighborEdge(v, _) = neighbors.pop().unwrap();

            b.remove(&v);

            s = v;
            a.add(s.clone());

            neighbors = calculate_neighbors(&edges, &a, &b);
        }
    }

    assert_eq!(min_cut, 3);

    min_a.0.len() * min_b.0.len()
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(_input: &str) -> &'static str {
    "Happy Christmas!"
}

pub fn part_1() -> usize {
    solve_1(&INPUT)
}

pub fn part_2() -> &'static str {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&EXAMPLE_1), 54);
    }
}
