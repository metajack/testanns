use std:: path::PathBuf;
use rand::Rng;

use clap::Parser;
use testanns::data;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    count: usize,
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let mut vs = vec![];
    for _ in 0..args.count {
        let x = rng.gen_range(0f32..1f32);
        let y = rng.gen_range(0f32..1f32);
        vs.push(vec![x, y]);
    }

    data::save_fvecs(args.output, &vs)?;

    Ok(())
}