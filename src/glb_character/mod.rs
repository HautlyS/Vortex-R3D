//! GLB Character module - Character spawning with Avian physics and Gaussian Splat integration

use avian3d::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::camera::CameraState;
use crate::gaussian_splat::{GaussianSplat, GaussianSplatBundle, SplatPhysicsBuilder};
use crate::ibl::IblLitModel;
use crate::loading::ModelAssets;
use crate::player::{Player, PlayerState};
use crate::world::{room_center, TOTAL_ROOMS};
use crate::GameState;

pub struct GlbCharacterPlugin;

impl Plugin for GlbCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CharacterConfig::default())
            .add_systems(OnEnter(GameState::Viewing), spawn_characters)
            .add_systems(Update, mirage_illusion.run_if(in_state(GameState::Viewing)))
            .add_systems(
                Update,
                breathing_animation.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                dynamic_character_lighting.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                update_character_physics.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                sync_character_splats.run_if(in_state(GameState::Viewing)),
            );
    }
}

#[derive(Resource)]
pub struct CharacterConfig {
    pub base_scale: f32,
    pub position: Vec2,
    pub physics_enabled: bool,
    pub splat_density: f32,
}

impl Default for CharacterConfig {
    fn default() -> Self {
        Self {
            base_scale: 1.8,
            position: Vec2::new(0.0, -10.0),
            physics_enabled: true,
            splat_density: 0.5,
        }
    }
}

#[derive(Component)]
pub struct RoomCharacter {
    pub room: usize,
    pub breath_phase: f32,
    pub current_scale: f32,
    pub splat_entities: Vec<Entity>,
}

#[derive(Component)]
pub struct CharacterLight;

#[allow(dead_code)]
#[derive(Component)]
pub struct CharacterPhysics {
    pub base_mass: f32,
    pub height: f32,
    pub radius: f32,
}

impl Default for CharacterPhysics {
    fn default() -> Self {
        Self {
            base_mass: 70.0,
            height: 3.6,
            radius: 0.8,
        }
    }
}

fn spawn_characters(
    mut cmd: Commands,
    models: Res<ModelAssets>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    config: Res<CharacterConfig>,
) {
    let mdl_handles = [&models.modelo1, &models.modelo2, &models.modelo3];

    for (room, mdl_handle) in mdl_handles.iter().enumerate().take(TOTAL_ROOMS) {
        let center = room_center(room);
        if let Some(gltf) = gltfs.get(*mdl_handle) {
            let char_pos = center + Vec3::new(config.position.x, -1.3, config.position.y);
            let transform = Transform::from_translation(char_pos)
                .with_scale(Vec3::splat(config.base_scale))
                .with_rotation(Quat::from_rotation_y(-PI / 2.0));

            let entity = spawn_gltf_model(&mut cmd, gltf, &gltf_meshes, transform, room, &config);
            cmd.entity(entity).insert((
                RoomCharacter {
                    room,
                    breath_phase: room as f32 * 0.7,
                    current_scale: config.base_scale,
                    splat_entities: Vec::new(),
                },
                IblLitModel,
                CharacterPhysics::default(),
            ));

            if config.physics_enabled {
                let builder = SplatPhysicsBuilder::static_body()
                    .with_capsule(1.8, 0.8)
                    .with_density(10.0)
                    .with_friction(0.8);
                let (body, rb, collider, mass, fric, rest, linear_damp, angular_damp, locked) =
                    builder.build();
                cmd.entity(entity).insert((
                    body,
                    rb,
                    collider,
                    mass,
                    fric,
                    rest,
                    linear_damp,
                    angular_damp,
                    locked,
                ));
            }

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

            spawn_character_gaussian_splats(&mut cmd, char_pos, room, &config, entity);
        }
    }
    info!(
        "👤 {} characters spawned with physics and Gaussian Splats",
        TOTAL_ROOMS
    );
}

