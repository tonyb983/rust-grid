use integer_sqrt::IntegerSquareRoot;
use num_traits::{PrimInt, Unsigned};
use stroke::{Bezier, Point, PointN};

use crate::data::GridPos;

/// Simple function to determine if the two given points are in the same row (y values are equal).
#[allow(dead_code)]
#[must_use]
pub fn on_same_row(first: GridPos, second: GridPos) -> bool {
    first.y == second.y
}

/// Simple function to determine if the two given points are in the same column (x values are equal).
#[allow(dead_code)]
#[must_use]
pub fn on_same_column(first: GridPos, second: GridPos) -> bool {
    first.x == second.x
}

/// Bresenham's line algorithm.
///
/// Bresenham's Line Algorithm is a way of drawing a line segment onto a square grid. It is especially useful
/// for roguelikes due to their cellular nature. A detailed explanation of the algorithm can be found
/// [`here`](`https://www.cs.helsinki.fi/group/goa/mallinnus/lines/bresenh.html`).
///
/// #### Implementation taken from [`RogueBasin's Rust Implementation`](`http://roguebasin.com/index.php/Bresenham%27s_Line_Algorithm#Rust`).
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
#[allow(dead_code)]
#[must_use]
pub fn bresenham_line<P1: Into<(usize, usize)>, P2: Into<(usize, usize)>>(
    first_point: P1,
    second_point: P2,
) -> Vec<(usize, usize)> {
    let first = first_point.into();
    let second = second_point.into();
    let mut points = Vec::new();
    let mut x1 = first.0 as i32;
    let mut y1 = first.1 as i32;
    let mut x2 = second.0 as i32;
    let mut y2 = second.1 as i32;
    let is_steep = (y2 - y1).abs() > (x2 - x1).abs();
    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    let mut reversed = false;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        reversed = true;
    }
    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;
    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }
    for x in x1..=x2 {
        if is_steep {
            points.push((y as usize, x as usize));
        } else {
            points.push((x as usize, y as usize));
        }
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }

    if reversed {
        for i in 0..(points.len() / 2) {
            let end = points.len() - 1;
            points.swap(i, end - i);
        }
    }

    points
}

/// Calculates a curved line between two points.
/// 
/// This uses a coin-flip to determine if the middle point is (first.x, second.y) or (second.x, first.y).
/// 
/// TODO: Currently this algorithm uses 1000 steps and then dedups the resulting point array, but it can probably be done better by calculating the distance between the two points and using a calculation from that value to determine the maximum steps, so that two points that are adjacent don't use the same number of steps as two points that are 1000 units apart.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn get_curve_between<P1: Into<(usize, usize)>, P2: Into<(usize, usize)>>(
    first_point: P1,
    second_point: P2,
) -> Vec<(usize, usize)> {
    let first = first_point.into();
    let second = second_point.into();
    let mid = if fastrand::bool() {
        (first.0, second.1)
    } else {
        (second.0, first.1)
    };
    let first_f = [first.0 as f64, first.1 as f64];
    let second_f = [second.0 as f64, second.1 as f64];
    let mid_f = [mid.0 as f64, mid.1 as f64];
    let curve = Bezier::new([
        PointN::new(first_f),
        PointN::new(mid_f),
        PointN::new(second_f),
    ]);

    let nsteps: usize = 1000;
    let mut points = Vec::with_capacity(nsteps);
    for t in 0..nsteps {
        let t = t as f64 * 1f64 / (nsteps as f64);
        let fp = curve.eval(t);
        let f1 = fp.axis(0);
        let f2 = fp.axis(1);
        points.push((f1.round() as usize, f2.round() as usize));
    }
    points.sort_unstable();
    points.dedup();

    points
}

/// Return the square root of `n` if `n` is square, `None` otherwise.
///
/// # Example
///
/// ```
/// use pathfinding::utils::uint_sqrt;
///
/// assert_eq!(uint_sqrt(100usize), Some(10));
/// assert_eq!(uint_sqrt(10usize), None);
/// ```
#[inline]
pub fn uint_sqrt<T>(n: T) -> Option<T>
where
    T: PrimInt + Unsigned,
{
    let root = n.integer_sqrt();
    (n == root * root).then(|| root)
}

/// Compute the absolute difference between two values.
///
/// # Example
///
/// The absolute difference between 4 and 17 as unsigned values will be 13.
///
/// ```
/// use pathfinding::utils::absdiff;
///
/// assert_eq!(absdiff(4u32, 17u32), 13u32);
/// assert_eq!(absdiff(17u32, 4u32), 13u32);
/// ```
#[inline]
pub fn absdiff<T>(x: T, y: T) -> T
where
    T: std::ops::Sub<Output = T> + PartialOrd,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absdiff_test() {
        assert_eq!(absdiff(4u32, 17u32), 13u32);
        assert_eq!(absdiff(17u32, 4u32), 13u32);

        assert_eq!(absdiff(40usize, 17usize), 23usize);
        assert_eq!(absdiff(17usize, 40usize), 23usize);
    }
}