use min_heap::MinHeap;

use crate::{
    algorithms::results::ResultSSSP,
    data_model::graphs::{Graph, GraphRepresentation, VertexId},
};

pub struct LinearDijkstra;

impl LinearDijkstra {
    pub fn run(graph: &Graph<impl GraphRepresentation>, source_id: VertexId) -> ResultSSSP {
        let vertex_count = graph.vertex_count();

        let mut predecessors: Vec<Option<VertexId>> = vec![None; vertex_count];
        let mut distances: Vec<Option<i64>> = vec![None; vertex_count];
        distances[source_id.0] = Some(0);

        // Keep track of vertices we've already picked.
        let mut settled: Vec<bool> = vec![false; vertex_count];

        for _ in 0..(vertex_count - 1) {
            let Some((current_vertex, current_distance)) =
                Self::find_closest_unsettled(&distances, &settled)
            else {
                break;
            };

            settled[current_vertex.0] = true;

            for arc in graph.outgoing_arcs(current_vertex) {
                let neighbor_id = arc.destination;
                let edge_weight = arc.weight; // Cost of going from the current vertex to its neighbor.

                let candidate_distance = current_distance + edge_weight;
                let known_distance = distances[neighbor_id.0];

                // Save this path if it's the first one found or shorter than the previous one.
                if known_distance.is_none_or(|distance| candidate_distance < distance) {
                    distances[neighbor_id.0] = Some(candidate_distance);
                    predecessors[neighbor_id.0] = Some(current_vertex);
                }
            }
        }

        ResultSSSP {
            distances,
            predecessors,
        }
    }

    fn find_closest_unsettled(
        distances: &[Option<i64>],
        settled: &[bool],
    ) -> Option<(VertexId, i64)> {
        let mut closest: Option<(VertexId, i64)> = None;

        for (vertex_index, is_settled) in settled.iter().enumerate() {
            if *is_settled {
                continue;
            }

            let Some(distance) = distances[vertex_index] else {
                continue;
            };

            if closest.is_none_or(|(_, closest_distance)| distance < closest_distance) {
                closest = Some((VertexId(vertex_index), distance));
            }
        }

        closest
    }
}

pub struct HeapDijkstra;

impl HeapDijkstra {
    pub fn run(graph: &Graph<impl GraphRepresentation>, source_id: VertexId) -> ResultSSSP {
        let vertex_count = graph.vertex_count();

        let mut predecessors: Vec<Option<VertexId>> = vec![None; vertex_count];
        let mut distances: Vec<Option<i64>> = vec![None; vertex_count];
        distances[source_id.0] = Some(0);

        // Pick the closest queued vertex first.
        let mut heap = MinHeap::<HeapEntry>::new();
        heap.push(HeapEntry::new(source_id, 0));

        while let Some(HeapEntry {
            vertex_id: current_vertex,
            distance,
        }) = heap.pop()
        {
            let current_distance = distances[current_vertex.0].unwrap(); // Queued vertices always have a known distance.

            // Skip this entry if we found a shorter path since adding it to the heap.
            if distance != current_distance {
                continue;
            }

            for arc in graph.outgoing_arcs(current_vertex) {
                let neighbor_id = arc.destination;
                let edge_weight = arc.weight; // Cost of going from the current vertex to its neighbor.

                let candidate_distance = current_distance + edge_weight;

                let known_distance = distances[neighbor_id.0];

                // Save a better path and add the new distance to the heap.
                if known_distance.is_none_or(|distance| candidate_distance < distance) {
                    distances[neighbor_id.0] = Some(candidate_distance);
                    predecessors[neighbor_id.0] = Some(current_vertex);
                    heap.push(HeapEntry::new(neighbor_id, candidate_distance));
                }
            }
        }

        ResultSSSP {
            distances,
            predecessors,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HeapEntry {
    vertex_id: VertexId,
    distance: i64,
}

impl HeapEntry {
    fn new(vertex_id: VertexId, distance: i64) -> HeapEntry {
        HeapEntry {
            vertex_id,
            distance,
        }
    }
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

#[cfg(test)]
mod tests {
    use super::{HeapDijkstra, LinearDijkstra};

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
    fn dijkstra_computes_distances_on_small_graph() {
        let definition = build_small_graph();
        let graph_list = AdjacencyListGraph::from_definition(&definition);
        let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

        let expected = vec![Some(0), Some(3), Some(2), Some(5), Some(8)];

        assert_eq!(HeapDijkstra::run(&graph_list, VertexId(0)).distances, expected);
        assert_eq!(
            HeapDijkstra::run(&graph_matrix, VertexId(0)).distances,
            expected
        );
        assert_eq!(LinearDijkstra::run(&graph_list, VertexId(0)).distances, expected);
        assert_eq!(
            LinearDijkstra::run(&graph_matrix, VertexId(0)).distances,
            expected
        );
    }

    #[test]
    fn dijkstra_computes_distances_on_larger_graph() {
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

        assert_eq!(HeapDijkstra::run(&graph_list, VertexId(0)).distances, expected);
        assert_eq!(
            HeapDijkstra::run(&graph_matrix, VertexId(0)).distances,
            expected
        );
        assert_eq!(LinearDijkstra::run(&graph_list, VertexId(0)).distances, expected);
        assert_eq!(
            LinearDijkstra::run(&graph_matrix, VertexId(0)).distances,
            expected
        );
    }
}
