use std::time::Instant;

use crate::data::MapGrid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineError {
    Message(String),
}

pub struct Context {
    // pub original_data: MapGrid,
    pub start_time: Instant,
    pub step_number: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridChange {
    pub row: usize,
    pub col: usize,
    pub new_value: bool,
}

pub struct Changelist(Vec<GridChange>);

impl Changelist {
    pub fn new() -> Self {
        Changelist(Vec::new())
    }

    pub fn add_change<C: Into<GridChange>>(&mut self, input: C) {
        self.0.push(input.into());
    }

    pub fn add_change_from(&mut self, row: usize, col: usize, new_value: bool) {
        self.0.push(GridChange {
            row,
            col,
            new_value,
        });
    }

    pub fn data(&self) -> &Vec<GridChange> {
        &self.0
    }
}

pub struct StepResult {
    pub changes: Vec<GridChange>,
}

pub trait PipelineStep {
    fn run(&mut self, ctx: Context) -> Result<StepResult, PipelineError>;
}
