//! GPU Shader Particles with Dynamic Quality Adjustment
//! Uses bevy_hanabi with real-time spawner rate control based on FPS

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::Gradient as HanabiGradient;

use crate::performance::{QualityChanged, QualitySettings};
use crate::player::PlayerState;
use crate::world::{room_center, TOTAL_ROOMS};
use crate::GameState;

pub struct GpuParticlesPlugin;

impl Plugin for GpuParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(OnEnter(GameState::Viewing), spawn_room_particles)
            .add_systems(
                Update,
                (update_particle_visibility, adjust_spawner_rates)
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

/// Marker with base spawn rate for dynamic adjustment
#[derive(Component)]
pub struct RoomParticle {
    pub room: usize,
    pub base_rate: f32,
}

/// Base spawn rates per effect type
const ORBS_RATE: f32 = 6.0;
const WISPS_RATE: f32 = 25.0;
const CIRCLES_RATE: f32 = 12.0;
const SPARKLES_RATE: f32 = 60.0;

fn spawn_room_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    quality: Res<QualitySettings>,
) {
    let mult = quality.level.spawner_rate_mult();

    let orbs = create_ambient_orbs();
    let wisps = create_energy_wisps();
    let circles = create_sacred_circles();
    let sparkles = create_sparkle_dust();

    let orbs_h = effects.add(orbs);
    let wisps_h = effects.add(wisps);
    let circles_h = effects.add(circles);
    let sparkles_h = effects.add(sparkles);

    for room in 0..TOTAL_ROOMS {
        let center = room_center(room);
        let base = center - Vec3::Z * 8.0;

        // Spawn with initial rate based on quality
        spawn_effect(&mut commands, orbs_h.clone(), base, room, ORBS_RATE, mult);
        spawn_effect(
            &mut commands,
            wisps_h.clone(),
            base + Vec3::Y * 0.5,
            room,
            WISPS_RATE,
            mult,
        );
        spawn_effect(
            &mut commands,
            circles_h.clone(),
            base,
            room,
            CIRCLES_RATE,
            mult,
        );
        spawn_effect(
            &mut commands,
            sparkles_h.clone(),
            base,
            room,
            SPARKLES_RATE,
            mult,
        );
    }

    info!(
        "✨ GPU particles: {} rooms, quality {:?}",
        TOTAL_ROOMS, quality.level
    );
}

fn spawn_effect(
    commands: &mut Commands,
    handle: Handle<EffectAsset>,
    pos: Vec3,
    room: usize,
    base_rate: f32,
    mult: f32,
) {
    let settings = SpawnerSettings::rate((base_rate * mult).into());
    commands.spawn((
        ParticleEffect::new(handle),
        EffectSpawner::new(&settings),
        Transform::from_translation(pos),
        RoomParticle { room, base_rate },
        RenderLayers::layer(room),
    ));
}

/// Adjust spawner rates when quality changes
fn adjust_spawner_rates(
    mut events: MessageReader<QualityChanged>,
    mut particles: Query<(&RoomParticle, &mut EffectSpawner)>,
) {
    for ev in events.read() {
        let mult = ev.new.spawner_rate_mult();
        for (rp, mut spawner) in particles.iter_mut() {
            spawner.settings = SpawnerSettings::rate((rp.base_rate * mult).into());
        }
        info!("🔧 Particle rates adjusted: mult={:.2}", mult);
    }
}

/// Update visibility based on current room
fn update_particle_visibility(
    player: Res<PlayerState>,
    mut particles: Query<(&RoomParticle, &mut EffectSpawner)>,
) {
    for (rp, mut spawner) in particles.iter_mut() {
        spawner.active = rp.room == player.room;
    }
}

// Effect creation functions - simplified for performance

