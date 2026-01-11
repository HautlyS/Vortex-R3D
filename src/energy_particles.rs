//! Energy Core Particles - Quality-reactive personal aura
//! Follows camera with dynamic rate adjustment

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::Gradient as HanabiGradient;

use crate::camera::{CameraState, GameCamera};
use crate::performance::QualityChanged;
use crate::GameState;

pub struct EnergyParticlesPlugin;

impl Plugin for EnergyParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Viewing), spawn_energy_effects)
            .add_systems(
                Update,
                (follow_camera, adjust_visibility, adjust_energy_quality)
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

/// Base rates for energy effects
const CORE_RATE: f32 = 20.0;
const AURA_RATE: f32 = 8.0;

#[derive(Component)]
pub struct EnergyCore {
    base_rate: f32,
}

#[derive(Component)]
pub struct EnergyAura {
    base_rate: f32,
}

fn spawn_energy_effects(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let core = create_inner_core();
    let aura = create_outer_aura();

    commands.spawn((
        ParticleEffect::new(effects.add(core)),
        Transform::from_translation(Vec3::new(0.0, -0.3, 0.0)),
        EnergyCore { base_rate: CORE_RATE },
    ));

    commands.spawn((
        ParticleEffect::new(effects.add(aura)),
        Transform::from_translation(Vec3::new(0.0, -0.3, 0.0)),
        EnergyAura { base_rate: AURA_RATE },
    ));

    info!("⚡ Energy particles spawned");
}

fn create_inner_core() -> EffectAsset {
    let mut m = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: m.lit(Vec3::ZERO),
        radius: m.lit(0.06),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: m.lit(Vec3::ZERO),
        speed: m.lit(0.02),
    };
    let init_life = SetAttributeModifier::new(Attribute::LIFETIME, m.lit(2.5));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, m.lit(0.008));

    let mut grad = HanabiGradient::new();
    grad.add_key(0.0, Vec4::new(0.3, 0.8, 1.0, 0.0));
    grad.add_key(0.2, Vec4::new(0.2, 0.6, 1.0, 0.15));
    grad.add_key(0.5, Vec4::new(0.15, 0.4, 0.9, 0.1));
    grad.add_key(1.0, Vec4::new(0.1, 0.2, 0.6, 0.0));

    let mut size_grad = HanabiGradient::new();
    size_grad.add_key(0.0, Vec3::splat(0.5));
    size_grad.add_key(0.3, Vec3::splat(1.0));
    size_grad.add_key(1.0, Vec3::splat(0.0));

    EffectAsset::new(256, SpawnerSettings::rate(CORE_RATE.into()), m)
        .with_name("EnergyCore")
        .init(init_pos)
        .init(init_vel)
        .init(init_life)
        .init(init_size)
        .render(ColorOverLifetimeModifier { gradient: grad, ..default() })
        .render(SizeOverLifetimeModifier { gradient: size_grad, ..default() })
}

fn create_outer_aura() -> EffectAsset {
    let mut m = Module::default();

    let init_pos = SetPositionSphereModifier {
        center: m.lit(Vec3::ZERO),
        radius: m.lit(0.15),
        dimension: ShapeDimension::Surface,
    };
    let init_vel = SetVelocitySphereModifier {
        center: m.lit(Vec3::ZERO),
        speed: m.lit(0.05),
    };
    let init_life = SetAttributeModifier::new(Attribute::LIFETIME, m.lit(4.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, m.lit(0.015));

    let mut grad = HanabiGradient::new();
    grad.add_key(0.0, Vec4::new(0.5, 0.3, 0.8, 0.0));
    grad.add_key(0.15, Vec4::new(0.4, 0.3, 0.7, 0.06));
    grad.add_key(0.5, Vec4::new(0.2, 0.3, 0.6, 0.04));
    grad.add_key(1.0, Vec4::new(0.1, 0.2, 0.4, 0.0));

    let mut size_grad = HanabiGradient::new();
    size_grad.add_key(0.0, Vec3::splat(0.0));
    size_grad.add_key(0.2, Vec3::splat(1.0));
    size_grad.add_key(0.8, Vec3::splat(1.2));
    size_grad.add_key(1.0, Vec3::splat(0.0));

    EffectAsset::new(128, SpawnerSettings::rate(AURA_RATE.into()), m)
        .with_name("EnergyAura")
        .init(init_pos)
        .init(init_vel)
        .init(init_life)
        .init(init_size)
        .render(ColorOverLifetimeModifier { gradient: grad, ..default() })
        .render(SizeOverLifetimeModifier { gradient: size_grad, ..default() })
}

fn follow_camera(
    camera: Query<&Transform, (With<GameCamera>, Without<EnergyCore>, Without<EnergyAura>)>,
    mut core: Query<&mut Transform, (With<EnergyCore>, Without<EnergyAura>)>,
    mut aura: Query<&mut Transform, (With<EnergyAura>, Without<EnergyCore>)>,
) {
    let Ok(cam) = camera.single() else { return };
    let pos = cam.translation - Vec3::Y * 0.3;

    if let Ok(mut t) = core.single_mut() {
        t.translation = pos;
    }
    if let Ok(mut t) = aura.single_mut() {
        t.translation = pos;
    }
}

fn adjust_visibility(
    state: Res<CameraState>,
    mut core: Query<&mut EffectSpawner, (With<EnergyCore>, Without<EnergyAura>)>,
    mut aura: Query<&mut EffectSpawner, (With<EnergyAura>, Without<EnergyCore>)>,
) {
    let active = state.pitch < -0.2;

    if let Ok(mut s) = core.single_mut() {
        s.active = active;
    }
    if let Ok(mut s) = aura.single_mut() {
        s.active = active;
    }
}

/// Adjust energy particle rates based on quality changes
fn adjust_energy_quality(
    mut events: MessageReader<QualityChanged>,
    mut core: Query<(&EnergyCore, &mut EffectSpawner), Without<EnergyAura>>,
    mut aura: Query<(&EnergyAura, &mut EffectSpawner), Without<EnergyCore>>,
) {
    for ev in events.read() {
        let mult = ev.new.spawner_rate_mult();

        if let Ok((c, mut s)) = core.single_mut() {
            s.settings = SpawnerSettings::rate((c.base_rate * mult).into());
        }
        if let Ok((a, mut s)) = aura.single_mut() {
            s.settings = SpawnerSettings::rate((a.base_rate * mult).into());
        }
    }
}
