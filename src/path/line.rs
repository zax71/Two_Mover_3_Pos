use crate::path::Path;
use isx::prelude::IsDefault;
use percentage::PercentageDecimal;
use vector3d::Vector3d;

#[derive(Debug, Default, PartialEq)]
pub struct NamedLine {
    pub name: String,
    pub line: Line,
}

impl IsDefault for NamedLine {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Line {
    pub start: Vector3d<f64>,
    pub end: Vector3d<f64>,
}

impl Path for Line {
    fn point_at(&self, index: &PercentageDecimal) -> Vector3d<f64> {
        // See https://www.desmos.com/calculator/tiwsdtcsfy for a more readable version of this
        self.start + (self.end - self.start) * index.value()
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
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let path = Line { start, end };

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
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let path = Line { start, end };

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
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let path = Line { start, end };

        assert_eq!(
            path.point_at(&Percentage::from_decimal(0.5)),
            Vector3d {
                x: 0.5,
                y: 0.5,
                z: 0.5
            }
        )
    }
}
