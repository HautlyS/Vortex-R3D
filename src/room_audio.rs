//! Room-based spatial audio with crossfade transitions
//! - Per-room soundtrack with reverb/echo effect
//! - Per-room narration (modelo(n).wav)
//! - Smooth crossfade when transitioning through portals

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

use crate::GameState;

pub struct RoomAudioPlugin;

impl Plugin for RoomAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<RoomAudioState>()
            .add_systems(OnEnter(GameState::Viewing), setup_room_audio)
            .add_systems(
                Update,
                (update_room_audio, handle_narration).run_if(in_state(GameState::Viewing)),
            );
    }
}

const FADE_DURATION: Duration = Duration::from_millis(1500);
const REVERB_PANNING: f32 = 0.15; // Slight stereo spread for echo effect

#[derive(Resource, Default)]
pub struct RoomAudioState {
    pub current_room: usize,
    prev_room: Option<usize>,
    soundtracks: [Option<Handle<AudioInstance>>; 3],
    narrations: [Option<Handle<AudioInstance>>; 3],
    narration_played: [bool; 3],
}

#[derive(Resource)]
pub struct AudioAssets {
    pub soundtracks: [Handle<AudioSource>; 3],
    pub narrations: [Handle<AudioSource>; 3],
}

fn setup_room_audio(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut state: ResMut<RoomAudioState>,
) {
    // Load audio assets
    let assets = AudioAssets {
        soundtracks: [
            asset_server.load("audio/modelo1.wav"),
            asset_server.load("audio/modelo2.wav"),
            asset_server.load("audio/modelo3.wav"),
        ],
        narrations: [
            asset_server.load("audio/modelo1.wav"),
            asset_server.load("audio/modelo2.wav"),
            asset_server.load("audio/modelo3.wav"),
        ],
    };

    // Start room 0 soundtrack with reverb effect
    let handle = audio
        .play(assets.soundtracks[0].clone())
        .looped()
        .with_volume(0.7)
        .fade_in(AudioTween::new(FADE_DURATION, AudioEasing::OutPowi(2)))
        .handle();
    state.soundtracks[0] = Some(handle);

    cmd.insert_resource(assets);
    info!("🎵 Room audio initialized");
}

fn update_room_audio(
    audio: Res<Audio>,
    assets: Option<Res<AudioAssets>>,
    mut state: ResMut<RoomAudioState>,
    mut instances: ResMut<Assets<AudioInstance>>,
    player_state: Option<Res<crate::player::PlayerState>>,
) {
    let Some(assets) = assets else { return };
    let Some(player) = player_state else { return };

    // Detect room change
    if player.room != state.current_room {
        let old_room = state.current_room;
        let new_room = player.room;
        state.prev_room = Some(old_room);
        state.current_room = new_room;

        // Fade out old room soundtrack
        if let Some(handle) = &state.soundtracks[old_room] {
            if let Some(instance) = instances.get_mut(handle) {
                instance.set_decibels(
                    -60.0,
                    AudioTween::new(FADE_DURATION, AudioEasing::InPowi(2)),
                );
            }
        }

        // Start or fade in new room soundtrack with spatial effect
        if let Some(handle) = &state.soundtracks[new_room] {
            if let Some(instance) = instances.get_mut(handle) {
                instance.set_decibels(
                    -3.0,
                    AudioTween::new(FADE_DURATION, AudioEasing::OutPowi(2)),
                );
            }
        } else {
            // First time entering this room - start soundtrack
            let panning = match new_room {
                0 => 0.0,             // Center
                1 => -REVERB_PANNING, // Slight left (echo effect)
                2 => REVERB_PANNING,  // Slight right
                _ => 0.0,
            };

            let handle = audio
                .play(assets.soundtracks[new_room].clone())
                .looped()
                .with_volume(-60.0)
                .with_panning(panning)
                .fade_in(AudioTween::new(FADE_DURATION, AudioEasing::OutPowi(2)))
                .handle();

            // Fade to target volume
            if let Some(instance) = instances.get_mut(&handle) {
                instance.set_decibels(
                    -3.0,
                    AudioTween::new(FADE_DURATION, AudioEasing::OutPowi(2)),
                );
            }
            state.soundtracks[new_room] = Some(handle);
        }

        info!(
            "🎵 Room {} → {} audio crossfade",
            old_room + 1,
            new_room + 1
        );
    }
}

fn handle_narration(
    keys: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    assets: Option<Res<AudioAssets>>,
    mut state: ResMut<RoomAudioState>,
    mut instances: ResMut<Assets<AudioInstance>>,
) {
    let Some(assets) = assets else { return };
    let room = state.current_room;

    // N key = play narration for current room (once per room visit)
    if keys.just_pressed(KeyCode::KeyN) && !state.narration_played[room] {
        // Stop any playing narration
        for handle in state.narrations.iter().flatten() {
            if let Some(instance) = instances.get_mut(handle) {
                let _ = instance.stop(AudioTween::new(
                    Duration::from_millis(500),
                    AudioEasing::Linear,
                ));
            }
        }

        // Play new narration with slight reverb panning
        let handle = audio
            .play(assets.narrations[room].clone())
            .with_volume(-1.0)
            .with_panning((room as f32 - 1.0) * 0.1) // Spatial positioning
            .fade_in(AudioTween::new(
                Duration::from_millis(300),
                AudioEasing::Linear,
            ))
            .handle();

        state.narrations[room] = Some(handle);
        state.narration_played[room] = true;
        info!("🎤 Playing narration for room {}", room + 1);
    }

    // Space = toggle narration pause
    if keys.just_pressed(KeyCode::Space) {
        if let Some(handle) = &state.narrations[room] {
            if let Some(instance) = instances.get_mut(handle) {
                match instance.state() {
                    PlaybackState::Playing { .. } => {
                        let _ = instance.pause(AudioTween::default());
                    }
                    PlaybackState::Paused { .. } => {
                        let _ = instance.resume(AudioTween::default());
                    }
                    _ => {}
                }
            }
        }
    }
}
