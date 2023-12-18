#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, Eq)]
struct Node {
    position: (usize, usize),
    cost: u32,
    direction_strength: usize,
    direction: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cost == other.cost
    }
}

impl Node {
    fn new(
        position: (usize, usize),
        cost: u32,
        direction_strength: usize,
        direction: usize,
    ) -> Self {
        Self {
            position,
            cost,
            direction_strength,
            direction,
        }
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn neighbors<'a, 'b: 'a, 'c: 'a>(
        &'a self,
        map: &'c Map<'b>,
    ) -> impl Iterator<Item = (usize, (usize, usize), u32, usize)> + 'a {
        [
            (3, (-1, 0)),
            (0, (0, -1)),
            (1, (1, 0)),
            (2, (0, 1)),
            (3, (-1, 0)),
            (0, (0, 1)),
        ][self.direction..=self.direction + 2]
            .iter()
            .filter_map(move |(direction, (dx, dy))| {
                if *direction == self.direction && self.direction_strength == 3 {
                    None
                } else {
                    let (x, y) = (self.position.0 as isize + dx, self.position.1 as isize + dy);
                    map.get((x as usize, y as usize))
                        .map(|cost| (*direction, (x as usize, y as usize), cost, 1))
                }
            })
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn neighbors_ultracrucible<'a, 'b: 'a, 'c: 'a>(
        &'a self,
        map: &'c Map<'b>,
    ) -> impl Iterator<Item = (usize, (usize, usize), u32, usize)> + 'a {
        [
            (3, (-1, 0)),
            (0, (0, -1)),
            (1, (1, 0)),
            (2, (0, 1)),
            (3, (-1, 0)),
            (0, (0, 1)),
        ][self.direction..=self.direction + 2]
            .iter()
            .filter_map(move |(direction, (dx, dy))| {
                if *direction == self.direction {
                    if self.direction_strength == 10 {
                        None
                    } else {
                        assert!(
                            self.direction_strength >= 4,
                            "current value: {}",
                            self.direction_strength
                        );

                        let steps = 1;

                        (1..=steps)
                            .try_fold((0, (0, 0)), |(acc, _), step| {
                                let (x, y) = (
                                    self.position.0 as isize + dx * step,
                                    self.position.1 as isize + dy * step,
                                );
                                map.get((x as usize, y as usize))
                                    .map(|cost| (acc + cost, (x as usize, y as usize)))
                            })
                            .map(|(cost, position)| (*direction, position, cost, steps as usize))
                    }
                } else {
                    assert!(
                        self.direction_strength >= 4,
                        "current value: {}",
                        self.direction_strength
                    );

                    let steps = 4;

                    (1..=steps)
                        .try_fold((0, (0, 0)), |(acc, _), step| {
                            let (x, y) = (
                                self.position.0 as isize + dx * step,
                                self.position.1 as isize + dy * step,
                            );
                            map.get((x as usize, y as usize))
                                .map(|cost| (acc + cost, (x as usize, y as usize)))
                        })
                        .map(|(cost, position)| (*direction, position, cost, steps as usize))
                }
            })
    }
}

struct Map<'a> {
    data: &'a [u8],
    ncols: usize,
    nrows: usize,
}

