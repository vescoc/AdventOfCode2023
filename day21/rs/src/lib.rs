#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::collections::VecDeque;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn fill(garden: &[u8], ncols: isize, nrows: isize, start: (isize, isize), steps: usize) -> u64 {
    let mut distances = vec![None; nrows as usize * ncols as usize];

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some(((x, y), distance)) = queue.pop_front() {
        match &mut distances[x as usize + y as usize * ncols as usize] {
            cell @ None => {
                *cell = Some(distance);
                for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                    let (x, y) = (x + dx, y + dy);
                    if (0..ncols).contains(&x)
                        && (0..nrows).contains(&y)
                        && garden[x as usize + y as usize * (ncols as usize + 1)] != b'#'
                    {
                        queue.push_back(((x, y), distance + 1));
                    }
                }
            }
            Some(dist) if *dist <= distance => {}
            _ => unreachable!(),
        }
    }

    distances
        .into_iter()
        .filter(|v| {
            if let Some(distance) = v {
                distance <= &steps && steps % 2 == distance % 2
            } else {
                false
            }
        })
        .count() as u64
}

fn sqr(value: isize) -> isize {
    value * value
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn parse(input: &str) -> (&[u8], isize, isize, (isize, isize)) {
    let garden = {
        let mut garden = input.as_bytes();
        while let Some(tile) = garden.last() {
            if tile.is_ascii_whitespace() {
                garden = &garden[0..garden.len() - 1];
            } else {
                break;
            }
        }
        garden
    };

    let ncols = garden
        .iter()
        .position(|&c| c == b'\n')
        .expect("invalid garden") as isize;
    let nrows = (garden.len() as isize + 1) / (ncols + 1);

    let Some(s) = garden.iter().position(|&c| c == b'S').map(|s| {
        (
            (s as isize).rem_euclid(ncols + 1),
            (s as isize).div_euclid(ncols + 1),
        )
    }) else {
        panic!("cannot find S");
    };

    (garden, ncols, nrows, s)
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> u64 {
    solve_1_steps(input, 64)
}

/// Solve part 1 with steps
///
/// # Panics
/// Panic if invalid input
fn solve_1_steps(input: &str, steps: usize) -> u64 {
    let (garden, ncols, nrows, s) = parse(input);

    fill(garden, ncols, nrows, s, steps)
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
#[allow(
    clippy::unreadable_literal,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
pub fn solve_2(input: &str) -> u64 {
    let garden = {
        let mut garden = input.as_bytes();
        while let Some(tile) = garden.last() {
            if tile.is_ascii_whitespace() {
                garden = &garden[0..garden.len() - 1];
            } else {
                break;
            }
        }
        garden
    };

    let ncols = garden
        .iter()
        .position(|&c| c == b'\n')
        .expect("invalid garden") as isize;
    let nrows = (garden.len() as isize + 1) / (ncols + 1);

    let grid_width = 26501365 / ncols - 1;
    let odd = sqr(grid_width / 2 * 2 + 1) as u64;
    let even = sqr((grid_width + 1) / 2 * 2) as u64;

    let odd_points = fill(
        garden,
        ncols,
        nrows,
        (ncols / 2, nrows / 2),
        2 * ncols as usize + 1,
    );
    let even_points = fill(
        garden,
        ncols,
        nrows,
        (ncols / 2, nrows / 2),
        2 * ncols as usize,
    );

    let corner_top = fill(
        garden,
        ncols,
        nrows,
        (ncols / 2, nrows - 1),
        nrows as usize - 1,
    );
    let corner_right = fill(garden, ncols, nrows, (0, nrows / 2), ncols as usize - 1);
    let corner_bottom = fill(garden, ncols, nrows, (ncols / 2, 0), nrows as usize - 1);
    let corner_left = fill(
        garden,
        ncols,
        nrows,
        (ncols - 1, nrows / 2),
        ncols as usize - 1,
    );

    let small_top_right = fill(garden, ncols, nrows, (0, nrows - 1), nrows as usize / 2 - 1);
    let small_top_left = fill(
        garden,
        ncols,
        nrows,
        (ncols - 1, nrows - 1),
        nrows as usize / 2 - 1,
    );
    let small_bottom_right = fill(garden, ncols, nrows, (0, 0), nrows as usize / 2 - 1);
    let small_bottom_left = fill(garden, ncols, nrows, (ncols - 1, 0), nrows as usize / 2 - 1);

    let large_top_right = fill(
        garden,
        ncols,
        nrows,
        (0, nrows - 1),
        nrows as usize * 3 / 2 - 1,
    );
    let large_top_left = fill(
        garden,
        ncols,
        nrows,
        (ncols - 1, nrows - 1),
        nrows as usize * 3 / 2 - 1,
    );
    let large_bottom_right = fill(garden, ncols, nrows, (0, 0), nrows as usize * 3 / 2 - 1);
    let large_bottom_left = fill(
        garden,
        ncols,
        nrows,
        (ncols - 1, 0),
        nrows as usize * 3 / 2 - 1,
    );

    odd_points * odd
        + even_points * even
        + corner_top
        + corner_right
        + corner_bottom
        + corner_left
        + (grid_width as u64 + 1)
            * (small_top_right + small_top_left + small_bottom_right + small_bottom_left)
        + grid_width as u64
            * (large_top_right + large_top_left + large_bottom_right + large_bottom_left)
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
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1_steps(&EXAMPLE_1, 6), 16);
    }

    // #[test]
    // fn same_results_2() {
    //     assert_eq!(solve_2_steps(&EXAMPLE_1, 6), 16);
    //     assert_eq!(solve_2_steps(&EXAMPLE_1, 10), 50);
    //     assert_eq!(solve_2_steps(&EXAMPLE_1, 50), 1594);
    //     assert_eq!(solve_2_steps(&EXAMPLE_1, 100), 6536);
    //     assert_eq!(solve_2_steps(&EXAMPLE_1, 500), 167004);
    //     assert_eq!(solve_2_steps(&EXAMPLE_1, 1000), 668697);
    //     assert_eq!(solve_2_steps(&EXAMPLE_1, 5000), 16733044);
    // }
}
