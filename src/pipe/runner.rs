use std::borrow::Borrow;

use crate::{
    data::MapGrid,
    pipe::{
        error::Error,
        pipeline::{Output, Pipeline},
        PipelineResult,
    },
};

/// TODO: Is this still a thing or am I scraping this?
#[allow(dead_code)]

pub struct Runner<'a>(Pipeline<'a>);

impl<'a> Runner<'a> {
    pub fn run(original: &MapGrid) -> PipelineResult {
        let mut pipeline = Pipeline::new();
        if pipeline.is_empty() {
            Err(Error::NoSteps)
        } else {
            pipeline.run(original)
        }
    }
}
