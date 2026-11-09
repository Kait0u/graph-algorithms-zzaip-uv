use crate::{
    algorithms::{
        bellman_ford::BellmanFord,
        dijkstra::{HeapDijkstra, LinearDijkstra},
    },
    data_model::graphs::{
        AdjacencyListGraph, AdjacencyMatrixGraph, GraphDefinition, GraphDefinitionBuilder, VertexId,
    },
    utils::experiments::graph_generator::RandomGraphGenerator,
};

mod algorithms;
mod data_model;
mod utils;

fn test1() {
    let mut def_builder = GraphDefinitionBuilder::default();
    let a = def_builder.add_vertex(None);
    let b = def_builder.add_vertex(None);
    let c = def_builder.add_vertex(None);
    let d = def_builder.add_vertex(None);
    let e = def_builder.add_vertex(None);

    def_builder.add_arc(a, b, 6);
    def_builder.add_arc(a, c, 2);
    def_builder.add_arc(b, d, 2);
    def_builder.add_arc(b, e, 9);
    def_builder.add_arc(c, b, 1);
    def_builder.add_arc(c, d, 7);
    def_builder.add_arc(d, e, 3);

    let definition: GraphDefinition = def_builder.build();

    let graph_list = AdjacencyListGraph::from_definition(&definition);
    let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

    println!("T1: LIST");
    println!("Heap:");
    let results = HeapDijkstra::run(&graph_list, a);
    println!("{results:?}");
    println!("Linear:");
    let results = LinearDijkstra::run(&graph_list, a);
    println!("{results:?}");

    println!("T1: MATRIX");
    println!("Heap:");
    let results = HeapDijkstra::run(&graph_matrix, a);
    println!("{results:?}");
    println!("Linear:");
    let results = LinearDijkstra::run(&graph_matrix, a);
    println!("{results:?}");
}

fn test2() {
    let mut def_builder = GraphDefinitionBuilder::default();

    let a = def_builder.add_vertex(None);
    let b = def_builder.add_vertex(None);
    let c = def_builder.add_vertex(None);
    let d = def_builder.add_vertex(None);
    let e = def_builder.add_vertex(None);
    let f = def_builder.add_vertex(None);
    let g = def_builder.add_vertex(None);
    let h = def_builder.add_vertex(None);
    let i = def_builder.add_vertex(None);
    let j = def_builder.add_vertex(None);
    let k = def_builder.add_vertex(None);

    // Starting routes from A
    def_builder.add_arc(a, b, 9);
    def_builder.add_arc(a, c, 4);
    def_builder.add_arc(a, d, 15);

    // B
    def_builder.add_arc(b, e, 7);
    def_builder.add_arc(b, f, 12);
    def_builder.add_arc(b, c, 2);

    // C
    def_builder.add_arc(c, b, 3);
    def_builder.add_arc(c, d, 5);
    def_builder.add_arc(c, f, 11);
    def_builder.add_arc(c, e, 16);

    // D
    def_builder.add_arc(d, f, 3);
    def_builder.add_arc(d, g, 12);
    def_builder.add_arc(d, c, 2);

    // E
    def_builder.add_arc(e, h, 9);
    def_builder.add_arc(e, f, 2);
    def_builder.add_arc(e, i, 17);

    // F
    def_builder.add_arc(f, e, 4);
    def_builder.add_arc(f, g, 6);
    def_builder.add_arc(f, h, 9);

    // G
    def_builder.add_arc(g, i, 4);
    def_builder.add_arc(g, j, 13);
    def_builder.add_arc(g, d, 1);

    // H
    def_builder.add_arc(h, i, 3);
    def_builder.add_arc(h, j, 7);

    // I
    def_builder.add_arc(i, j, 2);
    def_builder.add_arc(i, k, 9);

    // J
    def_builder.add_arc(j, k, 3);
    def_builder.add_arc(j, h, 1);

    // K
    def_builder.add_arc(k, g, 5);

    let definition: GraphDefinition = def_builder.build();

    let graph_list = AdjacencyListGraph::from_definition(&definition);
    let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

    println!("T2: LIST");
    println!("Heap:");
    let results = HeapDijkstra::run(&graph_list, a);
    println!("{results:?}");
    println!("Linear:");
    let results = LinearDijkstra::run(&graph_list, a);
    println!("{results:?}");

    println!("T2: MATRIX");
    println!("Heap:");
    let results = HeapDijkstra::run(&graph_matrix, a);
    println!("{results:?}");
    println!("Linear:");
    let results = LinearDijkstra::run(&graph_matrix, a);
    println!("{results:?}");
}

