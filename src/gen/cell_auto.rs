use crate::{
    data::MapGrid,
    logging::{trace, warn},
};

/// The arguments for the first, basic version, of the cellular automata algorithm. This should be created
/// by calling [`Algorithm::first`] or [`Algorithm::default_first`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FirstAlgArgs {
    on_min: usize,
    off_min: usize,
}

/// The argument for the flexible version of the cellular automata algorithm. It contains a predicate
/// that is passed the cell location, and the number of cells that are on in a 3x3 radius, and
/// the state of the current cell. This should be created by calling [`Algorithm::flex`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FlexArgs {
    predicate: fn((usize, usize), usize, bool) -> bool,
}

/// The argument for the second flexible version of the cellular automata algorithm. It contains a predicate
/// that is passed the cell location, and the number of cells that are on in a 5x5 radius, in a 3x3 radius
/// and the state of the current cell. This should be created by calling [`Algorithm::flex2`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Flex2Args {
    predicate: fn((usize, usize), usize, usize, bool) -> bool,
}

/// This enum is used to pass arguments to the [`CellularAutomata`] runner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Algorithm {
    /// The first version arguments. See [`FirstAlgArgs`].
    First(FirstAlgArgs),
    /// The flexible version arguments. See [`FlexArgs`].
    Flex(FlexArgs),
    /// The second version arguments. See [`Flex2Args`].
    Flex2(Flex2Args),
}

impl Algorithm {
    /// The default version of the basic cellular automata algorithm. Uses 4 as the minimum to turn a
    /// cell on if it is already on, and 5 to turn it on if it is not on.
    #[must_use]
    pub fn default_first() -> Self {
        Algorithm::First(FirstAlgArgs {
            on_min: 4,
            off_min: 5,
        })
    }

    /// Use the basic algorithm with the given on and off minimums.
    #[must_use]
    pub fn first(on_min: usize, off_min: usize) -> Self {
        Self::First(FirstAlgArgs { on_min, off_min })
    }

    /// Create a flexible version of the algorithm that uses the provided predicate.
    #[must_use]
    pub fn flex(predicate: fn((usize, usize), usize, bool) -> bool) -> Self {
        Self::Flex(FlexArgs { predicate })
    }

    /// Create a flexible (second) version of the algorithm that uses the provided predicate.
    #[must_use]
    pub fn flex2(predicate: fn((usize, usize), usize, usize, bool) -> bool) -> Self {
        Self::Flex2(Flex2Args { predicate })
    }
}

/// Static struct holding cellular automata algorithms.
pub struct CellularAutomata;

impl CellularAutomata {
    /// Executes the indicated algorithm on the provided map for the given number of passes.
    #[must_use]
    pub fn execute_on(original: &MapGrid, passes: usize, alg_args: Algorithm) -> MapGrid {
        trace!(
            "CellularAutomata::execute_on(Grid,{}, {:?})",
            passes,
            alg_args
        );

        match alg_args {
            Algorithm::First(faa) => {
                Self::first(original, passes, false, faa.on_min, faa.off_min).0
            }
            Algorithm::Flex(f) => Self::flexible(original, passes, false, &f.predicate).0,
            Algorithm::Flex2(f2) => Self::flexible2(original, passes, false, &f2.predicate).0,
        }
    }

    /// Executes the first cellular automata method, returning the final product
    /// as well as a list of intermediate products.
    #[must_use]
    pub fn execute_with_history(
        original: &MapGrid,
        passes: usize,
        alg_args: Algorithm,
    ) -> (MapGrid, Vec<MapGrid>) {
        trace!(
            "CellularAutomata::execute_with_history(Grid,{},{:?})",
            passes,
            alg_args
        );
        match alg_args {
            Algorithm::First(ffa) => Self::first(original, passes, true, ffa.on_min, ffa.off_min),
            Algorithm::Flex(f) => Self::flexible(original, passes, true, f.predicate),
            Algorithm::Flex2(f2) => Self::flexible2(original, passes, true, &f2.predicate),
        }
    }

    /// Creates a new random grid with the given size and runs the first cellular
    /// automata method on it [`passes`] times.
    ///
    /// ### Return value is (<OriginalGrid>, <FinalGrid>).
    #[must_use]
    pub fn create_and_run(
        size: (usize, usize),
        passes: usize,
        alg_args: Algorithm,
    ) -> (MapGrid, MapGrid, Vec<MapGrid>) {
        trace!(
            "CellularAutomata::create_and_run({:?},{}, {:?})",
            size,
            passes,
            alg_args
        );

        let original = MapGrid::random_fill_percent(size, 0.45);

        let (last, history) = match alg_args {
            Algorithm::First(ffa) => Self::first(&original, passes, false, ffa.on_min, ffa.off_min),
            Algorithm::Flex(f) => Self::flexible(&original, passes, false, f.predicate),
            Algorithm::Flex2(f2) => Self::flexible2(&original, passes, false, f2.predicate),
        };

        (original, last, history)
    }

    fn first(
        grid: &MapGrid,
        passes: usize,
        track_changes: bool,
        on_minimum: usize,
        off_minimum: usize,
    ) -> (MapGrid, Vec<MapGrid>) {
        Self::flexible(grid, passes, track_changes, |_, n, s| {
            if s {
                n >= on_minimum
            } else {
                n >= off_minimum
            }
        })
    }

