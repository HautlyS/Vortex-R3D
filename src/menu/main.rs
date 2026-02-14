//! Main menu orchestrator integration

use bevy::prelude::*;

use crate::menu::{MenuConfig, MenuState};

pub struct VortexMenuPlugin;

impl Plugin for VortexMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuConfig>()
            .init_resource::<MenuState>();
    }
}

#[allow(dead_code)]
pub fn initialize_menu_system(_app: &mut App) {}

#[allow(dead_code)]
pub fn monitor_menu_performance(_time: Res<Time>, _config: Res<MenuConfig>) {}

#[allow(dead_code)]
pub fn debug_menu_state(_state: Res<MenuState>, _config: Res<MenuConfig>) {}

#[allow(dead_code)]
pub fn configure_menu_aesthetics(
    _config: ResMut<MenuConfig>,
    _keyboard: Res<ButtonInput<KeyCode>>,
) {
}

#[allow(dead_code)]
pub fn cleanup_menu_resources(
    _commands: Commands,
    _query: Query<Entity, With<crate::menu::MenuOption>>,
) {
}

#[allow(dead_code)]
pub fn save_menu_state(_state: Res<MenuState>) {}

#[allow(dead_code)]
pub fn play_menu_audio(_time: Res<Time>, _config: Res<MenuConfig>) {}

#[cfg(feature = "vr")]
#[allow(dead_code)]
pub fn handle_vr_menu(_vr_input: Res<VRInput>, _menu_state: ResMut<MenuState>) {}

#[cfg(not(feature = "vr"))]
#[allow(dead_code)]
pub fn handle_vr_menu() {}

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
pub fn handle_web_menu(_web_input: Res<WebInput>, _menu_state: ResMut<MenuState>) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub fn handle_web_menu() {}

#[allow(dead_code)]
pub fn check_menu_accessibility(_state: Res<MenuState>) {}

#[allow(dead_code)]
pub fn track_menu_interactions(_state: Res<MenuState>) {}
