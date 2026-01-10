//! Holographic Light Beings - Rainbow particles and halos floating in the world

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

use crate::player::PlayerState;
use crate::world::{room_center, TOTAL_ROOMS};
use crate::GameState;

pub struct HolographicParticlesPlugin;

impl Plugin for HolographicParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Viewing), spawn_light_beings)
            .add_systems(
                Update,
                (animate_light_beings, animate_halos).run_if(in_state(GameState::Viewing)),
            );
    }
}

#[derive(Component)]
pub struct LightBeing {
    pub room: usize,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub vertical_speed: f32,
    pub phase: f32,
    pub hue_offset: f32,
}

#[derive(Component)]
pub struct Halo {
    pub room: usize,
    pub pulse_speed: f32,
    pub rotation_speed: f32,
    pub base_scale: f32,
}

fn spawn_light_beings(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    // Particle mesh - small sphere
    let particle_mesh = meshes.add(Sphere::new(0.08));
    // Halo mesh - thin torus
    let halo_mesh = meshes.add(Torus::new(0.4, 0.02));

    for room in 0..TOTAL_ROOMS {
        let center = room_center(room);

        // Spawn 12 light beings per room
        for i in 0..12 {
            let angle = (i as f32 / 12.0) * TAU;
            let radius = 3.0 + (i % 3) as f32 * 2.0;
            let height = -0.5 + (i % 4) as f32 * 0.8;
            let hue = (i as f32 / 12.0 + room as f32 * 0.33) % 1.0;

            let color = hue_to_rgb(hue);
            let emissive = LinearRgba::new(color.0 * 8.0, color.1 * 8.0, color.2 * 8.0, 1.0);

            let pos = center + Vec3::new(angle.cos() * radius, height, angle.sin() * radius - 8.0);

            cmd.spawn((
                Mesh3d(particle_mesh.clone()),
                MeshMaterial3d(mats.add(StandardMaterial {
                    base_color: Color::srgb(color.0, color.1, color.2),
                    emissive,
                    unlit: true,
                    alpha_mode: AlphaMode::Add,
                    ..default()
                })),
                Transform::from_translation(pos),
                RenderLayers::layer(room),
                LightBeing {
                    room,
                    orbit_radius: radius,
                    orbit_speed: 0.2 + (i % 5) as f32 * 0.1,
                    vertical_speed: 0.3 + (i % 3) as f32 * 0.15,
                    phase: angle,
                    hue_offset: hue,
                },
            ));
        }

        // Spawn 4 halos per room
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * TAU;
            let radius = 5.0 + (i % 2) as f32 * 3.0;
            let hue = (i as f32 / 4.0 + room as f32 * 0.25 + 0.5) % 1.0;

            let color = hue_to_rgb(hue);
            let emissive = LinearRgba::new(color.0 * 5.0, color.1 * 5.0, color.2 * 5.0, 1.0);

            let pos = center
                + Vec3::new(
                    angle.cos() * radius,
                    0.5 + (i % 2) as f32,
                    angle.sin() * radius - 8.0,
                );

            cmd.spawn((
                Mesh3d(halo_mesh.clone()),
                MeshMaterial3d(mats.add(StandardMaterial {
                    base_color: Color::srgba(color.0, color.1, color.2, 0.6),
                    emissive,
                    unlit: true,
                    alpha_mode: AlphaMode::Add,
                    ..default()
                })),
                Transform::from_translation(pos).with_rotation(Quat::from_rotation_x(PI * 0.5)),
                RenderLayers::layer(room),
                Halo {
                    room,
                    pulse_speed: 1.5 + (i % 3) as f32 * 0.5,
                    rotation_speed: 0.5 + (i % 2) as f32 * 0.3,
                    base_scale: 1.0 + (i % 2) as f32 * 0.5,
                },
            ));
        }
    }
    info!("✨ {} holographic light beings spawned", TOTAL_ROOMS * 16);
}

fn animate_light_beings(
    time: Res<Time>,
    player: Res<PlayerState>,
    mut beings: Query<(
        &mut Transform,
        &LightBeing,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let t = time.elapsed_secs();

    for (mut transform, being, mat_handle) in beings.iter_mut() {
        if being.room != player.room {
            continue;
        }

        let center = room_center(being.room);

        // Orbital motion
        let angle = being.phase + t * being.orbit_speed;
        let vertical = (t * being.vertical_speed + being.phase).sin() * 1.5;

        transform.translation = center
            + Vec3::new(
                angle.cos() * being.orbit_radius,
                vertical,
                angle.sin() * being.orbit_radius - 8.0,
            );

        // Pulsing scale
        let pulse = 0.8 + (t * 3.0 + being.phase).sin() * 0.3;
        transform.scale = Vec3::splat(pulse);

        // Shifting hue
        if let Some(mat) = mats.get_mut(&mat_handle.0) {
            let hue = (being.hue_offset + t * 0.1) % 1.0;
            let color = hue_to_rgb(hue);
            mat.emissive = LinearRgba::new(color.0 * 8.0, color.1 * 8.0, color.2 * 8.0, 1.0);
        }
    }
}

fn animate_halos(
    time: Res<Time>,
    player: Res<PlayerState>,
    mut halos: Query<(&mut Transform, &Halo, &MeshMaterial3d<StandardMaterial>)>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let t = time.elapsed_secs();

    for (mut transform, halo, mat_handle) in halos.iter_mut() {
        if halo.room != player.room {
            continue;
        }

        // Rotation
        let rot_y = t * halo.rotation_speed;
        let wobble = (t * 2.0).sin() * 0.2;
        transform.rotation = Quat::from_euler(EulerRot::XYZ, PI * 0.5 + wobble, rot_y, 0.0);

        // Pulsing scale
        let pulse = halo.base_scale + (t * halo.pulse_speed).sin() * 0.3;
        transform.scale = Vec3::splat(pulse);

        // Color shift
        if let Some(mat) = mats.get_mut(&mat_handle.0) {
            let hue = (t * 0.15) % 1.0;
            let color = hue_to_rgb(hue);
            mat.emissive = LinearRgba::new(color.0 * 6.0, color.1 * 6.0, color.2 * 6.0, 1.0);
        }
    }
}

/// Convert HSV hue (0-1) to RGB
fn hue_to_rgb(h: f32) -> (f32, f32, f32) {
    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - h.floor();

    match i % 6 {
        0 => (1.0, f, 0.0),
        1 => (1.0 - f, 1.0, 0.0),
        2 => (0.0, 1.0, f),
        3 => (0.0, 1.0 - f, 1.0),
        4 => (f, 0.0, 1.0),
        _ => (1.0, 0.0, 1.0 - f),
    }
}
