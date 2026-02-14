use avian3d::collision::collision_events::{CollisionEnd, CollisionStart};
use avian3d::prelude::*;
use bevy::pbr::MaterialPlugin;
use bevy::prelude::*;

pub mod asset;
pub mod render;
pub mod physics;
pub mod world;
pub mod splat_types;

#[cfg(test)]
pub mod tests;
#[cfg(test)]
pub mod integration_tests;

pub use asset::*;
pub use render::*;
pub use physics::*;
pub use world::*;
pub use splat_types::*;

use crate::GameState;

pub struct GaussianSplatPlugin;

impl Plugin for GaussianSplatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .add_plugins(GaussianSplatAssetPlugin)
            .add_plugins(GaussianSplatRenderPlugin)
            .add_plugins(GaussianSplatPhysicsPlugin)
            .add_plugins(GaussianSplatWorldPlugin)
            .add_plugins(MaterialPlugin::<SplatMaterial>::default())
            .insert_resource(Gravity(Vec3::new(0.0, -9.81, 0.0)))
            .add_systems(Startup, setup_physics_world)
            .add_systems(Update, update_splat_physics.run_if(in_state(GameState::Viewing)))
            .add_systems(Update, handle_splat_collisions.run_if(in_state(GameState::Viewing)))
            .add_systems(Update, optimize_splat_rendering.run_if(in_state(GameState::Viewing)));
    }
}

fn setup_physics_world() {
    info!("🌐 Gaussian Splat Physics World Initialized");
}

fn update_splat_physics(
    mut splat_query: Query<(&mut GaussianSplat, &mut Transform), With<RigidBody>>,
    time: Res<Time>,
) {
    for (mut splat, _transform) in splat_query.iter_mut() {
        splat.time_alive += time.delta_secs();
        splat.update_stability();
    }
}

fn handle_splat_collisions(
    mut collision_events: MessageReader<CollisionStart>,
    mut collision_ended_events: MessageReader<CollisionEnd>,
    mut splat_query: Query<&mut GaussianSplat>,
) {
    for event in collision_events.read() {
        if let Ok(mut splat1) = splat_query.get_mut(event.collider1) {
            splat1.on_collision();
        }
        if let Ok(mut splat2) = splat_query.get_mut(event.collider2) {
            splat2.on_collision();
        }
    }

    for event in collision_ended_events.read() {
        if let Ok(mut splat1) = splat_query.get_mut(event.collider1) {
            splat1.on_collision_end();
        }
        if let Ok(mut splat2) = splat_query.get_mut(event.collider2) {
            splat2.on_collision_end();
        }
    }
}

fn optimize_splat_rendering(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    splat_query: Query<(Entity, &Transform, &GaussianSplat, &ViewVisibility)>,
) {
    let Ok(camera_transform) = camera_query.single() else { return };
    let camera_pos = camera_transform.translation;
    
    for (entity, transform, splat, visibility) in splat_query.iter() {
        let distance = transform.translation.distance(camera_pos);
        let should_render = visibility.get() && distance < splat.cull_distance;
        
        if should_render {
            let lod_level = calculate_lod_level(distance, splat);
            commands.entity(entity).insert(SplatLOD { level: lod_level });
        }
    }
}

fn calculate_lod_level(distance: f32, splat: &GaussianSplat) -> u8 {
    if distance < splat.lod_distances[0] {
        0
    } else if distance < splat.lod_distances[1] {
        1
    } else if distance < splat.lod_distances[2] {
        2
    } else {
        3
    }
}
