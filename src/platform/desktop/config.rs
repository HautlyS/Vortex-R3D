//! Desktop platform configuration

use bevy::prelude::*;

/// Desktop-specific settings
#[derive(Resource, Clone)]
pub struct DesktopConfig {
    pub auto_lock_cursor: bool,
    pub allow_fullscreen: bool,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            auto_lock_cursor: true,
            allow_fullscreen: true,
        }
    }
}
