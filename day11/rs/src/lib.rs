#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::collections::HashSet;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn solve<const EXPANSION: u64>(input: &str) -> u64 {
    let input = input.as_bytes();
    let ncols = input
        .iter()
        .position(|&c| c == b'\n')
        .expect("invalid input");

    // assuming square input
    let nrows = ncols;

    let empty_columns = (0..ncols)
        .filter(|c| (0..nrows).all(|r| input[r * (ncols + 1) + c] == b'.'))
        .collect::<HashSet<_>>();

    let galaxy = input
        .chunks(ncols + 1)
        .scan(0, |row_index, row| {
            let galaxy_row = row[..row.len() - usize::from(row[row.len() - 1] == b'\n')]
                .iter()
                .enumerate()
                .scan(0, |column_index, (c, tile)| {
                    let r = Some((*column_index, tile));
                    if empty_columns.contains(&c) {
                        *column_index += EXPANSION as i64;
                    } else {
                        *column_index += 1;
                    }
                    r
                })
                .filter_map(|(c, &tile)| {
                    if tile == b'#' {
                        Some((c, *row_index))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if galaxy_row.is_empty() {
                *row_index += EXPANSION as i64;
            } else {
                *row_index += 1;
            }

            Some(galaxy_row)
        })
        .flatten()
        .collect::<Vec<_>>();

    galaxy
        .iter()
        .take(galaxy.len())
        .enumerate()
        .flat_map(|(i, (ax, ay))| {
            galaxy
                .iter()
                .skip(i)
                .map(move |(bx, by)| (ax - bx).abs() + (ay - by).abs())
        })
        .sum::<i64>() as _
}

pub fn solve_1(input: &str) -> u64 {
    solve::<2>(input)
}

pub fn solve_2(input: &str) -> u64 {
    solve::<1_000_000>(input)
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
        assert_eq!(solve_1(&EXAMPLE_1), 374);
    }

    #[test]
    fn same_results_2_1() {
        assert_eq!(solve::<10>(&EXAMPLE_1), 1030);
    }

    #[test]
    fn same_results_2_2() {
        assert_eq!(solve::<100>(&EXAMPLE_1), 8410);
    }
}
