//! VR platform plugin - OpenXR integration

use bevy::prelude::*;

use super::Platform;
use crate::camera::VrCameraPlugin;
use crate::input::VrInputPlugin;

pub struct VrPlatformPlugin;

impl Plugin for VrPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Platform::Vr)
            .add_plugins((VrInputPlugin, VrCameraPlugin));
        info!("🥽 VR platform initialized");
    }
}
