use percentage::Percentage;

use crate::{
    light::{Light, LightState},
    path::{Path, PathEnum},
};

pub struct Frame {
    delay: f64,
    light_states: Vec<LightState>,
}

/// Calculates where each of the lights needs to point during a move.
/// frames arg defines how many points should be calculated - must be >0 and the higher it is, the more accurate the move will be
pub fn calculate_move(
    path: PathEnum,
    lights: Vec<Light>,
    frames: u16,
    move_time: f64,
) -> Vec<Frame> {
    let percent_per_frame = (1 / frames) as f64;
    let mut out_frames: Vec<Frame> = vec![];

    // Loop over each frame...
    for frame in 1..frames {
        let mut current_frame = Frame {
            delay: move_time / (frames as f64),
            light_states: vec![],
        };

        // ...And for each frame point the current light at the right spot
        for light in &lights {
            current_frame
                .light_states
                .push(light.point_at(path.point_at(&Percentage::from_decimal(
                    percent_per_frame * (frame as f64),
                ))));
        }
        out_frames.push(current_frame);
    }

    out_frames
}
