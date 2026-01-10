//! GLB Character module - Character spawning with dynamic lighting and breathing animation

use bevy::camera::visibility::RenderLayers;
use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::camera::CameraState;
use crate::ibl::IblLitModel;
use crate::loading::ModelAssets;
use crate::player::PlayerState;
use crate::world::{room_center, TOTAL_ROOMS};
use crate::GameState;

pub struct GlbCharacterPlugin;

impl Plugin for GlbCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterConfig::default())
            .add_systems(OnEnter(GameState::Viewing), spawn_characters)
            .add_systems(
                Update,
                (
                    mirage_illusion,
                    breathing_animation,
                    dynamic_character_lighting,
                )
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

#[derive(Resource)]
pub struct CharacterConfig {
    pub base_scale: f32,
    pub position: Vec2,
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            base_scale: 1.8,
            position: Vec2::new(0.0, -10.0),
        }
    }
}

#[derive(Component)]
pub struct RoomCharacter {
    pub room: usize,
    pub breath_phase: f32,
    pub current_scale: f32,
}

#[derive(Component)]
pub struct CharacterLight;

fn spawn_characters(
    mut cmd: Commands,
    models: Res<ModelAssets>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    config: Res<CharacterConfig>,
) {
    let mdl_handles = [&models.modelo1, &models.modelo2, &models.modelo3];

    for room in 0..TOTAL_ROOMS {
        let center = room_center(room);
        if let Some(gltf) = gltfs.get(mdl_handles[room]) {
            let char_pos = center + Vec3::new(config.position.x, -1.3, config.position.y);
            let transform = Transform::from_translation(char_pos)
                .with_scale(Vec3::splat(config.base_scale))
                .with_rotation(Quat::from_rotation_y(-PI / 2.0));

            let entity = spawn_gltf_model(&mut cmd, gltf, &gltf_meshes, transform, room);
            cmd.entity(entity).insert((
                RoomCharacter {
                    room,
                    breath_phase: room as f32 * 0.7,
                    current_scale: config.base_scale,
                },
                IblLitModel,
            ));

            // Dynamic point light for character
            cmd.spawn((
                PointLight {
                    color: Color::srgb(0.9, 0.8, 1.0),
                    intensity: 5000.0,
                    radius: 8.0,
                    shadows_enabled: false,
                    ..default()
                },
                Transform::from_translation(char_pos + Vec3::Y * 2.0),
                RenderLayers::layer(room),
                CharacterLight,
            ));
        }
    }
    info!(
        "👤 {} characters spawned with dynamic lighting",
        TOTAL_ROOMS
    );
}

fn spawn_gltf_model(
    cmd: &mut Commands,
    gltf: &Gltf,
    gltf_meshes: &Assets<GltfMesh>,
    transform: Transform,
    room: usize,
) -> Entity {
    let parent = cmd
        .spawn((transform, Visibility::default(), RenderLayers::layer(room)))
        .id();
    for gltf_mesh_handle in &gltf.meshes {
        if let Some(gltf_mesh) = gltf_meshes.get(gltf_mesh_handle) {
            for primitive in &gltf_mesh.primitives {
                let mat = primitive
                    .material
                    .clone()
                    .unwrap_or_else(|| gltf.materials[0].clone());
                let child = cmd
                    .spawn((
                        Mesh3d(primitive.mesh.clone()),
                        MeshMaterial3d(mat),
                        Transform::default(),
                        RenderLayers::layer(room),
                    ))
                    .id();
                cmd.entity(parent).add_child(child);
            }
        }
    }
    parent
}

fn mirage_illusion(
    player: Res<PlayerState>,
    config: Res<CharacterConfig>,
    mut chars: Query<&mut RoomCharacter>,
) {
    let dist = (player.pos - config.position).length();
    let max_dist = 14.0;
    let peak_dist = 7.0;
    let min_dist = 4.0;

    let target_scale = if dist > peak_dist {
        let t = ((max_dist - dist) / (max_dist - peak_dist)).clamp(0.0, 1.0);
        config.base_scale * (1.0 + t * 1.5)
    } else {
        let t = ((dist - min_dist) / (peak_dist - min_dist)).clamp(0.0, 1.0);
        config.base_scale * (0.5 + t * 2.0)
    };

    for mut ch in chars.iter_mut() {
        if ch.room == player.room {
            ch.current_scale = target_scale;
        }
    }
}

fn breathing_animation(time: Res<Time>, mut chars: Query<(&mut Transform, &mut RoomCharacter)>) {
    let dt = time.delta_secs();

    for (mut tr, mut ch) in chars.iter_mut() {
        ch.breath_phase += dt * 1.2;
        let breath = (ch.breath_phase).sin() * 0.08;
        let micro_pulse = (ch.breath_phase * 3.0).sin() * 0.02;
        let final_scale = ch.current_scale * (1.0 + breath + micro_pulse);
        tr.scale = Vec3::splat(final_scale);
    }
}

/// Dynamic lighting that responds to camera rotation
fn dynamic_character_lighting(
    camera_state: Res<CameraState>,
    player: Res<PlayerState>,
    time: Res<Time>,
    mut lights: Query<(&mut PointLight, &mut Transform), With<CharacterLight>>,
    config: Res<CharacterConfig>,
) {
    let t = time.elapsed_secs();
    let center = room_center(player.room);
    let char_pos = center + Vec3::new(config.position.x, 0.0, config.position.y);

    let orbit_radius = 3.0;
    let light_angle = camera_state.yaw + PI;
    let pitch_factor = 1.0 + camera_state.pitch * 0.5;
    let pulse = 1.0 + (t * 2.0).sin() * 0.15;

    for (mut light, mut tr) in lights.iter_mut() {
        tr.translation = char_pos
            + Vec3::new(
                light_angle.cos() * orbit_radius,
                2.5 + camera_state.pitch * 1.5,
                light_angle.sin() * orbit_radius,
            );
        light.intensity = 5000.0 * pitch_factor * pulse;
        let hue_shift = (camera_state.yaw * 0.1).sin() * 0.1;
        light.color = Color::srgb(0.9 + hue_shift, 0.8, 1.0 - hue_shift);
    }
}
