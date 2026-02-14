//! Menu plugin integration - Connect menu system to main game

use bevy::prelude::*;

use crate::menu::{MenuConfig, MenuState};

#[allow(dead_code)]
pub struct MainMenuIntegrationPlugin;

impl Plugin for MainMenuIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuConfig>()
            .init_resource::<MenuState>();
    }
}

#[allow(dead_code)]
pub struct UpdateGameStatePlugin;

impl Plugin for UpdateGameStatePlugin {
    fn build(&self, _app: &mut App) {}
}
