use std::collections::HashSet;

use super::graph::Graph;

pub fn dist(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    assert!(a.len() == b.len());

    let mut sum = 0f32;
    for i in 0..a.len() {
        sum += (b[i] - a[i]).powi(2);
    }
    sum.sqrt()
}

pub fn medoid(g: &Graph<Vec<f32>>) -> Option<usize> {
    let mut result = None;
    let mut best = f32::MAX;

    for i in 0..g.vertices() {
        let mut sum = 0f32;
        for j in 0..g.vertices() {
            if i == j { continue; }
            sum += dist(g.vertex(i), g.vertex(j));
        }
        if sum < best {
            best = sum;
            result = Some(i);
        }
    }

    result
}

fn closest(g: &Graph<Vec<f32>>, q: &Vec<f32>, candidates: HashSet<usize>, k: usize) -> HashSet<usize> {
    let mut closest: Vec<_> = candidates
        .into_iter()
        .map(|v| (dist(q, g.vertex(v)), v))
        .collect();
    closest.sort_by(|a, b| a.partial_cmp(b).unwrap());
    closest.into_iter().take(k).map(|(_, v)| v).collect()
}

pub fn greedy_search(g: &Graph<Vec<f32>>, s: usize, q: &Vec<f32>, k: usize, l: usize) -> (HashSet<usize>, HashSet<usize>) {
    let mut result = HashSet::from([s]);
    let mut v = HashSet::new();

    loop {
        let mut p_star = usize::MAX;
        let mut min_dist = f32::MAX;

        let mut is_empty = true;
        for &i in result.difference(&v) {
            is_empty = false;
            let dist = dist(q, g.vertex(i));
            if dist < min_dist {
                min_dist = dist;
                p_star = i;
            }
        }
        if is_empty { break; }

        for neighbor in g.neighbors(p_star) {
            result.insert(neighbor);
        }
        v.insert(p_star);

        if result.len() > l {
            result = closest(g, q, result, l);
        }
    }

    // return closest k points from result
    (closest(g, q, result, k), v)
}

pub fn robust_prune(g: &mut Graph<Vec<f32>>, p: usize, mut visited: HashSet<usize>, alpha: f32, r: usize) {
    visited.extend(g.neighbors(p));
    visited.remove(&p);

    g.clear_neighbors(p);

    while !visited.is_empty() {
        let mut p_star = usize::MAX;
        let mut min_dist = f32::MAX;
        for &p_prime in &visited {
            let dist = dist(g.vertex(p), g.vertex(p_prime));
            if dist < min_dist {
                min_dist = dist;
                p_star = p_prime;
            }
        }

        g.add_edge(p, p_star);

        if g.neighbors(p).count() == r {
            break;
        }

        let mut to_remove = HashSet::new();
        for &p_prime in &visited {
            if alpha * dist(g.vertex(p_star), g.vertex(p_prime)) <= dist(g.vertex(p), g.vertex(p_prime)) {
                to_remove.insert(p_prime);
            }
        }
        for p_prime in to_remove {
            visited.remove(&p_prime);
        }
    }
}