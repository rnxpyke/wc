#![cfg_attr(test, feature(test))]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![cfg_attr(feature = "simd", feature(slice_as_chunks))]

use std::io::Write;

#[cfg(feature = "simd")]
mod simd;

#[cfg(feature = "simd")]
pub fn count_newlines(input: &[u8]) -> usize {
    simd::count_newlines_simd::<64>(input)
}

#[cfg(not(feature = "simd"))]
pub fn count_newlines(input: &[u8]) -> usize {
    count_newlines_slice(input)
}

fn count_newlines_slice(input: &[u8]) -> usize {
    let mut newlines = 0;
    let mut chunks = input.chunks_exact(32);
    for chunk in &mut chunks {
        for &c in chunk {
            newlines += if c == b'\n' { 1 } else { 0 };
        }
    }
    for &c in chunks.remainder() {
        newlines += if c == b'\n' { 1 } else { 0 };
    }

    newlines
}

#[derive(Default)]
pub struct WC {
    pub newlines: usize,
}

impl Write for WC {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.newlines += count_newlines(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    pub const LIBER_PRIMUS: &str = include_str!("liber-primus.txt");

    #[bench]
    fn bench_count_newlines_slice(b: &mut Bencher) {
        b.iter(|| count_newlines_slice(black_box(LIBER_PRIMUS.as_bytes())))
    }
}
