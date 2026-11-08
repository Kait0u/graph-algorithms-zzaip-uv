use crate::data_model::graphs::VertexId;

struct ResultSPSP {}

struct ResultSSSP {
    pub distances: Vec<Option<i64>>,
    pub predecessors: Vec<Option<VertexId>>,
}

struct ResultAPSP {}
