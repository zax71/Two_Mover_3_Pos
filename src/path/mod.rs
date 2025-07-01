pub mod bezier;
pub mod line;

use percentage::PercentageDecimal;
use vector3d::Vector3d;

pub trait Path {
    /// Calculate the 3D coordinate at index% in to this path
    fn point_at(&self, index: PercentageDecimal) -> Vector3d<f64>;
}
