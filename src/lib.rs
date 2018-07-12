//! Given a list of vertices, determine whether a point lies inside the polygon
//! they bound.

extern crate nalgebra as na;

pub fn in_polygon(query_point: na::Vector2<f64>, bounds: Vec<na::Vector2<f64>>) -> bool {
    // the simplest metric of inclusion
    let nan = 0. / 0.;
    let min_x = bounds.iter().map(|e| e[0]).fold(nan, f64::min);
    let max_x = bounds.iter().map(|e| e[0]).fold(nan, f64::max);
    let min_y = bounds.iter().map(|e| e[1]).fold(nan, f64::min);
    let max_y = bounds.iter().map(|e| e[1]).fold(nan, f64::max);

    if query_point[0] < min_x
        || query_point[0] > max_x
        || query_point[1] < min_y
        || query_point[1] > max_y
    {
        return false;
    }

    true
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
