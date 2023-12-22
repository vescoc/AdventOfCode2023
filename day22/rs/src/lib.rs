#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::str::FromStr;

use lazy_static::lazy_static;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Brick([usize; 3], [usize; 3]);

impl FromStr for Brick {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (start, end) = line.split_once('~').ok_or("invalid brick definition")?;

        let parse = |l: &str| {
            let mut i = l.split(',');

            let x = i
                .next()
                .ok_or("x undefined")?
                .parse()
                .map_err(|_| "invalid x")?;
            let y = i
                .next()
                .ok_or("y undefined")?
                .parse()
                .map_err(|_| "invalid y")?;
            let z = i
                .next()
                .ok_or("z undefined")?
                .parse()
                .map_err(|_| "invalid z")?;

            if i.next().is_none() {
                Ok([x, y, z])
            } else {
                Err("too many coordinates")
            }
        };

        let start = parse(start)?;
        let end = parse(end)?;

        Ok(Brick(start, end))
    }
}

impl Brick {
    fn intersect(&self, start: &[usize], end: &[usize]) -> bool {
        start
            .iter()
            .zip(end.iter())
            .zip(self.0.iter())
            .zip(self.1.iter())
            .all(|(((start, end), brick_start), brick_end)| {
                // start <= brick_start && brick_start <= end
                //     || start <= brick_end && brick_end <= end
                //     || brick_start <= start && start <= brick_end
                //     || brick_start <= end && end <= brick_end
                (start..=end).contains(&brick_start)
                    || (start..=end).contains(&brick_end)
                    || (brick_start..=brick_end).contains(&start)
                    || (brick_start..=brick_end).contains(&end)
            })
    }

    fn fall(&mut self, dz: usize) -> bool {
        self.0[2] -= dz;
        self.1[2] -= dz;
        dz > 0
    }

    fn removable(&self, bricks: &[Self]) -> bool {
        bricks
            .iter()
            .filter(|candidate| {
                candidate.0[2] == self.1[2] + 1 && candidate.intersect(&self.0[..2], &self.1[..2])
            })
            .all(|candidate| {
                let f = bricks.iter().filter(|base| base != &self).any(|base| {
                    base.1[2] + 1 == candidate.0[2]
                        && candidate.intersect(&base.0[..2], &base.1[..2])
                });

                f
            })
    }
}

fn fall(mut bricks: Vec<Brick>) -> (Vec<Brick>, usize) {
    let mut count = 0;
    let mut fallens: Vec<Brick> = Vec::new();
    while let Some(i) = bricks.iter().position(|brick| {
        bricks
            .iter()
            .filter(|other| other != &brick && other.intersect(&brick.0[..2], &brick.1[..2]))
            .all(|other| other.0[2] > brick.1[2])
    }) {
        let mut brick = bricks.remove(i);

        let falled = if let Some(base) = fallens
            .iter()
            .filter(|fallen| {
                fallen.intersect(&brick.0[..2], &brick.1[..2]) && fallen.1[2] < brick.0[2]
            })
            .max_by_key(|a| a.1[2])
        {
            brick.fall(brick.0[2] - base.1[2] - 1)
        } else {
            brick.fall(brick.0[2] - 1)
        };

        fallens.push(brick);
        count += usize::from(falled);
    }

    (fallens, count)
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> usize {
    let bricks = input
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("invalid input");

    let (bricks, _) = fall(bricks);

    #[cfg(feature = "rayon")]
    let i = bricks.par_iter();

    #[cfg(not(feature = "rayon"))]
    let i = bricks.iter();

    i
        .filter(|brick| brick.removable(&bricks))
        .count()
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> usize {
    let bricks = input
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("invalid input");

    let (bricks, _) = fall(bricks);


    #[cfg(feature = "rayon")]
    let i = bricks.par_iter();

    #[cfg(not(feature = "rayon"))]
    let i = bricks.iter();

    i
        .filter_map(|brick| {
            if brick.removable(&bricks) {
                None
            } else {
                Some(
                    fall(
                        bricks
                            .iter()
                            .copied()
                            .filter(|b| b != brick)
                            .collect::<Vec<_>>(),
                    )
                    .1,
                )
            }
        })
        .sum()
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
        assert_eq!(solve_1(&EXAMPLE_1), 5);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 7);
    }

    #[test]
    fn test_intersect_1() {
        let a = Brick([0, 0, 0], [1, 0, 0]);
        let b = Brick([0, 0, 1], [1, 0, 1]);

        assert!(a.intersect(&b.0[..2], &b.1[..2]));
    }

    #[test]
    fn test_intersect_2() {
        let a = Brick([1, 0, 1], [1, 2, 1]);
        let b = Brick([0, 0, 2], [2, 0, 2]);

        assert!(a.intersect(&b.0[..2], &b.1[..2]));
        assert!(b.intersect(&a.0[..2], &a.1[..2]));
    }

    #[test]
    fn test_intersect_3() {
        let a = Brick([0, 0, 2], [2, 0, 2]);
        let b = Brick([0, 2, 3], [2, 2, 3]);

        assert!(!a.intersect(&b.0[..2], &b.1[..2]));
        assert!(!b.intersect(&a.0[..2], &a.1[..2]));
    }
}
