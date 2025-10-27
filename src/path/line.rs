use crate::path::Path;
use isx::prelude::IsDefault;
use measurements::implement_display;
use percentage::PercentageDecimal;
use vector3d::Vector3d;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Line {
    pub name: String,
    pub start: Vector3d<f64>,
    pub end: Vector3d<f64>,
}

impl IsDefault for Line {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Path for Line {
    fn point_at(&self, index: &PercentageDecimal) -> Vector3d<f64> {
        println!("Getting point at {} on line", index.value());
        // See https://www.desmos.com/calculator/tiwsdtcsfy for a more readable version of this
        self.start + (self.end - self.start) * index.value()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Line {
    // Make a new Line where you do not care about it's name
    #[allow(dead_code)]
    pub fn new(start: Vector3d<f64>, end: Vector3d<f64>) -> Self {
        Self {
            name: String::default(),
            start,
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
            y: 0.0,
            z: 0.0,
        };
        let end = Vector3d {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let path = Line::new(start, end);

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

        let path = Line::new(start, end);

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

        let path = Line::new(start, end);

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
