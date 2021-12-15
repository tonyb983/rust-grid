use std::borrow::Borrow;

use crate::{pipe::{pipeline::{Pipeline, PipelineOutput, PipelineResult}, error::PipelineError}, data::MapGrid};

/// TODO: Is this still a thing or am I scraping this?
#[allow(dead_code)]

pub struct PipelineRunner<'a>(Pipeline<'a>);

impl<'a> PipelineRunner<'a> {
    pub fn run(original: &MapGrid) -> PipelineResult {
        let mut pipeline = Pipeline::new();
        if pipeline.is_empty() {
            Err(PipelineError::NoSteps)

        } else {
            pipeline.run(original)
        }
    }
}
