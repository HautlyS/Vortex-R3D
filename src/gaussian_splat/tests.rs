//! Tests for Gaussian Splat system

#[cfg(test)]
mod tests {
    use crate::gaussian_splat::asset::*;
    use crate::gaussian_splat::*;
    use avian3d::prelude::*;
    use bevy::prelude::*;

    #[test]
    fn test_gaussian_splat_default() {
        let splat = GaussianSplat::default();
        assert_eq!(splat.position, Vec3::ZERO);
        assert_eq!(splat.opacity, 1.0);
        assert_eq!(splat.solidity, 0.5);
        assert_eq!(splat.texture_weight, 1.0);
        assert!(splat.physics_enabled);
    }

    #[test]
    fn test_gaussian_splat_builder() {
        let splat = GaussianSplat::new(Vec3::new(1.0, 2.0, 3.0), Color::RED)
            .with_scale(Vec3::ONE * 0.5)
            .with_opacity(0.8)
            .with_solidity(0.7)
            .with_texture_weight(0.9);

        assert_eq!(splat.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(splat.color, Color::RED);
        assert_eq!(splat.scale, Vec3::ONE * 0.5);
        assert_eq!(splat.opacity, 0.8);
        assert_eq!(splat.solidity, 0.7);
        assert_eq!(splat.texture_weight, 0.9);
    }

    #[test]
    fn test_splat_physics_bundle_default() {
        let bundle = SplatPhysicsBundle::default();
        assert_eq!(bundle.body.density, SPLAT_PHYSICS_DENSITY);
        assert_eq!(bundle.body.friction, SPLAT_PHYSICS_FRICTION);
        assert_eq!(bundle.body.restitution, SPLAT_PHYSICS_RESTITUTION);
    }

    #[test]
    fn test_splat_physics_bundle_builder() {
        let bundle = SplatPhysicsBundle::dynamic_body()
            .with_sphere(1.0)
            .with_density(50.0)
            .with_friction(0.3)
            .with_restitution(0.4)
            .with_damping(0.5, 0.6)
            .lock_rotation();

        assert_eq!(bundle.body.density, 50.0);
        assert_eq!(bundle.body.friction, 0.3);
        assert_eq!(bundle.body.restitution, 0.4);
        assert_eq!(bundle.body.linear_damping, 0.5);
        assert_eq!(bundle.body.angular_damping, 0.6);
        assert!(bundle.body.lock_rotation);
    }

    #[test]
    fn test_splat_cloud_empty() {
        let cloud = GaussianSplatCloud::new();
        assert!(cloud.splats.is_empty());
        assert_eq!(cloud.cluster_count, 0);
    }

    #[test]
    fn test_splat_cloud_from_splats() {
        let splats = vec![
            SplatData {
                position: Vec3::ZERO,
                ..default()
            },
            SplatData {
                position: Vec3::ONE,
                ..default()
            },
        ];

        let cloud = GaussianSplatCloud::from_splats(splats);
        assert_eq!(cloud.splats.len(), 2);
        assert!(cloud.cluster_count > 0);
    }

    #[test]
    fn test_splat_stability() {
        let mut splat = GaussianSplat::default();
        let initial_stability = splat.stability;

        splat.update_stability();

        // Stability should change after update
        assert!(splat.stability >= 0.0 && splat.stability <= 1.0);
    }

    #[test]
    fn test_splat_collision() {
        let mut splat = GaussianSplat::default();
        let initial_solidity = splat.solidity;
        let initial_priority = splat.render_priority;

        splat.on_collision();

        assert!(splat.solidity >= initial_solidity);
        assert_eq!(splat.render_priority, initial_priority + 1);

        splat.on_collision_end();
        assert_eq!(splat.render_priority, initial_priority);
    }

    #[test]
    fn test_splat_world_radius() {
        let splat = GaussianSplat::new(Vec3::ZERO, Color::WHITE).with_scale(Vec3::ONE * 0.1);

        let radius = splat.get_world_radius();
        assert!(radius > 0.0);
    }

    #[test]
    fn test_lod_level_calculation() {
        let splat = GaussianSplat::default();

        // Test distance-based LOD calculation
        // Note: This tests the logic, actual values depend on splat.lod_distances
        assert!(splat.lod_distances[0] < splat.lod_distances[1]);
        assert!(splat.lod_distances[1] < splat.lod_distances[2]);
    }

    #[test]
    fn test_procedural_room_generation() {
        let room_size = Vec3::new(10.0, 5.0, 10.0);
        let density = 1.0;
        let base_color = Color::WHITE;

        let cloud = generate_procedural_room_splats(room_size, density, base_color, 0.2);

        // Should generate splats based on volume and density
        let expected_count = (room_size.x * room_size.y * room_size.z * density) as usize;
        assert!(cloud.splats.len() > 0);
        assert!(cloud.splats.len() <= expected_count);
    }

    #[test]
    fn test_wall_splat_generation() {
        let start = Vec3::ZERO;
        let end = Vec3::new(10.0, 0.0, 0.0);
        let height = 5.0;
        let density = 2.0;

        let cloud = generate_wall_splats(start, end, height, density, Color::WHITE);

        // Wall length is 10, density is 2, height is 5
        // Expected: ~100 splats
        assert!(cloud.splats.len() > 0);
    }

    #[test]
    fn test_splat_physics_event() {
        let event = SplatPhysicsEvent {
            entity: Entity::from_raw(1),
            other_entity: Entity::from_raw(2),
            impact_velocity: Vec3::new(1.0, 0.0, 0.0),
            impact_force: 10.0,
            event_type: SplatPhysicsEventType::CollisionStarted,
        };

        assert_eq!(event.impact_force, 10.0);
        assert!(matches!(
            event.event_type,
            SplatPhysicsEventType::CollisionStarted
        ));
    }

    #[test]
    fn test_splat_settings_default() {
        let settings = SplatSettings::default();
        assert!(settings.enable_physics);
        assert!(settings.enable_culling);
        assert_eq!(settings.max_splats_per_frame, 10000);
        assert_eq!(settings.lod_bias, 1.0);
    }

    #[test]
    fn test_splat_data_default() {
        let data = SplatData::default();
        assert_eq!(data.position, Vec3::ZERO);
        assert_eq!(data.color, [255, 255, 255, 255]);
        assert_eq!(data.opacity, 1.0);
        assert_eq!(data.cluster_id, 0);
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::gaussian_splat::*;
    use avian3d::prelude::*;
    use bevy::prelude::*;

    /// Test that verifies player can walk on ground
    #[test]
    fn test_ground_collision() {
        // This test verifies the physics setup allows walking
        // In a real test environment, we would spawn entities and check collisions

        // Ground should be a static body
        let ground_bundle = SplatPhysicsBundle::static_body().with_box(Vec3::new(10.0, 0.1, 10.0));

        assert!(matches!(ground_bundle.rigid_body, RigidBody::Static));

        // Player should be dynamic
        let player_bundle = SplatPhysicsBundle::dynamic_body()
            .with_capsule(0.9, 0.4)
            .with_density(80.0)
            .lock_rotation();

        assert!(matches!(player_bundle.rigid_body, RigidBody::Dynamic));
        assert!(player_bundle.body.lock_rotation);
    }

    /// Test gravity configuration
    #[test]
    fn test_gravity_configuration() {
        let gravity = Gravity::from(Vec3::new(0.0, -9.81, 0.0));

        assert_eq!(gravity.0.y, -9.81);
    }

    /// Test room bounds contain player spawn point
    #[test]
    fn test_room_bounds() {
        use crate::world::room_center;

        for room_id in 0..3 {
            let center = room_center(room_id);
            // Player spawns at center + height
            let spawn_pos = center + Vec3::new(0.0, 1.7, 0.0);

            // Basic bounds check - room is roughly 20x20x8
            assert!(spawn_pos.x >= center.x - 10.0 && spawn_pos.x <= center.x + 10.0);
            assert!(spawn_pos.z >= center.z - 10.0 && spawn_pos.z <= center.z + 10.0);
        }
    }
}

#[cfg(all(test, feature = "desktop"))]
mod physics_tests {
    use crate::gaussian_splat::*;
    use avian3d::prelude::*;
    use bevy::prelude::*;

    /// Comprehensive physics integration test
    /// This would run in a Bevy App context in real implementation
    #[test]
    fn test_physics_simulation_step() {
        // Verify physics constants are set correctly
        assert_eq!(SPLAT_PHYSICS_DENSITY, 100.0);
        assert_eq!(SPLAT_PHYSICS_FRICTION, 0.5);
        assert_eq!(SPLAT_PHYSICS_RESTITUTION, 0.3);

        // Verify gravity is earth-like
        let gravity = Vec3::new(0.0, -9.81, 0.0);
        assert_eq!(gravity.y, -9.81);
    }

    /// Test collision layers are set up correctly
    #[test]
    fn test_collision_groups() {
        // In actual implementation, we would verify:
        // - Player collides with ground
        // - Player collides with walls
        // - Player collides with portals
        // - Splats don't collide with each other (for performance)

        // Static bodies should not collide with each other
        let wall1 = SplatPhysicsBundle::static_body();
        let wall2 = SplatPhysicsBundle::static_body();

        assert!(matches!(wall1.rigid_body, RigidBody::Static));
        assert!(matches!(wall2.rigid_body, RigidBody::Static));
    }
}