fn spawn_gltf_model(
    cmd: &mut Commands,
    gltf: &Gltf,
    gltf_meshes: &Assets<GltfMesh>,
    transform: Transform,
    room: usize,
    config: &CharacterConfig,
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
                    .or_else(|| gltf.materials.first().cloned());

                let mut entity_cmd = cmd.spawn((
                    Mesh3d(primitive.mesh.clone()),
                    Transform::default(),
                    RenderLayers::layer(room),
                ));

                if let Some(mat) = mat {
                    entity_cmd.insert(MeshMaterial3d(mat));
                }

                if config.physics_enabled {
                    let builder = SplatPhysicsBuilder::static_body().with_box(Vec3::splat(0.5));
                    let (body, rb, collider, mass, fric, rest, linear_damp, angular_damp, locked) =
                        builder.build();
                    entity_cmd.insert((
                        body,
                        rb,
                        collider,
                        mass,
                        fric,
                        rest,
                        linear_damp,
                        angular_damp,
                        locked,
                    ));
                }

                let child = entity_cmd.id();
                cmd.entity(parent).add_child(child);
            }
        }
    }
    parent
}

fn spawn_character_gaussian_splats(
    commands: &mut Commands,
    position: Vec3,
    room: usize,
    config: &CharacterConfig,
    _parent: Entity,
) -> Vec<Entity> {
    let mut splat_entities = Vec::new();
    let splat_count = (100.0 * config.splat_density) as usize;

    for i in 0..splat_count {
        let angle = (i as f32 / splat_count as f32) * PI * 2.0;
        let radius = 1.5 + (i as f32 * 0.01);
        let height_offset = (i as f32 * 0.05) % 3.6;

        let splat_pos =
            position + Vec3::new(angle.cos() * radius, height_offset, angle.sin() * radius);

        let color = Color::srgba(
            0.8 + (angle.sin() * 0.2),
            0.6 + (angle.cos() * 0.2),
            0.9,
            0.7,
        );

        let builder = SplatPhysicsBuilder::dynamic_body()
            .with_sphere(0.08)
            .with_density(50.0)
            .with_damping(2.0, 2.0);
        let (body, rb, collider, mass, fric, rest, linear_damp, angular_damp, locked) =
            builder.build();

        let splat_entity = commands
            .spawn((
                GaussianSplatBundle::new(splat_pos, color),
                RenderLayers::layer(room),
                body,
                rb,
                collider,
                mass,
                fric,
                rest,
                linear_damp,
                angular_damp,
                locked,
            ))
            .id();

        splat_entities.push(splat_entity);
    }

    splat_entities
}

fn mirage_illusion(
    player: Res<PlayerState>,
    config: Res<CharacterConfig>,
    mut chars: Query<(&mut RoomCharacter, &Transform)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut ch, char_transform) in chars.iter_mut() {
        if ch.room != player.room {
            continue;
        }

        let dist = player_transform
            .translation
            .distance(char_transform.translation);
        let max_dist: f32 = 14.0;
        let peak_dist: f32 = 7.0;
        let min_dist: f32 = 4.0;

        let target_scale = if dist > peak_dist {
            let t = ((max_dist - dist) / (max_dist - peak_dist)).clamp(0.0, 1.0);
            config.base_scale * (1.0 + t * 1.5)
        } else {
            let t = ((dist - min_dist) / (peak_dist - min_dist)).clamp(0.0, 1.0);
            config.base_scale * (0.5 + t * 2.0)
        };

        ch.current_scale = target_scale;
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

fn update_character_physics(
    time: Res<Time>,
    mut char_query: Query<(
        &mut LinearVelocity,
        &Transform,
        &RoomCharacter,
        &CharacterPhysics,
    )>,
) {
    for (mut velocity, _transform, character, _physics) in char_query.iter_mut() {
        let breathing_force = (character.breath_phase.sin() * 2.0).clamp(-1.0, 1.0);

        velocity.0 += Vec3::Y * breathing_force * time.delta_secs() * 0.1;
        velocity.0 = velocity.0.clamp_length_max(0.5);
    }
}

fn sync_character_splats(
    mut splat_query: Query<(&mut GaussianSplat, &mut Transform), Without<RoomCharacter>>,
    char_query: Query<(&Transform, &RoomCharacter), Changed<Transform>>,
) {
    for (char_transform, character) in char_query.iter() {
        for &splat_entity in &character.splat_entities {
            if let Ok((mut splat, mut splat_transform)) = splat_query.get_mut(splat_entity) {
                let _offset = splat_transform.translation - char_transform.translation;
                splat_transform.scale = Vec3::ONE * 0.08 * char_transform.scale.x;
                splat.solidity = (char_transform.scale.x / 2.0).clamp(0.0, 1.0);
            }
        }
    }
}
