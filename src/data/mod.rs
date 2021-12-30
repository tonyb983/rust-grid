/// ## `Cell` Module
///
/// Contains the definition and implementation of the [`crate::data::Cell`] type, representing a single cell
/// inside of a [`crate::data::MapGrid`] whose state is determined by a [`crate::util::tri::TriState`].
mod cell;

/// ## `MapGrid` Module
/// This module contains the implementation of [`crate::data::grid::MapGrid`].
///
/// It is a 2D grid of [`crate::data::Cell`]s, with many utility functions
/// for manipulating, modifying, and querying the grid.
///
/// Basic Examples:
/// ```
/// # use dungen::data::MapGrid;
/// # use dungen::util::TriState;
/// # use dungen::data::Cell;
///
/// /// A new MapGrid full of 100 invalid cells.
/// let mut grid = MapGrid::new((10, 10));
/// assert_eq!((grid.rows(), grid.cols()), (10, 10));
/// assert!(grid.cell((5, 5)).unwrap().is_invalid());
/// assert!(grid.cell((10,10)).is_none());
///
/// /// A new MapGrid full of 25 cells.
/// let mut grid = MapGrid::empty((5, 5));
/// assert_eq!(grid.to_strings().join("\n"), ".....\n.....\n.....\n.....\n.....");
///
/// /// Set the edges of the grid to ON
/// assert!(grid.cell((0, 0)).unwrap().is_off());
/// assert!(grid.cell((4, 4)).unwrap().is_off());
/// grid.set_outer_cells(true);
/// assert!(grid.cell((0, 0)).unwrap().is_on());
/// assert!(grid.cell((4, 4)).unwrap().is_on());
/// assert_eq!(grid.to_strings().join("\n"), "#####\n#...#\n#...#\n#...#\n#####");
///
/// /// Iterate over all cells
/// for cell in grid.iter() {
///     assert!(cell.is_valid());
/// }
///
/// /// Iterate over each cell and position mutably
/// for (pos, cell) in grid.iter_pos_mut() {
///     if pos == (2, 2).into() {
///        cell.set_state(TriState::on());
///    }
/// }
///
/// assert_eq!(grid.to_strings().join("\n"), "#####\n#...#\n#.#.#\n#...#\n#####");
/// ```
mod grid;

/// ## `Premade` Module
/// This module contains several premade maps, useful for debugging and testing different implementations and algorithms.
mod premade;

/// ## `Types` Module
/// This module contains the common data types used throughout this library. Most (or all) types here are re-exported
/// by the parent module, [`crate::data`].
mod types;

pub use self::grid::{GridIterator, MapGrid};
pub use cell::TriCell as Cell;
pub use premade::{
    GridFiles as PremadeGridFiles, GridStrings as PremadeGridStrings, Grids as PremadeGrids,
};
pub use types::{pos, size, square, AsPos, GridIndex, GridPos, GridSize, GridSquare};
