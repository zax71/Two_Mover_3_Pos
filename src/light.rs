use serde::{Deserialize, Serialize};
use vector3d::Vector3d;

#[derive(Default, Deserialize, Serialize)]
pub struct Light {
    pub coordinates: Vector3d<f64>,
    pub minimum_beam: u16,
    pub maximum_beam: u16,
    pub name: String,
    pub address: usize,
}
