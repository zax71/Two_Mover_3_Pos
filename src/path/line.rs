use crate::path::Path;
use percentage::PercentageDecimal;
use vector3d::Vector3d;

pub struct Line {
    start: Vector3d<f64>,
    end: Vector3d<f64>,
}

impl Path for Line {
    fn point_at(&self, index: PercentageDecimal) -> Vector3d<f64> {
        // It's easier to work with our index as a vector
        let index_as_vector: Vector3d<f64> =
            Vector3d::new(index.value(), index.value(), index.value());

        // In the maths world, multiplying each value in a vector with the same value in another is called the "cross product"
        // See https://www.desmos.com/calculator/tiwsdtcsfy for a more readable version of this
        index_as_vector.cross(self.end - self.start) + self.start
    }
}
