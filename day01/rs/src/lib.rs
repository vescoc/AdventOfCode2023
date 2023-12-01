use std::iter;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: &'static str = include_str!("../../input");
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

fn solve_1(input: &str) -> u32 {
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

fn solve_2(input: &str) -> u32 {
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
