use std::backtrace::Backtrace;

use thiserror::Error as ThisError;

/// An error returned from the Pipeline.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error returns when a [`Pipeline`](`crate::pipe::Pipeline`) is run without any steps.
    #[error("No steps were added to the pipeline before execution.")]
    NoSteps,

    /// A catch-all error containing a message describing what occurred.
    #[error("Error occurred during pipeline execution: {0}")]
    Other(String),

    /// An unknown error.
    #[error("Unknown error occurred during pipeline processing")]
    Unknown,
}