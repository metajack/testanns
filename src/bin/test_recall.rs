use std:: path::PathBuf;

use clap::Parser;
use testanns::{data, index};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    alpha: f32,
    r: usize,
    l: usize,

    input: PathBuf,
    true_k: usize,
    total_k: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let index: Result<index::Index, anyhow::Error> = index::Index::build_from_fvecs(
        &args.input,
        args.alpha,
        args.l,
        args.r,
    );


    Ok(())
}