    /// Flexible Cellular Automata algorithm that iterates over each cell in the given grid
    /// [`passes`] times. The supplied [`StateFunc`] will be used to determine the new state
    /// of each cell in the grid. If the [`StateFunc`] returns true, the cell will be set to
    /// `on`, otherwise it will be set to `off`, and invalid cells are ignored entirely.
    /// Changes are "isolated" during each iteration, with changes only being applied after
    /// all neighbor calculations have been made.
    ///
    /// The [`StateFunc`] is passed:
    /// - The (x,y) or (row,col) coordinates of the cell
    /// - The number of active neighbors to the cell
    /// - The current state of the cell
    ///
    /// The returned tuple contains the final grid, as well as the complete history of each
    /// iteration **if [`track_changes`] is true**, otherwise it will be an empty [Vec].
    fn flexible<StateFunc>(
        original: &MapGrid,
        passes: usize,
        track_changes: bool,
        mut predicate: StateFunc,
    ) -> (MapGrid, Vec<MapGrid>)
    where
        StateFunc: FnMut((usize, usize), usize, bool) -> bool,
    {
        trace!("CellularAutomata::first(Grid,{},Pred1,Pred2)", passes);

        if passes < 1 {
            return (MapGrid::create_copy(original), Vec::new());
        }

        let mut grid = MapGrid::create_copy(original);
        let mut history = if track_changes {
            Vec::with_capacity(passes + 1)
        } else {
            Vec::new()
        };

        if track_changes {
            history.push(MapGrid::create_copy(&grid));
        }

        for p in 0..passes {
            trace!("CellularAutomata::flexible pass #{}/{}", p + 1, passes);
            let mut temp = MapGrid::create_copy(&grid);

            for x in 0..grid.cols() {
                for y in 0..grid.rows() {
                    if let Some(cell) = grid.cell((x, y)) {
                        let cell_state: bool = cell.state().into();
                        let neighbors = grid.active_neighbor_count((x, y), true);

                        let new_state = predicate((x, y), neighbors, cell_state);

                        temp.set_cell_state(x, y, new_state);
                    } else {
                        warn!(
                            "CellularAutomata::flexible Invalid cell found at ({}, {})",
                            x, y
                        );
                    }
                }
            }

            grid = temp;
            if track_changes {
                history.push(MapGrid::create_copy(&grid));
            }
        }

        (grid, history)
    }

    /// Flexible Cellular Automata algorithm that iterates over each cell in the given grid
    /// [`passes`] times. The supplied [`StateFunc`] will be used to determine the new state
    /// of each cell in the grid. If the [`StateFunc`] returns true, the cell will be set to
    /// `on`, otherwise it will be set to `off`, and invalid cells are ignored entirely.
    /// Changes are "isolated" during each iteration, with changes only being applied after
    /// all neighbor calculations have been made.
    ///
    /// The [`StateFunc`] is passed:
    /// - The (x,y) or (row,col) coordinates of the cell
    /// - The number of active neighbors to the cell
    /// - The current state of the cell
    ///
    /// The returned tuple contains the final grid, as well as the complete history of each
    /// iteration **if [`track_changes`] is true**, otherwise it will be an empty [Vec].
    fn flexible2<StateFunc>(
        original: &MapGrid,
        passes: usize,
        track_changes: bool,
        mut predicate: StateFunc,
    ) -> (MapGrid, Vec<MapGrid>)
    where
        StateFunc: FnMut((usize, usize), usize, usize, bool) -> bool,
    {
        trace!("CellularAutomata::first(Grid,{},Pred1,Pred2)", passes);

        if passes < 1 {
            return (MapGrid::create_copy(original), Vec::new());
        }

        let mut grid = MapGrid::create_copy(original);
        let mut history = if track_changes {
            Vec::with_capacity(passes + 1)
        } else {
            Vec::new()
        };

        if track_changes {
            history.push(MapGrid::create_copy(&grid));
        }

        for p in 0..passes {
            trace!("CellularAutomata::flexible pass #{}/{}", p + 1, passes);
            let mut temp = MapGrid::create_copy(&grid);

            for x in 0..grid.cols() {
                for y in 0..grid.rows() {
                    if let Some(cell) = grid.cell((x, y)) {
                        let cell_state: bool = cell.state().into();
                        let n = grid.active_neighbor_count((x, y), true);
                        let n2 = grid.active_neighbors_n(x, y, 2);

                        let new_state = predicate((x, y), n, n2, cell_state);

                        temp.set_cell_state(x, y, new_state);
                    } else {
                        warn!(
                            "CellularAutomata::flexible Invalid cell found at ({}, {})",
                            x, y
                        );
                    }
                }
            }

            grid = temp;
            if track_changes {
                history.push(MapGrid::create_copy(&grid));
            }
        }

        (grid, history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::data::MapGrid;
    use crate::util::testing::crate_before_test;

    #[test]
    fn ca_first() {
        crate_before_test();

        let original = MapGrid::parse_string("...\n.#.\n...", '#', '.')
            .expect("Unable to parse standard grid string");
        let result = CellularAutomata::execute_on(&original, 1, Algorithm::first(4, 5));
        assert_eq!(result.to_strings().join("\n"), "...\n...\n...");
    }
}
