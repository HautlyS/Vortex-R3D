//! World module - Room setup, skyboxes, lighting, rotation effects

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::loading::PanoramaAssets;
use crate::player::PlayerState;
use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldConfig::default())
            .insert_resource(SkyboxRotation::default())
            .add_systems(OnEnter(GameState::Viewing), setup_world)
            .add_systems(
                Update,
                (skybox_rotation_input, rotate_skybox)
                    .chain()
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

pub const ROOM_OFFSET: f32 = 500.0;
pub const TOTAL_ROOMS: usize = 3;

#[derive(Resource)]
pub struct WorldConfig {
    pub sky_sphere_radius: f32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            sky_sphere_radius: 80.0,
        }
    }
}

/// Skybox rotation state - Ctrl+R to toggle, Ctrl+1/2/3/4 for speed
#[derive(Resource)]
pub struct SkyboxRotation {
    pub enabled: bool,
    pub speed: f32,      // radians per second
    pub angle: f32,      // current rotation angle
    pub speed_level: u8, // 1-4
}

impl Default for SkyboxRotation {
    fn default() -> Self {
        Self {
            enabled: false,
            speed: 0.2,
            angle: 0.0,
            speed_level: 1,
        }
    }
}

#[derive(Component)]
pub struct Skybox {
    pub room: usize,
}

pub fn room_center(room: usize) -> Vec3 {
    Vec3::new(room as f32 * ROOM_OFFSET, 0.0, 0.0)
}

fn setup_world(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    pano: Res<PanoramaAssets>,
    config: Res<WorldConfig>,
) {
    let panos = [
        &pano.demo_panorama,
        &pano.demo2_panorama,
        &pano.demo3_panorama,
    ];
    let sky_mesh = meshes.add(create_sky_sphere(config.sky_sphere_radius));

    // Enhanced lighting for modern look
    cmd.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 600.0,
            ..default()
        },
        RenderLayers::from_layers(&[0, 1, 2]),
    ));
    cmd.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.98, 0.95),
            illuminance: 8000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[0, 1, 2]),
    ));

    // Skyboxes for each room
    for (room, pano) in panos.iter().enumerate().take(TOTAL_ROOMS) {
        let center = room_center(room);
        cmd.spawn((
            Mesh3d(sky_mesh.clone()),
            MeshMaterial3d(mats.add(StandardMaterial {
                base_color_texture: Some((*pano).clone()),
                unlit: true,
                double_sided: true,
                cull_mode: None,
                ..default()
            })),
            Transform::from_translation(center),
            RenderLayers::layer(room),
            Skybox { room },
        ));
    }
    info!("🌍 World: {} rooms created", TOTAL_ROOMS);
}

fn skybox_rotation_input(keys: Res<ButtonInput<KeyCode>>, mut rotation: ResMut<SkyboxRotation>) {
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

    // Ctrl+R toggles rotation
    if ctrl && keys.just_pressed(KeyCode::KeyR) {
        rotation.enabled = !rotation.enabled;
        info!(
            "🌀 Skybox rotation: {}",
            if rotation.enabled { "ON" } else { "OFF" }
        );
    }

    // Ctrl+1/2/3/4 sets speed
    if ctrl {
        if keys.just_pressed(KeyCode::Digit1) {
            rotation.speed_level = 1;
            rotation.speed = 0.1;
            info!("🌀 Rotation speed: 1 (slow)");
        }
        if keys.just_pressed(KeyCode::Digit2) {
            rotation.speed_level = 2;
            rotation.speed = 0.3;
            info!("🌀 Rotation speed: 2 (medium)");
        }
        if keys.just_pressed(KeyCode::Digit3) {
            rotation.speed_level = 3;
            rotation.speed = 0.6;
            info!("🌀 Rotation speed: 3 (fast)");
        }
        if keys.just_pressed(KeyCode::Digit4) {
            rotation.speed_level = 4;
            rotation.speed = 1.2;
            info!("🌀 Rotation speed: 4 (very fast)");
        }
    }
}

fn rotate_skybox(
    time: Res<Time>,
    mut rotation: ResMut<SkyboxRotation>,
    player: Res<PlayerState>,
    mut skyboxes: Query<(&mut Transform, &Skybox)>,
) {
    if !rotation.enabled {
        return;
    }

    rotation.angle += rotation.speed * time.delta_secs();
    if rotation.angle > PI * 2.0 {
        rotation.angle -= PI * 2.0;
    }

    for (mut transform, skybox) in skyboxes.iter_mut() {
        if skybox.room == player.room {
            let center = room_center(skybox.room);
            transform.translation = center;
            transform.rotation = Quat::from_rotation_y(rotation.angle);
        }
    }
}

pub fn create_sky_sphere(r: f32) -> Mesh {
    use bevy::mesh::{Indices, PrimitiveTopology};
    let (sec, stk) = (96u32, 48u32); // Higher resolution for smoother look
    let mut pos = Vec::new();
    let mut nrm = Vec::new();
    let mut uv = Vec::new();
    let mut idx = Vec::new();

    for i in 0..=stk {
        let v = i as f32 / stk as f32;
        let phi = PI * v;
        for j in 0..=sec {
            let u = j as f32 / sec as f32;
            let th = 2.0 * PI * u;
            let (x, y, z) = (
                r * phi.sin() * th.cos(),
                r * phi.cos(),
                r * phi.sin() * th.sin(),
            );
            pos.push([x, y, z]);
            nrm.push([-x / r, -y / r, -z / r]);
            uv.push([1.0 - u, v]);
        }
    }
    for i in 0..stk {
        for j in 0..sec {
            let a = i * (sec + 1) + j;
            let b = a + sec + 1;
            idx.extend([a, a + 1, b, b, a + 1, b + 1]);
        }
    }
    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nrm)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
        .with_inserted_indices(Indices::U32(idx))
}
