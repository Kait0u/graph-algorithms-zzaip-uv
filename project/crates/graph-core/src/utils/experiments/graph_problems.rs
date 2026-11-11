use rand::{RngExt, SeedableRng, rngs::StdRng, seq::IteratorRandom};

use crate::{
    data_model::graphs::{GraphDefinition, VertexId},
    utils::algos::bfs,
};

pub struct GraphProblem {
    pub graph_definition: GraphDefinition,
    pub source: VertexId,
    pub destination: VertexId,
    pub destination_accessible: bool,
}

impl GraphProblem {
    pub fn new(
        graph: GraphDefinition,
        source: VertexId,
        destination: VertexId,
        destination_accessible: bool,
    ) -> Self {
        GraphProblem {
            graph_definition: graph,
            source,
            destination,
            destination_accessible,
        }
    }
}

pub struct GraphProblemGenerator {
    rng: StdRng,
}

impl GraphProblemGenerator {
    pub fn new(seed: u64) -> Self {
        GraphProblemGenerator {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn random_problem(&mut self, graph_definition: GraphDefinition) -> GraphProblem {
        let vertex_count = graph_definition.vertices.len();

        let source_idx = self.rng.random_range(0..vertex_count);
        let source = VertexId(source_idx);

        let destination_idx = (0..vertex_count)
            .filter(|idx| idx != &source_idx)
            .choose(&mut self.rng)
            .unwrap();
        let destination = VertexId(destination_idx);

        let achievable_vertices = bfs(&graph_definition, VertexId(source_idx));
        let is_achievable = achievable_vertices.contains(&destination);

        GraphProblem::new(graph_definition.clone(), source, destination, is_achievable)
    }
}
