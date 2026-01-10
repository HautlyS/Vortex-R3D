use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, _app: &mut App) {
        // Characters are now managed by portal_doors.rs
        // This plugin is kept for future character-specific features
    }
}
