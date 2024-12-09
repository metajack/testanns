use std::path::Path;

use plotters::prelude::*;
use super::graph::Graph;

pub fn plot_graph<P: AsRef<Path>>(path: P, g: &Graph<Vec<f32>>) {
    let root = BitMapBackend::new(&path, (1000, 1000))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();
    let (pw, ph) = root.get_pixel_range();

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for i in 0..g.vertices() {
        let v = g.vertex(i);
        let x = v[0];
        let y = v[1];
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    let scale_x = (pw.end - pw.start) as f32 / (max_x - min_x);
    let scale_y = (ph.end - ph.start) as f32 / (max_y - min_y);

    for i in 0..g.vertices() {
        let vi = g.vertex(i);
        let xi = (vi[0] * scale_x) as i32 + pw.start;
        let yi = (vi[1] * scale_y) as i32 + ph.start;

        for j in g.neighbors(i) {
            let vj = g.vertex(j);
            let xj = (vj[0] * scale_x) as i32 + pw.start;
            let yj = (vj[1] * scale_y) as i32 + pw.start;

            root.draw(&PathElement::new(vec![(xi, yi), (xj, yj)], BLACK.mix(0.2))).unwrap();
        }
        //root.draw_pixel((xi, yi), &BLACK).unwrap();
    }

    root.present().unwrap();
}