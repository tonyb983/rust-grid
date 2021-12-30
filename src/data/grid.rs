use std::{fs::File, io::Read, num::ParseIntError, path::Path};

use pathfinding::grid::Grid as PFGrid;
use serde::{Deserialize, Serialize};

use crate::{
    data::{size, square, Cell, GridPos, GridSize, GridSquare},
    gen::room_based::GridClassification,
    logging::{error, info, trace, warn},
    util::TriState,
};

/// An iterator over all of the cells in a grid, in row-major order.
#[allow(clippy::module_name_repetitions)]
pub struct GridIterator<'lifetime> {
    grid: &'lifetime MapGrid,
    curr: (usize, usize),
}

impl<'a> GridIterator<'a> {
    /// Creates a new [`GridIterator`] over the given [`MapGrid`].
    #[must_use]
    pub fn new(grid: &'a MapGrid) -> Self {
        Self { grid, curr: (0, 0) }
    }
}

impl Iterator for GridIterator<'_> {
    type Item = ((usize, usize), bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.0 += 1;
        if self.curr.0 >= self.grid.width {
            self.curr.0 = 0;
            self.curr.1 += 1;

            if self.curr.1 >= self.grid.height {
                return None;
            }
        }

        if let Some(cell) = self.grid.cell((self.curr.0, self.curr.1)) {
            Some(((self.curr.0, self.curr.1), cell.is_on()))
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a MapGrid {
    type Item = ((usize, usize), bool);
    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator::new(self)
    }
}

const INVALID_MARKERS: [char; 3] = ['X', '@', '!'];

/// The result of a [`MapGrid`] file parsing operation.
pub type MapFileParseResult = Result<(MapGrid, GridPos, GridPos), Vec<String>>;

/// An error that occurs during a [`MapGrid`] parsing operation.
#[derive(Debug, Clone)]
pub struct MapParseError(String);

/// A map or grid of cells.
#[derive(Clone, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct MapGrid {
    name: Option<String>,
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl MapGrid {
    /// Creates a new grid with the given width and height, setting all cells to `Invalid`.
    ///
    /// *For a new empty grid, use [`MapGrid::empty()`] instead.*
    ///
    /// ### Panics
    /// Function panics if the size provided is less than 3x3.
    #[must_use]
    pub fn new<Size: Into<GridSize> + std::fmt::Debug>(size: Size) -> Self {
        trace!("MapGrid::new({:?})", size);
        let (width, height) = size.into().into();
        if width < 3 || height < 3 {
            error!("Grid must be at least 3x3");
        }

        assert!(width >= 3, "Width must be at least 3");
        assert!(height >= 3, "Height must be at least 3");

        let mut cells = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell::invalid());
            }

            cells.push(row);
        }

        Self {
            width,
            height,
            cells,
            name: None,
        }
    }

    /// Creates a new [`MapGrid`] with the given `size`, with `name` set for it's name.
    #[must_use]
    pub fn new_named<Size: Into<GridSize> + std::fmt::Debug, Text: AsRef<str> + std::fmt::Debug>(
        name: Text,
        size: Size,
    ) -> Self {
        trace!("MapGrid::new_named({:?}, {:?})", name, size);

        let mut grid = Self::new(size);
        grid.name = Some(name.as_ref().to_string());
        grid
    }

    /// Creates a new grid with the given width and height, setting all cells to `False` or `off`.
    ///
    /// ### Panics
    /// Function panics if the size provided is less than 3x3.
    #[must_use]
    pub fn empty<Size: Into<GridSize> + std::fmt::Debug>(size: Size) -> Self {
        trace!("MapGrid::empty({:?})", size);
        let (width, height) = size.into().into();
        if width < 3 || height < 3 {
            error!("Grid must be at least 3x3");
        }

        assert!(width >= 3, "Width must be at least 3");
        assert!(height >= 3, "Height must be at least 3");

        let mut cells = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell::off());
            }

            cells.push(row);
        }

        Self {
            width,
            height,
            cells,
            name: None,
        }
    }

    /// Creates a new *named* grid with the given width and height, setting all cells to `False` or `off`.
    #[must_use]
    pub fn empty_named<
        Size: Into<GridSize> + std::fmt::Debug,
        Text: AsRef<str> + std::fmt::Debug,
    >(
        name: Text,
        size: Size,
    ) -> Self {
        trace!("MapGrid::empty_named({:?}, {:?})", name, size);
        let mut grid = Self::empty(size);
        grid.name = Some(name.as_ref().to_string());
        grid
    }

    /// Creates a new grid with the given width and height, with each cell randomly set.
    ///
    /// ### Panics
    /// Function panics if the size provided is less than 3x3.
    #[must_use]
    pub fn random<Size: Into<GridSize> + std::fmt::Debug>(size: Size) -> Self {
        trace!("MapGrid::random({:?})", size);
        let (width, height) = size.into().into();
        if width < 3 || height < 3 {
            error!("Grid must be at least 3x3");
        }

        assert!(width >= 3, "Width must be at least 3");
        assert!(height >= 3, "Height must be at least 3");

        let mut cells = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell::random());
            }

            cells.push(row);
        }

        Self {
            width,
            height,
            cells,
            name: None,
        }
    }

    /// Creates a new *named* grid with the given width and height, with each cell randomly set.
    #[must_use]
    pub fn random_named<
        Text: AsRef<str> + std::fmt::Debug,
        Size: Into<GridSize> + std::fmt::Debug,
    >(
        name: Text,
        size: Size,
    ) -> Self {
        trace!("MapGrid::random_named({:?}, {:?})", name, size);
        let mut grid = Self::random(size);
        grid.name = Some(name.as_ref().to_string());

        grid
    }

    /// Creates a grid with [`fill_percent`]% of the cells set to `True` or `on`.
    ///
    /// ### Panics
    /// Function panics if the size given is less than 3x3.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    #[must_use]
    pub fn random_fill_percent<Size: Into<GridSize> + std::fmt::Debug>(
        size: Size,
        fill_percent: f64,
    ) -> Self {
        trace!("MapGrid::random_fill({:?})", size);
        let (width, height) = size.into().into();
        if width < 3 || height < 3 {
            error!("Grid must be at least 3x3");
        }

        let target = (((width * height) as f64) * fill_percent.clamp(0.0, 1.0)).floor() as usize;
        let mut grid = Self::empty((width, height));
        assert!(
            target <= grid.width * grid.height,
            "Target must be less than or equal to the total number of cells"
        );

        while grid.on_cells_count() < target {
            let (x, y) = grid.random_cell_pos().into();
            grid.set_cell(x, y, Cell::on());
        }

        grid
    }

    /// Creates a grid with [`fill_number`] cells set to `True` or `on`.
    #[must_use]
    pub fn random_fill_number<Size: Into<GridSize> + std::fmt::Debug>(
        size: Size,
        fill_number: usize,
    ) -> Self {
        trace!("MapGrid::random_fill_number({:?}, {})", size, fill_number);
        let (width, height) = size.into().into();

        if width < 3 || height < 3 {
            error!("Grid must be at least 3x3");
        }

        if fill_number > width * height {
            error!("Fill number must be less than or equal to the total number of cells");
        }

        let mut grid = Self::empty((width, height));
        while grid.on_cells_count() < fill_number {
            let (x, y) = grid.random_cell_pos().into();
            grid.set_cell(x, y, Cell::on());
        }

        grid
    }

    /// Creates a copy of the given grid. If the given grid has a name,
    /// the returned copy will be named "<Name> (Copy)"
    #[must_use]
    pub fn create_copy(other: &Self) -> Self {
        trace!("MapGrid::create_copy()");

        let mut grid = Self::empty(other.size());
        if let Some(n) = other.name_ref() {
            grid.set_name(format!("{} (Copy)", n));
        }

        for (pos, &cell) in other.iter_pos() {
            grid.set_cell(pos.0, pos.1, cell);
        }

        grid
    }

    /// Creates a new [`MapGrid`](`crate::data::MapGrid`) representing a [`section`](`crate::data::GridSection`) of the given [`original`](`crate::data::grid::MapGrid`) [`MapGrid`].
    ///
    /// ### Panics
    ///
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub fn sub_grid(original: &Self, section: &GridSquare) -> Self {
        // trace!("MapGrid::sub_grid({:?})", section);
        // Create X array

        let sub_size = section.size();
        // let x_end = section.x_range().end as isize;
        // let y_end = section.y_range().end as isize;
        // let mut x_range = section.cast().x_range().chain(std::iter::once(x_end));
        // let mut y_range = section.cast().y_range().chain(std::iter::once(y_end));
        let (x_range, y_range) = (section.cast().x_range(), section.cast().y_range());

        let mut grid = Self::new(sub_size);
        if let Some(name) = original.name_ref() {
            grid.set_name(format!("SubGrid of {}", name));
        }

        for (thisx, otherx) in x_range.enumerate() {
            for (thisy, othery) in y_range.clone().enumerate() {
                if let Some(cell) = original.cell_wrapped(otherx, othery) {
                    grid.set_cell(thisx, thisy, *cell);
                } else {
                    warn!(
                        "Error getting cell ({}, {}). thisx = {} thisy = {}",
                        otherx, othery, thisx, thisy
                    );
                }
            }
        }

        grid
    }

    /// Creates a new [`MapGrid`] using the given `row` [`GridClassification`] and `col` [`GridClassification`]
    /// to determine the size, and `default_state` to set the initial state of each cell.
    #[must_use]
    pub fn create_sized(
        row: GridClassification,
        col: GridClassification,
        default_state: Cell,
    ) -> Self {
        trace!("MapGrid::create_sized({:?}, {:?})", row, col);
        let mut grid = Self::empty((
            fastrand::usize(col.col_range()),
            fastrand::usize(row.row_range()),
        ));

        if default_state.is_invalid() {
            grid.set_all_invalid();
        } else {
            grid.set_all_cells(default_state.is_on());
        }

        grid
    }

    /// Creates a new [`MapGrid`] from `other` with all cells reversed.
    #[must_use]
    pub fn reverse(other: &Self) -> Self {
        let mut grid = Self::create_copy(other);
        for cell in grid.iter_mut() {
            cell.toggle();
        }

        grid
    }

    /// Combines multiple [`MapGrid`]s into a single [`MapGrid`].
    #[must_use]
    pub fn combine_multiple(grids: &[(&Self, GridPos)]) -> Self {
        let max_width: usize = {
            let mut max = 0;
            for (grid, pos) in grids {
                let x = grid.width + pos.x;
                if x > max {
                    max = x;
                }
            }

            max
        };
        let max_height: usize = {
            let mut max = 0;
            for (grid, pos) in grids {
                let y = grid.height + pos.y;
                if y > max {
                    max = y;
                }
            }

            max
        };
        let mut grid = Self::empty((max_width, max_height));
        for (g, pos) in grids {
            for ((x, y), other) in g.iter_pos() {
                grid.set_cell(x + pos.x, y + pos.y, *other);
            }
        }

        grid
    }

    /// Convenience function which calls:
    /// ```ignore
    /// # use dungen::data::MapGrid;
    /// # use dungen::data::Cell;
    /// # let mut grid = MapGrid::new((5, 5));
    /// # assert!(grid.cell_count() == 25);
    /// # let size = (10,10);
    /// # let cell_value = Cell::on();
    /// grid.resize_rows_with(size.0, cell_value);
    /// grid.resize_cols_with(size.1, cell_value);
    /// # assert!(grid.cell_count() == 100);
    /// ```
    ///
    /// ### Panics
    /// - Function panics if the resulting size of the grid is less than 3x3, which should not happen
    /// because the function first checks if the result is going to be less than 3.
    /// - Function panics if the actual resulting size of the grid does not match the expected end size
    /// (which means something probably went horribly wrong or was horribly coded)
    pub fn resize_with<P: Into<(usize, usize)>>(&mut self, size: P, cell_value: Cell) {
        let (width, height) = size.into();
        if self.width != width {
            self.resize_rows_with(height, cell_value);
        }
        if self.height != height {
            self.resize_cols_with(width, cell_value);
        }
    }

    /// Combines the data from the `first` [`MapGrid`] with the data from the
    /// `other` [`MapGrid`], prioritizing the data in `other` for any conflicts.
    #[must_use]
    pub fn union(first: &Self, other: &Self) -> Self {
        Self::integrate(first, other, (0, 0).into())
    }

    /// Creates a new [`MapGrid`] using the existing data from this instance,
    /// adding the data from the other instance.
    ///
    /// #### Does *not* modify the original existing instance.
    #[must_use]
    pub fn integrate(first: &Self, other: &Self, offset: GridPos) -> Self {
        let (other_width, other_height) = other.size().into();
        let (self_width, self_height) = first.size().into();
        let (start_x, start_y) = offset.into();

        let new_width = std::cmp::max(self_width, start_x + other_width);
        let new_height = std::cmp::max(self_height, start_y + other_height);
        let mut result = MapGrid::new((new_width, new_height));
        for ((x, y), &cell) in first.iter_pos() {
            result.set_cell(x, y, cell);
        }
        for ((x, y), &cell) in other.iter_pos() {
            result.set_cell(x + start_x, y + start_y, cell);
        }

        result
    }

    /// ## [`MapGrid::parse_string`]
    /// Attempts to parse a string into a grid, using the given on and off characters to determine
    /// the state of each cell. Will return a new [`MapGrid`] if the string is successfully parsed,
    /// or a [`String`] containing the error message if it fails.
    ///
    /// ### Errors
    /// Function will return an error if the string does not form a valid grid.
    #[allow(clippy::too_many_lines)]
    pub fn parse_string<S: AsRef<str> + std::fmt::Debug>(
        input: S,
        on: char,
        off: char,
    ) -> Result<Self, Vec<String>> {
        trace!("MapGrid::parse_string({:?}, {}, {})", input, on, off);

        if on == 'S' || on == 'E' {
            warn!("MapGrid::parse_string - ON character should not be S or E, these are used to designate start and end position in maze files.");
        }

        if off == 'S' || off == 'E' {
            warn!("MapGrid::parse_string - ON character should not be S or E, these are used to designate start and end position in maze files.");
        }

        let mut errors = Vec::new();
        let mut fatal_error = false;

        if input.as_ref().is_empty() {
            errors.push(String::from("Empty input"));
            return Err(errors);
        }

        let mut split: Vec<String> = input
            .as_ref()
            .split('\n')
            // .into_iter()
            .map(std::string::ToString::to_string)
            .collect();
        trace!("MapGrid::parse_string - split: {:?}", split);

        let mut name = None;

        if split[0].starts_with(|c: char| c != on && c != off && c.is_alphabetic()) {
            info!("MapGrid::parse_string - Found unexpected character at start of line, assuming grid name: {:?}", split[0]);
            name = Some(split.remove(0));
        }

        let (mut width, mut height) = (0usize, 0usize);

        if split[0].starts_with(|c: char| c != on && c != off && c.is_numeric()) {
            info!("MapGrid::parse_string - Found unexpected character at start of line, assuming grid dimensions: {:?}", split[0]);
            let line = split.remove(0);
            let halves = line
                .split_ascii_whitespace()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>();
            if halves.len() == 2 {
                if let Ok(w_u) = halves[0].parse::<usize>() {
                    width = w_u;
                }

                if let Ok(h_u) = halves[1].parse::<usize>() {
                    height = h_u;
                }
            } else {
                let msg = format!("Invalid grid size line: {}", line);
                warn!("{}", msg);
            }
        }

        if width == 0 {
            width = {
                let mut max = 0;
                for s in &split {
                    if s.len() > max {
                        max = s.len();
                    }
                }

                max
            };
        }

        if height == 0 {
            height = split.len();
        }
        trace!(
            "MapGrid::parse_string - width: {}, height: {}",
            width,
            height
        );
        if width < 3 {
            fatal_error = true;
            let msg = "MapGrid::parse_string - Width must be at least 3".to_string();
            error!("{}", &msg);
            errors.push(msg);
        }

        if height < 3 {
            fatal_error = true;
            let msg = "MapGrid::parse_string - Height must be at least 3".to_string();
            error!("{}", &msg);
            errors.push(msg);
        }

        if fatal_error {
            trace!("Fatal errors found, returning error(s): {:?}", errors);
            return Err(errors);
        }

        let mut grid = Self::new(size(width, height));
        grid.name = name;

        for (y, line) in split.iter().enumerate() {
            // let row_size = line.len();
            for (x, ch) in line.chars().enumerate() {
                if ch == on {
                    grid.set_cell_state(x, y, true);
                } else if ch == off {
                    grid.set_cell_state(x, y, false);
                } else if ch == 'S' || ch == 'G' {
                    info!(
                        "MapGrid::parse_string - Found {} point at ({}, {})",
                        if ch == 'S' { "Start" } else { "Goal" },
                        x,
                        y
                    );
                    grid.set_cell_state(x, y, false);
                } else {
                    errors.push(format!("Invalid character {} at ({},{})", ch, x, y));
                    grid.set_cell_invalid(x, y);
                }
            }
        }

        if errors.is_empty() {
            trace!(
                "No errors found while parsing, returning MapGrid:\n{}",
                grid
            );
            Ok(grid)
        } else {
            trace!(
                "Errors found while parsing, returning error(s): {:?}",
                errors
            );
            Err(errors)
        }
    }

    /// ## [`MapGrid::parse_file`](`crate::data::MapGrid::parse_file`)
    /// Parse a plain text file into a [`MapGrid`].
    ///
    /// The file format is:
    ///
    /// ```ignore
    /// <MapName>[\n]
    /// <MapWidth>[space]<MapHeight>[\n]
    /// <MapData>
    /// ```
    ///
    /// ### Errors
    /// Function will return an error if the file does not exist, cannot be opened, or does
    /// not represent a valid / parsable grid.
    ///
    /// ### Panics
    /// Function panics if the return value from [`std::fs::Metadata::len`] cannot be converted
    /// into a [`usize`] (which seems very unlikely).
    pub fn parse_map_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> MapFileParseResult {
        trace!("MapGrid::parse_map_file({:?})", path);
        let mut file = File::open(path).map_err(|e| vec![e.to_string()])?;
        let mut contents = if let Ok(meta) = file.metadata() {
            String::with_capacity(meta.len().try_into().unwrap())
        } else {
            String::new()
        };
        file.read_to_string(&mut contents)
            .map_err(|e| vec![e.to_string()])?;
        let split = contents
            .splitn(3, '\n')
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        if split.len() != 3 {
            let msg = "Invalid map file - Format is <Name>\\n<Width> <Height>\\n<Map>".to_string();
            error!("{}", &msg);
            return Err(vec![msg]);
        }

        let name = split[0].trim().to_string();
        let dims: Vec<Result<usize, ParseIntError>> =
            split[1].split_whitespace().map(str::parse).collect();
        if dims.len() != 2 {
            let msg = "Invalid map file - Format is <Name>\\n<Width> <Height>\\n<Map>".to_string();
            error!("{}", &msg);
            return Err(vec![msg]);
        }

        let width = *dims[0]
            .as_ref()
            .map_err(|e| vec![format!("Error parsing width - {:?}", e.to_string())])?;
        let height = *dims[1]
            .as_ref()
            .map_err(|e| vec![format!("Error parsing height - {:?}", e.to_string())])?;

        let mut start = (usize::MAX, usize::MAX);
        let mut goal = (usize::MAX, usize::MAX);
        let mut map = Self::empty((width, height));

        let map_lines = split[2]
            .split('\n')
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        let line_count = map_lines.len();
        for (y, line) in map_lines.iter().enumerate() {
            let row_size = line.len();
            for (x, ch) in line.chars().enumerate() {
                trace!(
                    "MapGrid::parse_map_file - x = {}/{}, y = {}/{}, ch = {}",
                    x,
                    row_size,
                    y,
                    line_count,
                    ch
                );
                if ch == '#' {
                    map.set_cell_state(x, y, true);
                } else if ch == '.' {
                    map.set_cell_state(x, y, false);
                } else if ch == 'S' {
                    map.set_cell_state(x, y, false);
                    start = (x, y);
                } else if ch == 'G' {
                    map.set_cell_state(x, y, false);
                    goal = (x, y);
                }
            }
        }

        map.name = Some(name);

        if map.size() != (width, height).into() {
            let msg = format!(
                "Invalid map file - Actual size ({},{}) does not match expected dimensions ({},{})",
                map.cols(),
                map.rows(),
                width,
                height
            );
            error!("{}", &msg);
            return Err(vec![msg]);
        }

        Ok((map, start.into(), goal.into()))
    }
}

