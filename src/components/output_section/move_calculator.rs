use std::{net::IpAddr, thread, time::Duration};

use anyhow::Result;
use eosc_rs::eos_desk::EosDesk;
use percentage::Percentage;

use crate::{
    app::GlobalState,
    light::{Light, LightState},
    path::{Path, PathEnum},
};

#[derive(Debug)]
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
    // Ensure that we actually have lights
    if lights.is_empty() {
        panic!("Lights must be passed into calculate_move, an empty vec was passed instead");
    }

    // Taking the reciprocal of the number of frames gives us how many percent per frame
    let percent_per_frame: f64 = (frames as f64).recip();
    let mut out_frames: Vec<Frame> = vec![];

    // Loop over each frame...
    for frame in 0..frames {
        let mut current_frame = Frame {
            delay: move_time / (frames as f64),
            light_states: vec![],
        };

        // ...And for each frame point the current light at the right spot
        for light in &lights {
            let path_point = path.point_at(&Percentage::from_decimal(
                percent_per_frame * (frame as f64),
            ));
            current_frame.light_states.push(light.point_at(path_point));
        }
        out_frames.push(current_frame);
    }

    out_frames
}

/// Turns a Vec<Frame> in to a set of EOS commands, as a Vec<String>
pub fn frames_to_commands(frames: Vec<Frame>, first_cue_number: u32) -> Vec<String> {
    let mut out_commands: Vec<String> = vec![];
    for (i, frame) in frames.iter().enumerate() {
        for light_state in &frame.light_states {
            out_commands.append(&mut light_state.to_commands());
        }

        // {:0>2} left pads the number so there's always 2 digits, e.g: i=1 becomes 01 etc
        let cue_number: String = format!("{}.{:0>2}", first_cue_number, i);

        out_commands.push(format!("Record Cue {} Time {:.2}", cue_number, frame.delay));
        out_commands.push(format!("Cue {} Follow {:.2}", cue_number, frame.delay));
    }

    out_commands
}

pub fn output_commands(commands: Vec<String>, app_state: &mut GlobalState) -> Result<()> {
    // Get the IP addresses from config
    let config = app_state.config_file.read()?.osc;
    let desk: EosDesk = EosDesk::new(
        (IpAddr::V4(config.host.0), config.host.1),
        (IpAddr::V4(config.desk.0), config.desk.1),
    )?;

    for command in commands {
        desk.command(&command)?;
        // TODO: Extremely cursed, please remove
        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
