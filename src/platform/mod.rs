//! Platform abstraction - Desktop, VR, WebXR plugins with runtime detection

#[cfg(feature = "desktop")]
mod desktop;
#[cfg(feature = "vr")]
mod vr;
#[cfg(feature = "webxr")]
pub mod webxr;

#[cfg(feature = "desktop")]
pub use desktop::DesktopPlatformPlugin;
#[cfg(feature = "vr")]
pub use vr::VrPlatformPlugin;
#[cfg(feature = "webxr")]
pub use webxr::{WebXrPlatformPlugin, WebXrPose, WebXrState};

use crate::core::{Os, PlatformEntity};
use bevy::prelude::*;

/// Active platform type
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Platform {
    #[default]
    Desktop,
    Vr,
    WebXr,
}

impl Platform {
    /// Auto-detect best platform for current OS and features
    #[allow(unused_variables)]
    pub fn auto_detect(os: &Os) -> Self {
        #[cfg(feature = "vr")]
        if os.supports_vr() {
            return Platform::Vr;
        }

        #[cfg(all(feature = "webxr", target_arch = "wasm32"))]
        {
            return Platform::WebXr;
        }

        #[cfg(not(all(feature = "webxr", target_arch = "wasm32")))]
        Platform::Desktop
    }
}

/// Run condition: desktop platform
pub fn on_desktop(platform: Res<Platform>) -> bool {
    *platform == Platform::Desktop
}

/// Run condition: VR platform
pub fn on_vr(platform: Res<Platform>) -> bool {
    *platform == Platform::Vr
}

/// Run condition: WebXR platform
pub fn on_webxr(platform: Res<Platform>) -> bool {
    *platform == Platform::WebXr
}

/// Event to trigger platform switch
#[derive(Message)]
pub struct SwitchPlatform(pub Platform);

/// System to handle platform switching with cleanup
fn handle_platform_switch(
    mut commands: Commands,
    mut events: MessageReader<SwitchPlatform>,
    mut platform: ResMut<Platform>,
    platform_entities: Query<Entity, With<PlatformEntity>>,
) {
    for SwitchPlatform(new_platform) in events.read() {
        if *platform != *new_platform {
            info!(
                "🔄 Switching platform: {:?} -> {:?}",
                *platform, new_platform
            );

            // Cleanup old platform entities
            for entity in &platform_entities {
                commands.entity(entity).despawn();
            }

            *platform = *new_platform;
        }
    }
}

/// Base platform plugin - sets up detection and switching
pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        let os = app
            .world()
            .get_resource::<Os>()
            .copied()
            .unwrap_or_default();
        let platform = Platform::auto_detect(&os);

        app.insert_resource(platform)
            .add_message::<SwitchPlatform>()
            .add_systems(PreUpdate, handle_platform_switch);

        info!("🎮 Platform: {:?} (OS: {:?})", platform, os);
    }
}
