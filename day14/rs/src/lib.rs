#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![cfg_attr(feature = "simd", feature(portable_simd))]

use lazy_static::lazy_static;

use std::collections::HashMap;

pub mod simple;
use simple::{cycle, load};

#[cfg(feature = "simd")]
pub mod simd;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

/// Parse input.
///
/// # Errors
/// Error if invalid input.
pub fn parse(input: &str) -> Result<(&[u8], usize, usize), &'static str> {
    let tiles = input.as_bytes();
    let ncols = tiles
        .iter()
        .position(|&t| t == b'\n')
        .ok_or("invalid input")?;
    let (tiles, nrows) = {
        let mut tiles = tiles;
        loop {
            let t = tiles[tiles.len() - 1];

            if t.is_ascii_whitespace() {
                tiles = &tiles[0..tiles.len() - 1];
            } else {
                break;
            }
        }

        (tiles, (tiles.len() + 1) / (ncols + 1))
    };

    Ok((tiles, ncols, nrows))
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> usize {
    let (tiles, ncols, nrows) = parse(input).expect("invalid input");

    (0..ncols)
        .zip(vec![nrows; ncols].iter_mut())
        .map(|(c, state)| {
            (0..nrows)
                .map(|r| match tiles.get((ncols + 1) * r + c) {
                    Some(b'O') => {
                        let weight = *state;
                        *state -= 1;
                        weight
                    }
                    Some(b'#') => {
                        *state = nrows - (r + 1);
                        0
                    }
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

/// Solve part 2
///
/// # Panics
/// Panic if input is invalid
pub fn solve_2(input: &str) -> usize {
    let (tiles, ncols, nrows) = parse(input).expect("invalid input");

    let mut history: HashMap<Vec<u8>, usize> = HashMap::with_capacity(1_024);
    let mut tiles = tiles.to_owned();
    for i in 0.. {
        tiles = cycle(tiles, ncols, nrows);
        if let Some(old_i) = history.get(&tiles) {
            let t = old_i - 1 + (1_000_000_000 - old_i) % (i - old_i);

            return history
                .iter()
                .find_map(|(tiles, &i)| {
                    if i == t {
                        Some(load(tiles, ncols, nrows))
                    } else {
                        None
                    }
                })
                .unwrap();
        }

        history.insert(tiles.clone(), i);
    }

    unreachable!()
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
        assert_eq!(solve_1(&EXAMPLE_1), 136);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 64);
    }
}
