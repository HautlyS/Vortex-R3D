//! Camera abstraction - unified state with platform-specific controllers

mod desktop;
#[cfg(feature = "vr")]
mod vr;
#[cfg(feature = "webxr")]
mod webxr;

#[allow(unused_imports)]
pub use desktop::DesktopCameraPlugin;
#[cfg(feature = "vr")]
pub use vr::VrCameraPlugin;
#[cfg(feature = "webxr")]
pub use webxr::WebXrCameraPlugin;

use bevy::prelude::*;

/// Marker for the main game camera
#[derive(Component)]
pub struct GameCamera;

/// Shared camera state across all platforms
#[derive(Resource)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub sensitivity: f32,
    // Desktop-only effects (zeroed on VR)
    pub walk_cycle: f32,
    pub move_speed: f32,
    pub motion_blur: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            fov: 75.0,
            sensitivity: 0.08,
            walk_cycle: 0.0,
            move_speed: 0.0,
            motion_blur: 0.0,
        }
    }
}

impl CameraState {
    /// Reset motion effects (call when switching to VR)
    pub fn reset_effects(&mut self) {
        self.walk_cycle = 0.0;
        self.move_speed = 0.0;
        self.motion_blur = 0.0;
    }
}

/// Base camera plugin - shared resources only
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraState>();
    }
}