impl MapGrid {
    /// Gets a reference to the name of the grid.
    #[must_use]
    pub fn name_ref(&self) -> &Option<String> {
        trace!("MapGrid::name_ref()");
        &self.name
    }

    /// Gets a mutable reference to the name of the grid.
    #[must_use]
    pub fn name_ref_mut(&mut self) -> &mut Option<String> {
        trace!("MapGrid::name_ref_mut()");
        &mut self.name
    }

    /// Gets a copy of the name of the grid if it has one, or `None` if it doesn't.
    #[must_use]
    pub fn name_copy(&self) -> Option<String> {
        trace!("MapGrid::name_copy()");
        self.name.clone()
    }

    /// Sets the name of the grid.
    pub fn set_name<S: AsRef<str> + std::fmt::Debug>(&mut self, name: S) {
        trace!("MapGrid::set_name({:?})", name);
        self.name = Some(name.as_ref().to_string());
    }

    /// Sets the name of the grid to None.
    pub fn clear_name(&mut self) {
        trace!("MapGrid::clear_name()");
        self.name = None;
    }

    /// Returns true if the grid has been given a name.
    #[must_use]
    pub fn has_name(&self) -> bool {
        trace!("MapGrid::has_name()");
        self.name.is_some()
    }

    /// Returns a newly constructed [`Vec`] containing the [`crate::data::GridPos`] and cell
    /// of each cell in this [`MapGrid`].
    #[must_use]
    pub fn dump_all_cells(&self) -> Vec<(GridPos, Cell)> {
        let mut result = Vec::with_capacity(self.width * self.height);

        for (p, &c) in self.iter_pos() {
            result.push((p.into(), c));
        }

        result
    }

