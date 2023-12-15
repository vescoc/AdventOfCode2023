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

    let row_mask = Mask::<isize, LANES>::from_array(array::from_fn(|i| i < nrows));
    let column_mask = Mask::<isize, LANES>::from_array(array::from_fn(|i| i < ncols));

    let first_row_idx = Simd::<usize, LANES>::from_slice(&IDX[0..LANES]);
    let first_column_idx = first_row_idx * Simd::splat(ncols + 1);

    let zero = Simd::<usize, LANES>::from_array([0; LANES]);
    let zero_u8 = Simd::<u8, LANES>::from_array([0; LANES]);
    let one = Simd::<usize, LANES>::from_array([1; LANES]);

    let o = Simd::<u8, LANES>::from_array([b'O'; LANES]);
    let sharp = Simd::<u8, LANES>::from_array([b'#'; LANES]);

    let ncols_1 = Simd::splat(ncols + 1);

    // north
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = zero;
        for r in 0..nrows {
            let r_idx = first_row_idx + Simd::splat((ncols + 1) * r);

            let values = Simd::gather_select(&tiles, row_mask, r_idx, zero_u8);

            let o_tiles = values.simd_eq(o).cast::<isize>() & row_mask;
            o.scatter_select(&mut new_tiles, o_tiles, state * ncols_1 + first_row_idx);

            let sharp_tiles = values.simd_eq(sharp).cast::<isize>() & row_mask;
            sharp.scatter_select(&mut new_tiles, sharp_tiles, r_idx);

            state = o_tiles.select(state + one, sharp_tiles.select(Simd::splat(r + 1), state));
        }
    }

    // west
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = zero;
        for c in 0..ncols {
            let c_idx = first_column_idx + Simd::splat(c);

            let values = Simd::gather_select(&tiles, column_mask, c_idx, zero_u8);

            let o_tiles = values.simd_eq(o).cast::<isize>() & column_mask;
            o.scatter_select(&mut new_tiles, o_tiles, state + first_column_idx);

            let sharp_tiles = values.simd_eq(sharp).cast::<isize>() & column_mask;
            sharp.scatter_select(&mut new_tiles, sharp_tiles, c_idx);

            state = o_tiles.select(state + one, sharp_tiles.select(Simd::splat(c + 1), state));
        }
    }

    // south
    tiles = new_tiles;
    let mut new_tiles = vec![b'.'; (ncols + 1) * nrows];
    {
        let mut state = Simd::<usize, LANES>::splat(nrows - 1);
        for r in (0..nrows).rev() {
            let r_idx = first_row_idx + Simd::splat((ncols + 1) * r);

            let values = Simd::gather_select(&tiles, row_mask, r_idx, zero_u8);

            let o_tiles = values.simd_eq(o).cast::<isize>() & row_mask;
            o.scatter_select(&mut new_tiles, o_tiles, state * ncols_1 + first_row_idx);

            let sharp_tiles = values.simd_eq(sharp).cast::<isize>() & row_mask;
            sharp.scatter_select(&mut new_tiles, sharp_tiles, r_idx);

            state = o_tiles.select(
                state.saturating_sub(one),
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
            let c_idx = first_column_idx + Simd::splat(c);

            let values = Simd::gather_select(&tiles, column_mask, c_idx, zero_u8);

            let o_tiles = values.simd_eq(o).cast::<isize>() & column_mask;
            o.scatter_select(&mut new_tiles, o_tiles, state + first_column_idx);

            let sharp_tiles = values.simd_eq(sharp).cast::<isize>() & column_mask;
            sharp.scatter_select(&mut new_tiles, sharp_tiles, c_idx);

            state = o_tiles.select(
                state.saturating_sub(one),
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
        ncols < LANES * 2 && nrows < LANES * 2 && ncols > LANES && nrows > LANES,
        "invalid LANES"
    );

    let base_idx_low = Simd::<usize, LANES>::from_slice(&IDX[0..LANES]);
    let base_idx_high = Simd::<usize, LANES>::from_slice(&IDX[LANES..LANES * 2]);
    let zero = Simd::splat(0_u8);
    let o = Simd::splat(b'O');
    let row_mask_high = Mask::from_array(array::from_fn(|i| i + LANES < ncols));

    let (mut acc_low, mut acc_high) = (
        Simd::<usize, LANES>::splat(0),
        Simd::<usize, LANES>::splat(0),
    );

    for r in 0..nrows {
        let ncols_1_r = Simd::splat((ncols + 1) * r);

        let (idx_low, idx_high) = (base_idx_low + ncols_1_r, base_idx_high + ncols_1_r);

        let (o_tiles_low, o_tiles_high) = (
            Simd::gather_or(tiles, idx_low, zero).simd_eq(o),
            Simd::gather_select(tiles, row_mask_high, idx_high, zero).simd_eq(o),
        );

        let nrows_r = Simd::splat(nrows - r);

        acc_low = o_tiles_low.cast().select(acc_low + nrows_r, acc_low);
        acc_high = o_tiles_high.cast().select(acc_high + nrows_r, acc_high);
    }

    acc_low.reduce_sum() + row_mask_high.select(acc_high, Simd::splat(0)).reduce_sum()
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use crate::parse;
    use crate::simd2 as simd;
    use crate::simple;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
    }

    #[test]
    fn test_same_results_for_load() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        assert_eq!(
            simd::load::<8>(&tiles, ncols, nrows),
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
    #[should_panic(expected = "invalid LANES")]
    fn test_load_with_invalid_lanes() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let _ = simd::load::<16>(&tiles, ncols, nrows);
    }

    #[test]
    #[should_panic(expected = "invalid LANES, must be > (nrows, ncols)")]
    fn test_cycle_with_invalid_lanes() {
        let (tiles, ncols, nrows) = parse(&EXAMPLE_1).unwrap();

        let _ = simd::cycle::<8>(tiles.to_vec(), ncols, nrows);
    }
}
