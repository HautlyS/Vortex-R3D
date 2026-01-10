use crate::camera::{CameraState, GameCamera};
use crate::GameState;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::Gradient as HanabiGradient;

pub struct EnergyParticlesPlugin;

impl Plugin for EnergyParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(OnEnter(GameState::Viewing), spawn_energy_core)
            .add_systems(
                Update,
                (follow_camera, adjust_visibility).run_if(in_state(GameState::Viewing)),
            );
    }
}

#[derive(Component)]
pub struct EnergyCore;

fn spawn_energy_core(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    // Subtle inner core - soft glow
    let mut core_gradient = HanabiGradient::new();
    core_gradient.add_key(0.0, Vec4::new(0.15, 0.4, 0.8, 0.08)); // very faint blue
    core_gradient.add_key(0.5, Vec4::new(0.1, 0.2, 0.5, 0.04)); // dimmer
    core_gradient.add_key(1.0, Vec4::ZERO);

    let mut module = Module::default();

    // Tight sphere for core
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.08),
        dimension: ShapeDimension::Volume,
    };

    // Slow outward drift
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(0.02),
    };

    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, module.lit(3.0));
    let init_size = SetAttributeModifier::new(Attribute::SIZE, module.lit(0.008));

    let effect = EffectAsset::new(512, SpawnerSettings::rate(30.0.into()), module)
        .with_name("EnergyCore")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_size)
        .render(ColorOverLifetimeModifier {
            gradient: core_gradient,
            ..default()
        });

    let handle = effects.add(effect);

    commands.spawn((
        ParticleEffect::new(handle),
        Transform::from_translation(Vec3::new(0.0, -0.3, 0.0)),
        EnergyCore,
    ));
}

fn follow_camera(
    camera: Query<&Transform, (With<GameCamera>, Without<EnergyCore>)>,
    mut energy: Query<&mut Transform, With<EnergyCore>>,
) {
    let Ok(cam) = camera.single() else { return };
    let Ok(mut core) = energy.single_mut() else {
        return;
    };

    // Position BELOW camera (at body level)
    core.translation = cam.translation - Vec3::Y * 0.3;
}

fn adjust_visibility(
    state: Res<CameraState>,
    mut spawners: Query<&mut EffectSpawner, With<EnergyCore>>,
) {
    let Ok(mut spawner) = spawners.single_mut() else {
        return;
    };

    // Only visible when looking down (negative pitch)
    // pitch < -0.3 means looking down
    let visibility = if state.pitch < -0.2 {
        let factor = ((-state.pitch - 0.2) / 0.8).clamp(0.0, 1.0);
        factor * factor // ease in
    } else {
        0.0
    };

    // Control spawn rate based on visibility
    spawner.active = visibility > 0.01;
}