    /// Returns the height or number of rows in the grid.
    #[must_use]
    pub fn rows(&self) -> usize {
        trace!("MapGrid::rows()");
        self.height
    }

    /// Returns the width or number of columns in the grid.
    #[must_use]
    pub fn cols(&self) -> usize {
        trace!("MapGrid::cols()");
        self.width
    }

    /// Gets the size of this [`MapGrid`]() as a [`GridSize`].
    #[must_use]
    pub fn size(&self) -> GridSize {
        trace!("MapGrid::size()");

        (self.width, self.height).into()
    }

    /// Gets the position (x,y) of a random cell in the grid.
    #[must_use]
    pub fn random_cell_pos(&self) -> GridPos {
        trace!("MapGrid::random_cell_pos()");

        (
            fastrand::usize(0..self.width),
            fastrand::usize(0..self.height),
        )
            .into()
    }

    /// Gets a reference to a random cell in the grid.
    ///
    /// ### Panics
    /// Function panics if the cell returned from [`random_cell`](`crate::data::MapGrid`)
    /// cannot be unwrapped (which should ostensibly never happen).
    #[must_use]
    pub fn random_cell(&self) -> &Cell {
        trace!("MapGrid::random_cell()");
        let (row, col) = self.random_cell_pos().into();

        self.cell((col, row)).unwrap_or_else(|| &self.cells[0][0])
    }

    /// Gets a mutable reference to a random cell in the grid.
    ///
    /// ### Panics
    /// Function panics if the cell returned from [`random_cell`](`crate::data::MapGrid`)
    /// cannot be unwrapped (which should ostensibly never happen).
    #[must_use]
    pub fn random_cell_mut(&mut self) -> &mut Cell {
        trace!("MapGrid::random_cell()");
        let (row, col) = self.random_cell_pos().into();

        self.cell_mut(col, row)
            .expect("random_cell_mut cell returned from cell_mut is none!")
    }

    /// Gets the number of cells in the grid by simply multiplying the width and height.
    #[must_use]
    pub fn cell_count(&self) -> usize {
        trace!("MapGrid::cell_count()");
        self.width * self.height
    }

    /// Gets the number of cells in the grid whose state is on.
    #[must_use]
    pub fn on_cells_count(&self) -> usize {
        trace!("MapGrid::on_cells_count()");
        self.iter().filter(|&&c| c.is_on()).count()
    }

    /// Gets the number of cells in the grid whose state is off.
    #[must_use]
    pub fn off_cells_count(&self) -> usize {
        trace!("MapGrid::off_cells_count()");
        self.iter().filter(|&&c| c.is_off()).count()
    }

    /// Gets the number of cells in the grid whose state is valid.
    #[must_use]
    pub fn valid_cells_count(&self) -> usize {
        trace!("MapGrid::valid_cells_count()");
        self.iter().filter(|&&c| c.is_valid()).count()
    }

