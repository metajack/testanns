use std::collections::HashSet;
use std::path::Path;

use anyhow::Result;
use indicatif::ProgressBar;
use rand::seq::SliceRandom;

use crate::{algo, data, graph::Graph};

pub struct Index {
    graph: Graph<Vec<f32>>,
    medoid: usize,
    l: usize,
}

impl Index {
    pub fn build_from_fvecs<P: AsRef<Path>>(path: P, alpha: f32, l: usize, r: usize) -> Result<Index> {
        let vs = data::load_fvecs(path)?;
        let mut graph = Graph::empty();
        for v in vs {
            graph.add_vertex(v);
        }
        graph.add_random_k_regular_out_edges(10);

        let medoid = algo::medoid(&graph).unwrap();

        let mut index = Index {
            graph,
            medoid,
            l,
        };

        //plot::plot_graph("build0.png", &index.graph);

        // // first pass with alpha=1.0
        // println!("indexing (first pass)...");
        // index.build(1.0f32, l, r);
        // plot::plot_graph("build1.png", &index.graph);

        // first pass with real alpha
        println!("indexing...");
        index.build(alpha, l, r);
        //plot::plot_graph("build2.png", &index.graph);

        Ok(index)
    }

    fn build(&mut self, alpha: f32, l: usize, r: usize) {
        let mut rng = rand::thread_rng();
        let mut sigma: Vec<_> = (0usize..self.graph.vertices()).into_iter().collect();
        sigma.shuffle(&mut rng);
        let sigma = sigma;

        let sigma_bar = ProgressBar::new(sigma.len() as u64);
        for &i in &sigma {
            let (_, v_set) = algo::greedy_search(&self.graph, self.medoid, self.graph.vertex(i), 1, l);

            algo::robust_prune(&mut self.graph, i, v_set, alpha, r);

            let n_out_i: HashSet<_> = self.graph.neighbors(i).collect();
            for j in n_out_i {
                let mut n_out_j: HashSet<_> = self.graph.neighbors(j).collect();
                n_out_j.insert(i);
                if n_out_j.len() > r {
                    algo::robust_prune(&mut self.graph, j, n_out_j, alpha, r);
                } else {
                    self.graph.clear_neighbors(j);
                    for j_prime in n_out_j {
                        self.graph.add_edge(j, j_prime);
                    }
                }
            }

            sigma_bar.inc(1);
        }
        sigma_bar.finish();
    }

    pub fn search(&self, q: &Vec<f32>, k: usize) -> HashSet<usize> {
        assert!(k <= self.l);

        let (l_set, _) = algo::greedy_search(&self.graph, self.medoid, q, k, self.l);
        l_set
    }

    pub fn item(&self, i: usize) -> &Vec<f32> {
        self.graph.vertex(i)
    }
}