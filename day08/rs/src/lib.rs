use lazy_static::lazy_static;

use num::integer::lcm;
use std::collections::HashMap;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse(input: &str) -> (&str, Network) {
    let mut parts = input.split("\n\n");

    let path = parts.next().expect("invalid input, cannot find path");

    let network = parts
        .next()
        .expect("invalid input, cannot find network")
        .lines()
        .map(|line| {
            let (source_node, destination_nodes) = line[..line.len() - 1]
                .split_once(" = (")
                .expect("invalid line");
            let (left_node, right_node) = destination_nodes
                .split_once(", ")
                .expect("invalid destination");
            (source_node, (left_node, right_node))
        })
        .collect::<HashMap<_, _>>();

    (path, network)
}

#[inline(always)]
fn steps(
    path: &str,
    network: &Network,
    start_node: &str,
    end_condition: impl Fn(&str) -> bool,
) -> u64 {
    path.chars()
        .cycle()
        .scan(start_node, |state, direction| {
            *state = {
                if direction == 'L' {
                    network[state].0
                } else {
                    network[state].1
                }
            };
            Some(*state)
        })
        .position(end_condition)
        .unwrap() as u64
        + 1
}

pub fn solve_1(input: &str) -> u64 {
    let (path, network) = parse(input);

    steps(path, &network, "AAA", |current| current == "ZZZ")
}

pub fn solve_2(input: &str) -> u64 {
    let (path, network) = parse(input);

    let from_a_to_z = |&node: &&str| {
        if node.ends_with('A') {
            Some(steps(path, &network, node, |current| {
                current.ends_with('Z')
            }))
        } else {
            None
        }
    };

    #[cfg(feature = "rayon")]
    let result = network
        .par_iter()
        .filter_map(|(node, _)| from_a_to_z(node))
        .reduce(|| 1, lcm);

    #[cfg(not(feature = "rayon"))]
    let result = network.keys().filter_map(from_a_to_z).reduce(lcm).unwrap();

    result
}

pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
        static ref EXAMPLE_2: &'static str = include_str!("../../example2");
        static ref EXAMPLE_3: &'static str = include_str!("../../example3");
    }

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(&EXAMPLE_1), 2);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(&EXAMPLE_2), 6);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_3), 6);
    }
}
