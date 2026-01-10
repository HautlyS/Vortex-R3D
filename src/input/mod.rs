//! Input abstraction - unified events with platform-specific readers

#[cfg(feature = "desktop")]
mod desktop;
#[cfg(all(feature = "desktop", target_arch = "wasm32"))]
mod touch;
#[cfg(feature = "vr")]
mod vr;
#[cfg(feature = "webxr")]
mod webxr;

#[cfg(feature = "desktop")]
pub use desktop::DesktopInputPlugin;
#[cfg(all(feature = "desktop", target_arch = "wasm32"))]
pub use touch::TouchInputPlugin;
#[cfg(feature = "vr")]
pub use vr::VrInputPlugin;
#[cfg(feature = "webxr")]
pub use webxr::WebXrInputPlugin;

use bevy::prelude::*;

/// Unified input events - consumed by camera and game systems
#[derive(Message, Clone, Debug)]
pub enum InputEvent {
    Move(Vec2),
    Look(Vec2),
    Interact,
    ToggleMenu,
    // ToggleBookReader removed - now handled by bevy_egui_kbgp
    NextRoom,
    AdjustFov(f32),
}

/// Current input state
#[derive(Resource, Default)]
pub struct InputState {
    pub movement: Vec2,
    pub look_delta: Vec2,
    pub interact_pressed: bool,
    pub cursor_locked: bool,
}

impl InputState {
    pub fn reset(&mut self) {
        self.movement = Vec2::ZERO;
        self.look_delta = Vec2::ZERO;
        self.interact_pressed = false;
    }
}

/// Tracks if UI wants pointer input (prevents cursor grab)
#[derive(Resource, Default)]
pub struct UiWantsPointer(pub bool);

/// Base input plugin - event infrastructure only
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<InputEvent>()
            .init_resource::<InputState>()
            .init_resource::<UiWantsPointer>();
    }
}
