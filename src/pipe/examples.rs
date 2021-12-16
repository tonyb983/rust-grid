use crate::{
    data::MapGrid,
    pipe::{
        changes::{Changelist, GridChange},
        context::Context,
        error::Error,
        pipeline::{Step, StepOutput},
    },
    util::TriState,
};

struct SetOutEdgeStep {
    state: TriState,
}

impl SetOutEdgeStep {
    pub fn new(state: TriState) -> Self {
        SetOutEdgeStep { state }
    }
}

impl Step for SetOutEdgeStep {
    fn run<'parent>(
        &mut self,
        _ctx: &Context<'parent>,
        current: &MapGrid,
    ) -> Result<StepOutput, Error> {
        let mut changes = Changelist::new();
        let mut output = current.clone();
        let (size_x, size_y) = output.size().into();

        for ((x, y), cell) in output.iter_pos_mut() {
            if (x == 0 || x == size_x - 1) && cell.state() != self.state {
                let change = GridChange {
                    row: x,
                    col: y,
                    prev_value: cell.state(),
                    new_value: self.state,
                };
                cell.set_state(self.state);
                changes.add_change(change);
            }

            if (y == 0 || y == size_y - 1) && cell.state() != self.state {
                let change = GridChange {
                    row: y,
                    col: x,
                    prev_value: cell.state(),
                    new_value: self.state,
                };
                cell.set_state(self.state);
                changes.add_change(change);
            }
        }

        Ok(StepOutput { output, changes })
    }
}

struct ReverseEntireGridStep;

impl Step for ReverseEntireGridStep {
    fn run<'parent>(
        &mut self,
        _ctx: &Context<'parent>,
        current: &MapGrid,
    ) -> Result<StepOutput, Error> {
        let mut changes = Changelist::new();
        let mut output = current.clone();

        for ((x, y), cell) in output.iter_pos_mut() {
            if cell.is_invalid() {
                continue;
            }
            let change = GridChange {
                row: x,
                col: y,
                prev_value: cell.state(),
                new_value: !cell.state(),
            };
            cell.toggle();
            changes.add_change(change);
        }

        Ok(StepOutput { output, changes })
    }
}

struct SetEntireRowStep {
    row: usize,
    state: TriState,
}

impl SetEntireRowStep {
    pub fn new(row: usize, state: TriState) -> Self {
        Self { row, state }
    }
}

impl Step for SetEntireRowStep {
    fn run<'parent>(
        &mut self,
        _ctx: &Context<'parent>,
        current: &MapGrid,
    ) -> Result<StepOutput, Error> {
        let mut changes = Changelist::new();
        let mut output = current.clone();

        for ((x, y), cell) in output.iter_pos_mut() {
            if y != self.row {
                continue;
            }
            if cell.state() == self.state {
                continue;
            }

            let change = GridChange {
                row: x,
                col: y,
                prev_value: cell.state(),
                new_value: self.state,
            };
            cell.set_state(self.state);
            changes.add_change(change);
        }

        Ok(StepOutput { output, changes })
    }
}

struct SetEntireColumnStep {
    column: usize,
    state: TriState,
}

impl SetEntireColumnStep {
    pub fn new(column: usize, state: TriState) -> Self {
        Self { column, state }
    }
}

