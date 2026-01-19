use super::*;
use std::{array::from_fn, hint::black_box};

fn setup_batch<const B: usize>(
    size: usize,
) -> (AlignedVec<Padded<*const usize, 64>>, [*const usize; B]) {
    let (v_usize, order) = util::derangement_with_order::<PaddedUsize>(size);
    let v: Vec<*const usize> = vec![0 as *const usize; v_usize.len()];
    let mut v = AlignedVec::<PaddedPointer>::from(&v);
    let offset = v.as_ptr();
    for i in 0..v.len() {
        v[i] = (unsafe { offset.offset(v_usize[i] as isize) } as *const usize).into();
    }
    let i0: [_; B] = from_fn(|i| *v[order[i * v.len() / B]]);
    (v, i0)
}

pub fn batch_impl<const B: usize, const PREFETCH: bool, const WORK: usize>(size: usize) -> Result {
    let loops = black_box(WORK);
    let (v, i0) = setup_batch::<B>(size);
    let offset = v.as_ptr() as *const usize;
    Result::new(size, *STEPS, B, &v, || {
        let mut is = i0;
        let mut sum = 0;
        for _ in 0..*STEPS / B {
            for i in &mut is {
                *i = unsafe { **i } as *const usize;
                if PREFETCH {
                    prefetch_ptr(*i);
                }
                if WORK > 0 {
                    let mut x = unsafe { (*i).offset_from(offset) } as usize;
                    let mut y = x;
                    let mut z = x;
                    for _ in 0..loops {
                        x = x + (x >> 1);
                        y = y.widening_mul(x).1;
                        z += y;
                    }
                    sum += z;
                }
            }
        }
        black_box(is);
        black_box(sum);
    })
}

pub fn batch<const B: usize>(size: usize) -> Result {
    batch_impl::<B, false, 0>(size)
}
#[allow(unused)]
pub fn batch_prefetch<const B: usize>(size: usize) -> Result {
    batch_impl::<B, true, 0>(size)
}
pub fn batch_work3<const B: usize>(size: usize) -> Result {
    batch_impl::<B, false, 3>(size)
}
pub fn batch_prefetch_work3<const B: usize>(size: usize) -> Result {
    batch_impl::<B, true, 3>(size)
}
pub fn batch_work6<const B: usize>(size: usize) -> Result {
    batch_impl::<B, false, 6>(size)
}
pub fn batch_prefetch_work6<const B: usize>(size: usize) -> Result {
    batch_impl::<B, true, 6>(size)
}
pub fn batch_work12<const B: usize>(size: usize) -> Result {
    batch_impl::<B, false, 12>(size)
}
pub fn batch_prefetch_work12<const B: usize>(size: usize) -> Result {
    batch_impl::<B, true, 12>(size)
}

pub fn batch_exp() {
    let results = &mut vec![];
    run_experiment(batch::<1>, results);
    run_experiment(batch::<2>, results);
    run_experiment(batch::<4>, results);
    run_experiment(batch::<8>, results);
    run_experiment(batch::<10>, results);
    run_experiment(batch::<11>, results);
    run_experiment(batch::<12>, results);
    run_experiment(batch::<13>, results);
    run_experiment(batch::<16>, results);
    run_experiment(batch::<32>, results);

    run_experiment(batch_prefetch::<16>, results);
    run_experiment(batch_prefetch::<32>, results);

    run_experiment(batch_work3::<16>, results);
    run_experiment(batch_work6::<16>, results);
    run_experiment(batch_work12::<16>, results);

    run_experiment(batch_prefetch_work3::<16>, results);
    run_experiment(batch_prefetch_work6::<16>, results);
    run_experiment(batch_prefetch_work12::<16>, results);

    save_results(results, "batch");
}
