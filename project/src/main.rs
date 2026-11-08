use crate::{
    algorithms::dijkstra::HeapDijkstra,
    data_model::graphs::{
        AdjacencyListGraph, AdjacencyMatrixGraph, GraphDefinition, GraphDefinitionBuilder,
    },
};

mod algorithms;
mod data_model;

fn test1() {
    let mut def_builder1 = GraphDefinitionBuilder::default();
    let a = def_builder1.add_vertex(None);
    let b = def_builder1.add_vertex(None);
    let c = def_builder1.add_vertex(None);
    let d = def_builder1.add_vertex(None);
    let e = def_builder1.add_vertex(None);

    def_builder1.add_arc(a, b, 6);
    def_builder1.add_arc(a, c, 2);
    def_builder1.add_arc(b, d, 2);
    def_builder1.add_arc(b, e, 9);
    def_builder1.add_arc(c, b, 1);
    def_builder1.add_arc(c, d, 7);
    def_builder1.add_arc(d, e, 3);

    let definition1: GraphDefinition = def_builder1.build();

    let graph1_list = AdjacencyListGraph::from_definition(&definition1);
    let graph1_matrix = AdjacencyMatrixGraph::from_definition(&definition1);

    println!("T1: LIST");
    let results = HeapDijkstra::run(&graph1_list, a);
    println!("{results:?}");

    println!("T1: MATRIX");
    let results = HeapDijkstra::run(&graph1_matrix, a);
    println!("{results:?}");
}

fn test2() {
    let mut def_builder2 = GraphDefinitionBuilder::default();

    let a = def_builder2.add_vertex(None);
    let b = def_builder2.add_vertex(None);
    let c = def_builder2.add_vertex(None);
    let d = def_builder2.add_vertex(None);
    let e = def_builder2.add_vertex(None);
    let f = def_builder2.add_vertex(None);
    let g = def_builder2.add_vertex(None);
    let h = def_builder2.add_vertex(None);
    let i = def_builder2.add_vertex(None);
    let j = def_builder2.add_vertex(None);
    let k = def_builder2.add_vertex(None);

    // Starting routes from A
    def_builder2.add_arc(a, b, 9);
    def_builder2.add_arc(a, c, 4);
    def_builder2.add_arc(a, d, 15);

    // B
    def_builder2.add_arc(b, e, 7);
    def_builder2.add_arc(b, f, 12);
    def_builder2.add_arc(b, c, 2);

    // C
    def_builder2.add_arc(c, b, 3);
    def_builder2.add_arc(c, d, 5);
    def_builder2.add_arc(c, f, 11);
    def_builder2.add_arc(c, e, 16);

    // D
    def_builder2.add_arc(d, f, 3);
    def_builder2.add_arc(d, g, 12);
    def_builder2.add_arc(d, c, 2);

    // E
    def_builder2.add_arc(e, h, 9);
    def_builder2.add_arc(e, f, 2);
    def_builder2.add_arc(e, i, 17);

    // F
    def_builder2.add_arc(f, e, 4);
    def_builder2.add_arc(f, g, 6);
    def_builder2.add_arc(f, h, 9);

    // G
    def_builder2.add_arc(g, i, 4);
    def_builder2.add_arc(g, j, 13);
    def_builder2.add_arc(g, d, 1);

    // H
    def_builder2.add_arc(h, i, 3);
    def_builder2.add_arc(h, j, 7);

    // I
    def_builder2.add_arc(i, j, 2);
    def_builder2.add_arc(i, k, 9);

    // J
    def_builder2.add_arc(j, k, 3);
    def_builder2.add_arc(j, h, 1);

    // K
    def_builder2.add_arc(k, g, 5);

    let definition2: GraphDefinition = def_builder2.build();

    let graph2_list = AdjacencyListGraph::from_definition(&definition2);
    let graph2_matrix = AdjacencyMatrixGraph::from_definition(&definition2);

    println!("T2: LIST");
    let results = HeapDijkstra::run(&graph2_list, a);
    println!("{results:?}");

    println!("T2: MATRIX");
    let results = HeapDijkstra::run(&graph2_matrix, a);
    println!("{results:?}");
}

fn main() {
    println!("Hello, world!");
    println!();
    test1();
    println!();
    test2();
}
