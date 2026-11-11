use rand::{RngExt, SeedableRng, rngs::StdRng, seq::SliceRandom};

use crate::data_model::{
    graphs::{GraphArc, GraphDefinition, GraphVertex, VertexId},
    math::IntCoords2D,
};

pub struct RandomGraphGenerator {
    rng: StdRng,
}

impl RandomGraphGenerator {
    const MIN_WEIGHT_SCALE: f64 = 1.0;

    pub fn new(seed: u64) -> Self {
        RandomGraphGenerator {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn random_graph(
        &mut self,
        vertex_count: u16,
        density: f64,
        max_weight_factor: Option<f64>,
    ) -> GraphDefinition {
        let weight_scale = max_weight_factor.unwrap_or(Self::MIN_WEIGHT_SCALE);
        let weight_scale = if weight_scale >= Self::MIN_WEIGHT_SCALE {
            weight_scale
        } else {
            Self::MIN_WEIGHT_SCALE
        };

        self.random_graph_internal(vertex_count, density, weight_scale)
    }

    pub fn random_graph_sample(
        &mut self,
        sample_size: u64,
        vertex_count: u16,
        density: f64,
        max_weight_factor: Option<f64>,
    ) -> Vec<GraphDefinition> {
        let mut results = Vec::new();

        for _ in 0..sample_size {
            results.push(self.random_graph(vertex_count, density, max_weight_factor));
        }

        results
    }

    fn random_graph_internal(
        &mut self,
        vertex_count: u16,
        density: f64,
        max_weight_factor: f64,
    ) -> GraphDefinition {
        let grid_side = (3f64 * (vertex_count as f64).sqrt()).ceil() as u16;
        let arc_count = (density * vertex_count as f64 * (vertex_count - 1) as f64).ceil() as u64;

        let mut coords: Vec<IntCoords2D> = coord_grid(grid_side as i32);
        coords.shuffle(&mut self.rng);

        let mut vertices: Vec<GraphVertex> = Vec::new();

        for idx in 0..vertex_count as usize {
            vertices.push(GraphVertex::new(coords[idx]));
        }

        let arcs = {
            let mut pairs = all_nonlooping_arcs(vertex_count);
            pairs.shuffle(&mut self.rng);
            pairs = pairs[..arc_count as usize].to_vec();

            let mut temp_arcs: Vec<GraphArc> = Vec::new();
            for (source, destination) in pairs {
                let source_coords = vertices[source].coords.unwrap();
                let dest_coords = vertices[destination].coords.unwrap();

                let dist = source_coords.euclidean_distance(&dest_coords);
                let factor = self.rng.random_range(1.0..=max_weight_factor);
                let weight = (factor * dist).ceil() as i64;

                temp_arcs.push(GraphArc::new(
                    VertexId(source),
                    VertexId(destination),
                    weight,
                ));
            }

            temp_arcs
        };

        GraphDefinition::new(vertices, arcs)
    }
}

fn coord_grid(grid_side: i32) -> Vec<IntCoords2D> {
    let mut result: Vec<IntCoords2D> = Vec::new();

    for x in 0..grid_side {
        for y in 0..grid_side {
            result.push(IntCoords2D::new(x, y));
        }
    }

    result
}

fn all_nonlooping_arcs(vertex_count: u16) -> Vec<(usize, usize)> {
    let mut possible_arcs: Vec<(usize, usize)> = Vec::new();

    for u in 0..vertex_count as usize {
        for v in 0..vertex_count as usize {
            if u != v {
                possible_arcs.push((u, v));
            }
        }
    }

    possible_arcs
}
