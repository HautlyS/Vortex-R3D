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
mod js_bridge;
mod loading;
mod panorama;
#[cfg(feature = "particles")]
mod particles;
mod performance;
mod platform;
mod player;
mod portals;
mod post_process;
mod room_audio;
mod routes;
mod upload_room;
mod vortex_transition;
mod world;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// Re-exports
pub use book_reader::BookReaderPlugin;
pub use camera::{CameraPlugin, CameraState, GameCamera};
pub use character::CharacterPlugin;
pub use core::{CorePlugin, DesktopOnly, Os, PlatformEntity, VrOnly};
#[cfg(feature = "particles")]
pub use energy_particles::EnergyParticlesPlugin;
pub use glb_character::GlbCharacterPlugin;
pub use holographic::HolographicParticlesPlugin;
pub use ibl::IblPlugin;
pub use input::{InputEvent, InputPlugin, InputState, UiWantsPointer};
pub use loading::LoadingPlugin;
pub use panorama::PanoramaPlugin;
#[cfg(feature = "particles")]
pub use particles::GpuParticlesPlugin;
pub use performance::{
    spawn_fps_overlay, update_fps_overlay, FpsMonitor, FpsOverlay, PerformancePlugin,
    QualityLevel, QualitySettings,
};
pub use platform::{on_desktop, on_vr, on_webxr, Platform, PlatformPlugin, SwitchPlatform};
pub use player::PlayerPlugin;
pub use portals::PortalsPlugin;
pub use post_process::PostProcessPlugin;
pub use room_audio::RoomAudioPlugin;
pub use routes::{get_app_mode, AppMode};
pub use upload_room::UploadRoomPlugin;
pub use vortex_transition::VortexTransitionPlugin;
pub use world::WorldPlugin;

#[cfg(feature = "webxr")]
pub use camera::WebXrCameraPlugin;
#[cfg(feature = "webxr")]
pub use input::WebXrInputPlugin;
#[cfg(feature = "desktop")]
pub use platform::DesktopPlatformPlugin;
#[cfg(feature = "vr")]
pub use platform::VrPlatformPlugin;
#[cfg(feature = "webxr")]
pub use platform::WebXrPlatformPlugin;

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

        // JS Bridge FIRST - needed for loading state communication
        app.add_plugins(js_bridge::JsBridgePlugin);

        // Performance system - other plugins depend on QualitySettings
        app.add_plugins(PerformancePlugin);

        // Shared plugins for all modes
        app.add_plugins((CorePlugin, InputPlugin, CameraPlugin));

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

                // GPU particles (desktop only)
                #[cfg(feature = "particles")]
                app.add_plugins((EnergyParticlesPlugin, GpuParticlesPlugin));

                info!("🏠 Full Experience Mode (Room 1)");
            }
            AppMode::UploadRoom => {
                app.add_plugins(UploadRoomPlugin);
                info!("📤 Upload Room Mode");
            }
        }

        #[cfg(debug_assertions)]
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ));
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
