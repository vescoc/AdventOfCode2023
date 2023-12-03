use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

pub fn solve_1(input: &str) -> u32 {
    let engine = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (r, row) in engine.iter().enumerate() {
        let mut c = 0;
        while c < row.len() {
            if row[c].is_ascii_digit() {
                let mut valid = false;
                let mut value = 0;
                loop {
                    if !valid {
                        'outher: for dx in -1..=1 {
                            for dy in -1..=1 {
                                if (dx != 0 || dy != 0)
                                    && (dx >= 0 || c != 0)
                                    && (dy >= 0 || r != 0)
                                {
                                    let neighbour = engine
                                        .get((r as isize + dy) as usize)
                                        .and_then(|row| row.get((c as isize + dx) as usize))
                                        .unwrap_or(&b'.');
                                    if !matches!(neighbour, b'.' | b'0'..=b'9') {
                                        valid = true;
                                        break 'outher;
                                    }
                                }
                            }
                        }
                    }
                    value = value * 10 + (row[c] - b'0') as u32;
                    c += 1;
                    if c >= row.len() || !row[c].is_ascii_digit() {
                        break;
                    }
                }
                if valid {
                    sum += value;
                }
            } else {
                c += 1;
            }
        }
    }

    sum
}

pub fn solve_2(input: &str) -> u32 {
    let engine = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut gears = HashMap::with_capacity(1024);
    for (r, row) in engine.iter().enumerate() {
        let mut c = 0;
        while c < row.len() {
            if row[c].is_ascii_digit() {
                let mut value = 0;
                let mut point = None;
                loop {
                    if point.is_none() {
                        'outher: for dx in -1..=1 {
                            for dy in -1..=1 {
                                if (dx != 0 || dy != 0)
                                    && (dx >= 0 || c != 0)
                                    && (dy >= 0 || r != 0)
                                {
                                    let neighbour = engine
                                        .get((r as isize + dy) as usize)
                                        .and_then(|row| row.get((c as isize + dx) as usize))
                                        .unwrap_or(&b'.');
                                    if *neighbour == b'*' {
                                        point = Some((
                                            (c as isize + dx) as usize,
                                            (r as isize + dy) as usize,
                                        ));
                                        break 'outher;
                                    }
                                }
                            }
                        }
                    }
                    value = value * 10 + (row[c] - b'0') as u32;
                    c += 1;
                    if c >= row.len() || !row[c].is_ascii_digit() {
                        break;
                    }
                }
                if let Some(point) = point {
                    gears
                        .entry(point)
                        .and_modify(|v: &mut Vec<_>| v.push(value))
                        .or_insert_with(|| vec![value]);
                }
            } else {
                c += 1;
            }
        }
    }

    gears
        .values()
        .filter_map(|v| if v.len() > 1 { Some(v[0] * v[1]) } else { None })
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
        assert_eq!(solve_1(&EXAMPLE_1), 4361);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 467835);
    }
}
