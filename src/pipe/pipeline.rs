#![allow(dead_code)]

use std::{
    borrow::Borrow,
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    data::MapGrid,
    pipe::{
        changes::{Changelist, GridChange},
        context::Context,
        error::Error,
        PipelineResult,
    },
};

/// The result of a pipeline step.
#[derive(Debug)]
pub struct StepOutput {
    /// The output of the step.
    pub output: MapGrid,
    /// A list of the individual changes that were made during this step.
    pub changes: Changelist,
}

/// A single step in the pipeline.
#[allow(clippy::module_name_repetitions)]
pub trait Step {
    /// Execute this step on the input [`MapGrid`](`crate::data::MapGrid`).
    ///
    /// ### Errors
    /// - Function can choose to return a [`crate::pipe::PipelineError`](`crate::pipe::error::Error`).
    fn run<'pipeline_exec>(&mut self, ctx: &Context<'pipeline_exec>, input: &MapGrid) -> Result<StepOutput, Error>;
}

/// An entry in the pipeline history.
#[derive(Debug)]
pub struct HistoryEntry {
    pub before: MapGrid,
    pub changes: Changelist,
    pub after: MapGrid,
}

/// The output of a full pipeline execution.
#[derive(Debug)]
pub struct Output {
    /// The original data that was provided to the pipeline upon execution.
    pub original: MapGrid,
    /// The final output of the pipeline after all steps have been executed.
    pub result: MapGrid,
    /// The history of each step in the pipeline execution.
    pub history: HashMap<usize, HistoryEntry>,
    /// The time(s) it took for each step to execute.
    pub step_times: HashMap<usize, Duration>,
    /// The amount of time it took for the pipeline to execute.
    pub time: Duration,
}

/// The data processing pipeline.
pub struct Pipeline<'pipeline> {
    steps: Vec<Box<dyn Step + 'pipeline>>,
}

impl<'pipeline> Default for Pipeline<'pipeline> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'pipeline> Pipeline<'pipeline> {
    /// Create a new pipeline with no steps.
    #[must_use]
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    /// Adds the given step to the pipeline.
    pub fn add_step<S: Step + 'pipeline>(&mut self, step: S) {
        self.steps.push(Box::new(step));
    }

    /// Returns `true` if this pipeline currently has no steps added to it.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Returns the number of steps in this pipeline.
    #[must_use]
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Executes this pipeline against the given [`MapGrid`](`crate::data::MapGrid`).
    ///
    /// ### Errors
    /// - Function returns any [`crate::pipe::PipelineError`]s that occur during step execution.
    pub fn run(&mut self, original_data: &MapGrid) -> Result<Output, Error> {
        let mut current = original_data.clone();
        let mut ctx = Context {
            original_data,
            start_time: Instant::now(),
            current_step: 0,
            total_steps: self.steps.len(),
        };

        let mut history = HashMap::new();
        let mut step_times = HashMap::new();

        for (i, step) in self.steps.iter_mut().enumerate() {
            ctx.current_step = i + 1;

            let now = Instant::now();
            let result = step.run(&ctx, &current)?;

            step_times.insert(ctx.current_step, now.elapsed());
            history.insert(
                ctx.current_step,
                HistoryEntry {
                    before: current.clone(),
                    changes: result.changes,
                    after: result.output.clone(),
                },
            );

            current = result.output;
        }

        let time = Instant::now().duration_since(ctx.start_time);

        let output = Output {
            original: ctx.original_data.clone(),
            result: current,
            history,
            step_times,
            time,
        };

        Ok(output)
    }
}
