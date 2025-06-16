use vector3d::Vector3d;

#[derive(Default)]
pub struct Light {
    pub coordinates: Vector3d<isize>,
    pub minimum_beam: u16,
    pub maximum_beam: u16,
    pub name: String,
    pub address: usize,
}