    /// Returns the number of cells in the grid whose state is invalid.
    #[must_use]
    pub fn invalid_cells_count(&self) -> usize {
        trace!("MapGrid::invalid_cells_count()");
        self.iter().filter(|&&c| c.is_invalid()).count()
    }

    /// Calculates the percentage of (**on**, **off**, **invalid**) cells in the grid.
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn cell_state_ratio(&self) -> (f64, f64, f64) {
        trace!("MapGrid::cell_state_ratio()");
        let total = self.cell_count();
        if total == 0 {
            error!("MapGrid::cell_state_ratio() total == 0, something is very wrong.");
            return (0.0, 0.0, 0.0);
        }

        let mut on = 0usize;
        let mut off = 0usize;

        for cell in self.iter() {
            if cell.is_on() {
                on += 1;
            } else if cell.is_off() {
                off += 1;
            }
        }

        let on_ratio = (on as f64) / (total as f64);
        let off_ratio = (off as f64) / (total as f64);
        let invalid_ratio = 1.0 - on_ratio - off_ratio;
        // let total = on_ratio + off_ratio + invalid_ratio;

        (on_ratio, off_ratio, invalid_ratio)
    }

    /// Gets a reference to the cell at the given x and y.
    pub fn cell<Pos: Into<GridPos> + std::fmt::Debug>(&self, xy: Pos) -> Option<&Cell> {
        trace!("MapGrid::cell({:?})", xy);
        let (x, y) = xy.into().into();
        if x >= self.width || y >= self.height {
            error!(
                "Out of bounds access at ({},{}) on grid of size ({},{})",
                x, y, self.width, self.height
            );
            return None;
        }

        Some(&self.cells[y][x])
    }

    /// Gets a reference to the cell at the given x and y, wrapping them if they are out of bounds.
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    #[must_use]
    pub fn cell_wrapped(&self, x: isize, y: isize) -> Option<&Cell> {
        trace!("MapGrid::cell_wrap({}, {})", x, y);
        let xx = if x < 0 {
            let mut r = x;
            while r < 0 {
                r += self.width as isize;
            }
            r as usize
        } else if (x as usize) >= self.width {
            x as usize % self.width
        } else {
            x as usize
        };

        let yy = if y < 0 {
            let mut r = y;
            while r < 0 {
                r += self.height as isize;
            }
            r as usize
        } else if (y as usize) >= self.height {
            y as usize % self.height
        } else {
            y as usize
        };

        self.cell((xx, yy))
    }

    /// Gets a mutable reference to the cell at the given x and y.
    pub fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        trace!("MapGrid::cell_mut({}, {})", x, y);
        if x >= self.width || y >= self.height {
            error!(
                "Out of bounds access at ({},{}) on grid of size ({},{})",
                x, y, self.width, self.height
            );
            return None;
        }

