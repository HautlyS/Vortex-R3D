//! VR input - Room-scale tracking + controller input

use bevy::prelude::*;
use bevy_mod_xr::session::XrTracker;

use super::{InputEvent, InputState};
use crate::platform::on_vr;

pub struct VrInputPlugin;

impl Plugin for VrInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VrInputState>()
            .add_systems(Update, read_vr_input.run_if(on_vr));
    }
}

#[derive(Resource, Default)]
pub struct VrInputState {
    pub last_head_pos: Vec3,
    pub last_head_rot: Quat,
}

fn read_vr_input(
    trackers: Query<&Transform, With<XrTracker>>,
    mut vr_state: ResMut<VrInputState>,
    mut input_state: ResMut<InputState>,
    mut events: EventWriter<InputEvent>,
) {
    let Ok(head) = trackers.get_single() else { return };
    
    // Movement from physical position
    let pos_delta = head.translation - vr_state.last_head_pos;
    if pos_delta.length() > 0.001 {
        let movement = Vec2::new(pos_delta.x, pos_delta.z);
        input_state.movement = movement;
        events.send(InputEvent::Move(movement));
    }
    
    // Look from head rotation
    let rot_delta = head.rotation * vr_state.last_head_rot.inverse();
    let (yaw, pitch, _) = rot_delta.to_euler(EulerRot::YXZ);
    if yaw.abs() > 0.001 || pitch.abs() > 0.001 {
        input_state.look_delta = Vec2::new(yaw, pitch);
        events.send(InputEvent::Look(input_state.look_delta));
    }
    
    vr_state.last_head_pos = head.translation;
    vr_state.last_head_rot = head.rotation;
    input_state.cursor_locked = true;
}
