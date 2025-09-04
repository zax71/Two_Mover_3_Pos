use isx::prelude::IsDefault;
use vector3d::Vector3d;

use crate::path::Path;

impl IsDefault for Bezier {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Bezier {
    pub name: String,
    pub start: Vector3d<f64>,
    pub midpoint: Vector3d<f64>,
    pub end: Vector3d<f64>,
}

impl Path for Bezier {
    fn point_at(&self, index: &percentage::PercentageDecimal) -> Vector3d<f64> {
        // See https://www.desmos.com/calculator/083535c5a3 for an easier to follow version of this,
        // The short of it is, you find the "index" point between the two end points (treating them as lines) and find the point at "index" along that line
        let start_line_point = self.midpoint + (self.end - self.midpoint) * index.value();
        let end_line_point = self.start + (self.midpoint - self.start) * index.value();

        end_line_point + (start_line_point - end_line_point) * index.value()
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}

impl Bezier {
    pub fn new(start: Vector3d<f64>, midpoint: Vector3d<f64>, end: Vector3d<f64>) -> Self {
        Self {
            name: String::default(),
            start,
            midpoint,
            end,
        }
    }

    pub fn with_name(
        name: String,
        start: Vector3d<f64>,
        midpoint: Vector3d<f64>,
        end: Vector3d<f64>,
    ) -> Self {
        Self {
            name,
            start,
            midpoint,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use percentage::Percentage;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_point_begin() {
        let start = Vector3d {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        };
        let midpoint = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };

        let path = Bezier {
            name: String::default(),
            start,
            midpoint,
            end,
        };

        assert_eq!(path.point_at(&Percentage::from_decimal(0.0)), start)
    }

    #[test]
    fn test_point_end() {
        let start = Vector3d {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        };
        let midpoint = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };
        let path = Bezier {
            name: String::default(),
            start,
            midpoint,
            end,
        };

        assert_eq!(path.point_at(&Percentage::from_decimal(1.0)), end)
    }

    #[test]
    fn test_point_middle() {
        let start = Vector3d {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        };
        let midpoint = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };
        let path = Bezier {
            name: String::default(),
            start,
            midpoint,
            end,
        };

        assert_eq!(
            path.point_at(&Percentage::from_decimal(0.5)),
            Vector3d {
                x: 1.0,
                y: 1.0,
                z: 0.0
            }
        )
    }
}
