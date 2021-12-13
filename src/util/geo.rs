use stroke::Point;

use crate::data::GridPos;

pub fn on_same_row(first: GridPos, second: GridPos) -> bool {
    first.y == second.y
}

pub fn on_same_column(first: GridPos, second: GridPos) -> bool {
    first.x == second.x
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn get_curve_between(first: GridPos, second: GridPos) -> Vec<(usize,usize)> {
    let mid = if fastrand::bool() {
        (first.x, second.y)
    } else {
        (second.x, first.y)
    };
    let first_f = [first.x as f64, first.y as f64];
    let second_f = [second.x as f64, second.y as f64];
    let mid_f = [mid.0 as f64, mid.1 as f64];
    let curve = stroke::Bezier::new([
        stroke::PointN::new(first_f),
        stroke::PointN::new(mid_f),
        stroke::PointN::new(second_f),
    ]);

    let nsteps: usize = 1000;
    let mut points = Vec::with_capacity(nsteps);
    for t in 0..nsteps {
        let t = t as f64 * 1f64 / (nsteps as f64);
        let mut fp = curve.eval(t);
        let f1 = fp.axis(0);
        let f2 = fp.axis(1);
        points.push((f1.round() as usize, f2.round() as usize));
    }
    points.sort_unstable();
    points.dedup();

    points
}