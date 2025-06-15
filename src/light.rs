use measurements::Angle;
use vector3d::Vector3d;

pub struct Light {
    coordinates: Vector3d<isize>,
    minimum_beam: Angle,
    maximum_beam: Angle,
    name: String,
}