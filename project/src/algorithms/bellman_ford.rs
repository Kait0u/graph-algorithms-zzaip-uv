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

#[cfg(test)]
mod tests {
    use super::BellmanFord;

    use crate::data_model::graphs::{
        AdjacencyListGraph, AdjacencyMatrixGraph, GraphDefinition, GraphDefinitionBuilder, VertexId,
    };

    fn build_small_graph() -> GraphDefinition {
        let mut builder = GraphDefinitionBuilder::default();
        let a = builder.add_vertex(None);
        let b = builder.add_vertex(None);
        let c = builder.add_vertex(None);
        let d = builder.add_vertex(None);
        let e = builder.add_vertex(None);

        builder.add_arc(a, b, 6);
        builder.add_arc(a, c, 2);
        builder.add_arc(b, d, 2);
        builder.add_arc(b, e, 9);
        builder.add_arc(c, b, 1);
        builder.add_arc(c, d, 7);
        builder.add_arc(d, e, 3);

        builder.build()
    }

    fn build_larger_graph() -> GraphDefinition {
        let mut builder = GraphDefinitionBuilder::default();

        let a = builder.add_vertex(None);
        let b = builder.add_vertex(None);
        let c = builder.add_vertex(None);
        let d = builder.add_vertex(None);
        let e = builder.add_vertex(None);
        let f = builder.add_vertex(None);
        let g = builder.add_vertex(None);
        let h = builder.add_vertex(None);
        let i = builder.add_vertex(None);
        let j = builder.add_vertex(None);
        let k = builder.add_vertex(None);

        builder.add_arc(a, b, 9);
        builder.add_arc(a, c, 4);
        builder.add_arc(a, d, 15);

        builder.add_arc(b, e, 7);
        builder.add_arc(b, f, 12);
        builder.add_arc(b, c, 2);

        builder.add_arc(c, b, 3);
        builder.add_arc(c, d, 5);
        builder.add_arc(c, f, 11);
        builder.add_arc(c, e, 16);

        builder.add_arc(d, f, 3);
        builder.add_arc(d, g, 12);
        builder.add_arc(d, c, 2);

        builder.add_arc(e, h, 9);
        builder.add_arc(e, f, 2);
        builder.add_arc(e, i, 17);

        builder.add_arc(f, e, 4);
        builder.add_arc(f, g, 6);
        builder.add_arc(f, h, 9);

        builder.add_arc(g, i, 4);
        builder.add_arc(g, j, 13);
        builder.add_arc(g, d, 1);

        builder.add_arc(h, i, 3);
        builder.add_arc(h, j, 7);

        builder.add_arc(i, j, 2);
        builder.add_arc(i, k, 9);

        builder.add_arc(j, k, 3);
        builder.add_arc(j, h, 1);

        builder.add_arc(k, g, 5);

        builder.build()
    }

    #[test]
    fn bellman_ford_computes_distances_on_small_graph() {
        let definition = build_small_graph();
        let graph_list = AdjacencyListGraph::from_definition(&definition);
        let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

        let expected = vec![Some(0), Some(3), Some(2), Some(5), Some(8)];

        assert_eq!(
            BellmanFord::run(&graph_list, VertexId(0)).unwrap().distances,
            expected
        );
        assert_eq!(
            BellmanFord::run(&graph_matrix, VertexId(0)).unwrap().distances,
            expected
        );
    }

    #[test]
    fn bellman_ford_computes_distances_on_larger_graph() {
        let definition = build_larger_graph();
        let graph_list = AdjacencyListGraph::from_definition(&definition);
        let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

        let expected = vec![
            Some(0),
            Some(7),
            Some(4),
            Some(9),
            Some(14),
            Some(12),
            Some(18),
            Some(21),
            Some(22),
            Some(24),
            Some(27),
        ];

        assert_eq!(
            BellmanFord::run(&graph_list, VertexId(0)).unwrap().distances,
            expected
        );
        assert_eq!(
            BellmanFord::run(&graph_matrix, VertexId(0)).unwrap().distances,
            expected
        );
    }
}
