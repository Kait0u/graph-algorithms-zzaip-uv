use min_heap::MinHeap;

use crate::data_model::graphs::{Graph, GraphRepresentation, VertexId};

pub struct HeapDijkstra {}

impl HeapDijkstra {
    pub fn run(
        graph: &Graph<impl GraphRepresentation>,
        start_id: VertexId,
    ) -> (Vec<Option<i64>>, Vec<Option<VertexId>>) {
        let vertex_count = graph.vertex_count();

        let mut distances: Vec<Option<i64>> = vec![None; vertex_count];
        distances[start_id.0] = Some(0);

        let mut predecessors: Vec<Option<VertexId>> = vec![None; vertex_count];

        // Priority Queue (MinHeap)
        let mut heap = MinHeap::<DijkstraHeapEntry>::new();
        heap.push(DijkstraHeapEntry::new(start_id, 0));

        while !heap.is_empty() {
            let DijkstraHeapEntry { vertex_id, weight } = heap.pop().unwrap(); // Not empty, so has to exist.
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
                    heap.push(DijkstraHeapEntry::new(
                        neighbor_id,
                        new_distance_to_neighbor,
                    ));
                }
            }
        }

        (distances, predecessors)
    }
}

#[derive(Debug, Clone, Copy)]
struct DijkstraHeapEntry {
    vertex_id: VertexId,
    weight: i64,
}

impl DijkstraHeapEntry {
    fn new(vertex_id: VertexId, weight: i64) -> DijkstraHeapEntry {
        DijkstraHeapEntry { vertex_id, weight }
    }
}

impl PartialEq for DijkstraHeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for DijkstraHeapEntry {}

impl PartialOrd for DijkstraHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraHeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}
