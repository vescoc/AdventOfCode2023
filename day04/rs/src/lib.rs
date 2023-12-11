#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::collections::HashSet;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| {
        let (_, card_numbers_part) = line.split_once(':').expect("invalid card");
        let (winning_numbers_part, numbers_part) = card_numbers_part
            .split_once('|')
            .expect("invalid numbers part");
        let winning_numbers = winning_numbers_part
            .split_ascii_whitespace()
            .map(|number| number.parse::<u32>().expect("invalid winning number"))
            .collect::<HashSet<_>>();

        let numbers = numbers_part
            .split_ascii_whitespace()
            .map(|number| number.parse().expect("invalid number"))
            .collect::<HashSet<_>>();

        numbers.intersection(&winning_numbers).count()
    })
}

#[allow(clippy::cast_possible_truncation)]
pub fn solve_1(input: &str) -> u32 {
    parse(input)
        .filter(|value| *value > 0)
        .map(|value| 2_u32.pow(value as u32 - 1))
        .sum()
}

pub fn solve_2(input: &str) -> u32 {
    let cards = parse(input).collect::<Vec<_>>();

    cards
        .iter()
        .enumerate()
        .fold(vec![1_u32; cards.len()], |mut values, (i, &card)| {
            let (current, remainder) = values.split_at_mut(i);
            remainder
                .iter_mut()
                .take(card)
                .for_each(|v| *v += current.last().unwrap_or(&1));
            values
        })
        .iter()
        .sum()
}

pub fn part_1() -> u32 {
    solve_1(&INPUT)
}

pub fn part_2() -> u32 {
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
        assert_eq!(solve_1(&EXAMPLE_1), 13);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 30);
    }
}
