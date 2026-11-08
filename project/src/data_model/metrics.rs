#[derive(Default, Debug, Clone, Copy)]
pub struct GeneralExecutionMetadata {
    comparison_count: u64,
    update_count: u64,
    datasctructure_op_count: u64,
    iteration_count: u64,
    expanded_vertices_count: u64,
}
