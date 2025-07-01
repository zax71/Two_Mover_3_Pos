use percentage::PercentageDecimal;
use vector3d::Vector3d;

use crate::path::{bezier::Bezier, Path};

#[derive(Debug, Default, PartialEq)]
pub struct CubicBezier {
    pub start: Vector3d<f64>,
    pub end: Vector3d<f64>,
    pub handle_1: Vector3d<f64>,
    pub handle_2: Vector3d<f64>,
}

impl Path for CubicBezier {
    fn point_at(&self, index: &PercentageDecimal) -> Vector3d<f64> {
        // See https://www.desmos.com/calculator/083535c5a3 for an easier to follow version of this,
        // The short of it is, you find the "index" point between the two end points (treating them as lines) and find the point at "index" along that line
        let bezier_1 = Bezier {
            start: self.start,
            midpoint: self.handle_1,
            end: self.end,
        };

        let bezier_2 = Bezier {
            start: self.start,
            midpoint: self.handle_2,
            end: self.end,
        };

        bezier_2.point_at(&index)
            + (bezier_1.point_at(&index) - bezier_2.point_at(&index)) * index.value()
    }
}

#[cfg(test)]
mod tests {
    use percentage::Percentage;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// Checks if two floating point values are equal, leaving some wiggle room for floating point error
    fn float_eq(value_1: f64, value_2: f64) -> bool {
        let diff = value_1 - value_2;
        diff < 0.000001 && diff > -0.000001
    }

    /// Checks if two vector3Ds are equal, leaving some wiggle room for floating point error
    fn eq_vector3d(vector_1: Vector3d<f64>, vector_2: Vector3d<f64>) -> bool {
        let x_eq = float_eq(vector_1.x, vector_2.x);
        let y_eq = float_eq(vector_1.y, vector_2.y);
        let z_eq = float_eq(vector_1.z, vector_2.z);

        x_eq && y_eq && z_eq
    }

    #[test]
    fn test_point_begin() {
        let start = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 4.0,
            y: 4.0,
            z: 0.0,
        };
        let handle_1 = Vector3d {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        };
        let handle_2 = Vector3d {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };

        let path = CubicBezier {
            start,
            handle_1,
            handle_2,
            end,
        };

        assert_eq!(path.point_at(&Percentage::from_decimal(0.0)), start)
    }

    #[test]
    fn test_point_end() {
        let start = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 4.0,
            y: 4.0,
            z: 0.0,
        };
        let handle_1 = Vector3d {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        };
        let handle_2 = Vector3d {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };

        let path = CubicBezier {
            start,
            handle_1,
            handle_2,
            end,
        };

        assert_eq!(path.point_at(&Percentage::from_decimal(1.0)), end)
    }

    #[test]
    fn test_point_middle() {
        let start = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 4.0,
            y: 4.0,
            z: 0.0,
        };
        let handle_1 = Vector3d {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        };
        let handle_2 = Vector3d {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };

        let path = CubicBezier {
            start,
            handle_1,
            handle_2,
            end,
        };

        assert!(eq_vector3d(
            path.point_at(&Percentage::from_decimal(0.2)),
            Vector3d {
                x: 1.184,
                y: 0.416,
                z: 0.0,
            },
        ))
    }
}
