//! Given a list of vertices, determine whether a point lies inside the polygon
//! they bound.

extern crate nalgebra as na;

use std::f64::NAN;
use std::f64::INFINITY as INF;
use std::f64::NEG_INFINITY as N_INF;

pub fn in_polygon(query_point: na::Vector2<f64>, bounds: Vec<na::Vector2<f64>>) -> bool {
    // the simplest metric of inclusion
    if !in_bbox(&query_point, &bounds) {
        return false;
    }

    true
}

/// Get the bounding box of a slice of 2D points.
fn get_bbox(points: &[na::Vector2<f64>]) -> [na::Vector2<f64>; 2] {
    // find bounds
    let min_x = points.iter().map(|e| e[0]).fold(NAN, f64::min);
    let max_x = points.iter().map(|e| e[0]).fold(NAN, f64::max);
    let min_y = points.iter().map(|e| e[1]).fold(NAN, f64::min);
    let max_y = points.iter().map(|e| e[1]).fold(NAN, f64::max);

    // package them up
    [
        na::Vector2::new(min_x, min_y),
        na::Vector2::new(max_x, max_y)
    ]
}

/// Test if a point lies inside the bounding box of a collection of points.
fn in_bbox(query_point: &na::Vector2<f64>, bounds: &[na::Vector2<f64>]) -> bool {
    // find bounds
    let bbox = get_bbox(bounds);

    // decide if it's in or out
    query_point[0] != NAN
        && query_point[0] != INF
        && query_point[0] != N_INF
        && query_point[1] != NAN
        && query_point[1] != INF
        && query_point[1] != N_INF
        && query_point[0] >= bbox[0][0]
        && query_point[0] <= bbox[1][0]
        && query_point[1] >= bbox[0][1]
        && query_point[1] <= bbox[1][1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_of_square() {
        let square_pts = vec![
            na::Vector2::new(-1.0, 1.0),
            na::Vector2::new(1.0, 1.0),
            na::Vector2::new(1.0, -1.0),
            na::Vector2::new(-1.0, -1.0),
        ];
        let query_pt = na::Vector2::new(0.0, 0.0);
        assert_eq!(in_polygon(query_pt, square_pts), true);
    }
}