impl Step for SetEntireColumnStep {
    fn run<'parent>(
        &mut self,
        _ctx: &Context<'parent>,
        current: &MapGrid,
    ) -> Result<StepOutput, Error> {
        let mut changes = Changelist::new();
        let mut output = current.clone();

        for ((x, y), cell) in output.iter_pos_mut() {
            if x != self.column {
                continue;
            }
            if cell.state() == self.state {
                continue;
            }

            let change = GridChange {
                row: x,
                col: y,
                prev_value: cell.state(),
                new_value: self.state,
            };
            cell.set_state(self.state);
            changes.add_change(change);
        }

        Ok(StepOutput { output, changes })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{data::MapGrid, pipe::pipeline::Pipeline};

    #[test]
    fn outer_edges_works() {
        crate::util::testing::crate_before_test();

        let grid = MapGrid::empty((3, 3));
        assert_eq!(grid.to_strings().join("\n"), "...\n...\n...");

        let mut pipeline = Pipeline::new();
        pipeline.add_step(SetOutEdgeStep::new(TriState::True));

        let result = pipeline.run(&grid);
        crate::logging::error!("Result: {:#?}", &result);
        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("Pipeline returned error!")
                .result
                .to_strings()
                .join("\n"),
            "###\n#.#\n###"
        );
    }

    #[test]
    fn reverse_grid_works() {
        crate::util::testing::crate_before_test();

        let grid = MapGrid::empty((3, 3));
        assert_eq!(grid.to_strings().join("\n"), "...\n...\n...");

        let mut pipeline = Pipeline::new();
        pipeline.add_step(ReverseEntireGridStep);

        let result = pipeline.run(&grid);
        crate::logging::error!("Result: {:#?}", &result);
        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("Pipeline returned error!")
                .result
                .to_strings()
                .join("\n"),
            "###\n###\n###"
        );
    }

    #[test]
    fn set_entire_row_works() {
        crate::util::testing::crate_before_test();

        let grid = MapGrid::empty((3, 3));
        assert_eq!(grid.to_strings().join("\n"), "...\n...\n...");

        let mut pipeline = Pipeline::new();
        pipeline.add_step(SetEntireRowStep::new(1, TriState::True));

        let result = pipeline.run(&grid);
        crate::logging::error!("Result: {:#?}", &result);
        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("Pipeline returned error!")
                .result
                .to_strings()
                .join("\n"),
            "...\n###\n..."
        );
    }

    #[test]
    fn set_entire_col_works() {
        crate::util::testing::crate_before_test();

        let grid = MapGrid::empty((3, 3));
        assert_eq!(grid.to_strings().join("\n"), "...\n...\n...");

        let mut pipeline = Pipeline::new();
        pipeline.add_step(SetEntireColumnStep::new(1, TriState::True));

        let result = pipeline.run(&grid);
        crate::logging::error!("Result: {:#?}", &result);
        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("Pipeline returned error!")
                .result
                .to_strings()
                .join("\n"),
            ".#.\n.#.\n.#."
        );
    }

    #[test]
    fn multi_set_row_col_works() {
        crate::util::testing::crate_before_test();

        let grid = MapGrid::empty((5, 5));
        assert_eq!(
            grid.to_strings().join("\n"),
            ".....\n.....\n.....\n.....\n....."
        );

        let mut pipeline = Pipeline::new();
        pipeline.add_step(SetEntireColumnStep::new(1, TriState::True));
        pipeline.add_step(SetEntireColumnStep::new(3, TriState::True));
        pipeline.add_step(SetEntireRowStep::new(1, TriState::True));
        pipeline.add_step(SetEntireRowStep::new(3, TriState::True));

        let result = pipeline.run(&grid);
        crate::logging::error!("Result: {:#?}", &result);
        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("Pipeline returned error!")
                .result
                .to_strings()
                .join("\n"),
            ".#.#.\n#####\n.#.#.\n#####\n.#.#."
        );
    }

    #[test]
    fn two_step_pipeline_works() {
        crate::util::testing::crate_before_test();

        let grid = MapGrid::empty((3, 3));
        assert_eq!(grid.to_strings().join("\n"), "...\n...\n...");

        let mut pipeline = Pipeline::new();
        pipeline.add_step(SetOutEdgeStep::new(TriState::True));
        pipeline.add_step(ReverseEntireGridStep);

        let result = pipeline.run(&grid);
        crate::logging::error!("Result: {:#?}", &result);
        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("Pipeline returned error!")
                .result
                .to_strings()
                .join("\n"),
            "...\n.#.\n..."
        );

        let grid = MapGrid::empty((5, 5));
        assert_eq!(
            grid.to_strings().join("\n"),
            ".....\n.....\n.....\n.....\n....."
        );
        let result = pipeline.run(&grid);
        crate::logging::error!("Result: {:#?}", &result);
        assert!(result.is_ok());
        assert_eq!(
            result
                .expect("Pipeline returned error!")
                .result
                .to_strings()
                .join("\n"),
            ".....\n.###.\n.###.\n.###.\n....."
        );
    }
}
