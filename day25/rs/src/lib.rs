#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::{fmt, iter};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

const MAX_N: usize = 2_048 * 2;

type Graph = HashMap<usize, HashMap<usize, usize>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct BitSet {
    data: Vec<u128>,
    len: usize,
}

impl BitSet {
    const BITS: usize = u128::BITS as usize;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity / Self::BITS + usize::from(capacity % Self::BITS == 0)],
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn contains(&self, value: usize) -> bool {
        if let Some(mask) = self.data.get(value / Self::BITS) {
            mask & 1 << (value % Self::BITS) != 0
        } else {
            false
        }
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        let mut current = 0;
        iter::from_fn(move || {
            while let Some(mask) = self.data.get(current / Self::BITS) {
                if mask & 1 << (current % Self::BITS) == 0 {
                    current += 1;
                } else {
                    let r = Some(current);
                    current += 1;
                    return r;
                }
            }
            None
        })
    }

    fn insert(&mut self, value: usize) {
        let i = value / Self::BITS;
        while self.data.len() <= i {
            self.data.push(0);
        }
        let mask = &mut self.data[i];
        let b = 1 << (value % Self::BITS);
        self.len += usize::from(*mask & b == 0);
        *mask |= b;
    }

    fn remove(&mut self, value: usize) {
        if let Some(mask) = self.data.get_mut(value / Self::BITS) {
            let b = 1 << (value % Self::BITS);
            self.len -= usize::from(*mask & b != 0);
            *mask &= !b;
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn into_iter(self) -> impl Iterator<Item = usize> {
        let mut current = 0;
        iter::from_fn(move || {
            while let Some(mask) = self.data.get(current / Self::BITS) {
                if mask & 1 << (current % Self::BITS) == 0 {
                    current += 1;
                } else {
                    let r = Some(current);
                    current += 1;
                    return r;
                }
            }
            None
        })
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<II: IntoIterator<Item = usize>>(ii: II) -> Self {
        let mut data = Vec::new();
        let mut len = 0;
        for value in ii {
            let i = value / Self::BITS;
            while data.len() <= i {
                data.push(0);
            }
            let mask = &mut data[i];
            let b = 1 << (value % Self::BITS);
            len += usize::from(*mask & b == 0);
            *mask |= b;
        }

        BitSet { data, len }
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{{")?;

        let mut first = true;
        for value in self.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }

            write!(f, "{value}")?;
        }
        write!(f, "}}")?;

        Ok(())
    }
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

fn min_neighbor(edges: &Graph, max_id: usize, a: &BitSet, b: &BitSet) -> usize {
    let mut neighbors = [0; MAX_N];
    let (v, _) = {
        if a.len() > b.len() {
            b.iter()
                .flat_map(|end| {
                    edges[&end].iter().filter_map(move |(start, weight)| {
                        if a.contains(*start) {
                            Some((end, *weight))
                        } else {
                            None
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
            a.iter()
                .flat_map(|start| {
                    edges[&start].iter().filter_map(|(end, weight)| {
                        if b.contains(*end) {
                            Some((*end, *weight))
                        } else {
                            None
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

        let mut a = BitSet::with_capacity(next_id);
        a.insert(start_node);

        let mut b = nodes.copied().collect::<BitSet>();
        if b.is_empty() {
            break;
        }

        let mut s = start_node;

        loop {
            if b.len() == 1 {
                let t = b.into_iter().next().unwrap();

                let cut = edges[&t].values().sum();

                if min_cut > cut {
                    min_a = a.iter().flat_map(|id| dictionary[&id].clone()).collect();
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

            b.remove(v);

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
