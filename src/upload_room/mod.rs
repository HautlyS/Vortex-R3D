//! Upload Room - Cross-platform file upload for panoramas and 3D models

mod file_picker;
mod scene;
mod ui;

use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};

use crate::input::UiWantsPointer;
use crate::js_bridge::{hide_loading_overlay, JsBridgeState};
use crate::GameState;
use file_picker::{load_pending_model, poll_file_data};
use scene::{rotate_ambient_light, setup_upload_room};
use ui::{handle_keyboard_shortcuts, upload_hud};

pub struct UploadRoomPlugin;

impl Plugin for UploadRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<UploadState>()
            .init_resource::<EguiReady>()
            .add_systems(Startup, go_to_viewing)
            .add_systems(
                OnEnter(GameState::Viewing),
                (setup_upload_room, hide_html_loading),
            )
            .add_systems(
                Update,
                (
                    mark_egui_ready,
                    update_ui_wants_pointer,
                    upload_hud.run_if(|ready: Res<EguiReady>| ready.0 >= 2),
                    handle_keyboard_shortcuts,
                    poll_file_data,
                    load_pending_model,
                    rotate_ambient_light,
                )
                    .chain()
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

/// Tracks if egui has been initialized (fonts ready after first frame)
#[derive(Resource, Default)]
pub struct EguiReady(pub u8);

fn mark_egui_ready(mut ready: ResMut<EguiReady>) {
    // Increment counter - egui is ready after frame 1 (fonts initialized after first Context::run)
    if ready.0 < 2 {
        ready.0 += 1;
    }
}

fn update_ui_wants_pointer(
    mut ctx: EguiContexts,
    mut ui_wants: ResMut<UiWantsPointer>,
    ready: Res<EguiReady>,
) {
    if ready.0 < 2 {
        ui_wants.0 = false;
        return;
    }
    let Ok(egui_ctx) = ctx.ctx_mut() else {
        ui_wants.0 = false;
        return;
    };
    ui_wants.0 = egui_ctx.wants_pointer_input() || egui_ctx.is_pointer_over_area();
}

fn go_to_viewing(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Viewing);
}

fn hide_html_loading(mut bridge: ResMut<JsBridgeState>) {
    if !bridge.loading_hidden {
        hide_loading_overlay();
        bridge.loading_hidden = true;
        info!("📤 Upload room ready - HTML loading hidden");
    }
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
