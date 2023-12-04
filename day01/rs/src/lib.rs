//! --- Day 1: Trebuchet?! ---
//!
//! Something is wrong with global snow production, and you've been
//! selected to take a look. The Elves have even given you a map; on
//! it, they've used stars to mark the top fifty locations that are
//! likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow
//! operations, you need to check all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made
//! available on each day in the Advent calendar; the second puzzle is
//! unlocked when you complete the first. Each puzzle grants one
//! star. Good luck!
//!
//! You try to ask why they can't just use a weather machine ("not
//! powerful enough") and where they're even sending you ("the sky")
//! and why your map looks mostly blank ("you sure ask a lot of
//! questions") and hang on did you just say the sky ("of course,
//! where do you think snow comes from") when you realize that the
//! Elves are already loading you into a trebuchet ("please hold
//! still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their
//! calibration document (your puzzle input) has been amended by a
//! very young Elf who was apparently just excited to show off her art
//! skills. Consequently, the Elves are having trouble reading the
//! values on the document.    
use std::iter;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

trait FirstAndLast: Iterator {
    fn first_and_last(&mut self) -> Option<(<Self as Iterator>::Item, <Self as Iterator>::Item)>;
}

impl<I: Iterator> FirstAndLast for I
where
    I::Item: Copy,
{
    fn first_and_last(&mut self) -> Option<(<Self as Iterator>::Item, <Self as Iterator>::Item)> {
        if let Some(first) = self.next() {
            if let Some(last) = self.last() {
                Some((first, last))
            } else {
                Some((first, first))
            }
        } else {
            None
        }
    }
}

fn match_number(r: &[u8]) -> Option<(u32, usize)> {
    if r.len() >= 5 {
        if &r[0..5] == "seven".as_bytes() {
            return Some((7, 5));
        } else if &r[0..5] == "three".as_bytes() {
            return Some((3, 5));
        } else if &r[0..5] == "eight".as_bytes() {
            return Some((8, 5));
        }
    }
    if r.len() >= 4 {
        if &r[0..4] == "four".as_bytes() {
            return Some((4, 4));
        } else if &r[0..4] == "five".as_bytes() {
            return Some((5, 4));
        } else if &r[0..4] == "nine".as_bytes() {
            return Some((9, 4));
        }
    }
    if r.len() >= 3 {
        if &r[0..3] == "one".as_bytes() {
            return Some((1, 3));
        } else if &r[0..3] == "two".as_bytes() {
            return Some((2, 3));
        } else if &r[0..3] == "six".as_bytes() {
            return Some((6, 3));
        }
    }

    None
}

/// The newly-improved calibration document consists of lines of text;
/// each line originally contained a specific calibration value that
/// the Elves now need to recover. On each line, the calibration value
/// can be found by combining the first digit and the last digit (in
/// that order) to form a single two-digit number.
///
/// # Examples
/// ```raw
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
///
/// In this example, the calibration values of these four lines are
/// 12, 38, 15, and 77. Adding these together produces 142.
///
/// Consider your entire calibration document.
///
/// ```rust
/// use day01::solve_1;
///
/// let input = r#"1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet"#;
///
/// assert_eq!(solve_1(&input), 142);
/// ```
pub fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first, last) = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .first_and_last()
                .expect("invalid line");
            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum()
}

/// Your calculation isn't quite right. It looks like some of the
/// digits are actually spelled out with letters: one, two, three,
/// four, five, six, seven, eight, and nine also count as valid
/// "digits".
///
/// Equipped with this new information, you now need to find the real
/// first and last digit on each line.
///
/// # Example
/// ```raw
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
///
/// In this example, the calibration values are 29, 83, 13, 24, 42,
/// 14, and 76. Adding these together produces 281.
/// ```rust
/// use day01::solve_2;
///
/// let input = r#"two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen"#;
///
/// assert_eq!(solve_2(&input), 281);
/// ```
pub fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let r = line.as_bytes();
            let mut i = 0;
            let (first, last) = iter::from_fn(move || {
                while i < r.len() {
                    if r[i].is_ascii_digit() {
                        let value = (r[i] - b'0') as u32;
                        i += 1;
                        return Some(value);
                    } else if let Some((value, _)) = match_number(&r[i..]) {
                        i += 1;
                        return Some(value);
                    } else {
                        i += 1;
                    }
                }
                None
            })
            .first_and_last()
            .unwrap();

            first * 10 + last
        })
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
        static ref INPUT_1: &'static str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        static ref INPUT_2: &'static str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT_1), 142);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT_2), 281);
    }
}
