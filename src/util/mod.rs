use integer_sqrt::IntegerSquareRoot;
use num_traits::{PrimInt, Unsigned};

pub mod geo;
pub mod random;
pub mod tri;

#[cfg(test)]
pub mod testing;

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub fn bresenham_line(a: (usize, usize), b: (usize, usize)) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let mut x1 = a.0 as i32;
    let mut y1 = a.1 as i32;
    let mut x2 = b.0 as i32;
    let mut y2 = b.1 as i32;
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


