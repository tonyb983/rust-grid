use std::{collections::HashMap, time::Instant};

use crate::data::MapGrid;

#[derive(Debug)]
/// The context for the Pipeline.
pub struct Context<'pipeline_exec> {
    /// The original [`MapGrid`](`crate::data::MapGrid`) that was provided to the pipeline.
    pub original_data: &'pipeline_exec MapGrid,
    /// The time that the pipeline was started.
    pub start_time: Instant,
    /// The current step that is being processed.
    pub current_step: usize,
    /// The total number of steps in the pipeline.
    pub total_steps: usize,
}