        Some(&mut self.cells[y][x])
    }

    /// Sets the cell at the given x and y to the given value.
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        trace!("MapGrid::set_cell({}, {}, {:?})", x, y, cell);
        if x >= self.width || y >= self.height {
            error!(
                "Out of bounds access at ({},{}) on grid of size ({},{})",
                x, y, self.width, self.height
            );
            return;
        }

        self.cells[y][x] = cell;
    }

    /// Sets the state of the cell at the given x and y to the given value.
    pub fn set_cell_state(&mut self, x: usize, y: usize, state: bool) {
        trace!("MapGrid::set_cell_state({}, {}, {})", x, y, state);
        self.set_cell(x, y, Cell::new(state.into()));
    }

    /// Sets the state of the cell at the given x and y as invalid.
    pub fn set_cell_invalid(&mut self, x: usize, y: usize) {
        trace!("MapGrid::set_cell_invalid({}, {})", x, y);
        self.set_cell(x, y, Cell::invalid());
    }

    /// Sets all cells in the [`MapGrid`] to the given `state`.
    pub fn set_all_cells(&mut self, state: bool) {
        trace!("MapGrid::set_all_cells({})", state);
        for cell in self.iter_mut() {
            cell.set_state(state.into());
        }
    }

    /// Sets all cells in the [`MapGrid`] to the `invalid` state.
    pub fn set_all_invalid(&mut self) {
        for cell in self.iter_mut() {
            cell.set_state(TriState::Invalid);
        }
    }

    /// Set all cells in the first and last rows and columns to the given state.
    pub fn set_outer_cells(&mut self, state: bool) {
        trace!("MapGrid::set_outer_cells({})", state);

        let ends = self.size();
        for ((x, y), cell) in self.iter_pos_mut() {
            if x == 0 || x == ends.width - 1 || y == 0 || y == ends.height - 1 {
                cell.set_state(state.into());
            }
        }
    }

    /// Toggles the cell at the given x and y, turning True to False, False to True, and Invalid to Invalid.
    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        trace!("MapGrid::toggle_cell({}, {})", x, y);
        if let Some(c) = self.cell_mut(x, y) {
            c.toggle();
        }
    }

    /// Gets the coordinates of the neighbors to the given cell, truncating edges.
    #[must_use]
    pub fn neighbor_positions<P: Into<(usize, usize)>>(
        &self,
        target_pos: P,
    ) -> Vec<(usize, usize)> {
        let pos = target_pos.into();
        trace!("MapGrid::neighbor_positions(pos = {:?})", pos);
        let xs: Vec<usize> = if pos.0 == 0 {
            vec![0, 1]
        } else if pos.0 == self.width - 1 {
            vec![self.width - 2, self.width - 1]
        } else {
            vec![pos.0 - 1, pos.0, pos.0 + 1]
        };

        let ys: Vec<usize> = if pos.1 == 0 {
            vec![0, 1]
        } else if pos.1 == self.height - 1 {
            vec![self.height - 2, self.height - 1]
        } else {
            vec![pos.1 - 1, pos.1, pos.1 + 1]
        };

        let mut positions = Vec::new();
        for x in xs {
            for y in &ys {
                if (x, *y) == pos {
                    continue;
                }

                positions.push((x, *y));
            }
        }

        positions
    }

    /// Gets the coordinates of the neighbors to the given cell, wrapping on edges.
    #[must_use]
    pub fn neighbor_positions_wrapping<P: Into<(usize, usize)>>(
        &self,
        target_pos: P,
    ) -> Vec<(usize, usize)> {
        let pos = target_pos.into();
        trace!("MapGrid::get_neighbor_positions({:?})", pos);
        info!(
            "Getting neighbors to ({:?}) in grid of size ({},{})",
            pos, self.width, self.height
        );
        let (x, y) = pos;
        let mut positions = Vec::new();
        let xs: [usize; 3] = if x == 0 {
            [self.width - 1, 0, 1]
        } else {
            [(x - 1) % self.width, x % self.width, (x + 1) % self.width]
        };
        let ys: [usize; 3] = if y == 0 {
            [self.height - 1, 0, 1]
        } else {
            [
                (y - 1) % self.height,
                y % self.height,
                (y + 1) % self.height,
            ]
        };

        info!("xs = {:?}", xs);
        info!("ys = {:?}", ys);
        for yy in ys {
            for xx in xs {
                if xx == x && yy == y {
                    continue;
                }
                positions.push((xx, yy));
            }
        }
        info!("positions = {:?}", positions);

        positions
    }

    /// Gets all neighbors of the given position whose state matches `state`. If `wrap_edges` is true,
    /// neighbors will be considered by wrapping first and last rows and columns.
    #[must_use]
    pub fn neighbors_with_state<P: Into<(usize, usize)>>(
        &self,
        target_pos: P,
        state: bool,
        wrap_edges: bool,
    ) -> Vec<(usize, usize)> {
        let pos = target_pos.into();
        trace!("MapGrid::neighbors_with_state({:?}, {})", pos, state);
        let mut neighbors = Vec::new();
        let range = if wrap_edges {
            self.neighbor_positions_wrapping(pos)
        } else {
            self.neighbor_positions(pos)
        };

        for (x, y) in range {
            if matches!(self.cell((x, y)), Some(cell) if cell.state() == state.into()) {
                neighbors.push((x, y));
            }
        }
        neighbors
    }

    /// Gets the number of neighboring cells whose state is True. This does not include the cell at the given x and y.
    #[must_use]
    pub fn active_neighbor_count(&self, pos: (usize, usize), wrapped: bool) -> usize {
        trace!(
            "MapGrid::active_neighbor_count(pos = {:?}, wrapped = {})",
            pos,
            wrapped
        );

        if wrapped {
            self.neighbors_with_state(pos, true, true).len()
        } else {
            self.neighbors_with_state(pos, true, false).len()
        }
    }

    /// Gets the number of neighboring cells in the range (pos.x - x)..=(pos.x + x) x (pos.y - y)..=(pos.y + y)
    /// whose state is `on` or `active`.
    #[must_use]
    pub fn active_neighbors_n(&self, x: usize, y: usize, n: usize) -> usize {
        trace!("MapGrid::active_neighbors_n({}, {}, {})", x, y, n);
        if n == 0 {
            0
        } else if n == 1 {
            self.active_neighbor_count((x, y), true)
        } else if (n * 2) + 1 > self.width || (n * 2) + 1 > self.height {
            warn!("Invalid neighbor extents");
            0
        } else {
            // #...# if called with (2, 2, 2)
            // ..... center is (2,2)
            // ..#.. top left is (0 (2 - 2),0 (2 - 2))
            // ..... bot right is (4 (2 + 2),4 (2 + 2))
            // #...# size is (5,5) (which means I might need to make the square((0 (2-2),0 (2-2)), 5 (2+2+1), 5 (2+2+1))?
            self.create_subgrid(&square(
                &(x.saturating_sub(n), y.saturating_sub(n)),
                x + n + 1,
                y + n + 1,
            ))
            .on_cells_count()
                - if self
                    .cell((x, y))
                    .expect("Unable to get cell for active_neighbors_n (else condition)")
                    .is_on()
                {
                    1
                } else {
                    0
                }
        }
    }

    /// Reverses this entire [`MapGrid`] by calling [`crate::data::TriCell::toggle()`] on each cell in the grid.
    pub fn reverse_in_place(&mut self) {
        trace!("MapGrid::reverse_in_place()");
        for cell in self.iter_mut() {
            cell.toggle();
        }
    }

    /// Returns an iterator over all of the cells in this [`MapGrid`].
    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().flat_map(|row| row.iter())
    }

    /// Returns an iterator over all of the cells along with their position in this [`MapGrid`].
    pub fn iter_pos(&self) -> impl Iterator<Item = ((usize, usize), &Cell)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| ((x, y), cell)))
    }

    /// Returns a mutable iterator over all of the cells in this [`MapGrid`].
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut().flat_map(|row| row.iter_mut())
    }

    /// Returns a mutable iterator over all of the cells along with their position in this [`MapGrid`].
    pub fn iter_pos_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut Cell)> {
        self.cells.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, cell)| ((x, y), cell))
        })
    }

    /// Creates a new grid from the given [`section`](`crate::data::types::GridSquare`) of the current grid.
    ///
    /// TODO: Fix this to either handle overflow (by wrapping) or fail more gracefully.
    ///
    /// ### Panics
    /// Function panics if the size of `section` is less than 3x3.
    #[must_use]
    pub fn create_subgrid(&self, section: &GridSquare) -> Self {
        if section.height() < 3 || section.width() < 3 {
            error!("Invalid GridSquare size: {:?}", section);
            panic!("Invalid GridSquare size");
        }

        if section.max.x > self.width || section.max.y > self.height {
            error!(
                "Section is too big for current grid: Grid Size = {:?} Section = {:?}",
                self.size(),
                section
            );
            panic!("Invalid GridSquare size");
        }

        MapGrid::sub_grid(self, section)
    }

    /// Resize all rows in the grid to the given size, using [`crate::data::Cell::invalid()`]
    /// as the default value for each added cell. Rows cannot be resized to be less than
    /// 3. If grid currently already has `new_row_size` rows, function will early out.
    ///
    /// #### This changes the SIZE OF EACH ROW aka the width of the [`MapGrid`], NOT the ROW COUNT (which would be the height).
    /// ##### This is the same as calling [`MapGrid::resize_rows_with(new_row_size, Cell::invalid())`].
    ///
    /// ### Panics
    /// Function panics if the resulting size of the grid is less than 3x3, which should not happen
    /// because the function first checks if the result is going to be less than 3.
    pub fn resize_rows(&mut self, new_row_size: usize) {
        trace!("MapGrid::resize_rows({})", new_row_size);
        self.resize_rows_with(new_row_size, Cell::invalid());
    }

    /// Resize all rows in the grid to the given size, using `cell_value` as the
    /// default value for each added cell. Rows cannot be resized to be less than
    /// 3. If grid currently already has `new_row_size` rows, function will early out.
    ///
    /// #### This changes the SIZE OF EACH ROW aka the width of the [`MapGrid`], NOT the ROW COUNT (which would be the height).
    ///
    /// ### Panics
    /// Function panics if the resulting size of the grid is less than 3x3, which should not happen
    /// because the function first checks if the result is going to be less than 3.
    pub fn resize_rows_with(&mut self, new_row_size: usize, cell_value: Cell) {
        trace!(
            "MapGrid::resize_rows_with({}, {:?})",
            new_row_size,
            cell_value
        );
        let safe_size = if new_row_size < 3 {
            error!("MapGrid::resize_rows_with - cannot resize row length to less than 3");
            3
        } else {
            new_row_size
        };

        if safe_size == self.cols() {
            info!("MapGrid::resize_rows_with - new size same as current size, bailing on resize");
            return;
        }

        for row in &mut self.cells {
            row.resize(safe_size, cell_value);
        }

        assert!(
            self.cells[0].len() == safe_size,
            "Actual row length (self.cells[0].len() = {}) does not equal safe_size ({})",
            self.cells[0].len(),
            safe_size
        );
        self.width = safe_size;
    }

    /// Resize all columns in the grid to the given size, using [`crate::data::Cell::invalid()`]
    /// as the default value for each added cell. Column count cannot be than 3.
    /// If grid currently already has `new_column_size` columns, function will early out.
    ///
    /// #### This changes the SIZE OF EACH COLUMN aka the height of the [`MapGrid`], NOT the COLUMN COUNT (which would be the width).
    /// ##### This is the same as calling [`MapGrid::resize_cols_with(new_column_size, Cell::invalid())`].
    ///
    /// ### Panics
    /// Function panics if the resulting size of the grid is less than 3x3, which should not happen
    /// because the function first checks if the result is going to be less than 3.
    pub fn resize_cols(&mut self, new_column_size: usize) {
        trace!("MapGrid::resize_cols({})", new_column_size);
        self.resize_cols_with(new_column_size, Cell::invalid());
    }

    /// Resize all columns in the grid to the given size, using `cell_value` as the
    /// default value for each added cell. Column count cannot be less than 3.
    /// If grid currently already has `new_column_size` columns, function will early out.
    ///
    /// #### This changes the SIZE OF EACH COLUMN aka the height of the [`MapGrid`], NOT the COLUMN COUNT (which would be the width).
    ///
    /// ### Panics
    /// Function panics if the resulting size of the grid is less than 3x3, which should not happen
    /// because the function first checks if the result is going to be less than 3.
    pub fn resize_cols_with(&mut self, new_column_size: usize, cell_value: Cell) {
        trace!(
            "MapGrid::resize_cols_with({}, {:?})",
            new_column_size,
            cell_value
        );
        let safe_size = if new_column_size < 3 {
            error!("MapGrid::resize_cols_with - cannot resize column count to less than 3");
            3
        } else {
            new_column_size
        };

        if safe_size == self.rows() {
            info!("MapGrid::resize_cols_with - new size same as current size, bailing on resize");
            return;
        }

        let row_size = self.cols();
        self.cells.resize(safe_size, vec![cell_value; row_size]);
        assert!(
            self.cells.len() == safe_size,
            "Actual col length (self.cells.len() = {}) does not equal safe_size ({})",
            self.cells.len(),
            safe_size
        );
        self.height = safe_size;
    }

    /// Convenience function which calls:
    /// ```ignore
    /// # use dungen::data::MapGrid;
    /// # let mut grid = MapGrid::new((5, 5));
    /// # assert!(grid.cell_count() == 25);
    /// # let size = (10,10);
    /// grid.resize_rows(size.0);
    /// grid.resize_cols(size.1);
    /// # assert!(grid.cell_count() == 100);
    /// ```
    ///
    /// ### Panics
    /// - Function panics if the resulting size of the grid is less than 3x3, which should not happen
    /// because the function first checks if the result is going to be less than 3.
    /// - Function panics if the actual resulting size of the grid does not match the expected end size
    /// (which means something probably went horribly wrong or was horribly coded)
    pub fn resize<P: Into<(usize, usize)>>(&mut self, size: P) {
        let (width, height) = size.into();
        if self.width != width {
            self.resize_rows(width);
        }
        if self.height != height {
            self.resize_cols(height);
        }

        let new_current: (usize, usize) = self.size().into();
        if new_current.0 != width.max(3) || new_current.1 != height.max(3) {
            error!(
                "MapGrid::resize - grid not set to the expected size. Actual = {:?} Expected = {:?}",
                self.size(),
                (width, height)
            );
            panic!("MapGrid::resize - failed to resize to requested size");
        }
    }

    /// Modifies this [`MapGrid`] by adding the contents of `other` to it
    /// at position (0,0).
    pub fn union_in_place(&mut self, other: &Self) {
        self.integrate_in_place(other, (0, 0).into());
    }

    /// Integrates the given [`MapGrid`] into this one at the given position. Newer data
    /// (from `other`) will take precedence over the currently existing data. This
    /// [`MapGrid`] will be resized if necessary.
    pub fn integrate_in_place(&mut self, other: &Self, offset: GridPos) {
        let offset_size = (other.width + offset.x, other.height + offset.y);
        if other.width + offset.x > self.width || other.height + offset.y > self.height {
            self.resize((
                offset_size.0.max(self.width),
                offset_size.1.max(self.height),
            ));
        }

        for ((x, y), &cell) in other.iter_pos() {
            self.set_cell(x + offset.x, y + offset.y, cell);
        }
    }

    /// Converts this [`MapGrid`] into an instance of [`pathfinding::grid::Grid`].
    #[must_use]
    pub fn to_pf_grid(&self) -> PFGrid {
        let mut pf_grid = PFGrid::new(self.width, self.height);
        pf_grid.enable_diagonal_mode();

        for ((x, y), cell) in self.iter_pos() {
            if cell.is_on() {
                pf_grid.add_vertex((x, y));
            }
        }

        pf_grid
    }

    /// Converts the grid to a [Vec] of [String]s, with each cell represented by the given
    /// character.
    #[must_use]
    pub fn to_strings_with(&self, on: char, off: char) -> Vec<String> {
        trace!("MapGrid::to_strings_with({}, {})", on, off);

        let invalid: char = {
            if INVALID_MARKERS[0] != on && INVALID_MARKERS[0] != off {
                INVALID_MARKERS[0]
            } else if INVALID_MARKERS[1] != on && INVALID_MARKERS[1] != off {
                INVALID_MARKERS[1]
            } else {
                INVALID_MARKERS[2]
            }
        };
        info!(
            "MapGrid::to_strings_with - Using '{}' as invalid character",
            invalid
        );

        let mut strings = Vec::with_capacity(self.height);

        for row in &self.cells {
            let mut string = String::with_capacity(row.len());
            for cell in row {
                string.push(if cell.is_on() {
                    on
                } else if cell.is_off() {
                    off
                } else {
                    invalid
                });
            }
            strings.push(string);
        }

        strings
    }

    /// Converts the grid to a [String] with each cell represented by the given on and off
    /// characters, with each row separated by the given separator.
    #[must_use]
    pub fn to_string_with(&self, on: char, off: char, div: char) -> String {
        trace!("MapGrid::to_string_with({}, {}, {})", on, off, div);
        self.to_strings_with(on, off).join(&div.to_string())
    }

    /// Gets a [Vec] of [String]s representing the grid, using the default on and off
    /// characters (`'#'` and `'.'` respectively).
    #[must_use]
    pub fn to_strings(&self) -> Vec<String> {
        trace!("MapGrid::to_strings()");
        self.to_strings_with('#', '.')
    }

    /// Gets a string representation of the grid with the default on and off characters
    /// (`'#'` and `'.'` respectively).
    #[must_use]
    pub fn as_string(&self) -> String {
        self.to_strings().join("\n")
    }
}

