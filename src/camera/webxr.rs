//! WebXR camera - applies XR pose to Bevy camera

use bevy::prelude::*;

use super::{CameraState, GameCamera};
use crate::platform::on_webxr;

#[cfg(feature = "webxr")]
use crate::platform::{WebXrPose, WebXrState};

pub struct WebXrCameraPlugin;

impl Plugin for WebXrCameraPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "webxr")]
        app.add_systems(Update, apply_webxr_camera.run_if(on_webxr));
    }
}

#[cfg(feature = "webxr")]
fn apply_webxr_camera(
    state: Res<WebXrState>,
    pose: Res<WebXrPose>,
    mut camera_state: ResMut<CameraState>,
    mut camera_q: Query<(&mut Transform, &mut Projection), With<GameCamera>>,
) {
    if !state.session_active {
        return;
    }

    let data = match pose.0.lock() {
        Ok(d) => d.clone(),
        Err(_) => return,
    };
    if !data.valid {
        return;
    }

    let Ok((mut transform, mut projection)) = camera_q.get_single_mut() else {
        return;
    };

    let quat = Quat::from_xyzw(
        data.orientation[0],
        data.orientation[1],
        data.orientation[2],
        data.orientation[3],
    );
    transform.rotation = quat;

    if let Projection::Perspective(ref mut p) = *projection {
        p.fov = 90.0_f32.to_radians();
    }

    let (yaw, pitch, _) = quat.to_euler(EulerRot::YXZ);
    camera_state.yaw = yaw;
    camera_state.pitch = pitch;
    camera_state.reset_effects();
}
