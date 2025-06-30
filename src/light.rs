use serde::{Deserialize, Serialize};
use vector3d::Vector3d;

#[derive(Default, Deserialize, Serialize, PartialEq)]
pub struct Light {
    pub coordinates: Vector3d<f64>,
    pub minimum_beam: u16,
    pub maximum_beam: u16,
    pub name: String,
    pub address: usize,
}

impl Light {
    /// Returns true if the light is at it's default values (hence, empty)
    pub fn empty(&self) -> bool {
        // Needs the * to copy the value as the pointer is not the same as the real value
        *self == Self::default()
    }
}
