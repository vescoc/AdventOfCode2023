#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::str::FromStr;

use lazy_static::lazy_static;

#[cfg(feature = "z3")]
mod z3;
#[cfg(feature = "z3")]
pub use z3::{solve_2, part_2};

#[cfg(not(feature = "z3"))]
mod simple;
#[cfg(not(feature = "z3"))]
pub use simple::{solve_2, part_2};

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, PartialEq)]
struct Hail2 {
    position: [f64; 2],
    velocity: [f64; 2],
    x: f64,
    y: f64,
    q: f64,
}

impl FromStr for Hail2 {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = input.split_once(" @ ").ok_or("invalid hail")?;

        let parse = |part: &str| {
            let mut part = part.split(", ");
            Ok([
                part.next()
                    .ok_or("cannot find x")?
                    .trim_start()
                    .parse::<f64>()
                    .map_err(|_| "invalid x")?,
                part.next()
                    .ok_or("cannot find y")?
                    .trim_start()
                    .parse::<f64>()
                    .map_err(|_| "invalid y")?,
            ])
        };

        Ok(Self::new(parse(position)?, parse(velocity)?))
    }
}

impl Hail2 {
    fn new(position: [f64; 2], velocity: [f64; 2]) -> Self {
        let dx = velocity[0];
        let dy = velocity[1];

        let (x, y, q) = {
            if dx == 0. {
                assert!(dy > 0.);
                (1., 0., position[0])
            } else if dy == 0. {
                assert!(dx > 0.);
                (0., 1., position[1])
            } else {
                (1. / dx, -1. / dy, position[0] / dx - position[1] / dy)
            }
        };

        Self {
            position,
            velocity,
            x,
            y,
            q,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Hail3 {
    position: [f64; 3],
    velocity: [f64; 3],
}

impl FromStr for Hail3 {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = input.split_once(" @ ").ok_or("invalid hail")?;

        let parse = |part: &str| {
            let mut part = part.split(", ");
            Ok([
                part.next()
                    .ok_or("cannot find x")?
                    .trim_start()
                    .parse::<f64>()
                    .map_err(|_| "invalid x")?,
                part.next()
                    .ok_or("cannot find y")?
                    .trim_start()
                    .parse::<f64>()
                    .map_err(|_| "invalid y")?,
                part.next()
                    .ok_or("cannot find z")?
                    .trim_start()
                    .parse::<f64>()
                    .map_err(|_| "invalid z")?,
            ])
        };

        Ok(Self::new(parse(position)?, parse(velocity)?))
    }
}

impl Hail3 {
    fn new(position: [f64; 3], velocity: [f64; 3]) -> Self {
        Self { position, velocity }
    }
}

/// Solve part 1 with range
///
/// # Panics
/// Panic if invalid input
fn solve_1_with_range(input: &str, start: f64, end: f64) -> usize {
    let hails = input
        .lines()
        .map(Hail2::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("invalid input");

    let mut count = 0;
    for (i, a) in hails.iter().enumerate().take(hails.len() - 1) {
        for b in hails.iter().skip(i + 1) {
            let d = a.x * b.y - a.y * b.x;
            if d == 0. {
                if (a.x - b.x).abs() < f64::EPSILON
                    && (a.y - b.y).abs() < f64::EPSILON
                    && (a.q - b.q).abs() < f64::EPSILON
                {
                    count += 1;
                }
            } else {
                let (x, y) = ((a.q * b.y - a.y * b.q) / d, (a.x * b.q - a.q * b.x) / d);
                if (start..=end).contains(&x) && (start..=end).contains(&y) {
                    let ta = if a.x != 0. {
                        (x - a.position[0]) / a.x
                    } else if a.y != 0. {
                        (y - a.position[1]) / a.y
                    } else {
                        unreachable!()
                    };

                    let tb = if b.x != 0. {
                        (x - b.position[0]) / b.x
                    } else if b.y != 0. {
                        (y - b.position[1]) / b.y
                    } else {
                        unreachable!()
                    };

                    if ta >= 0. && tb >= 0. {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
#[allow(clippy::unreadable_literal)]
pub fn solve_1(input: &str) -> usize {
    solve_1_with_range(input, 200000000000000., 400000000000000.)
}

pub fn part_1() -> usize {
    solve_1(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1_with_range(&EXAMPLE_1, 7., 27.), 2);
    }
}
