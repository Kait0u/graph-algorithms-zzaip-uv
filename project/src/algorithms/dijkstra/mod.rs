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
            let current_distance = distances[current_vertex.0].unwrap(); // We only pick vertices with a known distance.

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
