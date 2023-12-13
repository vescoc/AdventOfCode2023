#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::str::from_utf8;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

struct Land<'a> {
    land: &'a [u8],
    ncols: usize,
}

impl<'a> Land<'a> {
    fn parse(land: &'a str) -> Result<Self, &'static str> {
        let land = land.as_bytes();
        let ncols = land
            .iter()
            .position(|&c| c == b'\n')
            .ok_or("invalid land")?;

        Ok(Self { land, ncols })
    }
}

fn find_horizontal_mirror(Land { land, ncols }: &Land) -> Option<usize> {
    let mut r = 1;
    loop {
        let found = (0..r).rev().zip(r..).all(|(lr, hr)| {
            let row_upper = land
                .get((ncols + 1) * lr..(ncols + 1) * lr + ncols)
                .unwrap();
            if let Some(row_lower) = land.get((ncols + 1) * hr..(ncols + 1) * hr + ncols) {
                row_upper == row_lower
            } else {
                true
            }
        });
        if found {
            break Some(r);
        }

        r += 1;
        if land.get((ncols + 1) * r).is_none() {
            break None;
        }
    }
}

#[allow(clippy::maybe_infinite_iter)]
fn find_vertical_mirror(Land { land, ncols }: &Land) -> Option<usize> {
    let mut c = 1;
    loop {
        let found = (0..c).rev().zip(c..*ncols).all(|(lc, hc)| {
            (0..)
                .scan((), |_, r| {
                    land.get((ncols + 1) * r + lc)
                        .and_then(|l| land.get((ncols + 1) * r + hc).map(|h| l == h))
                })
                .all(|v| v)
        });
        if found {
            break Some(c);
        }

        c += 1;
        if c >= *ncols {
            break None;
        }
    }
}

fn find_horizontal_mirror_1(Land { land, ncols }: &Land) -> Option<usize> {
    let mut r = 1;
    loop {
        let count = (0..r).rev().zip(r..).fold(0, |mut count, (lr, hr)| {
            let row_upper = land
                .get((ncols + 1) * lr..(ncols + 1) * lr + ncols)
                .unwrap();
            if let Some(row_lower) = land.get((ncols + 1) * hr..(ncols + 1) * hr + ncols) {
                count += row_upper
                    .iter()
                    .zip(row_lower.iter())
                    .map(|(l, r)| usize::from(l != r))
                    .sum::<usize>();
            }

            count
        });
        if count == 1 {
            break Some(r);
        }

        r += 1;
        if land.get((ncols + 1) * r).is_none() {
            break None;
        }
    }
}

#[allow(clippy::maybe_infinite_iter)]
fn find_vertical_mirror_1(Land { land, ncols }: &Land) -> Option<usize> {
    let mut c = 1;
    loop {
        let count = (0..c)
            .rev()
            .zip(c..*ncols)
            .flat_map(|(lc, hc)| {
                (0..).scan((), move |_, r| {
                    land.get((ncols + 1) * r + lc)
                        .and_then(|l| land.get((ncols + 1) * r + hc).map(|h| usize::from(l != h)))
                })
            })
            .sum::<usize>();
        if count == 1 {
            break Some(c);
        }

        c += 1;
        if c >= *ncols {
            break None;
        }
    }
}

fn solve(
    input: &str,
    find_horizontal_mirror: impl Fn(&Land) -> Option<usize> + Sync,
    find_vertical_mirror: impl Fn(&Land) -> Option<usize> + Sync,
) -> usize {
    #[cfg(not(feature = "rayon"))]
    let input = input.split("\n\n");

    #[cfg(feature = "rayon")]
    let input = input.split("\n\n").par_bridge();

    input
        .map(|land| {
            let land = Land::parse(land).expect("invalid input");

            find_horizontal_mirror(&land).map_or_else(
                || {
                    find_vertical_mirror(&land).unwrap_or_else(|| {
                        panic!(
                            "cannot find neither vertical or horizontal mirror: \n{}",
                            from_utf8(land.land).unwrap()
                        )
                    })
                },
                |columns| columns * 100,
            )
        })
        .sum()
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> usize {
    solve(input, find_horizontal_mirror, find_vertical_mirror)
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> usize {
    solve(input, find_horizontal_mirror_1, find_vertical_mirror_1)
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
        assert_eq!(solve_1(&EXAMPLE_1), 405);
    }

    #[test]
    fn test_horizontal_mirror_ok() {
        let (_, land) = EXAMPLE_1.split_once("\n\n").unwrap();

        assert_eq!(find_horizontal_mirror(&Land::parse(land).unwrap()), Some(4));
    }

    #[test]
    fn test_horizontal_mirror_ko() {
        let (land, _) = EXAMPLE_1.split_once("\n\n").unwrap();

        assert_eq!(find_horizontal_mirror(&Land::parse(land).unwrap()), None);
    }

    #[test]
    fn test_vertical_mirror_ko() {
        let (_, land) = EXAMPLE_1.split_once("\n\n").unwrap();

        assert_eq!(find_vertical_mirror(&Land::parse(land).unwrap()), None);
    }

    #[test]
    fn test_vertical_mirror_ok() {
        let (land, _) = EXAMPLE_1.split_once("\n\n").unwrap();

        assert_eq!(find_vertical_mirror(&Land::parse(land).unwrap()), Some(5));
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 400);
    }

    #[test]
    fn test_horizontal_mirror_1_ok() {
        let (land, _) = EXAMPLE_1.split_once("\n\n").unwrap();

        assert_eq!(
            find_horizontal_mirror_1(&Land::parse(land).unwrap()),
            Some(3)
        );
    }

    #[test]
    fn test_vertical_mirror_1_ko() {
        let (_, land) = EXAMPLE_1.split_once("\n\n").unwrap();

        assert_eq!(find_vertical_mirror_1(&Land::parse(land).unwrap()), None);
    }
}
