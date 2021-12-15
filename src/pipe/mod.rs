#![allow(dead_code, unused_imports)]

/// ## `Pipeline::Changes` Module
mod changes;
/// ## `Pipeline::Context` Module
mod context;
/// ## `Pipeline::Error` Module
mod error;
/// ## `Pipeline::Examples` Module
/// This module contains simple pipeline step implementations for testing and demonstration purposes.
mod examples;
/// ## `Pipeline::Core` Module
mod pipeline;
/// ## `Pipeline::Runner` Module
mod runner;

pub use crate::pipe::{
    context::Context as PipelineContext,
    error::Error as PipelineError,
    pipeline::{
        Output as PipelineOutput,
        Pipeline,
        Step as PipelineStep, 
        StepOutput as PipelineStepOutput
    },
};

/// Result type used by [`Pipeline`](`crate::pipe::pipeline::Pipeline`).
pub type PipelineResult = Result<PipelineOutput, PipelineError>;

/// Result type used by [`Step::run`](`crate::pipe::pipeline::Step::run`).
pub type PipelineStepResult = Result<PipelineStepOutput, PipelineError>;