/// Serialization and Deserialization implementations.
impl MapGrid {
    /// Parse the given [`input`] [`serde_json::Value`] into a [`MapGrid`].
    ///
    /// ### Errors
    /// Function errors if [`serde_json::from_value`] fails.
    ///
    /// ##### See also: [`serde_json::from_value`]
    pub fn from_json<J: Into<serde_json::Value>>(input: J) -> Result<Self, serde_json::Error> {
        serde_json::from_value(input.into())
    }

    /// Parse the given [`input`] string into a [`MapGrid`].
    ///
    /// ### Errors
    /// Function errors if [`serde_json::from_str`] fails.
    ///
    /// ##### See also: [`serde_json::from_str`]
    pub fn from_json_str<S: AsRef<str>>(input: S) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input.as_ref())
    }

    /// Parse the given [`input`] bytes into a [`MapGrid`].
    ///
    /// ### Errors
    /// Function errors if [`serde_json::from_slice`] fails.
    ///
    /// ##### See also: [`serde_json::from_slice`]
    pub fn from_json_bytes<B: AsRef<[u8]>>(input: B) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(input.as_ref())
    }

    /// Parse the given [`reader`] into a [`MapGrid`].
    ///
    /// ### Errors
    /// Function errors if [`serde_json::from_reader`] fails.
    ///
    /// ##### See also: [`serde_json::from_reader`]
    pub fn from_json_reader<R: Read>(reader: R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    /// Open the [`path`](`std::convert::AsRef<std::path::Path>`) and parses the resulting
    /// reader into a [`MapGrid`] using [`MapGrid::from_json_reader`].
    ///
    /// ### Errors
    /// Function errors if [`serde_json::from_reader`] fails.
    ///
    /// ##### See also: [`serde_json::from_reader`]
    pub fn from_json_file<P: AsRef<std::path::Path>>(path: P) -> serde_json::Result<Self> {
        match File::open(path) {
            Ok(file) => Self::from_json_reader(file),
            Err(e) => Err(serde_json::Error::io(e)),
        }
    }

    /// Serialize this [`MapGrid`] into a [`Json Value`](`serde_json::Value`).
    ///
    /// ### Errors
    /// Function errors if [`serde_json::to_value`] fails.
    ///
    /// ##### See also: [`serde_json::to_value`]
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    /// Serialize this [`MapGrid`] into a [`Byte Array`](`std::collections::Vec<u8>`).
    ///
    /// ### Errors
    /// Function errors if [`serde_json::to_vec`] fails.
    ///
    /// ##### See also: [`serde_json::to_vec`]
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Serialize this [`MapGrid`] into a [`String`] containing the json. The [`pretty`]
    /// argument determines whether it is converted with pretty indentation for display.
    ///
    /// ### Errors
    /// Function errors if [`serde_json::to_string`] or [`serde_json::to_string_pretty`] fails.
    ///
    /// ##### See also: [`serde_json::to_string`] [`serde_json::to_string_pretty`]
    pub fn to_json_string(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(self)
        } else {
            serde_json::to_string(self)
        }
    }

    /// Deserialize the given `byte_ref` (which must implement [`std::convert::AsRef<[u8]>`] + [`?Sized`])
    /// containing msgpack data into a new [`MapGrid`]. This is performed in zero-copy manner whenever it
    /// is possible, borrowing the data from the reader itself. For example, strings and byte-arrays won’t
    /// be not copied.
    ///
    /// ### Errors
    /// Function errors if [`rmp_serde::from_read_ref`] fails.
    ///  
    /// ##### See also: [`rmp_serde::from_read_ref`].
    pub fn from_msgpack_ref<R: AsRef<[u8]> + ?Sized>(
        byte_ref: &R,
    ) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_read_ref(byte_ref)
    }

    /// Deserialize the given [`reader`](std::io::Read) containing msgpack data into a [`MapGrid`].
    ///
    /// ### Errors
    /// Function errors if [`rmp_serde::from_read`] fails.
    ///
    /// ##### See also: [`rmp_serde::from_read`].
    pub fn from_msgpack_reader<R: Read>(reader: R) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_read(reader)
    }

    /// Serialize this [`MapGrid`] into a [`Vec<u8>`] of msgpack data.
    ///
    /// ### Errors
    /// Function errors if [`rmp_serde::to_vec`] fails.
    ///
    /// ##### See also: [`rmp_serde::to_vec`]
    pub fn to_msgpack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(self)
    }
}