fn create_ambient_orbs() -> EffectAsset {
    let mut m = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: m.lit(Vec3::ZERO),
        radius: m.lit(12.0),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocityTangentModifier {
        origin: m.lit(Vec3::ZERO),
        axis: m.lit(Vec3::Y),
        speed: m.lit(0.3),
    };
    let init_life = SetAttributeModifier::new(Attribute::LIFETIME, m.lit(8.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, m.lit(0.15));

    let mut grad = HanabiGradient::new();
    grad.add_key(0.0, Vec4::new(0.2, 0.5, 1.0, 0.0));
    grad.add_key(0.15, Vec4::new(0.3, 0.4, 0.9, 0.12));
    grad.add_key(0.5, Vec4::new(0.5, 0.3, 0.8, 0.08));
    grad.add_key(1.0, Vec4::new(0.1, 0.3, 0.7, 0.0));

    let mut size_grad = HanabiGradient::new();
    size_grad.add_key(0.0, Vec3::splat(0.0));
    size_grad.add_key(0.2, Vec3::splat(1.0));
    size_grad.add_key(1.0, Vec3::splat(0.0));

    EffectAsset::new(256, SpawnerSettings::rate(ORBS_RATE.into()), m)
        .with_name("AmbientOrbs")
        .init(init_pos)
        .init(init_vel)
        .init(init_life)
        .init(init_size)
        .render(ColorOverLifetimeModifier {
            gradient: grad,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_grad,
            ..default()
        })
}

fn create_energy_wisps() -> EffectAsset {
    let mut m = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: m.lit(Vec3::ZERO),
        radius: m.lit(8.0),
        dimension: ShapeDimension::Surface,
    };
    let init_vel = SetVelocitySphereModifier {
        center: m.lit(Vec3::ZERO),
        speed: m.lit(2.0),
    };
    let init_life = SetAttributeModifier::new(Attribute::LIFETIME, m.lit(3.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, m.lit(0.04));
    let drag = LinearDragModifier::new(m.lit(0.5));

    let mut grad = HanabiGradient::new();
    grad.add_key(0.0, Vec4::new(1.0, 0.8, 0.2, 0.0));
    grad.add_key(0.1, Vec4::new(1.0, 0.6, 0.3, 0.4));
    grad.add_key(0.5, Vec4::new(0.9, 0.3, 0.6, 0.3));
    grad.add_key(1.0, Vec4::new(0.1, 0.3, 0.8, 0.0));

    let mut size_grad = HanabiGradient::new();
    size_grad.add_key(0.0, Vec3::splat(0.5));
    size_grad.add_key(0.3, Vec3::splat(1.0));
    size_grad.add_key(1.0, Vec3::splat(0.0));

    EffectAsset::new(512, SpawnerSettings::rate(WISPS_RATE.into()), m)
        .with_name("EnergyWisps")
        .init(init_pos)
        .init(init_vel)
        .init(init_life)
        .init(init_size)
        .update(drag)
        .render(ColorOverLifetimeModifier {
            gradient: grad,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_grad,
            ..default()
        })
}

fn create_sacred_circles() -> EffectAsset {
    let mut m = Module::default();

    let init_pos = SetPositionCircleModifier {
        center: m.lit(Vec3::ZERO),
        axis: m.lit(Vec3::Y),
        radius: m.lit(6.0),
        dimension: ShapeDimension::Surface,
    };
    let init_vel = SetVelocityTangentModifier {
        origin: m.lit(Vec3::ZERO),
        axis: m.lit(Vec3::Y),
        speed: m.lit(0.8),
    };
    let accel = AccelModifier::new(m.lit(Vec3::Y * 0.1));
    let init_life = SetAttributeModifier::new(Attribute::LIFETIME, m.lit(6.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, m.lit(0.06));

    let mut grad = HanabiGradient::new();
    grad.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.0));
    grad.add_key(0.1, Vec4::new(1.0, 0.95, 0.8, 0.25));
    grad.add_key(0.5, Vec4::new(1.0, 0.8, 0.4, 0.15));
    grad.add_key(1.0, Vec4::new(0.3, 0.2, 0.6, 0.0));

    let mut size_grad = HanabiGradient::new();
    size_grad.add_key(0.0, Vec3::splat(0.0));
    size_grad.add_key(0.1, Vec3::splat(1.0));
    size_grad.add_key(1.0, Vec3::splat(0.0));

    EffectAsset::new(384, SpawnerSettings::rate(CIRCLES_RATE.into()), m)
        .with_name("SacredCircles")
        .init(init_pos)
        .init(init_vel)
        .init(init_life)
        .init(init_size)
        .update(accel)
        .render(ColorOverLifetimeModifier {
            gradient: grad,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_grad,
            ..default()
        })
}

fn create_sparkle_dust() -> EffectAsset {
    let mut m = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: m.lit(Vec3::ZERO),
        radius: m.lit(15.0),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: m.lit(Vec3::ZERO),
        speed: m.lit(0.1),
    };
    let init_life = SetAttributeModifier::new(Attribute::LIFETIME, m.lit(2.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, m.lit(0.02));

    let mut grad = HanabiGradient::new();
    grad.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.0));
    grad.add_key(0.1, Vec4::new(1.0, 1.0, 0.9, 0.8));
    grad.add_key(0.3, Vec4::new(0.9, 0.95, 1.0, 0.5));
    grad.add_key(1.0, Vec4::new(0.7, 0.8, 1.0, 0.0));

    let mut size_grad = HanabiGradient::new();
    size_grad.add_key(0.0, Vec3::splat(0.0));
    size_grad.add_key(0.05, Vec3::splat(1.5));
    size_grad.add_key(0.2, Vec3::splat(1.0));
    size_grad.add_key(1.0, Vec3::splat(0.0));

    EffectAsset::new(1024, SpawnerSettings::rate(SPARKLES_RATE.into()), m)
        .with_name("SparkleDust")
        .init(init_pos)
        .init(init_vel)
        .init(init_life)
        .init(init_size)
        .render(ColorOverLifetimeModifier {
            gradient: grad,
            ..default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_grad,
            ..default()
        })
}
