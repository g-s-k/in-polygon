//! Given a list of vertices, determine whether a point lies inside the polygon
//! they bound.

extern crate nalgebra as na;

pub fn in_polygon(query_point: na::Vector2<f64>, bounds: Vec<na::Vector2<f64>>) -> bool {
    // the simplest metric of inclusion
    if !in_bbox(&query_point, &bounds) {
        return false;
    }

    true
}

// Test if a point lies inside the bounding box of a collection of points.
fn in_bbox(query_point: &na::Vector2<f64>, bounds: &Vec<na::Vector2<f64>>) -> bool {
    // constants
    let nan = 0. / 0.;
    let inf = 1. / 0.;
    let n_inf = -1. / 0.;

    // find bounds
    let min_x = bounds.iter().map(|e| e[0]).fold(nan, f64::min);
    let max_x = bounds.iter().map(|e| e[0]).fold(nan, f64::max);
    let min_y = bounds.iter().map(|e| e[1]).fold(nan, f64::min);
    let max_y = bounds.iter().map(|e| e[1]).fold(nan, f64::max);

    // decide if it's in or out
    query_point[0] != inf
        && query_point[0] != n_inf
        && query_point[1] != inf
        && query_point[1] != n_inf
        && query_point[0] >= min_x
        && query_point[0] <= max_x
        && query_point[1] >= min_y
        && query_point[1] <= max_y
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
