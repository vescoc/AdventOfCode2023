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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn iter_split(&self) -> impl Iterator<Item = usize> + '_ {
        let (mut mask, mut rest) = self
            .data
            .split_first()
            .map_or((None, &self.data[..]), |(mask, rest)| (Some(mask), rest));
        let mut index = 0;
        let mut b = 1;
        let mut index_b = 0;
        iter::from_fn(move || {
            while let Some(m) = mask {
                if *m == 0 {
                    b = 1;
                    index_b = 0;
                    index += 1;
                    (mask, rest) = rest
                        .split_first()
                        .map_or((None, rest), |(mask, rest)| (Some(mask), rest));
                } else {
                    loop {
                        if m & b == 0 {
                            index_b += 1;
                            if index_b == Self::BITS {
                                b = 1;
                                index_b = 0;
                                index += 1;
                                (mask, rest) = rest
                                    .split_first()
                                    .map_or((None, rest), |(mask, rest)| (Some(mask), rest));
                                break;
                            }

                            b <<= 1;
                        } else {
                            let r = Some(index * Self::BITS + index_b);
                            index_b += 1;
                            if index_b == Self::BITS {
                                b = 1;
                                index_b = 0;
                                index += 1;
                                (mask, rest) = rest
                                    .split_first()
                                    .map_or((None, rest), |(mask, rest)| (Some(mask), rest));
                            } else {
                                b <<= 1;
                            }

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
}

struct BitSetIter {
    mask: Option<BitSetType>,
    rest: std::vec::IntoIter<BitSetType>,
    index: usize,
    b: BitSetType,
    index_b: usize,
}

impl Iterator for BitSetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mask) = self.mask {
            if mask == 0 {
                self.b = 1;
                self.index_b = 0;
                self.index += 1;
                self.mask = self.rest.next();
            } else {
                loop {
                    if mask & self.b == 0 {
                        self.index_b += 1;
                        if self.index_b == BitSet::BITS {
                            self.b = 1;
                            self.index_b = 0;
                            self.index += 1;
                            self.mask = self.rest.next();
                            break;
                        }

                        self.b <<= 1;
                    } else {
                        let r = Some(self.index * BitSet::BITS + self.index_b);
                        self.index_b += 1;
                        if self.index_b == BitSet::BITS {
                            self.b = 1;
                            self.index_b = 0;
                            self.index += 1;
                            self.mask = self.rest.next();
                        } else {
                            self.b <<= 1;
                        }
                        return r;
                    }
                }
            }
        }
        None
    }
}

impl IntoIterator for BitSet {
    type Item = usize;
    type IntoIter = BitSetIter;

    fn into_iter(self) -> Self::IntoIter {
        let mut rest = self.data.into_iter();
        let mask = rest.next();
        let index = 0;
        let b = 1;
        let index_b = 0;

        BitSetIter {
            mask,
            rest,
            index,
            b,
            index_b,
        }
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

    let n_vertex = edges.len();

    let (mut edges, mut dictionary, mut next_id) = transform(&edges);

    let (mut min_b, mut min_cut) = (usize::MAX, usize::MAX);

    // let mut count = 0;
    loop {
        // count += 1;
        // println!("{count}");

        let mut b = edges.nodes.clone();

        let start_node = b.iter().next().unwrap();

        let mut a = BitSet::new();
        a.insert(start_node);
        b.remove(start_node);

        if b.is_empty() {
            break;
        }

        let mut s = start_node;

        let mut heap = BinaryHeap::new();

        loop {
            if b.len() == 1 {
                let t = b.into_iter().next().unwrap();
                //let t = b.iter().next().unwrap();

                let cut = edges.neighbors(t).sum();
                if min_cut > cut {
                    min_b = t;
                    min_cut = cut;
                }

                let st = next_id;
                next_id += 1;

                edges.merge(s, t, st);

                let mut sts = dictionary[&s].clone();
                sts.append(
                    &mut dictionary
                        .get(&t)
                        .unwrap_or_else(|| panic!("cannot find key: {t}"))
                        .clone(),
                );
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

    let min_b_len = dictionary[&min_b].len();
    (n_vertex - min_b_len) * min_b_len
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

    #[test]
    fn bitset_iter_next() {
        let mut set = BitSet::new();
        set.insert(BitSet::BITS + 1);
        let mut i = set.iter();
        assert_eq!(i.next(), Some(BitSet::BITS + 1));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn bitset_into_iter_next() {
        let mut set = BitSet::new();
        set.insert(BitSet::BITS + 1);

        let mut i = set.into_iter();
        assert_eq!(i.next(), Some(BitSet::BITS + 1));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn bitset_iter() {
        let e = 200;
        let set = (0..e).collect::<BitSet>();

        let mut i = set.iter();

        for v in 0..e {
            assert_eq!(i.next(), Some(v));
        }
    }
}
