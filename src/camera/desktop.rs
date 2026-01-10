//! Desktop camera - Transversal spin effect with CTRL+R

use bevy::prelude::*;
use std::f32::consts::PI;

use super::{CameraState, GameCamera};
use crate::input::InputEvent;
use crate::platform::on_desktop;

pub struct DesktopCameraPlugin;

impl Plugin for DesktopCameraPlugin {
    fn build(&self, app: &mut App) {
        use bevy::ecs::schedule::IntoScheduleConfigs;
        app.init_resource::<SpinEffect>().add_systems(
            Update,
            (
                handle_camera_input,
                handle_spin_trigger,
                update_spin_effect,
                apply_camera_transform,
            )
                .chain_ignore_deferred()
                .run_if(on_desktop),
        );
    }
}

/// Transversal spin effect state
#[derive(Resource)]
pub struct SpinEffect {
    pub active: bool,
    pub time: f32,
    pub duration: f32,
    pub yaw_speed: f32,
    pub pitch_speed: f32,
    pub roll_speed: f32,
    pub base_yaw: f32,
    pub base_pitch: f32,
    pub intensity: f32,
}

impl Default for SpinEffect {
    fn default() -> Self {
        Self {
            active: false,
            time: 0.0,
            duration: 3.0,
            yaw_speed: 0.0,
            pitch_speed: 0.0,
            roll_speed: 0.0,
            base_yaw: 0.0,
            base_pitch: 0.0,
            intensity: 0.0,
        }
    }
}

const SENSITIVITY_X: f32 = 0.003;
const SENSITIVITY_Y: f32 = 0.003;

fn handle_camera_input(
    mut events: MessageReader<InputEvent>,
    mut state: ResMut<CameraState>,
    spin: Res<SpinEffect>,
) {
    if spin.active {
        return;
    } // Disable manual control during spin

    for event in events.read() {
        match event {
            InputEvent::Look(delta) => {
                state.yaw -= delta.x * SENSITIVITY_X;
                state.pitch = (state.pitch - delta.y * SENSITIVITY_Y).clamp(-PI * 0.48, PI * 0.48);
            }
            InputEvent::AdjustFov(delta) => {
                state.fov = (state.fov + delta).clamp(50.0, 100.0);
            }
            _ => {}
        }
    }
}

fn handle_spin_trigger(
    keys: Res<ButtonInput<KeyCode>>,
    mut spin: ResMut<SpinEffect>,
    state: Res<CameraState>,
    time: Res<Time>,
) {
    // CTRL+R triggers transversal spin
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyR) && !spin.active {
        // Random spin directions
        let seed = time.elapsed_secs();
        spin.active = true;
        spin.time = 0.0;
        spin.base_yaw = state.yaw;
        spin.base_pitch = state.pitch;

        // Randomize spin velocities for transversal effect
        spin.yaw_speed = (seed * 17.3).sin() * 4.0 + 2.0; // 2-6 rad/s
        spin.pitch_speed = (seed * 23.7).cos() * 3.0; // -3 to 3 rad/s
        spin.roll_speed = (seed * 31.1).sin() * 2.0; // -2 to 2 rad/s

        info!("🌀 Transversal spin activated!");
    }

    // ESC cancels spin
    if keys.just_pressed(KeyCode::Escape) && spin.active {
        spin.active = false;
        spin.intensity = 0.0;
    }
}

fn update_spin_effect(
    mut spin: ResMut<SpinEffect>,
    mut state: ResMut<CameraState>,
    time: Res<Time>,
) {
    if !spin.active {
        // Decay intensity when not active
        spin.intensity = (spin.intensity - time.delta_secs() * 3.0).max(0.0);
        return;
    }

    let dt = time.delta_secs();
    spin.time += dt;

    // Easing: smooth start and end
    let progress = spin.time / spin.duration;
    let ease = if progress < 0.2 {
        // Ease in
        (progress / 0.2).powi(2)
    } else if progress > 0.8 {
        // Ease out
        let t = (progress - 0.8) / 0.2;
        1.0 - t.powi(2)
    } else {
        1.0
    };

    spin.intensity = ease;

    // Apply transversal rotation
    state.yaw = spin.base_yaw + spin.time * spin.yaw_speed * ease;

    // Oscillating pitch for vertical movement
    let pitch_wave = (spin.time * spin.pitch_speed * 2.0).sin() * 0.4 * ease;
    state.pitch = (spin.base_pitch + pitch_wave).clamp(-PI * 0.45, PI * 0.45);

    // End spin
    if spin.time >= spin.duration {
        spin.active = false;
        info!("🌀 Spin complete");
    }
}

fn apply_camera_transform(
    state: Res<CameraState>,
    spin: Res<SpinEffect>,
    mut camera_q: Query<(&mut Transform, &mut Projection), With<GameCamera>>,
) {
    let Ok((mut transform, mut projection)) = camera_q.get_single_mut() else {
        return;
    };

    // Base rotation
    let mut rotation = Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);

    // Add roll during spin for transversal effect
    if spin.intensity > 0.01 {
        let roll = (spin.time * spin.roll_speed * 3.0).sin() * 0.15 * spin.intensity;
        rotation *= Quat::from_rotation_z(roll);
    }

    transform.rotation = rotation;

    // FOV pulse during spin (creates zoom effect)
    if let Projection::Perspective(ref mut p) = *projection {
        let fov_pulse = spin.intensity * (spin.time * 8.0).sin() * 3.0;
        p.fov = (state.fov + fov_pulse).to_radians();
    }
}
