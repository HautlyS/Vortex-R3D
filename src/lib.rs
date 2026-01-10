#![allow(clippy::type_complexity)]

// Core modules
mod book_reader;
mod camera;
mod character;
mod core;
#[cfg(feature = "particles")]
mod energy_particles;
mod glb_character;
mod holographic;
mod ibl;
mod input;
mod loading;
mod panorama;
mod platform;
mod player;
mod portals;
mod post_process;
mod room_audio;
mod routes;
mod upload_room;
mod vortex_transition;
mod world;

use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

// Re-exports
pub use book_reader::BookReaderPlugin;
pub use camera::{CameraPlugin, CameraState, GameCamera};
pub use character::CharacterPlugin;
pub use core::{CorePlugin, Os, PlatformEntity, DesktopOnly, VrOnly};
#[cfg(feature = "particles")]
pub use energy_particles::EnergyParticlesPlugin;
pub use glb_character::GlbCharacterPlugin;
pub use holographic::HolographicParticlesPlugin;
pub use ibl::IblPlugin;
pub use input::{InputPlugin, InputEvent, InputState};
pub use loading::LoadingPlugin;
pub use panorama::PanoramaPlugin;
pub use platform::{Platform, PlatformPlugin, SwitchPlatform, on_desktop, on_vr, on_webxr};
pub use player::PlayerPlugin;
pub use portals::PortalsPlugin;
pub use post_process::PostProcessPlugin;
pub use room_audio::RoomAudioPlugin;
pub use routes::{AppMode, get_app_mode};
pub use upload_room::UploadRoomPlugin;
pub use vortex_transition::VortexTransitionPlugin;
pub use world::WorldPlugin;

#[cfg(feature = "desktop")]
pub use platform::DesktopPlatformPlugin;
#[cfg(feature = "vr")]
pub use platform::VrPlatformPlugin;
#[cfg(feature = "webxr")]
pub use platform::WebXrPlatformPlugin;
#[cfg(feature = "webxr")]
pub use camera::WebXrCameraPlugin;
#[cfg(feature = "webxr")]
pub use input::WebXrInputPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Viewing,
}

/// Core game plugin - shared logic for all platforms
pub struct GameCorePlugin;

impl Plugin for GameCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();

        let mode = get_app_mode();

        // Shared plugins for all modes
        app.add_plugins((
            CorePlugin,
            InputPlugin,
            CameraPlugin,
        ));

        match mode {
            AppMode::FullExperience => {
                app.add_plugins((
                    LoadingPlugin,
                    PanoramaPlugin,
                    CharacterPlugin,
                    VortexTransitionPlugin,
                    IblPlugin,
                    WorldPlugin,
                    PlayerPlugin,
                    PortalsPlugin,
                    GlbCharacterPlugin,
                    HolographicParticlesPlugin,
                    RoomAudioPlugin,
                    BookReaderPlugin,
                    PostProcessPlugin,
                ));
                info!("🏠 Full Experience Mode (Room 1)");
            }
            AppMode::UploadRoom => {
                app.add_plugins(UploadRoomPlugin);
                info!("📤 Upload Room Mode");
            }
        }

        #[cfg(debug_assertions)]
        app.add_plugins((FrameTimeDiagnosticsPlugin::default(), LogDiagnosticsPlugin::default()));
    }
}

/// Desktop game plugin
#[cfg(feature = "desktop")]
pub struct GamePlugin;

#[cfg(feature = "desktop")]
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCorePlugin, DesktopPlatformPlugin));
    }
}

/// VR game plugin
#[cfg(feature = "vr")]
pub struct VrGamePlugin;

#[cfg(feature = "vr")]
impl Plugin for VrGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameCorePlugin, VrPlatformPlugin));
    }
}

/// WebXR game plugin
#[cfg(feature = "webxr")]
pub struct WebXrGamePlugin;

#[cfg(feature = "webxr")]
impl Plugin for WebXrGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameCorePlugin,
            WebXrPlatformPlugin,
            WebXrCameraPlugin,
            WebXrInputPlugin,
        ));
    }
}
