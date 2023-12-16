use std::{
    array,
    simd::{prelude::*, LaneCount, SupportedLaneCount},
};

use super::{splat, range_from, range_from_with};

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

    let row_mask = Mask::<isize, LANES>::from_array(array::from_fn(|i| i < nrows));
    let column_mask = Mask::<isize, LANES>::from_array(array::from_fn(|i| i < ncols));

    let ncols_1 = Simd::splat(ncols + 1);

    // north
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = splat(0);
        for r in 0..nrows {
            let r_idx = range_from(0) + Simd::splat((ncols + 1) * r);

            let values = Simd::gather_select(&tiles, row_mask, r_idx, splat(0));

            let o_tiles = values.simd_eq(splat(b'O')).cast::<isize>() & row_mask;
            splat(b'O').scatter_select(&mut new_tiles, o_tiles, state * ncols_1 + range_from(0));

            let sharp_tiles = values.simd_eq(splat(b'#')).cast::<isize>() & row_mask;
            splat(b'#').scatter_select(&mut new_tiles, sharp_tiles, r_idx);

            state = o_tiles.select(
                state + splat(1),
                sharp_tiles.select(Simd::splat(r + 1), state),
            );
        }
    }

    // west
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = splat(0);
        for c in 0..ncols {
            let c_idx = range_from_with(0, ncols + 1) + Simd::splat(c);

            let values = Simd::gather_select(&tiles, column_mask, c_idx, splat(0));

            let o_tiles = values.simd_eq(splat(b'O')).cast::<isize>() & column_mask;
            splat(b'O').scatter_select(
                &mut new_tiles,
                o_tiles,
                state + range_from_with(0, ncols + 1),
            );

            let sharp_tiles = values.simd_eq(splat(b'#')).cast::<isize>() & column_mask;
            splat(b'#').scatter_select(&mut new_tiles, sharp_tiles, c_idx);

            state = o_tiles.select(
                state + splat(1),
                sharp_tiles.select(Simd::splat(c + 1), state),
            );
        }
    }

    // south
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = Simd::<usize, LANES>::splat(nrows - 1);
        for r in (0..nrows).rev() {
            let r_idx = range_from(0) + Simd::splat((ncols + 1) * r);

            let values = Simd::gather_select(&tiles, row_mask, r_idx, splat(0));

            let o_tiles = values.simd_eq(splat(b'O')).cast::<isize>() & row_mask;
            splat(b'O').scatter_select(&mut new_tiles, o_tiles, state * ncols_1 + range_from(0));

            let sharp_tiles = values.simd_eq(splat(b'#')).cast::<isize>() & row_mask;
            splat(b'#').scatter_select(&mut new_tiles, sharp_tiles, r_idx);

            state = o_tiles.select(
                state.saturating_sub(splat(1)),
                sharp_tiles.select(Simd::splat(r.saturating_sub(1)), state),
            );
        }
    }

    // est
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = Simd::splat(ncols - 1);
        for c in (0..ncols).rev() {
            let c_idx = range_from_with(0, ncols + 1) + Simd::splat(c);

            let values = Simd::gather_select(&tiles, column_mask, c_idx, splat(0));

            let o_tiles = values.simd_eq(splat(b'O')).cast::<isize>() & column_mask;
            splat(b'O').scatter_select(
                &mut new_tiles,
                o_tiles,
                state + range_from_with(0, ncols + 1),
            );

            let sharp_tiles = values.simd_eq(splat(b'#')).cast::<isize>() & column_mask;
            splat(b'#').scatter_select(&mut new_tiles, sharp_tiles, c_idx);

            state = o_tiles.select(
                state.saturating_sub(splat(1)),
                sharp_tiles.select(Simd::splat(c.saturating_sub(1)), state),
            );
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

    let row_mask = Mask::<isize, LANES>::from_array(array::from_fn(|i| i < ncols));

    let mut acc = splat(0);
    for r in 0..nrows {
        let idx = range_from(0) + splat((ncols + 1) * r);

        let o_tiles = Simd::gather_select(tiles, row_mask, idx, splat(0)).simd_eq(splat(b'O'));

        acc = o_tiles.cast().select(acc + splat(nrows - r), acc);
    }

    row_mask.select(acc, splat(0)).reduce_sum()
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use crate::parse;
    use crate::simple;

    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../../example1");
    }

    #[test]
    fn test_same_results_for_load() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        assert_eq!(
            load::<16>(&tiles, ncols, nrows),
            simple::load(&tiles, ncols, nrows)
        );
    }

    #[test]
    fn test_same_results_for_cycle() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let simd_r = cycle::<16>(tiles.to_vec(), ncols, nrows);
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

        let _ = load::<8>(&tiles, ncols, nrows);
    }

    #[test]
    #[should_panic(expected = "invalid LANES, must be > (nrows, ncols)")]
    fn test_cycle_with_invalid_lanes() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let _ = cycle::<8>(tiles.to_vec(), ncols, nrows);
    }
}
