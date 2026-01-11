//! Holographic Light Beings - GPU-accelerated 2026 shader particles
//! Using bevy_hanabi for production-ready performance

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::performance::QualitySettings;
use crate::player::PlayerState;
use crate::world::{room_center, TOTAL_ROOMS};
use crate::GameState;

#[cfg(feature = "particles")]
use bevy_hanabi::prelude::*;
#[cfg(feature = "particles")]
use bevy_hanabi::Gradient as HanabiGradient;

pub struct HolographicParticlesPlugin;

impl Plugin for HolographicParticlesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "particles")]
        app.add_systems(OnEnter(GameState::Viewing), spawn_holographic_particles)
            .add_systems(
                Update,
                update_holographic_visibility.run_if(in_state(GameState::Viewing)),
            );

        #[cfg(not(feature = "particles"))]
        app.add_systems(OnEnter(GameState::Viewing), spawn_mesh_fallback)
            .add_systems(
                Update,
                (update_mesh_visibility, animate_mesh_beings).run_if(in_state(GameState::Viewing)),
            );
    }
}

/// Marker for holographic effects (particles feature only)
#[cfg(feature = "particles")]
#[derive(Component)]
pub struct HolographicEffect {
    pub room: usize,
    pub effect_type: HoloType,
}

#[cfg(feature = "particles")]
#[derive(Clone, Copy)]
pub enum HoloType {
    LightBeings,
    Halos,
}

// ============================================================================
// GPU PARTICLE IMPLEMENTATION (feature = "particles")
// ============================================================================

#[cfg(feature = "particles")]
fn spawn_holographic_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    quality: Res<QualitySettings>,
) {
    let beings = create_light_beings_effect(&quality);
    let halos = create_halos_effect(&quality);

    let beings_handle = effects.add(beings);
    let halos_handle = effects.add(halos);

    for room in 0..TOTAL_ROOMS {
        let center = room_center(room);
        let base_pos = center - Vec3::Z * 8.0;

        // Light beings - floating orbs
        commands.spawn((
            ParticleEffect::new(beings_handle.clone()),
            Transform::from_translation(base_pos),
            HolographicEffect {
                room,
                effect_type: HoloType::LightBeings,
            },
            RenderLayers::layer(room),
        ));

        // Halos - ring formations
        commands.spawn((
            ParticleEffect::new(halos_handle.clone()),
            Transform::from_translation(base_pos + Vec3::Y * 0.5),
            HolographicEffect {
                room,
                effect_type: HoloType::Halos,
            },
            RenderLayers::layer(room),
        ));
    }

    info!(
        "✨ Holographic GPU particles spawned for {} rooms",
        TOTAL_ROOMS
    );
}

#[cfg(feature = "particles")]
fn create_light_beings_effect(quality: &QualitySettings) -> EffectAsset {
    let mut module = Module::default();
    let count = quality.particle_count(512);

    // Spawn in orbital pattern
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(8.0),
        dimension: ShapeDimension::Volume,
    };

    // Slow orbital motion
    let init_vel = SetVelocityTangentModifier {
        origin: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Y),
        speed: module.lit(0.4),
    };

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(10.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, module.lit(0.1));

    // Ethereal color cycling - blue to purple to cyan to gold
    let mut gradient = HanabiGradient::new();
    gradient.add_key(0.0, Vec4::new(0.2, 0.5, 1.0, 0.0));
    gradient.add_key(0.1, Vec4::new(0.3, 0.5, 0.9, 0.2));
    gradient.add_key(0.3, Vec4::new(0.6, 0.3, 0.8, 0.15));
    gradient.add_key(0.5, Vec4::new(0.2, 0.7, 0.9, 0.12));
    gradient.add_key(0.7, Vec4::new(0.9, 0.7, 0.3, 0.1));
    gradient.add_key(0.9, Vec4::new(0.4, 0.4, 0.8, 0.08));
    gradient.add_key(1.0, Vec4::new(0.2, 0.3, 0.6, 0.0));

    // Pulsing size
    let mut size_gradient = HanabiGradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.0));
    size_gradient.add_key(0.1, Vec3::splat(0.8));
    size_gradient.add_key(0.5, Vec3::splat(1.2));
    size_gradient.add_key(0.9, Vec3::splat(0.9));
    size_gradient.add_key(1.0, Vec3::splat(0.0));

    // Gentle vertical oscillation
    let accel = AccelModifier::new(module.lit(Vec3::Y * 0.05));

    EffectAsset::new(count as u32, SpawnerSettings::rate(15.0.into()), module)
        .with_name("LightBeings")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_size)
        .update(accel)
        .render(ColorOverLifetimeModifier {
            gradient,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            ..default()
        })
}