impl<'a> Map<'a> {
    fn parse(input: &'a str) -> Result<Self, &'static str> {
        let data = {
            let mut data = input.as_bytes();
            while !data.is_empty() && data[data.len() - 1].is_ascii_whitespace() {
                data = &data[0..data.len() - 1];
            }
            data
        };

        let Some(ncols) = data.iter().position(u8::is_ascii_whitespace) else {
            return Err("invalid input");
        };

        let nrows = (data.len() + 1) / (ncols + 1);

        Ok(Self { data, ncols, nrows })
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<u32> {
        if (0..self.ncols).contains(&x) && (0..self.nrows).contains(&y) {
            Some(u32::from(self.data[x + (self.ncols + 1) * y] - b'0'))
        } else {
            None
        }
    }
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> u32 {
    let map = Map::parse(input).expect("invalid input");

    let mut visited = HashMap::with_capacity(map.ncols * map.nrows);
    visited.insert((0, 0), 0);

    let mut dist = HashMap::with_capacity(map.ncols * map.nrows);

    let mut current = BinaryHeap::new();
    current.push(Node::new((0, 1), map.get((0, 1)).unwrap(), 1, 2));
    current.push(Node::new((1, 0), map.get((1, 0)).unwrap(), 1, 1));

    while let Some(node) = current.pop() {
        visited
            .entry(node.position)
            .and_modify(|v| *v = node.cost.min(*v))
            .or_insert(node.cost);

        if visited.contains_key(&(map.nrows - 1, map.ncols - 1)) {
            break;
        }

        if let Some(&cost) = dist.get(&(node.direction, node.position, node.direction_strength)) {
            if cost < node.cost {
                continue;
            }
        }

        for (target_direction, target_position, cost, steps) in Node::neighbors(&node, &map) {
            let target_cost = node.cost + cost;
            let target_direction_strength = if target_direction == node.direction {
                node.direction_strength + steps
            } else {
                steps
            };

            let target_key = (target_direction, target_position, target_direction_strength);
            let current_cost = dist.get(&target_key).unwrap_or(&u32::MAX);
            if target_cost < *current_cost {
                let target = Node::new(
                    target_position,
                    target_cost,
                    target_direction_strength,
                    target_direction,
                );
                current.push(target);
                dist.insert(target_key, target_cost);
            }
        }
    }

    visited[&(map.ncols - 1, map.nrows - 1)]
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> u32 {
    let map = Map::parse(input).expect("invalid input");

    let mut visited = HashMap::with_capacity(map.ncols * map.nrows);
    visited.insert((0, 0), 0);

    let mut dist = HashMap::with_capacity(map.ncols * map.nrows);

    let mut nodes = BinaryHeap::new();

    let zero_four_cost = (1..=4).filter_map(|y| map.get((0, y))).sum();
    let four_zero_cost = (1..=4).filter_map(|x| map.get((x, 0))).sum();

    nodes.push(Node::new((0, 4), zero_four_cost, 4, 2));
    nodes.push(Node::new((4, 0), four_zero_cost, 4, 1));

    while let Some(node) = nodes.pop() {
        visited
            .entry(node.position)
            .and_modify(|v| *v = node.cost.min(*v))
            .or_insert(node.cost);

        if visited.contains_key(&(map.nrows - 1, map.ncols - 1)) {
            break;
        }

        if let Some(&cost) = dist.get(&(node.direction, node.position, node.direction_strength)) {
            if cost < node.cost {
                continue;
            }
        }

        for (target_direction, target_position, cost, steps) in node.neighbors_ultracrucible(&map) {
            let target_cost = node.cost + cost;
            let target_direction_strength = if target_direction == node.direction {
                node.direction_strength + steps
            } else {
                steps
            };

            assert!((4..=10).contains(&target_direction_strength));

            let target_key = (target_direction, target_position, target_direction_strength);
            let current_cost = dist.get(&target_key).unwrap_or(&u32::MAX);
            if target_cost < *current_cost {
                let target = Node::new(
                    target_position,
                    target_cost,
                    target_direction_strength,
                    target_direction,
                );
                nodes.push(target);
                dist.insert(target_key, target_cost);
            }
        }
    }

    visited[&(map.ncols - 1, map.nrows - 1)]
}

pub fn part_1() -> u32 {
    solve_1(&INPUT)
}

pub fn part_2() -> u32 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
        static ref EXAMPLE_2: &'static str = include_str!("../../example2");
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&EXAMPLE_1), 102);
    }

    #[test]
    fn same_results_2_1() {
        assert_eq!(solve_2(&EXAMPLE_1), 94);
    }

    #[test]
    fn same_results_2_2() {
        assert_eq!(solve_2(&EXAMPLE_2), 71);
    }
}
