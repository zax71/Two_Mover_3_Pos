pub mod bezier;
pub mod cubic_bezier;
pub mod line;

use enum_dispatch::enum_dispatch;
use percentage::PercentageDecimal;
use vector3d::Vector3d;

use crate::path::{bezier::Bezier, cubic_bezier::CubicBezier, line::Line};

#[enum_dispatch]
#[derive(Clone, PartialEq)]
pub enum PathEnum {
    Line,
    Bezier,
    CubicBezier,
}

#[enum_dispatch(PathEnum)]
pub trait Path: Clone + PartialEq {
    /// Calculate the 3D coordinate at index% in to this path
    fn point_at(&self, index: &PercentageDecimal) -> Vector3d<f64>;
    fn name(&self) -> String;
}
