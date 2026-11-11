use graph_core::algorithms::bellman_ford::BellmanFord;
use graph_core::algorithms::dijkstra::{HeapDijkstra, LinearDijkstra};
use graph_core::data_model::graphs::{AdjacencyListGraph, AdjacencyMatrixGraph, VertexId};
use graph_core::utils::experiments::graph_generator::RandomGraphGenerator;

#[test]
fn algorithms_agree_on_generated_graph() {
    let mut generator = RandomGraphGenerator::new(42u64);
    let definition = generator.random_graph(20, 0.5, None);

    let graph_list = AdjacencyListGraph::from_definition(&definition);
    let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

    let source = VertexId(0);

    let bf_list = BellmanFord::run(&graph_list, source).unwrap().distances;
    let bf_matrix = BellmanFord::run(&graph_matrix, source).unwrap().distances;
    let heap_list = HeapDijkstra::run(&graph_list, source).distances;
    let heap_matrix = HeapDijkstra::run(&graph_matrix, source).distances;
    let linear_list = LinearDijkstra::run(&graph_list, source).distances;
    let linear_matrix = LinearDijkstra::run(&graph_matrix, source).distances;

    assert_eq!(bf_list, bf_matrix);
    assert_eq!(bf_list, heap_list);
    assert_eq!(bf_list, heap_matrix);
    assert_eq!(bf_list, linear_list);
    assert_eq!(bf_list, linear_matrix);
}
