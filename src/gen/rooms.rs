use std::ops::Range;

use crate::{
    data::{square, Cell, GridSquare, MapGrid},
    logging::trace,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoomSize {
    Small,
    Medium,
    Big,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Room(GridSquare);

impl Room {
    pub fn new(upper_left: (usize, usize), width: usize, height: usize) -> Self {
        Self(square(&upper_left, width, height))
    }

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

    pub fn check_intersects(first: &Self, other: &Self) -> bool {
        first.0.intersects(&other.0)
    }

    pub fn fits_in_grid(room: &Self, grid: &MapGrid) -> bool {
        room.0.min.x < grid.cols()
            && room.0.min.y < grid.rows()
            && room.0.max.x < grid.cols()
            && room.0.max.y < grid.rows()
    }
}

impl Room {
    pub fn intersects(&self, other: &Self) -> bool {
        Self::check_intersects(self, other)
    }

    pub fn intersects_with_buffer(&self, other: &Self, buffer: usize) -> bool {
        let mut room = *self;
        room.0.min.x -= if room.0.min.x > buffer { buffer } else { 0 };
        room.0.min.y -= if room.0.min.y > buffer { buffer } else { 0 };
        room.0.max.x += buffer;
        room.0.max.y += buffer;

        Self::check_intersects(&room, other)
    }

    pub fn square(&self) -> GridSquare {
        self.0
    }

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
