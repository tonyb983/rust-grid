use std::time::Instant;

use crate::data::MapGrid;

#[derive(Debug)]
/// The context for the Pipeline.
pub struct Context {
    pub original_data: MapGrid,
    pub start_time: Instant,
    pub current_step: usize,
    pub total_steps: usize,
}