impl From<PFGrid> for MapGrid {
    fn from(pfg: PFGrid) -> Self {
        let mut grid = MapGrid::empty((pfg.width, pfg.height));
        for (x, y) in pfg.iter() {
            grid.set_cell_state(x, y, true);
        }

        grid
    }
}

impl From<&PFGrid> for MapGrid {
    fn from(pfg: &PFGrid) -> Self {
        let mut grid = MapGrid::empty((pfg.width, pfg.height));
        for (x, y) in pfg.iter() {
            grid.set_cell_state(x, y, true);
        }

        grid
    }
}

impl PartialEq for MapGrid {
    /// Checks whether `other` is equal to this [`MapGrid`].
    ///
    /// This does check ***EACH CELL*** in the [`MapGrid`], but it has early outs
    /// if the dimensions or name of the grids are not equal.
    fn eq(&self, other: &MapGrid) -> bool {
        trace!("MapGrid::eq()");
        if self.width != other.width || self.height != other.height {
            return false;
        }

        if self.name != other.name {
            return false;
        }

        for (row, other_row) in self.cells.iter().zip(other.cells.iter()) {
            for (cell, other_cell) in row.iter().zip(other_row.iter()) {
                if cell != other_cell {
                    return false;
                }
            }
        }

        true
    }
}

