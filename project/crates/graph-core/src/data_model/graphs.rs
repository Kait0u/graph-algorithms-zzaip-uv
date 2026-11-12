use std::usize;

use crate::data_model::math::IntCoords2D;

#[derive(Debug, Clone)]
pub struct GraphDefinition {
    pub vertices: Vec<GraphVertex>,
    pub arcs: Vec<GraphArc>,
}

impl GraphDefinition {
    pub fn new(vertices: Vec<GraphVertex>, arcs: Vec<GraphArc>) -> Self {
        GraphDefinition { vertices, arcs }
    }
}

#[derive(Default)]
pub struct GraphDefinitionBuilder {
    vertices: Vec<GraphVertex>,
    arcs: Vec<GraphArc>,
}

impl GraphDefinitionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, coords: Option<IntCoords2D>) -> VertexId {
        let id = VertexId(self.vertices.len());
        self.vertices.push(GraphVertex { coords });
        id
    }

    pub fn add_arc(&mut self, from: VertexId, to: VertexId, cost: i64) {
        let arc = GraphArc::new(from, to, cost);
        self.arcs.push(arc);
    }

    pub fn build(self) -> GraphDefinition {
        GraphDefinition::new(self.vertices, self.arcs)
    }
}

// -------------------------------------------------------------------------------------------------

pub struct Graph<R: GraphRepresentation> {
    vertices: Vec<GraphVertex>,
    representation: R,
}

impl<R: GraphRepresentation> Graph<R> {
    pub fn vertices(&self) -> &[GraphVertex] {
        &self.vertices
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn representation(&self) -> &R {
        &self.representation
    }

    pub fn outgoing_arcs(&self, from: VertexId) -> impl Iterator<Item = GraphArc> + '_ {
        self.representation().outgoing_arcs(from)
    }

    pub fn arcs(&self) -> impl Iterator<Item = GraphArc> + '_ {
        (0..self.vertex_count()).flat_map(move |id: usize| self.outgoing_arcs(VertexId(id)))
    }
}

pub type AdjacencyListGraph = Graph<AdjacencyList>;
impl AdjacencyListGraph {
    pub fn new(vertices: Vec<GraphVertex>, adjacency_list: AdjacencyList) -> AdjacencyListGraph {
        AdjacencyListGraph {
            vertices,
            representation: adjacency_list,
        }
    }

    pub fn from_definition(definition: &GraphDefinition) -> AdjacencyListGraph {
        let vertices = definition.vertices.clone();
        let mut adjacency_list = AdjacencyList::default();

        let vertex_count = vertices.len();
        for _ in 0..vertex_count {
            adjacency_list.outgoing.push(Vec::new());
        }

        for arc in &definition.arcs {
            adjacency_list.outgoing[arc.source.0].push(*arc);
        }

        AdjacencyListGraph::new(vertices, adjacency_list)
    }
}

pub type AdjacencyMatrixGraph = Graph<AdjacencyMatrix>;
impl AdjacencyMatrixGraph {
    pub fn new(
        vertices: Vec<GraphVertex>,
        adjacency_matrix: AdjacencyMatrix,
    ) -> AdjacencyMatrixGraph {
        AdjacencyMatrixGraph {
            vertices,
            representation: adjacency_matrix,
        }
    }

    pub fn from_definition(definition: &GraphDefinition) -> AdjacencyMatrixGraph {
        let vertices = definition.vertices.clone();
        let vertex_count = vertices.len();

        let mut adjacency_matrix = AdjacencyMatrix::new(vertex_count);

        for GraphArc {
            source,
            destination,
            weight,
        } in &definition.arcs
        {
            adjacency_matrix.set(source.0, destination.0, Some(*weight));
        }

        AdjacencyMatrixGraph::new(vertices, adjacency_matrix)
    }
}

pub trait GraphRepresentation {
    fn outgoing_arcs(&self, from: VertexId) -> impl Iterator<Item = GraphArc> + '_;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexId(pub usize);

#[derive(Default)]
pub struct AdjacencyList {
    outgoing: Vec<Vec<GraphArc>>,
}

impl GraphRepresentation for AdjacencyList {
    fn outgoing_arcs(&self, from: VertexId) -> impl Iterator<Item = GraphArc> + '_ {
        self.outgoing[from.0].iter().copied()
    }
}

pub struct AdjacencyMatrix {
    vertex_count: usize,

    // Matrix of weights, or None if no connection
    // Linear row-major representation: M[r, c] = r * vertex_count + c
    // Row is FROM, Column is TO
    matrix: Vec<Option<i64>>,
}

impl GraphRepresentation for AdjacencyMatrix {
    fn outgoing_arcs(&self, from: VertexId) -> impl Iterator<Item = GraphArc> + '_ {
        let n = self.vertex_count;
        assert!(from.0 < n);

        let start = from.0 * n;
        let row = &self.matrix[start..start + n];

        row.iter()
            .enumerate()
            .filter_map(move |(to, &weight)| weight.map(|w| GraphArc::new(from, VertexId(to), w)))
    }
}

impl AdjacencyMatrix {
    pub fn new(vertex_count: usize) -> AdjacencyMatrix {
        let cell_count = vertex_count
            .checked_mul(vertex_count)
            .expect("Adjacency matrix dimensions overflow usize");

        AdjacencyMatrix {
            vertex_count,
            matrix: vec![None; cell_count],
        }
    }

    fn calculate_idx(&self, row_idx: usize, col_idx: usize) -> usize {
        row_idx * self.vertex_count + col_idx
    }

    pub fn get(&self, row_idx: usize, col_idx: usize) -> Option<i64> {
        let idx = self.calculate_idx(row_idx, col_idx);
        self.matrix[idx]
    }

    pub fn set(&mut self, row_idx: usize, col_idx: usize, value: Option<i64>) {
        let idx = self.calculate_idx(row_idx, col_idx);
        self.matrix[idx] = value;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GraphVertex {
    pub coords: Option<IntCoords2D>,
}

impl GraphVertex {
    pub fn new(coords: IntCoords2D) -> Self {
        GraphVertex {
            coords: Some(coords),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GraphArc {
    pub source: VertexId,
    pub destination: VertexId,
    pub weight: i64,
}

impl GraphArc {
    pub fn new(source: VertexId, destination: VertexId, weight: i64) -> Self {
        GraphArc {
            source,
            destination,
            weight,
        }
    }
}
