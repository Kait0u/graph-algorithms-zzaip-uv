use crate::data_model::metrics::AlgorithmLogicMetrics;

pub trait AlgorithmObserver {
    fn finalize(&self) -> Option<AlgorithmLogicMetrics> {
        None
    }

    fn on_comparison(&mut self) {}
    fn on_update(&mut self) {}
    fn on_datastructure_op(&mut self) {}
    fn on_iteration_complete(&mut self) {}
    fn on_vertex_expanded(&mut self) {}
}

pub struct NoOpAlgorithmObserver;
impl AlgorithmObserver for NoOpAlgorithmObserver {}

pub struct MetricsAlgorithmObserver {
    metrics: AlgorithmLogicMetrics,
}
impl AlgorithmObserver for MetricsAlgorithmObserver {
    fn finalize(&self) -> Option<AlgorithmLogicMetrics> {
        Some(self.metrics)
    }

    fn on_comparison(&mut self) {
        self.metrics.increment_comparison_count();
    }

    fn on_update(&mut self) {
        self.metrics.increment_update_count();
    }

    fn on_datastructure_op(&mut self) {
        self.metrics.increment_datastructure_op_count();
    }

    fn on_iteration_complete(&mut self) {
        self.metrics.increment_iteration_count();
    }

    fn on_vertex_expanded(&mut self) {
        self.metrics.increment_expanded_vertices_count();
    }
}
