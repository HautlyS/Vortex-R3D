//! Desktop platform plugin

use bevy::prelude::*;

use super::Platform;
use crate::camera::DesktopCameraPlugin;
use crate::input::DesktopInputPlugin;

pub struct DesktopPlatformPlugin;

impl Plugin for DesktopPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Platform::Desktop)
            .add_plugins((DesktopInputPlugin, DesktopCameraPlugin));
        info!("🖥️ Desktop platform initialized");
    }
}
