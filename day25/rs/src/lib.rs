#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

const MAX_N: usize = 2_048 * 2;

type Graph = HashMap<usize, HashMap<usize, usize>>;

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
struct NeighborEdge<T>(T, usize);

impl<T: Ord> PartialOrd for NeighborEdge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for NeighborEdge<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

fn transform<'a>(
    edges: &HashMap<&'a str, HashSet<&'a str>>,
) -> (Graph, HashMap<usize, Vec<&'a str>>, usize) {
    let nodes = edges.keys().copied().collect::<Vec<_>>();
    let dictionary = nodes
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (i, vec![n]))
        .collect::<HashMap<_, _>>();
    let str2id = nodes
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    let next_id = dictionary.keys().max().unwrap() + 1;

    (
        edges
            .iter()
            .map(|(n, es)| (str2id[n], es.iter().map(|n| (str2id[n], 1)).collect()))
            .collect(),
        dictionary,
        next_id,
    )
}

fn min_neighbor(edges: &Graph, max_id: usize, a: &HashSet<usize>, b: &HashSet<usize>) -> usize {
    const BITS: usize = u128::BITS as usize;

    let mut neighbors = [0; MAX_N];
    let (v, _) = {
        let mut mask = [0_u128; MAX_N / BITS + 1];
        if a.len() > b.len() {
            for node in a {
                mask[node / BITS] |= 1 << (node % BITS);
            }

            b.iter()
                .flat_map(|end| {
                    edges[end].iter().filter_map(|(start, weight)| {
                        if mask[start / BITS] & 1 << (start % BITS) == 0 {
                            None
                        } else {
                            Some((*end, *weight))
                        }
                    })
                })
                .fold(&mut neighbors, |acc, (node, weight)| {
                    acc[node] += weight;
                    acc
                })
                .iter()
                .enumerate()
                .take(max_id)
                .max_by_key(|(_, w)| *w)
                .unwrap()
        } else {
            for node in b {
                mask[node / BITS] |= 1 << (node % BITS);
            }

            a.iter()
                .flat_map(|start| {
                    edges[start].iter().filter_map(|(end, weight)| {
                        if mask[end / BITS] & 1 << (end % BITS) == 0 {
                            None
                        } else {
                            Some((*end, *weight))
                        }
                    })
                })
                .fold(&mut neighbors, |acc, (node, weight)| {
                    acc[node] += weight;
                    acc
                })
                .iter()
                .enumerate()
                .take(max_id)
                .max_by_key(|(_, w)| *w)
                .unwrap()
        }
    };

    v
}

/// Solve part 1.
///
/// Using Stoer-Wagner algorithm.
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

    let (mut edges, mut dictionary, mut next_id) = transform(&edges);

    let (mut min_a, mut min_b, mut min_cut) = (vec![], vec![], usize::MAX);

    // let mut count = 0;
    loop {
        // count += 1;
        // println!("{count}");

        let mut nodes = edges.keys();

        let start_node = nodes.next().copied().unwrap();

        let mut a = set! {start_node};
        let mut b = nodes.copied().collect::<HashSet<_>>();
        if b.is_empty() {
            break;
        }

        let mut s = start_node;

        loop {
            if b.len() == 1 {
                let t = b.into_iter().next().unwrap();

                let cut = edges[&t].values().sum();

                if min_cut > cut {
                    min_a = a.iter().flat_map(|id| dictionary[id].clone()).collect();
                    min_b = dictionary[&t].clone();
                    min_cut = cut;
                }

                let ss = edges
                    .remove(&s)
                    .unwrap()
                    .keys()
                    .copied()
                    .collect::<HashSet<_>>();
                let ts = edges
                    .remove(&t)
                    .unwrap()
                    .keys()
                    .copied()
                    .collect::<HashSet<_>>();

                let oes = ss.union(&ts);

                let st = next_id;
                next_id += 1;

                let mut es = HashMap::new();
                for node in oes {
                    if let Some(ee) = edges.get_mut(node) {
                        let weight = ee.remove(&s).unwrap_or(0) + ee.remove(&t).unwrap_or(0);
                        ee.insert(st, weight);
                        es.insert(*node, weight);
                    }
                }
                edges.insert(st, es);

                let mut sts = dictionary[&s].clone();
                sts.append(&mut dictionary[&t].clone());
                dictionary.insert(st, sts);

                break;
            }

            let v = min_neighbor(&edges, next_id, &a, &b);

            b.remove(&v);

            s = v;
            a.insert(s);
        }
    }

    assert!(min_cut == 3);

    min_a.len() * min_b.len()
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
