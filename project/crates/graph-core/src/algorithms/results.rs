use crate::data_model::graphs::VertexId;

pub struct ResultSPSP {}

#[derive(Debug, Clone)]
pub struct ResultSSSP {
    pub distances: Vec<Option<i64>>,
    pub predecessors: Vec<Option<VertexId>>,
}

pub struct ResultAPSP {}
