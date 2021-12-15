use std::backtrace::Backtrace;

use thiserror::Error;

/// An error returned from the Pipeline.
#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("No steps were added to the pipeline before execution.")]
    NoSteps,
    #[error("Error occurred during pipeline execution: {0}")]
    Other(String),
    #[error("Unknown error occurred during pipeline processing")]
    Unknown,
}