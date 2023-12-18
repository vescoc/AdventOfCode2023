#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::{ops::Mul, str::FromStr};

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err("invalid direction"),
        }
    }
}

impl Mul<i64> for Direction {
    type Output = (i64, i64);

    fn mul(self, length: i64) -> Self::Output {
        let (dx, dy) = match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        (dx * length, dy * length)
    }
}

/// Parse mode 1
///
/// # Panics
/// Panic if invalid input
fn parse_1(line: &str) -> (Direction, i64) {
    let mut parts = line.split_whitespace();

    let direction = parts
        .next()
        .expect("cannot find direction")
        .parse::<Direction>()
        .expect("invalid direction");
    let length = parts
        .next()
        .expect("cannot find length")
        .parse::<i64>()
        .expect("cannot parse length");

    (direction, length)
}

/// Parse mode 2
///
/// # Panics
/// Panic if invalid input
fn parse_2(line: &str) -> (Direction, i64) {
    let part = line
        .as_bytes()
        .split(|&c| c == b' ')
        .nth(2)
        .expect("invalid input");

    let length = i64::from_str_radix(
        std::str::from_utf8(part.get(2..2 + 5).expect("invalid length part"))
            .expect("invalid length part"),
        16,
    )
    .expect("invalid length");
    let direction = match part.get(2 + 5) {
        Some(b'0') => Direction::Right,
        Some(b'1') => Direction::Down,
        Some(b'2') => Direction::Left,
        Some(b'3') => Direction::Up,
        invalid => panic!("invalid direction: {invalid:?}"),
    };

    (direction, length)
}

/// Generic solve
///
/// # Panics
/// Panic if invalid input
fn solve<F: FnMut(&str) -> (Direction, i64)>(input: &str, parse: F) -> i64 {
    let (_, area, perimeter) = input.lines().map(parse).fold(
        ((0, 0), 0, 0),
        |(position, area, perimeter), (direction, length)| {
            let (dx, dy) = direction * length;

            let next = (position.0 + dx, position.1 + dy);

            (
                next,
                area + position.0 * next.1 - next.0 * position.1,
                perimeter + (position.0 - next.0).abs() + (position.1 - next.1).abs(),
            )
        },
    );

    (area + perimeter) / 2 + 1
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> i64 {
    solve(input, parse_1)
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> i64 {
    solve(input, parse_2)
}

pub fn part_1() -> i64 {
    solve_1(&INPUT)
}

pub fn part_2() -> i64 {
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
        assert_eq!(solve_1(&EXAMPLE_1), 62);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 952408144115);
    }
}
