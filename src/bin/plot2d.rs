use std::path::PathBuf;

use rand::Rng;

use clap::Parser;
use testanns::{
    algo::{self, greedy_search},
    data,
    graph::Graph,
    plot,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("reading {}...", args.input.display());

    let vs = data::load_fvecs(args.input)?;

    assert!(!vs.is_empty());
    println!("{} vectors of dim {}", vs.len(), vs[0].len());

    let mut g = Graph::empty();
    for v in &vs {
        g.add_vertex(v.clone());
    }
    g.add_random_k_regular_out_edges(10);

    let medoid = algo::medoid(&g).unwrap();
    println!("medoid = {:?}", g.vertex(medoid));

    let mut rng = rand::thread_rng();
    let s = medoid;
    let q = vec![rng.gen_range(0f32..1f32), rng.gen_range(0f32..1f32)];
    let results = greedy_search(&g, s, &q, 10, 25);
    println!("closest = {:?}", results.0);

    plot::plot_graph("rand2d10k.png", &g);
    Ok(())
}
