#![allow(incomplete_features)]
#![feature(generic_const_exprs, slice_index_methods, bigint_helper_methods)]

use clap::Parser;
use std::{hint::black_box, sync::LazyLock};

mod batch;
mod latency;
mod mt_throughput;
mod util;

use util::*;

const CACHELINE: usize = 64;
type PaddedUsize = Padded<usize, CACHELINE>;
type PaddedPointer = Padded<*const usize, CACHELINE>;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    from: Option<usize>,
    #[clap(short, long)]
    to: Option<usize>,
    #[clap(short, long)]
    release: bool,
    #[clap(short, long)]
    dense: bool,
    #[clap(short = 'j', long)]
    threads: usize,
    experiment: Option<Experiment>,
}

static ARGS: LazyLock<Args> = LazyLock::new(|| Args::parse());
static STEPS: LazyLock<usize> =
    LazyLock::new(|| if ARGS.release { 10_000_000 } else { 10_000_000 });

#[derive(clap::ValueEnum, Copy, Clone)]
enum Experiment {
    Latency,
    Batch,
    MtThroughput,
}

fn main() {
    let e = ARGS.experiment.unwrap_or(Experiment::Latency);
    match e {
        Experiment::Latency => latency::latency_exp(),
        Experiment::Batch => batch::batch_exp(),
        Experiment::MtThroughput => {
            eprintln!("\nThreads {}", ARGS.threads);
            for b in sizes() {
                mt_throughput::mt_throughput(b, ARGS.threads);
                mt_throughput::mt_throughput_pairs(b, ARGS.threads);
            }
        }
    }
}
