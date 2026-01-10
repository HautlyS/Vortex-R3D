//! VR platform plugin - OpenXR integration

use bevy::prelude::*;

use crate::input::VrInputPlugin;
use crate::camera::VrCameraPlugin;
use super::Platform;

pub struct VrPlatformPlugin;

impl Plugin for VrPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Platform::Vr)
            .add_plugins((VrInputPlugin, VrCameraPlugin));
        info!("🥽 VR platform initialized");
    }
}
