use std::{hint::black_box, thread};

use crate::PaddedUsize;
use rand::prelude::SliceRandom;

pub fn mt_throughput(b: usize, t: usize) {
    let n = b / 64;
    let data = std::hint::black_box(vec![PaddedUsize::from(1); n]);

    let mut perm: Vec<usize> = std::hint::black_box((0usize..n).collect());
    // shuffle
    perm.shuffle(&mut rand::thread_rng());
    let perm = &perm;

    let start = std::time::Instant::now();
    thread::scope(|scope| {
        let nt = n / t;
        for i in 0..t {
            let start = i * nt;
            let end = (i + 1) * nt;
            let data = &data;
            scope.spawn(move || {
                let mut sum = 0;
                for i in start..end {
                    unsafe {
                        sum += **data.get_unchecked(*perm.get_unchecked(i));
                    }
                }
                black_box(sum);
            });
        }
    });
    let d = start.elapsed();
    let ns_per_query = d.as_secs_f32() * 1.0e9 / n as f32;
    eprintln!(
        "bytes {:>6.3} MiB threads {t}  thrps {ns_per_query:>8.3} ns/q",
        b as f32 / 1024. / 1024.
    );
}

pub fn mt_throughput_pairs(b: usize, t: usize) {
    let n = b / 64;
    let data = std::hint::black_box(vec![PaddedUsize::from(1); n + 1]);

    let mut perm: Vec<usize> = std::hint::black_box((0usize..n).collect());
    // shuffle
    perm.shuffle(&mut rand::thread_rng());
    let perm = &perm;

    let start = std::time::Instant::now();
    thread::scope(|scope| {
        let nt = n / t;
        for i in 0..t {
            let start = i * nt;
            let end = (i + 1) * nt;
            let data = &data;
            scope.spawn(move || {
                let mut sum = 0;
                for i in (start..end).step_by(2) {
                    unsafe {
                        sum += **data.get_unchecked(*perm.get_unchecked(i));
                        sum += **data.get_unchecked(*perm.get_unchecked(i) + 1);
                    }
                }
                black_box(sum);
            });
        }
    });
    let d = start.elapsed();
    let ns_per_query = d.as_secs_f32() * 1.0e9 / n as f32;
    eprintln!(
        "bytes {:>6.3} GiB threads {t}  thrps {ns_per_query:>8.3} ns/q pairs",
        b as f32 / 1024. / 1024. / 1024.
    );
}
