use std::ops::Range;

use crate::{
    data::{square, GridSquare, MapGrid},
    logging::trace,
};

/// Different sizes for rooms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoomSize {
    /// A small room.
    Small,
    /// A medium room.
    Medium,
    /// A big room.
    Big,
    /// A huge room.
    Huge,
}

static SIZES: [RoomSize; 4] = [
    RoomSize::Small,
    RoomSize::Medium,
    RoomSize::Big,
    RoomSize::Huge,
];

impl RoomSize {
    /// Get an iterator over all room sizes.
    pub fn all_sizes() -> impl Iterator<Item = RoomSize> {
        SIZES.into_iter()
    }
}

/// A room for the [`crate::gen::RoomBasedGenerator`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Room(GridSquare);

impl Room {
    /// Create a new [`Room`] from the upper left and width and height.
    #[must_use] 
    pub fn new(upper_left: (usize, usize), width: usize, height: usize) -> Self {
        Self(square(&upper_left, width, height))
    }

    /// Creates a new [`Room`] within the ranges provided.
    #[must_use] 
    #[allow(clippy::similar_names)]
    pub fn random(
        start_x_range: Range<usize>,
        start_y_range: Range<usize>,
        width_range: Range<usize>,
        height_range: Range<usize>,
    ) -> Self {
        trace!(
            "Room::random center_x_range: {:?}, center_y_range: {:?}, width_range: {:?}, height_range: {:?}",
            start_x_range,
            start_y_range,
            width_range,
            height_range
        );
        let x = fastrand::usize(start_x_range);
        let y = fastrand::usize(start_y_range);
        let width = fastrand::usize(width_range);
        let height = fastrand::usize(height_range);
        let half_x = x / 2;
        let half_y = y / 2;
        trace!(
            "Room::random x: {}, y: {}, width: {}, height: {} half_x: {}, half_y: {}",
            x,
            y,
            width,
            height,
            half_x,
            half_y
        );

        let left = if let Some(l) = x.checked_sub(half_x) {
            l
        } else {
            0
        };
        let upper = if let Some(u) = y.checked_sub(half_y) {
            u
        } else {
            0
        };

        Room::new((left, upper), width, height)
    }

    /// Check whether the `first` room intersects with the `second`.
    #[must_use] 
    pub fn check_intersects(first: &Self, other: &Self) -> bool {
        first.0.intersects(&other.0)
    }

    /// Checks whether the `room` fits within the `grid`.
    #[must_use] 
    pub fn fits_in_grid(room: &Self, grid: &MapGrid) -> bool {
        room.0.min.x < grid.cols()
            && room.0.min.y < grid.rows()
            && room.0.max.x < grid.cols()
            && room.0.max.y < grid.rows()
    }
}

impl Room {
    /// Check whether this room intersects with `other`.
    #[must_use] 
    pub fn intersects(&self, other: &Self) -> bool {
        Self::check_intersects(self, other)
    }

    /// Checks whether this room, plus the `buffer` (on all sides), intersects with `other`.
    #[must_use] 
    pub fn intersects_with_buffer(&self, other: &Self, buffer: usize) -> bool {
        let mut room = *self;
        room.0.min.x -= if room.0.min.x > buffer { buffer } else { 0 };
        room.0.min.y -= if room.0.min.y > buffer { buffer } else { 0 };
        room.0.max.x += buffer;
        room.0.max.y += buffer;

        Self::check_intersects(&room, other)
    }

    /// Gets the inner [`GridSquare`] of this [`Room`].
    #[must_use] 
    pub fn square(&self) -> GridSquare {
        self.0
    }

    /// Get a vec containing the positions of each cell in the edge of this [`Room`].
    #[must_use] 
    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        let (min_x, min_y) = self.0.min.into();
        let (max_x, max_y) = self.0.max.into();

        for x in min_x..max_x {
            for y in min_y..max_y {
                if x == min_x || x == (max_x - 1) || y == min_y || y == (max_y - 1) {
                    edges.push((x, y));
                }
            }
        }

        edges
    }
}