impl std::fmt::Debug for MapGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MapGrid {{ name: {:?}, width: {}, height: {} }}",
            self.name, self.width, self.height
        )?;
        writeln!(f)?;
        let mut i = 0;
        write!(f, " ")?;
        while i < self.width {
            write!(f, "{}", i % 10)?;
            i += 1;
        }
        writeln!(f)?;
        let grid = self.to_strings();
        for (y, grid) in grid.iter().enumerate() {
            writeln!(f, "{}{}", y % 10, grid)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for MapGrid {
    /// Displays a fancy [`MapGrid`] over multiple lines.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title_line = if self.has_name() {
            format!(
                "{} ({}x{})",
                self.name.as_ref().unwrap(),
                self.width,
                self.height
            )
        } else {
            format!("MapGrid ({}x{})", self.width, self.height)
        };
        write!(
            f,
            "|  {}\n|{}\n",
            title_line,
            "-".repeat(title_line.len() + 4)
        )?;
        for line in &self.to_strings() {
            writeln!(f, "|{}", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::cognitive_complexity, clippy::too_many_lines, unused)]
mod tests {
    use super::*;

    use assert_float_eq::{
        afe_abs, afe_absolute_error_msg, afe_is_absolute_eq, afe_is_relative_eq,
        afe_relative_error_msg, assert_float_absolute_eq, assert_float_relative_eq,
    };

    use crate::assert_unordered_match;
    use crate::data::pos;
    use crate::util::testing::crate_before_test;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
        crate::util::random::init_rng_seeded(0);
    }

    #[test]
    fn construction_works() {
        crate_before_test();

        let mut grid = MapGrid::new(size(10, 10));
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.rows(), 10);
        assert_eq!(grid.cols(), 10);
        assert_eq!(grid.cells.len(), 10);
        assert_eq!(grid.cells[0].len(), 10);
        assert_eq!(grid.invalid_cells_count(), 100);
        assert_eq!(grid.on_cells_count(), 0);
        assert_eq!(grid.off_cells_count(), 0);

        grid.set_cell(0, 0, Cell::on());
        grid.set_cell(1, 0, Cell::off());
        assert_eq!(grid.invalid_cells_count(), 98);
        assert_eq!(grid.on_cells_count(), 1);
        assert_eq!(grid.off_cells_count(), 1);

        let mut grid = MapGrid::empty((5, 5));
        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.rows(), 5);
        assert_eq!(grid.cols(), 5);
        assert_eq!(grid.cells.len(), 5);
        assert_eq!(grid.cells[0].len(), 5);
        assert_eq!(grid.invalid_cells_count(), 0);
        assert_eq!(grid.on_cells_count(), 0);
        assert_eq!(grid.off_cells_count(), 25);

        grid.set_cell_state(0, 0, true);
        assert_eq!(grid.invalid_cells_count(), 0);
        assert_eq!(grid.on_cells_count(), 1);
        assert_eq!(grid.off_cells_count(), 24);

        grid.set_cell_invalid(0, 1);
        assert_eq!(grid.invalid_cells_count(), 1);
        assert_eq!(grid.on_cells_count(), 1);
        assert_eq!(grid.off_cells_count(), 23);
    }

    #[test]
    fn random_fill_works() {
        init();

        let grid = MapGrid::random_fill_percent((10, 10), 0.5);
        assert_eq!(grid.on_cells_count(), 50);
        assert_eq!(grid.off_cells_count(), 50);

        let grid = MapGrid::random_fill_number((10, 10), 50);
        assert_eq!(grid.on_cells_count(), 50);
        assert_eq!(grid.off_cells_count(), 50);
    }

    #[test]
    fn set_all_cells() {
        init();

        let mut grid = MapGrid::new(size(5, 5));
        assert_eq!(grid.invalid_cells_count(), 25);
        assert_eq!(grid.on_cells_count(), 0);
        assert_eq!(grid.off_cells_count(), 0);

        grid.set_all_cells(true);
        assert_eq!(grid.invalid_cells_count(), 0);
        assert_eq!(grid.on_cells_count(), 25);
        assert_eq!(grid.off_cells_count(), 0);

        grid.set_all_cells(false);
        assert_eq!(grid.invalid_cells_count(), 0);
        assert_eq!(grid.on_cells_count(), 0);
        assert_eq!(grid.off_cells_count(), 25);
    }

    #[test]
    fn reverse_in_place() {
        init();

        let mut grid = MapGrid::empty((5, 5));
        assert_eq!(grid.invalid_cells_count(), 0);
        assert_eq!(grid.on_cells_count(), 0);
        assert_eq!(grid.off_cells_count(), 25);

        grid.reverse_in_place();
        assert_eq!(grid.invalid_cells_count(), 0);
        assert_eq!(grid.on_cells_count(), 25);
        assert_eq!(grid.off_cells_count(), 0);
    }

    #[test]
    fn set_outer_works() {
        init();

        let mut grid = MapGrid::empty((3, 3));
        grid.set_outer_cells(true);
        assert_eq!(grid.on_cells_count(), 8);
        assert_eq!(grid.off_cells_count(), 1);
        assert_eq!(
            grid.to_strings().join("\n"),
            "###\n#.#\n###",
            "Grid did not match expected output"
        );

        let mut grid = MapGrid::empty((5, 5));
        grid.set_outer_cells(true);
        assert_eq!(grid.on_cells_count(), 16);
        assert_eq!(grid.off_cells_count(), 9);
        assert_eq!(
            grid.to_strings().join("\n"),
            "#####\n#...#\n#...#\n#...#\n#####"
        );

        grid.reverse_in_place();
        assert_eq!(
            grid.to_strings().join("\n"),
            ".....\n.###.\n.###.\n.###.\n....."
        );
    }

    #[test]
    fn random_cells_work() {
        init();

        let mut grid = MapGrid::empty((10, 10));
        for _ in 0..50 {
            let (x, y) = grid.random_cell_pos().into();
            assert!(x < 25);
            assert!(y < 25);
        }

        for _ in 0..50 {
            let _ = grid.random_cell();
        }

        for _ in 0..50 {
            let _ = grid.random_cell_mut();
        }
    }

    #[test]
    #[should_panic]
    fn panics_invalid_height() {
        init();
        MapGrid::empty((100, 2));
    }

    #[test]
    #[should_panic]
    fn panics_invalid_width() {
        init();
        MapGrid::empty((2, 100));
    }

    #[test]
    fn names_work() {
        init();

        let mut grid = MapGrid::empty((5, 5));
        assert_eq!(grid.name, None);
        assert!(!grid.has_name());

        grid.set_name("test");
        assert_eq!(grid.name, Some("test".to_string()));
        assert!(grid.has_name());

        let mut grid = MapGrid::empty_named("Test", (5, 5));
        assert_eq!(grid.name, Some("Test".to_string()));
        assert!(grid.has_name());

        grid.clear_name();
        assert_eq!(grid.name, None);
        assert!(!grid.has_name());
    }

    #[test]
    fn get_neighbors() {
        init();
        let grid1 = MapGrid::parse_string("###\n#-#\n###", '#', '-')
            .expect("Failed to parse standard grid, something is very wrong.");
        assert_eq!(
            grid1.size(),
            (3, 3).into(),
            "Grid should have 3 rows & 3 columns."
        );
        assert_unordered_match!(
            grid1.neighbor_positions_wrapping((1, 1)),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2)
            ]
        );
        assert_unordered_match!(
            grid1.neighbor_positions_wrapping((1, 0)),
            vec![
                (0, 2),
                (1, 2),
                (2, 2),
                (0, 0),
                (2, 0),
                (0, 1),
                (1, 1),
                (2, 1)
            ]
        );
        assert_unordered_match!(
            grid1.neighbor_positions_wrapping((0, 0)),
            vec![
                (2, 2),
                (0, 2),
                (1, 2),
                (2, 0),
                (1, 0),
                (2, 1),
                (0, 1),
                (1, 1)
            ]
        );

        let grid2 = MapGrid::parse_string("#-#-#\n-#-#-\n#-#-#", '#', '-')
            .expect("Failed to parse standard grid, something is very wrong.");
        assert_eq!(
            grid2.size(),
            (5, 3).into(),
            "Grid should be 5 wide and 3 tall"
        );
        assert_unordered_match!(
            grid2.neighbor_positions_wrapping((3, 1)),
            vec![
                (2, 0),
                (3, 0),
                (4, 0),
                (2, 1),
                (4, 1),
                (2, 2),
                (3, 2),
                (4, 2)
            ]
        );
        assert_unordered_match!(
            grid2.neighbor_positions_wrapping((4, 2)),
            vec![
                (3, 1),
                (4, 1),
                (0, 1),
                (3, 2),
                (0, 2),
                (3, 0),
                (4, 0),
                (0, 0)
            ]
        );

        let grid3 =
            MapGrid::parse_string("#-#-#\n-#-#-\n#-#-#", '#', '-').expect("Unable to parse grid3");
        let neighbors = grid3.neighbor_positions((3, 1));
        assert_eq!(neighbors.len(), 8);
        assert_unordered_match!(
            neighbors,
            [
                (2, 0),
                (3, 0),
                (4, 0),
                (2, 1),
                (4, 1),
                (2, 2),
                (3, 2),
                (4, 2),
            ]
        );
        let neighbors = grid3.neighbor_positions((0, 0));
        assert_eq!(neighbors.len(), 3);
        assert_unordered_match!(neighbors, [(0, 1), (1, 1), (1, 0)]);
        let neighbors = grid3.neighbor_positions((1, 0));
        assert_eq!(neighbors.len(), 5);
        assert_unordered_match!(neighbors, [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);
    }

    #[test]
    fn neighbor_count_works() {
        init();
        let grid1 = MapGrid::parse_string("###\n#-#\n###", '#', '-')
            .expect("Failed to parse standard grid, something is very wrong.");
        assert_eq!(grid1.rows(), 3, "Grid 1 should have 3 rows.");
        assert_eq!(grid1.cols(), 3, "Grid 1 should have 3 cols.");

        assert_eq!(
            grid1.active_neighbor_count((1, 1), true),
            8,
            "Wrong neighbor count for grid 1 cell (1,1)"
        );
        assert_eq!(
            grid1.active_neighbor_count((1, 0), true),
            7,
            "Wrong neighbor count for grid 1 cell (1,0)"
        );
        assert_eq!(
            grid1.active_neighbor_count((0, 0), true),
            7,
            "Wrong neighbor count for grid 1 cell (0,0)"
        );

        let grid2 = MapGrid::parse_string("0000\n0110\n0000", '1', '0')
            .expect("Failed to parse standard grid, something is very wrong.");
        assert_eq!(grid2.rows(), 3, "Grid 2 should have 3 rows.");
        assert_eq!(grid2.cols(), 4, "Grid 2 should have 4 cols.");
        assert_eq!(
            grid2.active_neighbor_count((1, 1), true),
            1,
            "Wrong neighbor count for grid 2 cell (1,1)"
        );
        assert_eq!(
            grid2.active_neighbor_count((2, 1), true),
            1,
            "Wrong neighbor count for grid 2 cell (2,1)"
        );
        assert_eq!(
            grid2.active_neighbor_count((1, 0), true),
            2,
            "Wrong neighbor count for grid 1 cell (2,0)"
        );
        assert_eq!(
            grid2.active_neighbor_count((2, 0), true),
            2,
            "Wrong neighbor count for grid 1 cell (1,0)"
        );
    }

    #[test]
    fn subgrids() {
        init();

        // #########
        // #.......#
        // #.#####.#
        // #.#...#.#
        // #.#.#.#.#
        // #.#...#.#
        // #.#####.#
        // #.......#
        // #########
        let grid = MapGrid::parse_string("#########\n#.......#\n#.#####.#\n#.#...#.#\n#.#.#.#.#\n#.#...#.#\n#.#####.#\n#.......#\n#########", '#', '.').expect("Unable to parse grid.");
        assert_eq!(grid.width, 9);
        assert_eq!(grid.height, 9);
        assert_eq!(grid.to_strings().join("\n"), "#########\n#.......#\n#.#####.#\n#.#...#.#\n#.#.#.#.#\n#.#...#.#\n#.#####.#\n#.......#\n#########");

        let square = square(&(1, 1), 7, 7);
        // let section = GridSection {
        //     center: (4, 4),
        //     u_extent: 3,
        //     d_extent: 3,
        //     l_extent: 3,
        //     r_extent: 3,
        // };

        assert_eq!((square.width(), square.height()), (7, 7));
        assert_eq!(square.size(), size(7, 7));
        assert_eq!(square.center(), pos((4, 4)));
        assert_eq!(square.x_range(), 1..8);

        let sub = MapGrid::sub_grid(&grid, &square);
        assert_eq!(
            sub.to_strings().join("\n"),
            ".......\n.#####.\n.#...#.\n.#.#.#.\n.#...#.\n.#####.\n......."
        );
        assert_eq!(sub.size(), (7, 7).into());
    }

    #[test]
    fn n_neighbors() {
        init();

        // #...#
        // ..#..
        // ..#..
        // ..#..
        // #...#
        let grid = MapGrid::parse_string("#...#\n..#..\n..#..\n..#..\n#...#", '#', '.')
            .expect("Unable to parse grid!");
        assert_eq!(grid.on_cells_count(), 7);
        assert_eq!(grid.active_neighbor_count((2, 2), true), 2);
        assert_eq!(grid.active_neighbors_n(2, 2, 2), 6);
    }

    #[test]
    fn cell_ratio() {
        init();

        let grid = MapGrid::parse_string("####\n####\n....\n....", '#', '.')
            .expect("Failed to parse standard grid, something is very wrong.");

        let (on, off, inv) = grid.cell_state_ratio();
        assert_float_relative_eq!(on, 0.5);
        assert_float_relative_eq!(off, 0.5);
        assert_float_absolute_eq!(inv, 0.0);

        let grid = MapGrid::new(size(4, 4));
        let (on, off, inv) = grid.cell_state_ratio();
        assert_float_absolute_eq!(on, 0.0);
        assert_float_absolute_eq!(off, 0.0);
        assert_float_relative_eq!(inv, 1.0);

        let grid = MapGrid::empty((4, 4));
        let (on, off, inv) = grid.cell_state_ratio();
        assert_float_absolute_eq!(on, 0.0);
        assert_float_relative_eq!(off, 1.0);
        assert_float_absolute_eq!(inv, 0.0);

        let mut grid = MapGrid::parse_string("#..\n#..\n#..", '#', '.')
            .expect("Failed to parse standard grid, something is very wrong.");

        let (on, off, inv) = grid.cell_state_ratio();
        assert_float_relative_eq!(on, (1.0 / 3.0));
        assert_float_relative_eq!(off, (2.0 / 3.0));
        assert_float_absolute_eq!(inv, 0.0);
        grid.reverse_in_place();
        let (on, off, inv) = grid.cell_state_ratio();
        assert_float_relative_eq!(on, (2.0 / 3.0));
        assert_float_relative_eq!(off, (1.0 / 3.0));
        assert_float_absolute_eq!(inv, 0.0);
    }

    #[test]
    fn resize_works() {
        init();

        let mut grid = MapGrid::empty((5, 5));
        assert_eq!(grid.cell_count(), 25);
        let size = (10,10);
        let cell_value = Cell::on();
        grid.resize_rows_with(size.0, cell_value);
        grid.resize_cols_with(size.1, cell_value);
        warn!("{}", grid.to_string());
        assert_eq!(grid.cell_count(), 100);
    }
}
