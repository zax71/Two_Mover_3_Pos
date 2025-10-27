use isx::prelude::IsDefault;
use trig::Trig;
use vector3d::Vector3d;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Light {
    pub coordinates: Vector3d<f64>,
    pub minimum_beam: u16,
    pub maximum_beam: u16,
    pub name: String,
    pub address: u16,
}

#[derive(Debug, PartialEq)]
pub struct LightState {
    pub pan: f64,
    pub tilt: f64,
    pub address: u16,
}

impl IsDefault for Light {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Light {
    /// Calculates the required pan and tilt for this light to point at the specified point in space
    /// coordinates in form (x,y,z) with z being up, x right and y forward
    pub fn point_at(&self, coordinate: Vector3d<f64>) -> LightState {
        // Desmos link: https://www.desmos.com/3d/ez2rjf9ahn (We're working with y up, this is working with z up fyi)

        // Calculate distances between two points in x,y and z as well as the "as the crow flies distance" that we'll call distance_straight
        let distance_x = coordinate.x - self.coordinates.x;
        let distance_y = coordinate.y - self.coordinates.y;
        let distance_z = coordinate.z - self.coordinates.z;
        let distance_straight = (distance_x.powi(2) + distance_y.powi(2)).sqrt();

        // Now some trig with those values to calculate the pan and tilt (in degrees)
        // Adding f64::MIN_POSITIVE to stop divide by zero errors
        let pan = (coordinate.x - self.coordinates.x).atan2d(&(coordinate.y - self.coordinates.y));
        let tilt = -((distance_straight / distance_z + f64::MIN_POSITIVE).atand()); // * -1 as it is always -ve

        LightState {
            pan,
            tilt,
            address: self.address,
        }
    }
}

impl LightState {
    pub fn to_commands(&self) -> Vec<String> {
        vec![
            format!("{} Pan {:.4}", self.address, self.pan),
            format!("{} Tilt {:.4}", self.address, self.tilt),
        ]
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

    fn eq_light_state(state_1: LightState, state_2: LightState) -> bool {
        let address_eq = state_1.address == state_2.address;
        let pan_eq = float_eq(state_1.pan, state_2.pan);
        let tilt_eq = float_eq(state_1.tilt, state_2.tilt);

        address_eq && pan_eq && tilt_eq
    }

    #[test]
    fn test_light_point_at_1() {
        let light = Light {
            coordinates: Vector3d {
                x: 6.0,
                y: 4.0,
                z: 1.0,
            },
            minimum_beam: 0,
            maximum_beam: 0,
            name: "N/A".to_string(),
            address: 5,
        };

        let point = Vector3d {
            x: 6.0,
            y: 3.0,
            z: 0.0,
        };

        let out_light_state = LightState {
            pan: 180.0,
            tilt: 45.0,
            address: 5,
        };

        assert_eq!(light.point_at(point), out_light_state)
    }

    #[test]
    fn test_light_point_at_2() {
        let light = Light {
            coordinates: Vector3d {
                x: 6.0,
                y: 4.0,
                z: 1.0,
            },
            minimum_beam: 0,
            maximum_beam: 0,
            name: "N/A".to_string(),
            address: 5,
        };

        let point = Vector3d {
            x: 7.0,
            y: 4.0,
            z: 0.0,
        };

        let out_light_state = LightState {
            pan: 90.0,
            tilt: 45.0,
            address: 5,
        };

        assert_eq!(light.point_at(point), out_light_state)
    }

    #[test]
    fn test_light_point_at_3() {
        let light = Light {
            coordinates: Vector3d {
                x: 6.0,
                y: 4.0,
                z: 1.0,
            },
            minimum_beam: 0,
            maximum_beam: 0,
            name: "N/A".to_string(),
            address: 5,
        };

        let point = Vector3d {
            x: 5.0,
            y: 3.0,
            z: 0.0,
        };

        let out_light_state = LightState {
            pan: -135.0,
            tilt: 54.7356103172,
            address: 5,
        };

        assert!(eq_light_state(light.point_at(point), out_light_state));
    }

    #[test]
    fn test_light_point_at_4() {
        let light = Light {
            coordinates: Vector3d {
                x: -3.2,
                y: -1.8,
                z: 6.0,
            },
            minimum_beam: 0,
            maximum_beam: 0,
            name: "N/A".to_string(),
            address: 5,
        };

        let point = Vector3d {
            x: -4.0,
            y: -2.8,
            z: -2.47,
        };

        let out_light_state = LightState {
            pan: -141.340191746,
            tilt: 8.59773680459,
            address: 5,
        };

        assert!(eq_light_state(light.point_at(point), out_light_state));
    }
}
