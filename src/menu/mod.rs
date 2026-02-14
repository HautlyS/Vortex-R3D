//! Menu system - Cyberpunk orchestrator for world transitions

pub mod integration;
pub mod main;

use bevy::prelude::*;

pub use main::VortexMenuPlugin;

#[derive(Resource)]
pub struct MenuConfig {
    pub voxel_size: f32,
    pub neon_intensity: f32,
    pub glitch_frequency: f32,
    pub menu_radius: f32,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            voxel_size: 0.5,
            neon_intensity: 1.0,
            glitch_frequency: 0.02,
            menu_radius: 8.0,
        }
    }
}

#[derive(Resource)]
pub struct MenuState {
    pub current_selection: usize,
    pub total_options: usize,
    pub glitch_active: bool,
    pub glitch_timer: f32,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            current_selection: 0,
            total_options: 4,
            glitch_active: false,
            glitch_timer: 0.0,
        }
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct MenuOption {
    pub option_type: MenuOptionType,
    pub selected: bool,
    pub rotation_speed: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum MenuOptionType {
    StartGame,
    LoadGame,
    Settings,
    Credits,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct MenuVoxel;

#[allow(dead_code)]
#[derive(Component)]
pub struct MenuNeonText;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuConfig>()
            .init_resource::<MenuState>();
    }
}

pub struct MenuOrchestratorPlugin;

impl Plugin for MenuOrchestratorPlugin {
    fn build(&self, _app: &mut App) {}
}
