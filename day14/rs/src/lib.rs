#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

fn cycle(mut tiles: Vec<u8>, ncols: usize, nrows: usize) -> Vec<u8> {
    // north
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    for c in 0..ncols {
        let mut state = 0;
        for r in 0..nrows {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * state + c] = b'O';
                    state += 1;
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = r + 1;
                }
                _ => {}
            }
        }
    }

    // west
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];        
    for r in 0..nrows {
        let mut state = 0;
        for c in 0..ncols {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * r + state] = b'O';
                    state += 1;
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = c + 1;
                }
                _ => {}
            }
        }
    }

    // south
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];        
    for c in 0..ncols {
        let mut state = nrows - 1;
        for r in (0..nrows).rev() {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * state + c] = b'O';
                    state = state.saturating_sub(1);
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = r.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    // est
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];        
    for r in 0..nrows {
        let mut state = ncols - 1;
        for c in (0..ncols).rev() {
            match tiles.get((ncols + 1) * r + c) {
                Some(b'O') => {
                    new_tiles[(ncols + 1) * r + state] = b'O';
                    state = state.saturating_sub(1);
                }
                Some(b'#') => {
                    new_tiles[(ncols + 1) * r + c] = b'#';
                    state = c.saturating_sub(1);
                }
                _ => {}
            }
        }
    }

    new_tiles
}

fn load(tiles: &[u8], ncols: usize, nrows: usize) -> usize {
    (0..nrows)
        .flat_map(|r| {
            tiles
                .get((ncols + 1) * r..(ncols + 1) * r + ncols)
                .unwrap()
                .iter()
                .map(move |&t| if t == b'O' { nrows - r } else { 0 })
        })
        .sum()
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> usize {
    let tiles = input.as_bytes();
    let ncols = tiles
        .iter()
        .position(|&t| t == b'\n')
        .expect("invalid platform");
    let nrows = {
        let mut tiles = tiles;
        loop {
            let t = tiles[tiles.len() - 1];

            if t.is_ascii_whitespace() {
                tiles = &tiles[0..tiles.len() - 1];
            } else {
                break;
            }
        }

        (tiles.len() + 1) / (ncols + 1)
    };

    (0..ncols)
        .zip(vec![nrows; ncols].iter_mut())
        .map(|(c, state)| {
            (0..nrows)
                .map(|r| match tiles.get((ncols + 1) * r + c) {
                    Some(b'O') => {
                        let weight = *state;
                        *state -= 1;
                        weight
                    }
                    Some(b'#') => {
                        *state = nrows - (r + 1);
                        0
                    }
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

/// Solve part 2
///
/// # Panics
/// Panic if input is invalid
pub fn solve_2(input: &str) -> usize {
    let tiles = input.as_bytes();
    let ncols = tiles
        .iter()
        .position(|&t| t == b'\n')
        .expect("invalid platform");
    let (tiles, nrows) = {
        let mut tiles = tiles;
        loop {
            let t = tiles[tiles.len() - 1];

            if t.is_ascii_whitespace() {
                tiles = &tiles[0..tiles.len() - 1];
            } else {
                break;
            }
        }

        (tiles, (tiles.len() + 1) / (ncols + 1))
    };

    let mut history: HashMap<Vec<u8>, usize> = HashMap::with_capacity(1_024);
    let mut tiles = tiles.to_owned();
    for i in 0.. {
        tiles = cycle(tiles, ncols, nrows);
        if let Some(old_i) = history.get(&tiles) {
            let t = old_i - 1 + (1_000_000_000 - old_i) % (i - old_i);

            return history
                .iter()
                .find_map(|(tiles, &i)| if i == t { Some(load(tiles, ncols, nrows)) } else { None })
                .unwrap();
        }

        history.insert(tiles.clone(), i);
    }

    unreachable!()
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
        assert_eq!(solve_1(&EXAMPLE_1), 136);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 64);
    }

    #[test]
    fn cycle_1() {
        assert_eq!(std::str::from_utf8(&cycle(EXAMPLE_1.as_bytes()
                                              .iter()
                                              .copied()
                                              .collect::<Vec<_>>(),
                                              10,
                                              10)).unwrap().to_string(),
                   r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
".replace('\n', "."));
    }    
}
