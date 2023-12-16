use std::{
    array,
    simd::{prelude::*, LaneCount, SupportedLaneCount},
};

use super::*;

/// Cycle one times.
/// # Panics
/// Panic if LANES is either minor of nrows or ncols
pub fn cycle<const LANES: usize>(mut tiles: Vec<u8>, ncols: usize, nrows: usize) -> Vec<u8>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    assert!(
        ncols < LANES * 2 && nrows < LANES * 2 && ncols > LANES && nrows > LANES,
        "invalid LANES"
    );

    let row_mask_high = Mask::<isize, LANES>::from_array(array::from_fn(|i| i + LANES < nrows));
    let column_mask_high = Mask::<isize, LANES>::from_array(array::from_fn(|i| i + LANES < ncols));

    let ncols_1 = Simd::splat(ncols + 1);

    // north
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let (mut state_low, mut state_high) = (splat(0), splat(0));
        let (mut r_idx_low, mut r_idx_high) = (range_from(0), range_from(LANES));
        let mut r_1 = splat(1);
        for _ in 0..nrows {
            let (values_low, values_high) = (
                Simd::gather_or(&tiles, r_idx_low, splat(0)),
                Simd::gather_select(&tiles, row_mask_high, r_idx_high, splat(0)),
            );

            let (o_tiles_low, o_tiles_high) = (
                values_low.simd_eq(splat(b'O')).cast::<isize>(),
                values_high.simd_eq(splat(b'O')).cast::<isize>() & row_mask_high,
            );
            splat(b'O').scatter_select(&mut new_tiles, o_tiles_low, state_low * ncols_1 + range_from(0));
            splat(b'O').scatter_select(&mut new_tiles, o_tiles_high, state_high * ncols_1 + range_from(LANES));

            let (sharp_tiles_low, sharp_tiles_high) = (
                values_low.simd_eq(splat(b'#')).cast::<isize>(),
                values_high.simd_eq(splat(b'#')).cast::<isize>() & row_mask_high,
            );
            splat(b'#').scatter_select(&mut new_tiles, sharp_tiles_low, r_idx_low);
            splat(b'#').scatter_select(&mut new_tiles, sharp_tiles_high, r_idx_high);

            (state_low, state_high) = (
                o_tiles_low.select(state_low + splat(1), sharp_tiles_low.select(r_1, state_low)),
                o_tiles_high.select(state_high + splat(1), sharp_tiles_high.select(r_1, state_high)),
            );

            (r_idx_low, r_idx_high) = (r_idx_low + ncols_1, r_idx_high + ncols_1);
            r_1 += splat(1);
        }
    }

    // // west
    // tiles = new_tiles;
    // let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    // {
    //     let mut state = zero;
    //     for c in 0..ncols {
    //         let c_idx = first_column_idx + Simd::splat(c);

    //         let values = Simd::gather_select(&tiles, column_mask, c_idx, zero_u8);

    //         let o_tiles = values.simd_eq(o).cast::<isize>() & column_mask;
    //         o.scatter_select(&mut new_tiles, o_tiles, state + first_column_idx);

    //         let sharp_tiles = values.simd_eq(sharp).cast::<isize>() & column_mask;
    //         sharp.scatter_select(&mut new_tiles, sharp_tiles, c_idx);

    //         state = o_tiles.select(state + one, sharp_tiles.select(Simd::splat(c + 1), state));
    //     }
    // }

    // // south
    // tiles = new_tiles;
    // let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    // {
    //     let mut state = Simd::<usize, LANES>::splat(nrows - 1);
    //     for r in (0..nrows).rev() {
    //         let r_idx = first_row_idx + Simd::splat((ncols + 1) * r);

    //         let values = Simd::gather_select(&tiles, row_mask, r_idx, zero_u8);

    //         let o_tiles = values.simd_eq(o).cast::<isize>() & row_mask;
    //         o.scatter_select(&mut new_tiles, o_tiles, state * ncols_1 + first_row_idx);

    //         let sharp_tiles = values.simd_eq(sharp).cast::<isize>() & row_mask;
    //         sharp.scatter_select(&mut new_tiles, sharp_tiles, r_idx);

    //         state = o_tiles.select(
    //             state.saturating_sub(one),
    //             sharp_tiles.select(Simd::splat(r.saturating_sub(1)), state),
    //         );
    //     }
    // }

    // // est
    // tiles = new_tiles;
    // let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    // {
    //     let mut state = Simd::splat(ncols - 1);
    //     for c in (0..ncols).rev() {
    //         let c_idx = first_column_idx + Simd::splat(c);

    //         let values = Simd::gather_select(&tiles, column_mask, c_idx, zero_u8);

    //         let o_tiles = values.simd_eq(o).cast::<isize>() & column_mask;
    //         o.scatter_select(&mut new_tiles, o_tiles, state + first_column_idx);

    //         let sharp_tiles = values.simd_eq(sharp).cast::<isize>() & column_mask;
    //         sharp.scatter_select(&mut new_tiles, sharp_tiles, c_idx);

    //         state = o_tiles.select(
    //             state.saturating_sub(one),
    //             sharp_tiles.select(Simd::splat(c.saturating_sub(1)), state),
    //         );
    //     }
    // }

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
        ncols < LANES * 2 && nrows < LANES * 2 && ncols > LANES && nrows > LANES,
        "invalid LANES"
    );

    let row_mask_high = Mask::from_array(array::from_fn(|i| i + LANES < ncols));
    let ncols_1 = splat(ncols + 1);
    
    let (mut acc_low, mut acc_high) = (splat(0), splat(0));
    let mut ncols_1_r = splat(0);
    let mut nrows_r = Simd::splat(nrows);
    for _ in 0..nrows {
        let (idx_low, idx_high) = (range_from(0) + ncols_1_r, range_from(LANES) + ncols_1_r);

        let (o_tiles_low, o_tiles_high) = (
            Simd::gather_or(tiles, idx_low, splat(0)).simd_eq(splat(b'O')),
            Simd::gather_select(tiles, row_mask_high, idx_high, splat(0)).simd_eq(splat(b'O')),
        );

        acc_low = o_tiles_low.cast().select(acc_low + nrows_r, acc_low);
        acc_high = o_tiles_high.cast().select(acc_high + nrows_r, acc_high);

        ncols_1_r += ncols_1;
        nrows_r -= splat(1);
    }

    acc_low.reduce_sum() + row_mask_high.select(acc_high, Simd::splat(0)).reduce_sum()
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use crate::parse;
    use super::*;
    use crate::simple;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../../example1");
    }

    #[test]
    fn test_same_results_for_load() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        assert_eq!(
            load::<8>(&tiles, ncols, nrows),
            simple::load(&tiles, ncols, nrows)
        );
    }

    #[test]
    fn test_same_results_for_cycle() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let simd_r = cycle::<8>(tiles.to_vec(), ncols, nrows);
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
    #[should_panic(expected = "invalid LANES")]
    fn test_load_with_invalid_lanes() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let _ = load::<16>(&tiles, ncols, nrows);
    }

    #[test]
    #[should_panic(expected = "invalid LANES, must be > (nrows, ncols)")]
    fn test_cycle_with_invalid_lanes() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let _ = cycle::<8>(tiles.to_vec(), ncols, nrows);
    }
}
