//! Upload Room - Cross-platform file upload for panoramas and 3D models

mod file_picker;
mod scene;
mod ui;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use crate::GameState;
use file_picker::{load_pending_model, poll_file_data};
use scene::{rotate_ambient_light, setup_upload_room};
use ui::{handle_keyboard_shortcuts, upload_hud};

pub struct UploadRoomPlugin;

impl Plugin for UploadRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<UploadState>()
            .add_systems(Startup, go_to_viewing)
            .add_systems(OnEnter(GameState::Viewing), setup_upload_room)
            .add_systems(
                Update,
                (
                    handle_keyboard_shortcuts,
                    poll_file_data,
                    load_pending_model,
                    rotate_ambient_light,
                )
                    .run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                EguiPrimaryContextPass,
                upload_hud.run_if(in_state(GameState::Viewing)),
            );
    }
}

fn go_to_viewing(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Viewing);
}

#[derive(Resource)]
pub struct UploadState {
    pub model_handle: Option<Handle<Gltf>>,
    pub hud_open: bool,
    pub refraction: f32,
    pub skybox_brightness: f32,
}

impl Default for UploadState {
    fn default() -> Self {
        Self {
            model_handle: None,
            hud_open: true,
            refraction: 1.0,
            skybox_brightness: 1.0,
        }
    }
}

#[derive(Component)]
pub struct UploadSphere;

#[derive(Component)]
pub struct UploadModel;
