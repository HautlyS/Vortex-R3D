//! WebXR input - controller and gaze-based input

use bevy::prelude::*;

use super::InputState;
use crate::platform::on_webxr;

#[cfg(feature = "webxr")]
use crate::platform::WebXrState;

pub struct WebXrInputPlugin;

impl Plugin for WebXrInputPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "webxr")]
        app.add_systems(Update, handle_webxr_input.run_if(on_webxr));
    }
}

#[cfg(feature = "webxr")]
fn handle_webxr_input(state: Res<WebXrState>, mut input_state: ResMut<InputState>) {
    if !state.session_active {
        return;
    }
    input_state.cursor_locked = true;
}
