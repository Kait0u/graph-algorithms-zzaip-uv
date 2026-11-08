use min_heap::MinHeap;

use crate::{
    algorithms::results::ResultSSSP,
    data_model::graphs::{Graph, GraphRepresentation, GraphVertex, VertexId},
};

pub struct LinearDijkstra;

impl LinearDijkstra {
    pub fn run(graph: &Graph<impl GraphRepresentation>, start_id: VertexId) -> ResultSSSP {
        let vertex_count = graph.vertex_count();

        let mut predecessors: Vec<Option<VertexId>> = vec![None; vertex_count];
        let mut distances: Vec<Option<i64>> = vec![None; vertex_count];
        distances[start_id.0] = Some(0);

        // Visitation tracker
        let mut visited: Vec<bool> = vec![false; vertex_count];

        for _ in 0..(vertex_count - 1) {
            let Some(vertex_id) = Self::find_min_unvisited(&distances, &visited) else {
                break;
            };

            visited[vertex_id] = true;
            let curr_dist_to_vertex = distances[vertex_id].unwrap(); // Has to exist, otherwise wouldn't be a min.

            for outgoing_arc in graph.outgoing_arcs(VertexId(vertex_id)) {
                let neighbor_id = outgoing_arc.destination;
                let outgoing_weight = outgoing_arc.weight; // Current to Neighbor

                let potential_new_dist_to_neighbor = curr_dist_to_vertex + outgoing_weight;
                let curr_dist_to_neighbor = distances[neighbor_id.0];

                // If there is no alternative or if the new one is better, improve and add to the heap.
                if curr_dist_to_neighbor.is_none()
                    || curr_dist_to_neighbor.unwrap() > potential_new_dist_to_neighbor
                {
                    let new_distance_to_neighbor = potential_new_dist_to_neighbor;
                    distances[neighbor_id.0] = Some(new_distance_to_neighbor);
                    predecessors[neighbor_id.0] = Some(VertexId(vertex_id));
                }
            }
        }

        ResultSSSP {
            distances,
            predecessors,
        }
    }

    fn find_min_unvisited(distances: &[Option<i64>], visited: &[bool]) -> Option<usize> {
        let mut candidate: Option<usize> = None;
        let mut min_so_far: Option<i64> = None;

        for (idx, is_visited) in visited.iter().enumerate() {
            if *is_visited {
                continue;
            }

            let dist = distances[idx];
            if let Some(d) = dist {
                if min_so_far.is_none_or(|min| d < min) {
                    candidate = Some(idx);
                    min_so_far = Some(d);
                }
            }
        }

        candidate
    }
}

pub struct HeapDijkstra;

impl HeapDijkstra {
    pub fn run(graph: &Graph<impl GraphRepresentation>, start_id: VertexId) -> ResultSSSP {
        let vertex_count = graph.vertex_count();

        let mut predecessors: Vec<Option<VertexId>> = vec![None; vertex_count];
        let mut distances: Vec<Option<i64>> = vec![None; vertex_count];
        distances[start_id.0] = Some(0);

        // Priority Queue (MinHeap)
        let mut heap = MinHeap::<DijkstraCandidate>::new();
        heap.push(DijkstraCandidate::new(start_id, 0));

        while !heap.is_empty() {
            let DijkstraCandidate { vertex_id, weight } = heap.pop().unwrap(); // Not empty, so has to exist.
            let curr_dist_to_vertex = distances[vertex_id.0].unwrap(); // Was inserted into the heap so has to exist.

            // If current vertex already equals that value, skip.
            if weight != curr_dist_to_vertex {
                continue;
            }

            for outgoing_arc in graph.outgoing_arcs(vertex_id) {
                let neighbor_id = outgoing_arc.destination;
                let outgoing_weight = outgoing_arc.weight; // Current to Neighbor

                let potential_new_dist_to_neighbor = curr_dist_to_vertex + outgoing_weight;

                let curr_dist_to_neighbor = distances[neighbor_id.0];

                // If there is no alternative or if the new one is better, improve and add to the heap.
                if curr_dist_to_neighbor.is_none()
                    || curr_dist_to_neighbor.unwrap() > potential_new_dist_to_neighbor
                {
                    let new_distance_to_neighbor = potential_new_dist_to_neighbor;
                    distances[neighbor_id.0] = Some(new_distance_to_neighbor);
                    predecessors[neighbor_id.0] = Some(vertex_id);
                    heap.push(DijkstraCandidate::new(
                        neighbor_id,
                        new_distance_to_neighbor,
                    ));
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
struct DijkstraCandidate {
    vertex_id: VertexId,
    weight: i64,
}

impl DijkstraCandidate {
    fn new(vertex_id: VertexId, weight: i64) -> DijkstraCandidate {
        DijkstraCandidate { vertex_id, weight }
    }
}

impl PartialEq for DijkstraCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for DijkstraCandidate {}

impl PartialOrd for DijkstraCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}
