use rand::seq::SliceRandom;

pub struct Graph<T> {
    vertices: Vec<T>,
    adjacency: Vec<Vec<bool>>,
}

impl<T> Graph<T> {
    pub fn empty() -> Graph<T> {
        let vertices = vec![];
        let adjacency = vec![];
        Graph { vertices, adjacency }
    }

    pub fn add_random_k_regular_out_edges(&mut self, k: usize) {
        let n = self.vertices.len();
        let mut rng = rand::thread_rng();
        let choices: Vec<usize> = (0..n-1).into_iter().collect();
        for i in 0..n {
            for &j in choices.choose_multiple(&mut rng, k) {
                // we skip our own index, so increment choice by one for later vertices
                let y = if j >= i { j + 1 } else { j };

                self.add_edge(i, y);
            }
        }
    }

    pub fn add_vertex(&mut self, vertex: T) {
        self.vertices.push(vertex);
        self.adjacency.push(vec![]);
        let n = self.vertices.len();
        for v in 0..n {
            while self.adjacency[v].len() < n {
                self.adjacency[v].push(false);
            }
        }
    }

    pub fn add_edge(&mut self, x: usize, y: usize) {
        assert!(x != y);
        self.adjacency[x][y] = true;
    }

    pub fn vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertex(&self, i: usize) -> &T {
        &self.vertices[i]
    }

    pub fn neighbors<'a>(&'a self, vertex: usize) -> NeighborIterator<'a, T> {
        NeighborIterator::new(self, vertex)
    }

    pub fn clear_neighbors(&mut self, vertex: usize) {
        self.adjacency[vertex] = vec![false; self.vertices.len()];
    }

    #[allow(dead_code)]
    pub fn print_dot(&self) {
        println!("strict digraph {{");
        for i in 0..self.vertices.len() {
            print!("  V{} -> {{ ", i);
            let mut first = true;
            for (j, &is_connected) in self.adjacency[i].iter().enumerate() {
                if is_connected {
                    if !first {
                        print!(", ");
                    }
                    first = false;
                    print!("V{}", j);
                }
            }
            println!(" }}");
        }
        println!("}}");
    }
}

pub struct NeighborIterator<'a, T> {
    graph: &'a Graph<T>,
    vertex: usize,
    cursor: usize,
}

impl<'a, T> NeighborIterator<'a, T> {
    fn new(graph: &'a Graph<T>, vertex: usize) -> Self {
        Self {
            graph,
            vertex,
            cursor: 0,
        }
    }
}

impl<'a, T> Iterator for NeighborIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        let mut i = self.cursor;
        while result.is_none() && i < self.graph.vertices.len() {
            if self.graph.adjacency[self.vertex][i] {
                result = Some(i);
            }
            i += 1;
        }
        self.cursor = i;
        result
    }
}
