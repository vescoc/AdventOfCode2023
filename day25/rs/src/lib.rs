#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{cmp, fmt, iter};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

const MAX_N: usize = 2_048 * 2;

#[derive(Debug, PartialEq, Eq)]
struct EdgeInfo(usize, usize);

impl cmp::PartialOrd for EdgeInfo {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for EdgeInfo {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
    }
}

struct Graph {
    n: usize,
    nodes: BitSet,
    edges: Vec<usize>,
}

impl Graph {
    fn neighbors(&self, node: usize) -> Edges {
        Edges {
            edges: &self.edges[node * self.n..(node + 1) * self.n],
            nodes: &self.nodes,
        }
    }

    fn nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.nodes.iter()
    }

    fn merge(&mut self, s: usize, t: usize, st: usize) {
        for (node, weight) in self.neighbors(s).iter().collect::<Vec<_>>() {
            self.edges[node + st * self.n] += weight;
            self.edges[st + node * self.n] += weight;
        }

        for (node, weight) in self.neighbors(t).iter().collect::<Vec<_>>() {
            self.edges[node + st * self.n] += weight;
            self.edges[st + node * self.n] += weight;
        }

        self.nodes.insert(st);
        self.nodes.remove(s);
        self.nodes.remove(t);
    }
}

struct Edges<'a> {
    edges: &'a [usize],
    nodes: &'a BitSet,
}

impl<'a> Edges<'a> {
    fn iter(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.nodes.iter().filter_map(|node| {
            let weight = self.edges[node];
            if weight > 0 {
                Some((node, weight))
            } else {
                None
            }
        })
    }

    fn sum(&self) -> usize {
        self.nodes.iter().map(|node| self.edges[node]).sum()
    }

    #[allow(dead_code)]
    fn nodes(&self) -> BitSet {
        self.iter().map(|(node, _)| node).collect::<BitSet>()
    }

    fn get(&self, node: usize) -> Option<usize> {
        if self.nodes.contains(node) {
            let weight = self.edges[node];
            if weight > 0 {
                Some(weight)
            } else {
                None
            }
        } else {
            None
        }
    }
}

type BitSetType = u64;

#[derive(Clone, PartialEq, Eq, Hash)]
struct BitSet {
    data: Vec<BitSetType>,
    len: usize,
}

impl BitSet {
    const BITS: usize = BitSetType::BITS as usize;

    fn new() -> Self {
        Self {
            data: vec![],
            len: 0,
        }
    }

