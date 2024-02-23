use std::simd::{LaneCount, SupportedLaneCount};

pub fn count_newlines_simd<const LANES: usize>(input: &[u8]) -> usize
where
    LaneCount<LANES>: SupportedLaneCount,
{
    use std::simd::cmp::SimdPartialEq;
    use std::simd::Simd;
    let mut newlines = 0;
    let (chunks, remainder) = input.as_chunks::<LANES>();
    for chunk in chunks {
        let simd: Simd<u8, LANES> = Simd::from_array(*chunk);
        let mask = simd.simd_eq(Simd::splat(b'\n')).to_bitmask();
        newlines += mask.count_ones() as usize;
    }
    for &c in remainder {
        newlines += if c == b'\n' { 1 } else { 0 };
    }

    newlines
}

#[cfg(test)]
mod tests {
    extern crate test;
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    pub const LIBER_PRIMUS: &str = include_str!("liber-primus.txt");

    #[bench]
    fn bench_count_newlines_simd32(b: &mut Bencher) {
        b.iter(|| count_newlines_simd::<32>(black_box(LIBER_PRIMUS.as_bytes())))
    }

    #[bench]
    fn bench_count_newlines_simd16(b: &mut Bencher) {
        b.iter(|| count_newlines_simd::<16>(black_box(LIBER_PRIMUS.as_bytes())))
    }

    #[bench]
    fn bench_count_newlines_simd64(b: &mut Bencher) {
        b.iter(|| count_newlines_simd::<64>(black_box(LIBER_PRIMUS.as_bytes())))
    }
}
