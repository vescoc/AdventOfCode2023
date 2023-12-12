#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::collections::HashMap;
use std::iter;
use std::str::FromStr;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

type MemoizeKey = (Vec<u8>, Vec<usize>);
type MemoizeValue = u64;

fn arrangements(
    line: &[u8],
    groups: &[usize],
    memoize: &mut HashMap<MemoizeKey, MemoizeValue>,
) -> MemoizeValue {
    let key = (line.to_owned(), groups.to_owned());
    if let Some(value) = memoize.get(&key) {
        return *value;
    }

    let r = if line.is_empty() && groups.is_empty() {
        1
    } else if groups.iter().sum::<usize>() > line.len() {
        0
    } else if let Some(&head) = groups.iter().next() {
        let mut line = line;
        while !line.is_empty() && line[0] == b'.' {
            line = &line[1..];
        }

        if head > line.len() {
            0
        } else {
            // case 1. prefix in group
            let case1 = if line.iter().take(head).all(|&c| c != b'.')
                && line.get(head).unwrap_or(&b'.') != &b'#'
            {
                arrangements(line.get(head + 1..).unwrap_or(&[]), &groups[1..], memoize)
            } else {
                0
            };

            // case 2. prefix not in group
            let case2 = if line[0] == b'?' {
                arrangements(line.get(1..).unwrap_or(&[]), groups, memoize)
            } else {
                0
            };

            case1 + case2
        }
    } else {
        u64::from(!line.contains(&b'#'))
    };

    memoize.insert(key, r);

    r
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> u64 {
    #[cfg(feature = "rayon")]
    let lines = input.par_lines();

    #[cfg(not(feature = "rayon"))]
    let lines = input.lines();

    lines
        .map(|line| {
            let (line, groups) = line.split_once(' ').expect("invalid input");

            let groups = groups
                .split(',')
                .map(usize::from_str)
                .collect::<Result<Vec<_>, _>>()
                .expect("invalid number");

            arrangements(line.as_bytes(), &groups, &mut HashMap::with_capacity(1_024)) as u64
        })
        .sum()
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> u64 {
    #[cfg(feature = "rayon")]
    let lines = input.par_lines();

    #[cfg(not(feature = "rayon"))]
    let lines = input.lines();

    lines
        .map(|input| {
            let (line, groups) = input.split_once(' ').expect("invalid input");

            let groups = groups
                .split(',')
                .map(usize::from_str)
                .collect::<Result<Vec<_>, _>>()
                .expect("invalid number");

            let groups = iter::once(groups)
                .cycle()
                .take(5)
                .flatten()
                .collect::<Vec<_>>();

            let l = line.as_bytes().len();
            let line = line
                .as_bytes()
                .iter()
                .chain(iter::once(&b'?'))
                .cycle()
                .take((l + 1) * 5 - 1)
                .copied()
                .collect::<Vec<_>>();

            arrangements(&line, &groups, &mut HashMap::with_capacity(1_024)) as u64
        })
        .sum()
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
        assert_eq!(solve_1(&EXAMPLE_1), 21);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 525152);
    }

    #[test]
    fn test_sample_1_1() {
        assert_eq!(solve_1("???.### 1,1,3"), 1);
    }

    #[test]
    fn test_sample_1_2() {
        assert_eq!(solve_1(".??..??...?##. 1,1,3"), 4);
    }

    #[test]
    fn test_sample_1_3() {
        assert_eq!(solve_1("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    }

    #[test]
    fn test_sample_1_4() {
        assert_eq!(solve_1("????.#...#... 4,1,1"), 1);
    }

    #[test]
    fn test_sample_1_5() {
        assert_eq!(solve_1("????.######..#####. 1,6,5"), 4);
    }

    #[test]
    fn test_sample_1_6() {
        assert_eq!(solve_1("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_sample_1_7() {
        assert_eq!(solve_1("### 3"), 1);
    }

    #[test]
    fn test_sample_1_8() {
        assert_eq!(solve_1("#?# 3"), 1);
    }

    #[test]
    fn test_sample_2_1() {
        assert_eq!(solve_2("???.### 1,1,3"), 1);
    }

    #[test]
    fn test_sample_2_2() {
        assert_eq!(solve_2(".??..??...?##. 1,1,3"), 16384);
    }

    #[test]
    fn test_sample_2_3() {
        assert_eq!(solve_2("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    }

    #[test]
    fn test_sample_2_4() {
        assert_eq!(solve_2("????.#...#... 4,1,1"), 16);
    }

    #[test]
    fn test_sample_2_5() {
        assert_eq!(solve_2("????.######..#####. 1,6,5"), 2500);
    }

    #[test]
    fn test_sample_2_6() {
        assert_eq!(solve_2("?###???????? 3,2,1"), 506250);
    }
}