    #[allow(dead_code)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity / Self::BITS + usize::from(capacity % Self::BITS == 0)],
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    #[allow(dead_code)]
    fn union(&self, other: &Self) -> BitSet {
        let n = self.data.len().max(other.data.len());
        let mut data = Vec::with_capacity(n);
        let mut len = 0;

        let ai = self.data.iter().copied().chain(iter::repeat(0));
        let bi = other.data.iter().copied().chain(iter::repeat(0));
        for (a, b) in ai.zip(bi).take(n) {
            let r = a | b;
            len += r.count_ones() as usize;
            data.push(r);
        }

        Self { data, len }
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
                if *mask == 0 {
                    current = (current / Self::BITS + 1) * Self::BITS;
                } else {
                    let mut b = 1 << (current % Self::BITS);
                    loop {
                        if mask & b == 0 {
                            current += 1;
                            if current % Self::BITS == 0 {
                                break;
                            }
                            b <<= 1;
                        } else {
                            let r = Some(current);
                            current += 1;
                            return r;
                        }
                    }
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
                if *mask == 0 {
                    current = (current / Self::BITS + 1) * Self::BITS;
                } else {
                    let mut b = 1 << (current % Self::BITS);
                    loop {
                        if mask & b == 0 {
                            current += 1;
                            if current % Self::BITS == 0 {
                                break;
                            }
                            b <<= 1;
                        } else {
                            let r = Some(current);
                            current += 1;
                            return r;
                        }
                    }
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

    let n = next_id * 2;
    let nodes = dictionary.keys().copied().collect::<BitSet>();
    let edges = {
        let mut res = vec![0; n * n];
        for (start, es) in edges {
            for end in es {
                res[str2id[start] + str2id[end] * n] = 1;
            }
        }
        res
    };

    (Graph { n, nodes, edges }, dictionary, next_id)
}

fn min_neighbor(edges: &Graph, a: &BitSet, b: &BitSet, heap: &mut BinaryHeap<EdgeInfo>) -> usize {
    if heap.is_empty() {
        let mut neighbors = [0; MAX_N];
        let mut n = 0;
        if a.len() > b.len() {
            for end in b.iter() {
                for (start, weight) in edges.neighbors(end).iter() {
                    if a.contains(start) {
                        neighbors[end] += weight;
                        n = n.max(end);
                    }
                }
            }
        } else {
            for start in a.iter() {
                for (end, weight) in edges.neighbors(start).iter() {
                    if b.contains(end) {
                        neighbors[end] += weight;
                        n = n.max(end);
                    }
                }
            }
        }

        *heap = neighbors
            .iter()
            .take(n + 1)
            .enumerate()
            .filter_map(|(n, &w)| if w > 0 { Some(EdgeInfo(n, w)) } else { None })
            .collect::<BinaryHeap<_>>();
    }

    heap.pop().unwrap().0
}

fn adjust_data(
    edges: &Graph,
    s: usize,
    a: &mut BitSet,
    b: &mut BitSet,
    heap: &mut BinaryHeap<EdgeInfo>,
) {
    assert!(
        heap.iter().all(|EdgeInfo(v, _)| !a.contains(*v)),
        "invalid heap pre"
    );

    // remove from heap every edge n in a | n -> s
    // add into heap every edge n in s neighbors not a
    // adjust weight edges in a with weigh s

    let s_edges = edges.neighbors(s);
    let mut s_neighbors = s_edges.nodes();

    let mut es = vec![];
    heap.retain(|EdgeInfo(v, w)| {
        if *v == s {
            false
        } else if let Some(weight) = s_edges.get(*v) {
            es.push((*v, w + weight));
            s_neighbors.remove(*v);
            false
        } else {
            true
        }
    });

    for (e, w) in es {
        heap.push(EdgeInfo(e, w));
    }

    for n in s_neighbors.iter() {
        if !a.contains(n) {
            if let Some(w) = s_edges.get(n) {
                heap.push(EdgeInfo(n, w));
            }
        }
    }

    b.remove(s);
    a.insert(s);

    assert!(
        heap.iter().all(|EdgeInfo(v, _)| !a.contains(*v)),
        "invalid heap post"
    );
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

        let mut nodes = edges.nodes();

        let start_node = nodes.next().unwrap();

        let mut a = BitSet::new();
        a.insert(start_node);

        let mut b = nodes.collect::<BitSet>();
        if b.is_empty() {
            break;
        }

        let mut s = start_node;

        let mut heap = BinaryHeap::new();

        loop {
            if b.len() == 1 {
                let t = b.into_iter().next().unwrap();

                let cut = edges.neighbors(t).sum();
                if min_cut > cut {
                    min_a = a.iter().flat_map(|id| dictionary[&id].clone()).collect();
                    min_b = dictionary[&t].clone();
                    min_cut = cut;
                }

                let st = next_id;
                next_id += 1;

                edges.merge(s, t, st);

                let mut sts = dictionary[&s].clone();
                sts.append(&mut dictionary[&t].clone());
                dictionary.insert(st, sts);

                break;
            }

            let v = min_neighbor(&edges, &a, &b, &mut heap);

            s = v;

            adjust_data(&edges, s, &mut a, &mut b, &mut heap);

            // println!("{s} {a:?} {b:?} {heap:?}");
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
