#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::array;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

fn hash<'a>(part: impl IntoIterator<Item = &'a u8>) -> u32 {
    part.into_iter()
        .filter(|c| !c.is_ascii_whitespace())
        .fold(0, |current, &c| (current + u32::from(c)) * 17 % 256)
}

pub fn solve_1(input: &str) -> u32 {
    input.as_bytes().split(|&c| c == b',').map(hash).sum()
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> usize {
    input
        .as_bytes()
        .split(|&c| c == b',')
        .fold(
            &mut array::from_fn::<Vec<(&[u8], usize)>, 256, _>(|_| Vec::new()),
            |boxes, lens| {
                let mut parts = lens.split_inclusive(|&c| c == b'-' || c == b'=');
                let label = parts.next().expect("label not found");
                let (label, action) = { (&label[0..label.len() - 1], label[label.len() - 1]) };

                let box_index = hash(label) as usize;
                let b = &mut boxes[box_index];
                if action == b'-' {
                    if let Some(index) = b.iter().position(|(l, _)| l == &label) {
                        b.remove(index);
                    }
                } else if let Some((_, v)) = b.iter_mut().find(|(l, _)| l == &label).as_mut() {
                    *v = usize::from(parts.next().unwrap()[0] - b'0');
                } else {
                    b.push((label, usize::from(parts.next().unwrap()[0] - b'0')));
                }

                boxes
            },
        )
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(s, (_, l))| (i + 1) * (s + 1) * l)
                .sum::<usize>()
        })
        .sum()
}

pub fn part_1() -> u32 {
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
        assert_eq!(solve_1(&EXAMPLE_1), 1320);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 145);
    }
}
