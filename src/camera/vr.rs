//! VR camera - syncs XrCamera state, no motion effects

use bevy::prelude::*;
use bevy_mod_xr::camera::XrCamera;

use super::CameraState;
use crate::platform::on_vr;

pub struct VrCameraPlugin;

impl Plugin for VrCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_vr_camera_state.run_if(on_vr));
    }
}

fn sync_vr_camera_state(
    xr_cameras: Query<&Transform, With<XrCamera>>,
    mut state: ResMut<CameraState>,
) {
    if let Some(transform) = xr_cameras.iter().next() {
        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        state.yaw = yaw;
        state.pitch = pitch;
        state.reset_effects(); // No motion effects in VR
    }
}

/// Marker for entities that follow VR head
#[derive(Component)]
pub struct FollowVrHead;

pub fn follow_vr_head(
    xr_cameras: Query<&Transform, With<XrCamera>>,
    mut followers: Query<&mut Transform, (With<FollowVrHead>, Without<XrCamera>)>,
) {
    let Some(head) = xr_cameras.iter().next() else {
        return;
    };
    for mut t in &mut followers {
        t.translation = head.translation;
    }
}
