use std::time::Duration;

/// Per-run algorithm metrics (Report I, Par. 2.4, Table 2.2).
#[derive(Default, Debug, Clone, Copy)]
pub struct AlgorithmMetrics {
    /// Deterministic operation counters.
    pub logic: AlgorithmLogicMetrics,
    /// Environment-dependent time and memory.
    pub execution: AlgorithmExecutionMetrics,
}

/// Deterministic operation counters (Report I, Par. 2.4, Table 2.2).
#[derive(Default, Debug, Clone, Copy)]
pub struct AlgorithmLogicMetrics {
    /// C_comp - comparison count.
    pub comparison_count: u64,

    /// N_upd — successful distance/predessor updates.
    pub update_count: u64,

    /// N_op - data-structure read/write count.
    pub datastructure_op_count: u64,

    /// I_iter - full relaxation iterations (Bellman-Ford only).
    pub iteration_count: Option<u64>,

    /// V_exp - expanded vertices (A* only).
    pub expanded_vertices_count: Option<u64>,
}

impl AlgorithmLogicMetrics {
    pub fn increment_comparison_count(&mut self) {
        self.comparison_count += 1;
    }

    pub fn increment_update_count(&mut self) {
        self.update_count += 1;
    }

    pub fn increment_datastructure_op_count(&mut self) {
        self.datastructure_op_count += 1;
    }

    pub fn increment_iteration_count(&mut self) {
        self.iteration_count = Some(self.iteration_count.map_or(1, |count| count + 1));
    }

    pub fn increment_expanded_vertices_count(&mut self) {
        self.expanded_vertices_count =
            Some(self.expanded_vertices_count.map_or(1, |count| count + 1));
    }
}

/// Environment-dependent execution metrics (Report I, Par. 2.4, Table 2.2).
#[derive(Default, Debug, Clone, Copy)]
pub struct AlgorithmExecutionMetrics {
    /// t_exec - wall-clock execution time.
    pub wall_clock_time: Duration,

    /// M_aux - peak auxiliary memory, in bytes. 'None' when not yet measured.
    pub auxiliary_memory_bytes: Option<u64>,
}
