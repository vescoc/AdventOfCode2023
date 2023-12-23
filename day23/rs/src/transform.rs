use std::collections::{HashMap, HashSet};
use std::ops;

pub type EdgesPoint<T> = HashMap<T, HashSet<(T, usize)>>;
pub type Edges<T> = HashMap<T, Vec<(T, usize)>>;

pub trait Set<T> {
    fn insert(&mut self, element: T);
    fn contains(&self, element: &T) -> bool;
}

impl<T> Set<T> for T
where
    T: ops::Shl<Output = T>,
    T: ops::BitOrAssign,
    T: ops::BitAnd<T, Output = T>,
    T: From<u16>,
    T: PartialEq<T>,
    T: Copy,
{
    fn insert(&mut self, element: T) {
        *self |= T::from(1) << element;
    }

    fn contains(&self, element: &T) -> bool {
        *self & (T::from(1) << *element) != T::from(0)
    }
}

pub fn transform(
    nrows: usize,
    ncols: usize,
    nodes: HashSet<(usize, usize)>,
    edges: EdgesPoint<(usize, usize)>,
) -> (Edges<u64>, u64, u64) {
    let lookup = nodes
        .into_iter()
        .enumerate()
        .map(|(i, node)| (node, i as u64))
        .collect::<HashMap<_, _>>();

    (
        edges
            .into_iter()
            .map(|(node, neighbors)| {
                (
                    lookup[&node],
                    neighbors
                        .into_iter()
                        .map(|(node, len)| (lookup[&node], len))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>(),
        lookup[&(1, 0)],
        lookup[&(ncols - 2, nrows - 1)],
    )
}
