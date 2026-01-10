//! Post-Processing with Spin Motion Blur

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

use crate::camera::{CameraState, GameCamera};
use crate::ibl::IblLightProbe;
use crate::GameState;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpinBlurState>()
            .init_resource::<EnvironmentMood>()
            .add_systems(OnEnter(GameState::Viewing), setup_post_process)
            .add_systems(
                Update,
                (detect_environment_mood, update_spin_blur, apply_effects)
                    .chain()
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

#[derive(Resource, Default)]
pub struct SpinBlurState {
    pub prev_yaw: f32,
    pub prev_pitch: f32,
    pub angular_velocity: f32,
    pub blur_intensity: f32,
}

#[derive(Resource)]
pub struct EnvironmentMood {
    pub warmth: f32,
    pub light_intensity: f32,
    pub is_night: bool,
}

impl Default for EnvironmentMood {
    fn default() -> Self {
        Self {
            warmth: 0.5,
            light_intensity: 1.0,
            is_night: false,
        }
    }
}

fn setup_post_process(mut commands: Commands, cameras: Query<Entity, With<GameCamera>>) {
    for entity in cameras.iter() {
        commands.entity(entity).insert(Tonemapping::AcesFitted);
    }
    info!("✨ Dream post-processing initialized");
}

fn detect_environment_mood(ibl: Option<Res<IblLightProbe>>, mut mood: ResMut<EnvironmentMood>) {
    let Some(ibl) = ibl else { return };
    if !ibl.analyzed || !ibl.is_changed() {
        return;
    };

    let color = ibl.dominant_light_color.to_srgba();
    let r = color.red;
    let g = color.green;
    let b = color.blue;

    mood.warmth = ((r - b) * 0.5 + 0.5).clamp(0.0, 1.0);
    mood.light_intensity = (r + g + b) / 3.0;
    mood.is_night = mood.light_intensity < 0.3;

    if mood.is_night {
        info!("🌙 Night detected");
    } else if mood.warmth > 0.6 {
        info!(
            "☀️ Daylight detected (warmth={:.2}, intensity={:.2})",
            mood.warmth, mood.light_intensity
        );
    }
}

fn update_spin_blur(
    camera_state: Res<CameraState>,
    mut blur: ResMut<SpinBlurState>,
    time: Res<Time>,
) {
    let dt = time.delta_secs().max(0.001);

    // Calculate angular velocity
    let yaw_delta = (camera_state.yaw - blur.prev_yaw).abs();
    let pitch_delta = (camera_state.pitch - blur.prev_pitch).abs();
    let velocity = (yaw_delta + pitch_delta) / dt;

    // Smooth velocity
    blur.angular_velocity = blur.angular_velocity.lerp(velocity, dt * 5.0);

    // Convert to blur intensity (higher velocity = more blur)
    let target_blur = (blur.angular_velocity * 0.15).clamp(0.0, 1.0);
    blur.blur_intensity = blur.blur_intensity.lerp(target_blur, dt * 8.0);

    blur.prev_yaw = camera_state.yaw;
    blur.prev_pitch = camera_state.pitch;
}

fn apply_effects(
    camera_state: Res<CameraState>,
    blur: Res<SpinBlurState>,
    mood: Res<EnvironmentMood>,
    mut camera_q: Query<&mut Projection, With<GameCamera>>,
    mut ambient: ResMut<AmbientLight>,
) {
    let Ok(mut projection) = camera_q.get_single_mut() else {
        return;
    };

    // FOV responds to spin (slight zoom during fast rotation)
    if let Projection::Perspective(ref mut p) = *projection {
        let blur_fov = blur.blur_intensity * 5.0;
        p.fov = (camera_state.fov + blur_fov).to_radians();
    }

    // Ambient light based on mood
    let warm_color = Color::srgb(1.0, 0.9, 0.8);
    let cool_color = Color::srgb(0.8, 0.85, 1.0);

    ambient.color = if mood.is_night {
        Color::srgb(0.6, 0.65, 0.9)
    } else {
        warm_color.mix(&cool_color, 1.0 - mood.warmth)
    };
}
