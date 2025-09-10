use isx::prelude::IsDefault;
use serde::{Deserialize, Serialize};
use trig::Trig;
use vector3d::Vector3d;

#[derive(Default, Deserialize, Serialize, PartialEq, Clone)]
pub struct Light {
    pub coordinates: Vector3d<f64>,
    pub minimum_beam: u16,
    pub maximum_beam: u16,
    pub name: String,
    pub address: u16,
}

#[derive(Debug, PartialEq)]
pub struct LightState {
    pan: f64,
    tilt: f64,
    address: u16,
}

impl IsDefault for Light {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Light {
    /// Calculates the required pan and tilt for this light to point at the specified point in space
    pub fn point_at(&self, coordinate: Vector3d<f64>) -> LightState {
        // Desmos link: https://www.desmos.com/3d/ez2rjf9ahn (We're working with y up, this is working with z up fyi)

        // Calculate distances between two points in x,y and z as well as the "as the crow flies distance" that we'll call distance_straight
        let distance_x = coordinate.x - self.coordinates.x;
        let distance_y = coordinate.y - self.coordinates.y;
        let distance_z = coordinate.z - self.coordinates.z;
        let distance_straight = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

        // Now some trig with those values to calculate the pan and tilt (in degrees)
        // Take 90 degrees from the pan as zero is towards -x by default but we want towards +z
        // Adding f64::MIN_POSITIVE to stop divide by zero errors
        let pan = ((distance_y / distance_x + f64::MIN_POSITIVE).atand()) - 90.0;
        let tilt = (distance_straight / distance_z + f64::MIN_POSITIVE).atand();

        LightState {
            pan,
            tilt,
            address: self.address,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// Checks if two floating point values are equal, leaving some wiggle room for floating point error
    fn float_eq(value_1: f64, value_2: f64) -> bool {
        let diff = value_1 - value_2;
        diff < 0.000001 && diff > -0.000001
    }

    #[test]
    fn test_light_point_at_1() {
        let light = Light {
            coordinates: Vector3d {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            minimum_beam: 0,
            maximum_beam: 0,
            name: "N/A".to_string(),
            address: 5,
        };

        let point = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let out_light_state = LightState {
            pan: -90.0,
            tilt: 90.0,
            address: 5,
        };

        assert_eq!(light.point_at(point), out_light_state)
    }

    #[test]
    fn test_light_point_at_2() {
        let light = Light {
            coordinates: Vector3d {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            minimum_beam: 0,
            maximum_beam: 0,
            name: "N/A".to_string(),
            address: 5,
        };

        let point = Vector3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let out_light_state = LightState {
            pan: -90.0,
            tilt: 45.0,
            address: 5,
        };

        assert_eq!(light.point_at(point), out_light_state)
    }
}
