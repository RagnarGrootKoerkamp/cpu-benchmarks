use super::*;

/// Pointer-chase a derangement.
pub fn pointer_chasing_checked(size: usize) -> Result {
    let v = util::derangement::<usize>(size);
    Result::new(size, *STEPS, 1, &v, || {
        let mut i = 0;
        for _ in 0..*STEPS {
            i = v[i];
        }
        black_box(i);
    })
}

// From here on, everything is unchecked.
pub fn pointer_chasing(size: usize) -> Result {
    let v = Unchecked(util::derangement::<usize>(size));
    Result::new(size, *STEPS, 1, &v, || {
        let mut i = 0;
        for _ in 0..*STEPS {
            i = v[i];
        }
        black_box(i);
    })
}

pub fn pointer_chasing_padded(size: usize) -> Result {
    let v = util::derangement::<PaddedUsize>(size);
    let v: Unchecked<Vec<PaddedUsize>> = Unchecked(v.iter().map(|&x| x.into()).collect());

    Result::new(size, *STEPS, 1, &v, || {
        let mut i = 0;
        for _ in 0..*STEPS {
            i = *v[i];
        }
        black_box(i);
    })
}

/// Replace indices with actual pointers.
pub fn raw_pointer_chasing(size: usize) -> Result {
    let v_usize = util::derangement::<usize>(size);
    let mut v: Vec<*const usize> = vec![0 as *const usize; v_usize.len()];
    let offset = v.as_ptr();
    for i in 0..v.len() {
        v[i] = (unsafe { offset.offset(v_usize[i] as isize) } as *const usize).into();
    }
    Result::new(size, *STEPS, 1, &v, || {
        let mut i: *const usize = v[0];
        for _ in 0..*STEPS {
            i = unsafe { *i } as *const usize;
        }
        black_box(i);
    })
}

pub fn raw_pointer_chasing_padded(size: usize) -> Result {
    // Use PaddedUsize to ensure the length is correct.
    let v_usize = util::derangement::<PaddedUsize>(size);
    let mut v: Vec<PaddedPointer> = vec![(0 as *const usize).into(); v_usize.len()];
    let offset = v.as_ptr();
    for i in 0..v.len() {
        v[i] = (unsafe { offset.offset(v_usize[i] as isize) } as *const usize).into();
    }
    Result::new(size, *STEPS, 1, &v, || {
        let mut i: *const usize = *v[0];
        for _ in 0..*STEPS {
            i = unsafe { *i } as *const usize;
        }
        black_box(i);
    })
}

// From here on, everything is padded.

pub fn pointer_chasing_padded_aligned(size: usize) -> Result {
    let v = util::derangement::<PaddedUsize>(size);
    let v = AlignedVec::<PaddedUsize>::from(&v);

    Result::new(size, *STEPS, 1, &v, || {
        let mut i = 0;
        for _ in 0..*STEPS {
            i = *v[i];
        }
        black_box(i);
    })
}

pub fn raw_pointer_chasing_padded_aligned(size: usize) -> Result {
    // Use PaddedUsize to ensure the length is correct.
    let v_usize = util::derangement::<PaddedUsize>(size);
    let v: Vec<*const usize> = vec![0 as *const usize; v_usize.len()];
    let mut v = AlignedVec::<PaddedPointer>::from(&v);
    let offset = v.as_ptr();
    for i in 0..v.len() {
        v[i] = (unsafe { offset.offset(v_usize[i] as isize) } as *const usize).into();
    }
    Result::new(size, *STEPS, 1, &v, || {
        let mut i: *const usize = *v[0];
        for _ in 0..*STEPS {
            i = unsafe { *i } as *const usize;
        }
        black_box(i);
    })
}

pub fn latency_exp() {
    let results = &mut vec![];

    // run_experiment(pointer_chasing_checked, results);
    // run_experiment(pointer_chasing, results);
    // run_experiment(pointer_chasing_padded, results);
    // run_experiment(raw_pointer_chasing, results);
    // run_experiment(raw_pointer_chasing_padded, results);
    // run_experiment(pointer_chasing_padded_aligned, results);
    run_experiment(raw_pointer_chasing_padded_aligned, results);

    save_results(results, "latency");
}
