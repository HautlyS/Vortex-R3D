//! Upload Room - Cross-platform file upload for panoramas and 3D models

mod file_picker;
mod scene;
mod ui;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use crate::input::UiWantsPointer;
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
                PreUpdate,
                update_ui_wants_pointer.run_if(in_state(GameState::Viewing)),
            )
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

/// Update UiWantsPointer in PreUpdate so it's available before input handling
fn update_ui_wants_pointer(mut ctx: EguiContexts, mut ui_wants: ResMut<UiWantsPointer>) {
    if let Ok(egui_ctx) = ctx.ctx_mut() {
        ui_wants.0 = egui_ctx.wants_pointer_input() || egui_ctx.is_pointer_over_area();
    } else {
        ui_wants.0 = false;
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
