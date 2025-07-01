use vector3d::Vector3d;

use crate::path::Path;

pub struct Bezier {
    start: Vector3d<f64>,
    midpoint: Vector3d<f64>,
    end: Vector3d<f64>,
}

impl Path for Bezier {
    fn point_at(&self, index: percentage::PercentageDecimal) -> Vector3d<f64> {
        // It's easier to work with our index as a vector
        let index_as_vector: Vector3d<f64> =
            Vector3d::new(index.value(), index.value(), index.value());

        // In the maths world, multiplying each value in a vector with the same value in another is called the "cross product"
        // See https://www.desmos.com/calculator/083535c5a3 for an easier to follow version of this,
        // The short of it is, you find the "index" point between the two end points (treating them as lines) and find the point at "index" along that line
        let start_line_point = self.midpoint + index_as_vector.cross(self.start - self.midpoint);
        let end_line_point = self.midpoint + index_as_vector.cross(self.midpoint - self.end);

        end_line_point + index_as_vector.cross(start_line_point - end_line_point)
    }
}