fn test3() {
    let mut def_builder = GraphDefinitionBuilder::default();
    let a = def_builder.add_vertex(None);
    let b = def_builder.add_vertex(None);
    let c = def_builder.add_vertex(None);
    let d = def_builder.add_vertex(None);
    let e = def_builder.add_vertex(None);

    def_builder.add_arc(a, b, 6);
    def_builder.add_arc(a, c, 2);
    def_builder.add_arc(b, d, 2);
    def_builder.add_arc(b, e, 9);
    def_builder.add_arc(c, b, 1);
    def_builder.add_arc(c, d, 7);
    def_builder.add_arc(d, e, 3);

    let definition: GraphDefinition = def_builder.build();

    let graph_list = AdjacencyListGraph::from_definition(&definition);
    let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

    println!("T3: LIST");
    let results = BellmanFord::run(&graph_list, a);
    println!("{results:?}");

    println!("T3: MATRIX");
    let results = BellmanFord::run(&graph_matrix, a);
    println!("{results:?}");
}

fn test4() {
    let mut def_builder = GraphDefinitionBuilder::default();

    let a = def_builder.add_vertex(None);
    let b = def_builder.add_vertex(None);
    let c = def_builder.add_vertex(None);
    let d = def_builder.add_vertex(None);
    let e = def_builder.add_vertex(None);
    let f = def_builder.add_vertex(None);
    let g = def_builder.add_vertex(None);
    let h = def_builder.add_vertex(None);
    let i = def_builder.add_vertex(None);
    let j = def_builder.add_vertex(None);
    let k = def_builder.add_vertex(None);

    // Starting routes from A
    def_builder.add_arc(a, b, 9);
    def_builder.add_arc(a, c, 4);
    def_builder.add_arc(a, d, 15);

    // B
    def_builder.add_arc(b, e, 7);
    def_builder.add_arc(b, f, 12);
    def_builder.add_arc(b, c, 2);

    // C
    def_builder.add_arc(c, b, 3);
    def_builder.add_arc(c, d, 5);
    def_builder.add_arc(c, f, 11);
    def_builder.add_arc(c, e, 16);

    // D
    def_builder.add_arc(d, f, 3);
    def_builder.add_arc(d, g, 12);
    def_builder.add_arc(d, c, 2);

    // E
    def_builder.add_arc(e, h, 9);
    def_builder.add_arc(e, f, 2);
    def_builder.add_arc(e, i, 17);

    // F
    def_builder.add_arc(f, e, 4);
    def_builder.add_arc(f, g, 6);
    def_builder.add_arc(f, h, 9);

    // G
    def_builder.add_arc(g, i, 4);
    def_builder.add_arc(g, j, 13);
    def_builder.add_arc(g, d, 1);

    // H
    def_builder.add_arc(h, i, 3);
    def_builder.add_arc(h, j, 7);

    // I
    def_builder.add_arc(i, j, 2);
    def_builder.add_arc(i, k, 9);

    // J
    def_builder.add_arc(j, k, 3);
    def_builder.add_arc(j, h, 1);

    // K
    def_builder.add_arc(k, g, 5);

    let definition: GraphDefinition = def_builder.build();

    let graph_list = AdjacencyListGraph::from_definition(&definition);
    let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

    println!("T4: LIST");
    let results = BellmanFord::run(&graph_list, a);
    println!("{results:?}");

    println!("T4: MATRIX");
    let results = BellmanFord::run(&graph_matrix, a);
    println!("{results:?}");
}

fn test5() {
    let mut generator = RandomGraphGenerator::new(42u64);
    let definition: GraphDefinition = generator.random_graph(20, 0.5, None);

    let graph_list = AdjacencyListGraph::from_definition(&definition);
    let graph_matrix = AdjacencyMatrixGraph::from_definition(&definition);

    println!("T5: BF-LIST");
    let results = BellmanFord::run(&graph_list, VertexId(0));
    println!("{results:?}");

    println!("T5: BF-MATRIX");
    let results = BellmanFord::run(&graph_matrix, VertexId(0));
    println!("{results:?}");

    println!("T5: DH-LIST");
    let results = HeapDijkstra::run(&graph_list, VertexId(0));
    println!("{results:?}");

    println!("T5: DH-MATRIX");
    let results = HeapDijkstra::run(&graph_matrix, VertexId(0));
    println!("{results:?}");

    println!("T5: DL-LIST");
    let results = LinearDijkstra::run(&graph_list, VertexId(0));
    println!("{results:?}");

    println!("T5: DL-MATRIX");
    let results = LinearDijkstra::run(&graph_matrix, VertexId(0));
    println!("{results:?}");
}

fn main() {
    println!("Hello, world!");
    println!("---------------------------------------------");
    test1();
    println!("---------------------------------------------");
    test2();
    println!("---------------------------------------------");
    test3();
    println!("---------------------------------------------");
    test4();
    println!("---------------------------------------------");
    test5();
}
