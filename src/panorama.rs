//! Panorama Light Orb - Optimized with LOD, quality awareness, and smart visibility
//! 2026 WASM-optimized: reduced draw calls, batched updates, distance-based detail

use crate::camera::GameCamera;
use crate::performance::QualitySettings;
use crate::GameState;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct PanoramaPlugin;

impl Plugin for PanoramaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PanoramaState>()
            .add_systems(OnEnter(GameState::Viewing), setup_panorama)
            .add_systems(
                Update,
                (animate_core, animate_orbs_batched, animate_lights)
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

pub use crate::camera::GameCamera as PanoramaCamera;

/// Shared state for batched updates
#[derive(Resource, Default)]
struct PanoramaState {
    last_material_update: f32,
}

#[derive(Component)]
pub struct CoreGlow {
    pub layer: u32,
}

#[derive(Component)]
pub struct LightOrb {
    pub angle: f32,
    pub speed: f32,
    pub radius: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct EnergyWisp {
    pub id: u32,
}

#[derive(Component)]
pub struct OrbPointLight {
    pub base: f32,
    pub phase: f32,
}

const ORB_Y: f32 = -0.65;

fn setup_panorama(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    quality: Res<QualitySettings>,
) {
    // Quality-based mesh detail
    let core_detail = match quality.level {
        crate::performance::QualityLevel::Ultra | crate::performance::QualityLevel::High => 2,
        crate::performance::QualityLevel::Medium => 1,
        _ => 0,
    };

    // === PURE LIGHT ORB - Core ===
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.02).mesh().ico(core_detail).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: LinearRgba::new(50.0, 45.0, 60.0, 1.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, ORB_Y, 0.0),
        CoreGlow { layer: 0 },
    ));

    // Glow layers - reduced on low quality
    let glow_layers = quality.particle_count(4);
    for i in 1..=glow_layers {
        let size = 0.03 + i as f32 * 0.04;
        let alpha = 0.15 - i as f32 * 0.03;
        let emission = (20.0 - i as f32 * 4.0) * quality.effect_intensity;

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(size).mesh().ico(core_detail).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.7, 0.5, 1.0, alpha),
                emissive: LinearRgba::new(emission * 0.8, emission * 0.5, emission, 1.0),
                unlit: true,
                alpha_mode: AlphaMode::Add,
                ..default()
            })),
            Transform::from_xyz(0.0, ORB_Y, 0.0),
            CoreGlow { layer: i as u32 },
        ));
    }

    // === ORBITING LIGHT POINTS - Quality scaled ===
    let orb_count = quality.particle_count(16);
    let tiny_sphere = meshes.add(Sphere::new(0.008).mesh().ico(0).unwrap());

    for i in 0..orb_count {
        let angle = (i as f32 / orb_count as f32) * PI * 2.0;
        let radius = 0.08 + (i % 3) as f32 * 0.03;
        let speed = 2.0 + (i % 4) as f32 * 0.5;
        let brightness = (25.0 + (i % 5) as f32 * 5.0) * quality.effect_intensity;

        commands.spawn((
            Mesh3d(tiny_sphere.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: LinearRgba::new(brightness * 0.9, brightness * 0.7, brightness, 1.0),
                unlit: true,
                ..default()
            })),
            Transform::from_xyz(0.0, ORB_Y, 0.0)
                .with_scale(Vec3::splat(0.5 + (i % 3) as f32 * 0.3)),
            LightOrb {
                angle,
                speed,
                radius,
                phase: i as f32 * 0.4,
            },
        ));
    }

    // === ENERGY WISPS - Quality scaled ===
    let wisp_count = quality.particle_count(12);
    let wisp_mesh = meshes.add(create_wisp_mesh());

    for i in 0..wisp_count {
        let brightness = (30.0 + (i % 4) as f32 * 8.0) * quality.effect_intensity;

        commands.spawn((
            Mesh3d(wisp_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.9, 1.0, 0.8),
                emissive: LinearRgba::new(brightness, brightness * 0.8, brightness * 1.2, 1.0),
                unlit: true,
                alpha_mode: AlphaMode::Add,
                ..default()
            })),
            Transform::from_xyz(0.0, ORB_Y, 0.0).with_scale(Vec3::splat(0.3)),
            EnergyWisp { id: i as u32 },
        ));
    }

    // === POINT LIGHTS - Quality limited ===
    let max_lights = quality.max_lights.min(5) as usize;

    if max_lights > 0 {
        commands.spawn((
            PointLight {
                color: Color::srgb(0.85, 0.7, 1.0),
                intensity: 50000.0,
                radius: 5.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(0.0, ORB_Y, 0.0),
            OrbPointLight {
                base: 50000.0,
                phase: 0.0,
            },
        ));
    }

    for i in 0..(max_lights.saturating_sub(1).min(4)) {
        let angle = (i as f32 / 4.0) * PI * 2.0;
        commands.spawn((
            PointLight {
                color: Color::srgb(0.6, 0.4, 1.0),
                intensity: 8000.0,
                radius: 2.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(angle.cos() * 0.1, ORB_Y, angle.sin() * 0.1),
            OrbPointLight {
                base: 8000.0,
                phase: i as f32 * 1.5,
            },
        ));
    }

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Tonemapping::AcesFitted,
        GameCamera,
    ));

    info!(
        "🔮 Light Orb ready (quality: {:?}, {} orbs, {} wisps)",
        quality.level, orb_count, wisp_count
    );
}