#[cfg(feature = "particles")]
fn create_halos_effect(quality: &QualitySettings) -> EffectAsset {
    let mut module = Module::default();
    let count = quality.particle_count(256);

    // Spawn on circles at different heights
    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Y),
        radius: module.lit(5.0),
        dimension: ShapeDimension::Surface,
    };

    // Tangential rotation
    let init_vel = SetVelocityTangentModifier {
        origin: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Y),
        speed: module.lit(0.6),
    };

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(8.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, module.lit(0.05));

    // Sacred gold to white to violet
    let mut gradient = HanabiGradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.6, 0.0));
    gradient.add_key(0.15, Vec4::new(1.0, 0.95, 0.8, 0.2));
    gradient.add_key(0.4, Vec4::new(1.0, 1.0, 1.0, 0.15));
    gradient.add_key(0.7, Vec4::new(0.7, 0.5, 0.9, 0.1));
    gradient.add_key(1.0, Vec4::new(0.4, 0.3, 0.7, 0.0));

    let mut size_gradient = HanabiGradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.0));
    size_gradient.add_key(0.1, Vec3::splat(1.0));
    size_gradient.add_key(0.8, Vec3::splat(1.1));
    size_gradient.add_key(1.0, Vec3::splat(0.0));

    EffectAsset::new(count as u32, SpawnerSettings::rate(12.0.into()), module)
        .with_name("Halos")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_size)
        .render(ColorOverLifetimeModifier {
            gradient,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            ..default()
        })
}

#[cfg(feature = "particles")]
fn update_holographic_visibility(
    player: Res<PlayerState>,
    mut particles: Query<(&HolographicEffect, &mut EffectSpawner)>,
) {
    for (holo, mut spawner) in particles.iter_mut() {
        spawner.active = holo.room == player.room;
    }
}

// ============================================================================
// MESH FALLBACK (no particles feature - WASM)
// ============================================================================

#[cfg(not(feature = "particles"))]
use std::f32::consts::TAU;

#[cfg(not(feature = "particles"))]
#[derive(Component)]
pub struct MeshBeing {
    pub room: usize,
    pub phase: f32,
    pub speed: f32,
}

#[cfg(not(feature = "particles"))]
fn spawn_mesh_fallback(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    quality: Res<QualitySettings>,
) {
    let mesh = meshes.add(Sphere::new(0.08).mesh().ico(1).unwrap());
    let count = quality.particle_count(8);

    for room in 0..TOTAL_ROOMS {
        let center = room_center(room);

        for i in 0..count {
            let angle = (i as f32 / count as f32) * TAU;
            let radius = 5.0 + (i % 3) as f32 * 2.0;
            let pos = center + Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius - 8.0);

            let hue = i as f32 / count as f32;
            let color = hue_to_color(hue);

            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: color,
                    emissive: LinearRgba::new(
                        color.to_linear().red * 3.0,
                        color.to_linear().green * 3.0,
                        color.to_linear().blue * 3.0,
                        1.0,
                    ),
                    unlit: true,
                    alpha_mode: AlphaMode::Add,
                    ..default()
                })),
                Transform::from_translation(pos),
                Visibility::Hidden,
                RenderLayers::layer(room),
                MeshBeing {
                    room,
                    phase: angle,
                    speed: 0.3 + (i % 3) as f32 * 0.1,
                },
            ));
        }
    }

    info!("✨ Mesh fallback beings spawned");
}

#[cfg(not(feature = "particles"))]
fn update_mesh_visibility(
    player: Res<PlayerState>,
    mut beings: Query<(&MeshBeing, &mut Visibility)>,
) {
    for (being, mut vis) in beings.iter_mut() {
        *vis = if being.room == player.room {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[cfg(not(feature = "particles"))]
fn animate_mesh_beings(time: Res<Time>, mut beings: Query<(&mut Transform, &MeshBeing)>) {
    let t = time.elapsed_secs();

    for (mut transform, being) in beings.iter_mut() {
        let center = room_center(being.room);
        let angle = being.phase + t * being.speed;
        let radius = 5.0;
        let vertical = (t * 0.5 + being.phase).sin() * 1.0;

        transform.translation =
            center + Vec3::new(angle.cos() * radius, vertical, angle.sin() * radius - 8.0);
    }
}

#[cfg(not(feature = "particles"))]
fn hue_to_color(h: f32) -> Color {
    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - h.floor();

    let (r, g, b) = match i % 6 {
        0 => (1.0, f, 0.0),
        1 => (1.0 - f, 1.0, 0.0),
        2 => (0.0, 1.0, f),
        3 => (0.0, 1.0 - f, 1.0),
        4 => (f, 0.0, 1.0),
        _ => (1.0, 0.0, 1.0 - f),
    };

    Color::srgb(r, g, b)
}
