#![allow(unused)]
use std::{hint::black_box, time::Instant};

use rand::{thread_rng, Rng};
use rayon::prelude::*;

const CACHELINE: usize = 64;
const ITS: usize = 2;

fn main() {
    // Allocate 16GB bytes of random data.
    let n: usize = 16_000_000_000;
    let data: Vec<u8> = (0..n)
        .into_par_iter()
        .map_init(thread_rng, |rng, _| rng.gen())
        .collect();

    // test_full(&data);
    // test_cacheline(&data);
    // test_stride(&data);
    let strides = (0..200)
        .map(|_| thread_rng().gen_range(1..256) | 1)
        .collect::<Vec<usize>>();
    for threads in [1, 4, 6, 12] {
        let chunk_size = n / threads;
        let data = data.chunks(chunk_size).collect::<Vec<_>>();

        eprint!("Threads: {}", threads);
        let start = Instant::now();
        rayon::scope(|scope| {
            for (data, stride) in data.iter().zip(&strides) {
                // let data = data.clone();
                scope.spawn(move |_| test_stride::<true>(data, *stride));
                // scope.spawn(move |_| test_full(data));
            }
        });
        let e = start.elapsed();
        eprintln!(
            "  {:>8.2?} {:>10.3}GB/s",
            e,
            (ITS * threads * n) as f64 / e.as_nanos() as f64
        );
    }
}

#[inline(never)]
fn test_full(data: &[u8]) {
    let n = data.len();
    let mut sum1 = 0;
    for _ in 0..ITS {
        for x in data {
            sum1 += *x as u64;
        }
    }
    black_box(sum1);
}

#[inline(never)]
fn test_cacheline(data: &Vec<u8>) {
    let n = data.len();
    let mut sum2 = 0;
    for _ in 0..ITS {
        for i in (0..n).step_by(CACHELINE) {
            unsafe {
                sum2 += *data.get_unchecked(i) as u64;
            }
        }
    }
    black_box(sum2);
}

#[inline(never)]
fn test_stride<const PREFETCH: bool>(data: &[u8], s: usize) {
    let n = data.len();
    let mut sum2 = 0;
    let cs = s * CACHELINE;
    for _ in 0..ITS {
        for offset in 0..s {
            for i in (offset * CACHELINE..n).step_by(cs) {
                unsafe {
                    if PREFETCH {
                        // FIXME
                        // ptr_hash::util::prefetch_index(data, i + 10 * cs);
                    }
                    sum2 += *data.get_unchecked(i) as u64;
                }
            }
        }
    }
    black_box(sum2);
}
