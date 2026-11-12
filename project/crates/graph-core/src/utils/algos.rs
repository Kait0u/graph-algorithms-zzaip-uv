use std::collections::{HashSet, VecDeque};

use crate::data_model::graphs::{GraphArc, GraphDefinition, VertexId};

pub fn bfs(graph_definition: &GraphDefinition, source: VertexId) -> Vec<VertexId> {
    let mut achievable = vec![source];

    let arcs = &graph_definition.arcs;

    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    visited.insert(source.0);
    queue.push_back(source.0);

    while let Some(curr_idx) = queue.pop_front() {
        for &GraphArc {
            source: VertexId(arc_source_idx),
            destination: VertexId(arc_destination_idx),
            weight: _,
        } in arcs
        {
            if arc_source_idx == curr_idx && visited.insert(arc_destination_idx) {
                achievable.push(VertexId(arc_destination_idx));
                queue.push_back(arc_destination_idx);
            }
        }
    }

    achievable
}

#[cfg(test)]
mod tests {
    use super::bfs;
    use crate::data_model::graphs::GraphDefinitionBuilder;

    /// Checks that BFS visits every reachable vertex once, even when the graph has a cycle.
    #[test]
    fn bfs_visits_reachable_vertices_once_in_a_graph_with_a_cycle() {
        let mut builder = GraphDefinitionBuilder::default();
        let source = builder.add_vertex(None);
        let first = builder.add_vertex(None);
        let second = builder.add_vertex(None);
        let unreachable = builder.add_vertex(None);

        builder.add_arc(first, source, 1);
        builder.add_arc(source, first, 1);
        builder.add_arc(source, second, 1);
        builder.add_arc(second, first, 1);

        let graph = builder.build();
        let reachable = bfs(&graph, source);

        assert_eq!(reachable, vec![source, first, second]);
        assert!(!reachable.contains(&unreachable));
    }
}
