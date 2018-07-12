//! Given a list of vertices, determine whether a point lies inside the polygon
//! they bound.

extern crate nalgebra as na;

pub fn in_polygon(query_point: na::Vector2<f64>, bounds: Vec<na::Vector2<f64>>) -> bool {
    false
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
