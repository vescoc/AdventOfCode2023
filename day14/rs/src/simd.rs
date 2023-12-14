use std::{
    array,
    simd::{prelude::*, LaneCount, SupportedLaneCount},
};

const IDX: [usize; 128] = {
    let mut init = [0; 128];
    let mut i = 0;
    loop {
        init[i] = i;
        i += 1;
        if i == init.len() {
            break;
        }
    }
    init
};

/// Cycle one times.
/// # Panics
/// Panic if LANES is either minor of nrows or ncols
pub fn cycle<const LANES: usize>(mut tiles: Vec<u8>, ncols: usize, nrows: usize) -> Vec<u8>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    assert!(
        ncols < LANES && nrows < LANES,
        "invalid LANES, must be > (nrows, ncols)"
    );

    let row_mask = Mask::<isize, LANES>::from_array(array::from_fn(|i| i < ncols));

    // north
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = Simd::<usize, LANES>::splat(0);
        for r in 0..nrows {
            let idx = Simd::<usize, LANES>::from_slice(&IDX[0..LANES]);
            let r_idx = idx + Simd::splat((ncols + 1) * r);

            let values = Simd::gather_or_default(&tiles, r_idx);

            let o_tiles = values.simd_eq(Simd::splat(b'O')).cast::<isize>() & row_mask;
            Simd::splat(b'O').scatter_select(
                &mut new_tiles,
                o_tiles,
                state * Simd::splat(ncols + 1) + idx,
            );

            let sharp_tiles = values.simd_eq(Simd::splat(b'#')).cast::<isize>() & row_mask;
            Simd::splat(b'#').scatter_select(&mut new_tiles, sharp_tiles, r_idx);

            state = o_tiles.select(
                state + Simd::splat(1),
                sharp_tiles.select(Simd::splat(r + 1), state),
            );
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

/// Calculate load
///
/// # Panics
/// Panics if `LANES` is either minor of `nrows` or `ncols`.
pub fn load<const LANES: usize>(tiles: &[u8], ncols: usize, nrows: usize) -> usize
where
    LaneCount<LANES>: SupportedLaneCount,
{
    assert!(
        ncols < LANES && nrows < LANES,
        "invalid LANES, must be > (nrows, ncols)"
    );

    let sum = (0..nrows).fold(Simd::<usize, LANES>::splat(0), |acc, r| {
        let idx = Simd::<usize, LANES>::from_slice(&IDX[0..LANES]) + Simd::splat((ncols + 1) * r);

        let o_tiles = Simd::gather_or_default(tiles, idx).simd_eq(Simd::splat(b'O'));

        o_tiles.cast().select(acc + Simd::splat(nrows - r), acc)
    });

    Mask::from_array(array::from_fn(|i| i < ncols))
        .select(sum, Simd::splat(0))
        .reduce_sum()
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use crate::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
    }

    #[test]
    fn test_same_results_for_load() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        assert_eq!(
            simd::load::<16>(&tiles, ncols, nrows),
            simple::load(&tiles, ncols, nrows)
        );
    }

    #[test]
    fn test_same_results_for_cycle() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let simd_r = simd::cycle::<16>(tiles.to_vec(), ncols, nrows);
        let simple_r = simple::cycle(tiles.to_vec(), ncols, nrows);
        assert_eq!(
            simd_r,
            simple_r,
            "simd:\n{}\nsimple:\n{}",
            std::str::from_utf8(&simd_r).unwrap(),
            std::str::from_utf8(&simple_r).unwrap(),
        );
    }

    #[test]
    #[should_panic(expected = "invalid LANES, must be > (nrows, ncols)")]
    fn test_load_with_invalid_lanes() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let _ = simd::load::<8>(&tiles, ncols, nrows);
    }

    #[test]
    #[should_panic(expected = "invalid LANES, must be > (nrows, ncols)")]
    fn test_cycle_with_invalid_lanes() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let _ = simd::cycle::<8>(tiles.to_vec(), ncols, nrows);
    }
}
