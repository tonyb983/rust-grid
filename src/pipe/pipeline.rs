#![allow(dead_code)]

use std::{
    collections::HashMap,
    time::{Duration, Instant}, borrow::Borrow,
};

use crate::{
    data::MapGrid,
    pipe::{
        changes::{Changelist, GridChange},
        context::Context,
        error::PipelineError,
    },
};

/// The result of a pipeline step.
#[derive(Debug)]
pub struct StepResult {
    pub output: MapGrid,
    pub changes: Changelist,
}

/// A single step in the pipeline.
pub trait PipelineStep {
    fn run(&mut self, ctx: &Context, current: &MapGrid) -> Result<StepResult, PipelineError>;
}

#[derive(Debug)]
pub struct PipelineHistoryEntry {
    pub before: MapGrid,
    pub changes: Changelist,
    pub after: MapGrid,
}

#[derive(Debug)]
pub struct PipelineOutput {
    pub original: MapGrid,
    pub result: MapGrid,
    pub history: HashMap<usize, PipelineHistoryEntry>,
    pub time: Duration,
}

pub type PipelineResult = Result<PipelineOutput, PipelineError>;

/// The data processing pipeline.
pub struct Pipeline<'pipeline> {
    steps: Vec<Box<dyn PipelineStep + 'pipeline>>,
}

impl<'pipeline> Pipeline<'pipeline> {
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    pub fn add_step<S: PipelineStep + 'pipeline>(&mut self, step: S) {
        self.steps.push(Box::new(step));
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn run(&mut self, original_data: &MapGrid) -> Result<PipelineOutput, PipelineError> {
        let mut current = original_data.clone();
        let mut ctx = Context {
            original_data: current.clone(),
            start_time: Instant::now(),
            current_step: 1,
            total_steps: self.steps.len(),
        };

        let mut history = HashMap::new();

        for (i, step) in self.steps.iter_mut().enumerate() {
            let result = step.run(&ctx, &current)?;
            history.insert(
                i,
                PipelineHistoryEntry {
                    before: current.clone(),
                    changes: result.changes,
                    after: result.output.clone(),
                },
            );
            current = result.output;
            ctx.current_step += 1;
        }

        let time = Instant::now().duration_since(ctx.start_time);

        let output = PipelineOutput {
            original: ctx.original_data,
            result: current,
            history,
            time,
        };

        Ok(output)
    }
}
