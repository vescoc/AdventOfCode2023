#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

struct Map<'a> {
    data: &'a [u8],
    nrows: usize,
    ncols: usize,
}

impl<'a> Map<'a> {
    fn parse(input: &'a str) -> Result<Self, &'static str> {
        let mut data = input.as_bytes();
        while let Some(tile) = data.last() {
            if tile.is_ascii_whitespace() {
                data = &data[..data.len() - 1];
            } else {
                break;
            }
        }

        let ncols = data
            .iter()
            .position(u8::is_ascii_whitespace)
            .ok_or("invalid input map, cannot find columns")?;
        let nrows = (data.len() + 1) / (ncols + 1);

        Ok(Self { data, nrows, ncols })
    }
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn solve_1(input: &str) -> usize {
    let Map { data, nrows, ncols } = Map::parse(input).expect("invalid input");

    let mut longest_path_len = 0;
    let mut paths = vec![((1, 0), HashSet::new())];

    while let Some(((mut x, mut y), mut path)) = paths.pop() {
        loop {
            path.insert((x, y));

            if (x, y) == (ncols - 2, nrows - 1) {
                longest_path_len = longest_path_len.max(path.len());
                break;
            }

            let directions: &[(isize, isize)] = match data[x + y * (ncols + 1)] {
                b'^' => &[(0, -1)],
                b'>' => &[(1, 0)],
                b'v' => &[(0, 1)],
                b'<' => &[(-1, 0)],
                b'.' => &[(0, -1), (1, 0), (0, 1), (-1, 0)],
                _ => unreachable!(),
            };
            let mut next = directions.iter().filter_map(|(dx, dy)| {
                let (x, y) = (x as isize + dx, y as isize + dy);
                if (0..ncols as isize).contains(&x)
                    && (0..nrows as isize).contains(&y)
                    && data[x as usize + y as usize * (ncols + 1)] != b'#'
                    && !path.contains(&(x as usize, y as usize))
                {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            });
            if let Some((new_x, new_y)) = next.next() {
                for p in next {
                    paths.push((p, path.clone()));
                }
                (x, y) = (new_x, new_y);
            } else {
                break;
            }
        }
    }

    longest_path_len - 1
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn edges(
    data: &[u8],
    nrows: usize,
    ncols: usize,
    nodes: &HashSet<(usize, usize)>,
    start: (usize, usize),
) -> HashSet<((usize, usize), usize)> {
    let mut edges = HashSet::new();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((node, len)) = queue.pop_front() {
        if node != start && nodes.contains(&node) {
            edges.insert((node, len));
        } else {
            let (x, y) = (node.0 as isize, node.1 as isize);
            for (x, y) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
                if (0..ncols as isize).contains(&x)
                    && (0..nrows as isize).contains(&y)
                    && data[x as usize + y as usize * (ncols + 1)] != b'#'
                {
                    let node = (x as usize, y as usize);
                    if !visited.contains(&node) {
                        visited.insert(node);
                        queue.push_back((node, len + 1));
                    }
                }
            }
        }
    }

    edges
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn solve_2(input: &str) -> usize {
    let Map { data, nrows, ncols } = Map::parse(input).expect("invalid input");

    let nodes = (1..ncols - 1)
        .flat_map(|x| {
            (1..nrows - 1).filter_map(move |y| {
                if data[x + y * (ncols + 1)] != b'#'
                    && [(0, 1), (0, -1), (1, 0), (-1, 0)]
                        .iter()
                        .filter(|(dx, dy)| {
                            let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                            data[x + y * (ncols + 1)] != b'#'
                        })
                        .count()
                        > 2
                {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .chain([(1, 0), (ncols - 2, nrows - 1)])
        .collect::<HashSet<_>>();

    let graph = nodes
        .iter()
        .map(|&node| (node, edges(data, nrows, ncols, &nodes, node)))
        .collect::<HashMap<_, _>>();

    let mut longest_path_len = 0;

    let mut paths = vec![((1, 0), HashSet::new(), 0)];
    while let Some((mut node, mut path, mut len)) = paths.pop() {
        loop {
            path.insert(node);

            if node == (ncols - 2, nrows - 1) {
                longest_path_len = longest_path_len.max(len);
                break;
            }

            let mut next = graph[&node].iter().filter(|(node, _)| !path.contains(node));

            if let Some((new_node, weight)) = next.next() {
                for (new_node, new_weight) in next {
                    paths.push((*new_node, path.clone(), len + new_weight));
                }

                node = *new_node;
                len += weight;
            } else {
                break;
            }
        }
    }

    longest_path_len
}

pub fn part_1() -> usize {
    solve_1(&INPUT)
}

pub fn part_2() -> usize {
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
        assert_eq!(solve_1(&EXAMPLE_1), 94);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 154);
    }
}
