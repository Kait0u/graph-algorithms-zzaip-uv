use crate::{
    algorithms::results::ResultSSSP,
    data_model::graphs::{Graph, GraphArc, GraphRepresentation, VertexId},
};

pub struct BellmanFord;

impl BellmanFord {
    pub fn run(
        graph: &Graph<impl GraphRepresentation>,
        source_id: VertexId,
    ) -> Result<ResultSSSP, BellmanFordError> {
        let vertex_count = graph.vertex_count();

        let mut predecessors: Vec<Option<VertexId>> = vec![None; vertex_count];
        let mut distances: Vec<Option<i64>> = vec![None; vertex_count];

        distances[source_id.0] = Some(0);

        for _ in 0..vertex_count - 1 {
            let mut state_changed: bool = false;

            for GraphArc {
                source,
                destination,
                weight,
            } in graph.arcs()
            {
                if let Some(source_distance) = distances[source.0]
                    && let candidate_distance = source_distance + weight
                    && distances[destination.0].is_none_or(|destination_distance| {
                        candidate_distance < destination_distance
                    })
                {
                    distances[destination.0] = Some(candidate_distance);
                    predecessors[destination.0] = Some(source);
                    state_changed = true;
                }
            }

            // Check if it can finish early.
            // We can even skip the negative cycle check if nothing has changed.
            // That means there's none
            if !state_changed {
                return Ok(ResultSSSP {
                    distances,
                    predecessors,
                });
            }
        }

        // If improvements can still be made, we've got a negative cycle.
        for GraphArc {
            source,
            destination,
            weight,
        } in graph.arcs()
        {
            if let Some(source_distance) = distances[source.0]
                && let candidate_distance = source_distance + weight
                && distances[destination.0]
                    .is_none_or(|destination_distance| candidate_distance < destination_distance)
            {
                return Err(BellmanFordError::NegativeCycle);
            }
        }

        Ok(ResultSSSP {
            distances,
            predecessors,
        })
    }
}

#[derive(Debug)]
pub enum BellmanFordError {
    NegativeCycle,
}
