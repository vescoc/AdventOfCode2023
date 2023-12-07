use lazy_static::lazy_static;

use std::collections::HashMap;

use std::cmp::Ordering;

const J_AS_A_CARD: u8 = 3;
const J_AS_A_JOKER: u8 = 100;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Hand<'a, const J: u8> {
    FiveOfAKind(CamelCards<'a, J>),
    FourOfAKind(CamelCards<'a, J>),
    FullHouse(CamelCards<'a, J>),
    ThreeOfAKind(CamelCards<'a, J>),
    TwoPair(CamelCards<'a, J>),
    OnePair(CamelCards<'a, J>),
    HighCard(CamelCards<'a, J>),
}

impl<'a> Hand<'a, J_AS_A_CARD> {
    fn parse(input: &'a str) -> Self {
        let groups = input
            .as_bytes()
            .iter()
            .fold(HashMap::with_capacity(5), |mut cards, card| {
                cards
                    .entry(card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                cards
            })
            .values()
            .copied()
            .collect::<Vec<_>>();

        if groups.contains(&5) {
            Hand::FiveOfAKind(CamelCards(input))
        } else if groups.contains(&4) {
            Hand::FourOfAKind(CamelCards(input))
        } else if groups.contains(&3) {
            if groups.contains(&2) {
                Hand::FullHouse(CamelCards(input))
            } else {
                Hand::ThreeOfAKind(CamelCards(input))
            }
        } else if groups.contains(&2) {
            if groups.len() == 3 {
                Hand::TwoPair(CamelCards(input))
            } else {
                Hand::OnePair(CamelCards(input))
            }
        } else {
            Hand::HighCard(CamelCards(input))
        }
    }
}

impl<'a> Hand<'a, J_AS_A_JOKER> {
    fn parse(input: &'a str) -> Self {
        let mut groups =
            input
                .as_bytes()
                .iter()
                .fold(HashMap::with_capacity(5), |mut cards, card| {
                    cards
                        .entry(card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    cards
                });

        let jokers = groups.remove(&b'J').unwrap_or_default();

        let groups = groups.values().copied().collect::<Vec<_>>();

        if jokers > 3 {
            Hand::FiveOfAKind(CamelCards(input))
        } else if jokers > 2 {
            if groups.contains(&2) {
                Hand::FiveOfAKind(CamelCards(input))
            } else {
                Hand::FourOfAKind(CamelCards(input))
            }
        } else if jokers > 1 {
            if groups.contains(&3) {
                Hand::FiveOfAKind(CamelCards(input))
            } else if groups.contains(&2) {
                Hand::FourOfAKind(CamelCards(input))
            } else {
                Hand::ThreeOfAKind(CamelCards(input))
            }
        } else if jokers > 0 {
            if groups.contains(&4) {
                Hand::FiveOfAKind(CamelCards(input))
            } else if groups.contains(&3) {
                Hand::FourOfAKind(CamelCards(input))
            } else if groups.contains(&2) {
                if groups.len() == 2 {
                    Hand::FullHouse(CamelCards(input))
                } else {
                    Hand::ThreeOfAKind(CamelCards(input))
                }
            } else {
                Hand::OnePair(CamelCards(input))
            }
        } else if groups.contains(&5) {
            Hand::FiveOfAKind(CamelCards(input))
        } else if groups.contains(&4) {
            Hand::FourOfAKind(CamelCards(input))
        } else if groups.contains(&3) {
            if groups.contains(&2) {
                Hand::FullHouse(CamelCards(input))
            } else {
                Hand::ThreeOfAKind(CamelCards(input))
            }
        } else if groups.contains(&2) {
            if groups.len() == 3 {
                Hand::TwoPair(CamelCards(input))
            } else {
                Hand::OnePair(CamelCards(input))
            }
        } else {
            Hand::HighCard(CamelCards(input))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct CamelCards<'a, const J_PRIORITY: u8>(&'a str);

impl<'a, const J_PRIORITY: u8> Ord for CamelCards<'a, J_PRIORITY> {
    fn cmp(&self, CamelCards(b): &Self) -> Ordering {
        let priority = |card: &u8| match card {
            b'A' => 0,
            b'K' => 1,
            b'Q' => 2,
            b'J' => J_PRIORITY,
            b'T' => 10,
            b'2'..=b'9' => 20 - (card - b'0'),
            _ => panic!("invalid card"),
        };

        for (a, b) in self
            .0
            .as_bytes()
            .iter()
            .map(priority)
            .zip(b.as_bytes().iter().map(priority))
        {
            let c = a.cmp(&b);
            if c != Ordering::Equal {
                return c;
            }
        }

        Ordering::Equal
    }
}

fn solve<'a, const J: u8>(input: &'a str, parse: impl Fn(&'a str) -> Hand<'a, J>) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').expect("invalid input");
            (parse(hand), bid.parse::<u64>().expect("invalid bid"))
        })
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| b.cmp(a));

    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u64 + 1) * bid)
        .sum()
}

pub fn solve_1(input: &str) -> u64 {
    solve(input, Hand::<J_AS_A_CARD>::parse)
}

pub fn solve_2(input: &str) -> u64 {
    solve(input, Hand::<J_AS_A_JOKER>::parse)
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
        assert_eq!(solve_1(&EXAMPLE_1), 6440);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 5905);
    }
}
