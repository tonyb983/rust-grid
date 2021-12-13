use euclid::{point2, Box2D, Point2D, Size2D};

/// Marker for [`euclid::Point2D`].
pub struct MapGridData;
/// Alias for [`usize`] that denotes an index into a [`crate::data::MapGrid`].
pub type GridIndex = usize;
/// Alias for [`euclid::Point2D`] that denotes a position in a [`crate::data::MapGrid`].
pub type GridPos = Point2D<GridIndex, MapGridData>;
/// Alias for [`euclid::Box2D`] that denotes a rectangle in a [`crate::data::MapGrid`].
pub type GridSquare = Box2D<GridIndex, MapGridData>;
/// Alias for [`euclid::Size2D`] that denotes the size of a [`crate::data::MapGrid`].
pub type GridSize = Size2D<GridIndex, MapGridData>;

/// Create a new [`crate::data::Pos`].
pub fn pos<T: AsPos<U> + Copy, U>(xy: T) -> GridPos {
    xy.as_pos()
}

pub fn square<T1: AsPos<B1>, B1>(top_left: &T1, x_size: usize, y_size: usize) -> GridSquare {
    GridSquare::from_origin_and_size(top_left.as_pos(), Size2D::new(x_size, y_size))
}

pub fn size(width: usize, height: usize) -> GridSize {
    GridSize::new(width, height)
}

pub trait AsPos<T> {
    fn as_pos(&self) -> GridPos;
}

impl<T> AsPos<T> for T
where
    (usize, usize): From<T>,
    T: Copy
{
    fn as_pos(&self) -> GridPos {
        let tup: (usize, usize) = (*self).into();

        GridPos::new(tup.0, tup.1)
    }
}
