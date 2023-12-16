use std::simd::{LaneCount, SimdElement, SupportedLaneCount, Simd};

pub mod simd1;

pub mod simd2;

const fn splat<T, const LANES: usize>(v: T) -> Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    Simd::from_array([v; LANES])
}

const fn range_from<const LANES: usize>(v: usize) -> Simd<usize, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut r = [0; LANES];
    let mut i = 0;
    while i < LANES {
        r[i] = v + i;
        i += 1;
    }
    Simd::from_array(r)
}

const fn range_from_with<const LANES: usize>(v: usize, inc: usize) -> Simd<usize, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut r = [0; LANES];
    let mut i = 0;
    while i < LANES {
        r[i] = v + i * inc;
        i += 1;
    }
    Simd::from_array(r)
}
