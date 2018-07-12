//! Given a list of vertices, determine whether a point lies inside the polygon
//! they bound.

extern crate nalgebra as na;

pub fn in_polygon(query_point: na::Vector2<f64>, bounds: Vec<na::Vector2<f64>>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
