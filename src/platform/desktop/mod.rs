//! Desktop platform plugin

mod config;

pub use config::DesktopConfig;

use bevy::prelude::*;

use crate::camera::DesktopCameraPlugin;
use crate::input::DesktopInputPlugin;
use crate::platform::Platform;
#[cfg(target_arch = "wasm32")]
use crate::input::TouchInputPlugin;

pub struct DesktopPlatformPlugin;

impl Plugin for DesktopPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Platform::Desktop)
            .init_resource::<DesktopConfig>()
            .add_plugins((DesktopInputPlugin, DesktopCameraPlugin));

        #[cfg(target_arch = "wasm32")]
        app.add_plugins(TouchInputPlugin);

        info!("🖥️ Desktop platform initialized");
    }
}
