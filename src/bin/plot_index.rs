use std::path::PathBuf;

use clap::Parser;
use testanns::index;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let _index: Result<index::Index, anyhow::Error> = index::Index::build_from_fvecs(&args.input, 1.2, 50, 10);

    Ok(())
}
