//! Desktop platform plugin

use bevy::prelude::*;

use crate::input::DesktopInputPlugin;
use crate::camera::DesktopCameraPlugin;
use super::Platform;

pub struct DesktopPlatformPlugin;

impl Plugin for DesktopPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Platform::Desktop)
            .add_plugins((DesktopInputPlugin, DesktopCameraPlugin));
        info!("🖥️ Desktop platform initialized");
    }
}