/// Batched core animation - all cores in one system
fn animate_core(
    time: Res<Time>,
    mut cores: Query<(&mut Transform, &CoreGlow)>,
    cam_q: Query<&Transform, (With<PanoramaCamera>, Without<CoreGlow>)>,
) {
    let t = time.elapsed_secs();
    let cam_yaw = cam_q
        .single()
        .map(|c| c.rotation.to_euler(EulerRot::YXZ).0)
        .unwrap_or(0.0);

    for (mut transform, core) in cores.iter_mut() {
        let phase = t * 3.0 + core.layer as f32 * 0.5;
        let pulse = 1.0 + phase.sin() * 0.15;
        let micro_pulse = 1.0 + (t * 12.0).sin() * 0.05;

        transform.scale = Vec3::splat(pulse * micro_pulse);

        let drift_x = (t * 1.5 + core.layer as f32).sin() * 0.003 + cam_yaw.sin() * 0.002;
        let drift_z = (t * 1.3 + core.layer as f32).sin() * 0.003 + cam_yaw.cos() * 0.002;

        transform.translation.x = drift_x;
        transform.translation.y = ORB_Y + (t * 1.8 + core.layer as f32).cos() * 0.003;
        transform.translation.z = drift_z;
    }
}

/// Batched orb + wisp animation - single iteration
fn animate_orbs_batched(
    time: Res<Time>,
    quality: Res<QualitySettings>,
    mut state: ResMut<PanoramaState>,
    cam_q: Query<&Transform, (With<PanoramaCamera>, Without<LightOrb>, Without<EnergyWisp>)>,
    mut orbs: Query<(&mut Transform, &mut LightOrb), Without<EnergyWisp>>,
    mut wisps: Query<(&mut Transform, &mut Visibility, &EnergyWisp), Without<LightOrb>>,
) {
    let t = time.elapsed_secs();
    let dt = time.delta_secs();

    // Check material update timing
    let should_update = quality.should_update_materials(t, state.last_material_update);
    if should_update {
        state.last_material_update = t;
    }

    let cam_yaw = cam_q
        .single()
        .map(|c| c.rotation.to_euler(EulerRot::YXZ).0)
        .unwrap_or(0.0);

    // Animate orbs
    for (mut transform, mut orb) in orbs.iter_mut() {
        orb.angle += orb.speed * dt;
        let adjusted_angle = orb.angle + cam_yaw * 0.3;

        let wobble = (t * 4.0 + orb.phase).sin() * 0.02;
        let vertical = (t * 2.0 + orb.phase * 2.0).sin() * 0.04;

        transform.translation = Vec3::new(
            adjusted_angle.cos() * (orb.radius + wobble),
            ORB_Y + vertical,
            adjusted_angle.sin() * (orb.radius + wobble),
        );

        let twinkle = 0.6 + (t * 8.0 + orb.phase * 3.0).sin().abs() * 0.4;
        transform.scale = Vec3::splat(twinkle);
    }

    // Animate wisps
    for (mut transform, mut vis, wisp) in wisps.iter_mut() {
        let id = wisp.id as f32;
        let cycle = (t * 1.5 + id * 0.5) % 3.0;

        if !(0.1..=2.5).contains(&cycle) {
            *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Visible;

            let progress = (cycle - 0.1) / 2.4;
            let angle = id * 0.52 + (t * 0.3).sin() * 0.5;
            let dist = progress * 0.25;

            transform.translation = Vec3::new(
                angle.cos() * dist,
                ORB_Y + (id * 0.3).sin() * 0.05,
                angle.sin() * dist,
            );

            let stretch = 1.0 + progress * 2.0;
            let fade = 1.0 - progress * 0.7;
            transform.scale = Vec3::new(fade * 0.3, fade * 0.3, stretch * 0.5);
            transform.look_to(Vec3::new(angle.cos(), 0.0, angle.sin()), Vec3::Y);
        }
    }
}

/// Light animation - simple pulse
fn animate_lights(time: Res<Time>, mut lights: Query<(&mut PointLight, &OrbPointLight)>) {
    let t = time.elapsed_secs();
    for (mut light, orb) in lights.iter_mut() {
        let pulse = 1.0 + (t * 3.0 + orb.phase).sin() * 0.3;
        light.intensity = orb.base * pulse;
    }
}

fn create_wisp_mesh() -> Mesh {
    let positions = vec![[0.0, 0.01, 0.0], [0.0, -0.01, 0.0], [0.0, 0.0, 0.05]];
    let normals = vec![[0.0, 0.0, -1.0]; 3];
    let indices = vec![0, 1, 2];

    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}
