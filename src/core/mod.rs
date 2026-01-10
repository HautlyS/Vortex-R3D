//! Core systems - OS detection, shared types, platform lifecycle

use bevy::prelude::*;

/// Detected operating system at runtime
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Os {
    Windows,
    MacOs,
    Linux,
    Android,
    Ios,
    Web,
    Unknown,
}

impl Default for Os {
    fn default() -> Self {
        Self::detect()
    }
}

impl Os {
    /// Detect OS at runtime
    pub fn detect() -> Self {
        #[cfg(target_os = "windows")]
        return Os::Windows;
        #[cfg(target_os = "macos")]
        return Os::MacOs;
        #[cfg(target_os = "linux")]
        return Os::Linux;
        #[cfg(target_os = "android")]
        return Os::Android;
        #[cfg(target_os = "ios")]
        return Os::Ios;
        #[cfg(target_arch = "wasm32")]
        return Os::Web;
        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
            target_os = "android",
            target_os = "ios",
            target_arch = "wasm32"
        )))]
        return Os::Unknown;
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, Os::Windows | Os::MacOs | Os::Linux)
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Os::Android | Os::Ios)
    }

    pub fn supports_vr(&self) -> bool {
        matches!(self, Os::Windows | Os::Linux | Os::Android)
    }
}

/// Marker for platform-specific entities (despawned on platform change)
#[derive(Component)]
pub struct PlatformEntity;

/// Marker for desktop-only entities
#[derive(Component)]
pub struct DesktopOnly;

/// Marker for VR-only entities
#[derive(Component)]
pub struct VrOnly;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        let os = Os::detect();
        info!("🖥️ Detected OS: {:?}", os);
        app.insert_resource(os);
    }
}
