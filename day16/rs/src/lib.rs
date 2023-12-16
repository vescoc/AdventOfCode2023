#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

struct Grid<'a> {
    data: &'a [u8],
    nrows: usize,
    ncols: usize,
}

struct Delta;

impl From<Direction> for u8 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

impl From<u8> for Direction {
    fn from(n: u8) -> Self {
        match n.rem_euclid(4) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl Delta {
    #[inline]
    fn from(direction: Direction) -> (isize, isize) {
        [(0, -1), (1, 0), (0, 1), (-1, 0)][u8::from(direction) as usize]
    }
}

fn parse(input: &str) -> Result<Grid, &'static str> {
    let data = {
        let mut data = input.as_bytes();
        while !data.is_empty() && data[data.len() - 1].is_ascii_whitespace() {
            data = &data[0..data.len() - 1];
        }
        data
    };

    if data.is_empty() {
        Err("grid is empty")
    } else {
        let Some(ncols) = data.iter().position(|&c| c == b'\n') else {
            return Err("invalid grid");
        };
        let nrows = (data.len() + 1) / (ncols + 1);

        Ok(Grid { data, nrows, ncols })
    }
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn energize(Grid { data, nrows, ncols }: &Grid, beam: ((usize, usize), Direction)) -> usize {
    let (nrows, ncols) = (*nrows, *ncols);

    let state = &mut vec![0_u8; nrows * ncols];

    let mut beams = Vec::with_capacity(1_024);
    beams.push(beam);

    let mut next = Vec::with_capacity(2);
    while let Some(((mut x, mut y), mut direction)) = beams.pop() {
        loop {
            let s = &mut state[x + y * ncols];
            if *s & (1 << u8::from(direction)) != 0 {
                break;
            }

            *s |= 1 << u8::from(direction);

            match (data[x + y * (ncols + 1)], direction) {
                // empty
                (b'.', direction) => next.push(direction),

                // mirrors
                (b'\\', Direction::Up) | (b'/', Direction::Down) => next.push(Direction::Left),
                (b'\\', Direction::Right) | (b'/', Direction::Left) => next.push(Direction::Down),
                (b'\\', Direction::Down) | (b'/', Direction::Up) => next.push(Direction::Right),
                (b'\\', Direction::Left) | (b'/', Direction::Right) => next.push(Direction::Up),

                // splits
                (b'-', Direction::Left | Direction::Right)
                | (b'|', Direction::Up | Direction::Down) => next.push(direction),
                (b'-', Direction::Up | Direction::Down) => {
                    next.push(Direction::Left);
                    next.push(Direction::Right);
                }
                (b'|', Direction::Left | Direction::Right) => {
                    next.push(Direction::Up);
                    next.push(Direction::Down);
                }

                // invalid
                _ => panic!("invalid tile"),
            }

            let mut directions = next.drain(..);
            let next_step = if let Some(new_direction) = directions.next() {
                let (dx, dy) = Delta::from(new_direction);
                let (new_x, new_y) = (x as isize + dx, y as isize + dy);

                if (0..ncols as isize).contains(&new_x) && (0..nrows as isize).contains(&new_y) {
                    let next_step = Some(((new_x as usize, new_y as usize), new_direction));

                    if let Some(new_direction) = directions.next() {
                        let (dx, dy) = Delta::from(new_direction);
                        let (new_x, new_y) = (x as isize + dx, y as isize + dy);

                        if (0..ncols as isize).contains(&new_x)
                            && (0..nrows as isize).contains(&new_y)
                            && state[new_x as usize + new_y as usize * ncols]
                                & (1 << u8::from(new_direction))
                                == 0
                        {
                            beams.push(((new_x as usize, new_y as usize), new_direction));
                        }
                    }

                    next_step
                } else if let Some(new_direction) = directions.next() {
                    let (dx, dy) = Delta::from(new_direction);
                    let (new_x, new_y) = (x as isize + dx, y as isize + dy);

                    if (0..ncols as isize).contains(&new_x) && (0..nrows as isize).contains(&new_y)
                    {
                        Some(((new_x as usize, new_y as usize), new_direction))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(next_step) = next_step {
                ((x, y), direction) = next_step;
            } else {
                break;
            }
        }
    }

    state.iter().map(|&v| usize::from(v != 0)).sum()
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> usize {
    let grid = parse(input).expect("invalid input");

    energize(&grid, ((0, 0), Direction::Right))
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> usize {
    let grid = parse(input).expect("invalid input");

    let (nrows, ncols) = (grid.nrows, grid.ncols);

    let perimeter = (0..ncols)
        .map(|x| ((x, 0), Direction::Down))
        .chain((0..ncols).map(|x| ((x, (nrows - 1)), Direction::Up)))
        .chain((0..nrows).map(|y| ((0, y), Direction::Right)))
        .chain((0..nrows).map(|y| ((ncols - 1, y), Direction::Left)));

    #[cfg(feature = "rayon")]
    let perimeter = perimeter.par_bridge();

    perimeter.map(|beam| energize(&grid, beam)).max().unwrap()
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
        assert_eq!(solve_1(&EXAMPLE_1), 46);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 51);
    }
}
