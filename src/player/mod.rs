//! Player module - FPS movement with Avian physics integration

use avian3d::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::input::InputState;
use crate::panorama::PanoramaCamera;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_systems(OnEnter(GameState::Viewing), init_player)
            .add_systems(
                Update,
                player_physics_movement.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                update_camera_from_physics.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                handle_player_ground_check.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                apply_physics_input.run_if(in_state(GameState::Viewing)),
            );
    }
}

#[derive(Resource)]
pub struct PlayerState {
    pub room: usize,
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub height: f32,
    pub velocity: Vec3,
    pub grounded: bool,
    #[allow(dead_code)]
    pub physics_enabled: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            room: 0,
            pos: Vec2::ZERO,
            prev_pos: Vec2::ZERO,
            height: 1.7,
            velocity: Vec3::ZERO,
            grounded: false,
            physics_enabled: true,
        }
    }
}

#[derive(Component)]
pub struct Player;

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerPhysics {
    pub height: f32,
    pub radius: f32,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_force: f32,
    pub max_velocity: f32,
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        Self {
            height: 1.8,
            radius: 0.4,
            walk_speed: 3.0,
            run_speed: 6.0,
            jump_force: 8.0,
            max_velocity: 10.0,
        }
    }
}

fn init_player(
    mut commands: Commands,
    cam_q: Query<Entity, (With<PanoramaCamera>, Without<RenderLayers>)>,
) {
    for entity in cam_q.iter() {
        commands.entity(entity).insert(RenderLayers::layer(0));
    }

    let capsule_half_height = 0.9;
    let capsule_radius = 0.4;
    let spawn_height = capsule_half_height + capsule_radius + 0.1;

    commands.spawn((
        Player,
        PlayerPhysics::default(),
        Transform::from_translation(Vec3::new(0.0, spawn_height, 0.0)),
        GlobalTransform::default(),
        Visibility::Visible,
        RigidBody::Dynamic,
        Collider::capsule(capsule_half_height, capsule_radius),
        ColliderDensity(80.0),
        LinearVelocity::default(),
        LinearDamping(4.0),
        AngularDamping(10.0),
        Friction::new(0.1),
        Restitution::new(0.0),
        LockedAxes::ROTATION_LOCKED,
        Name::new("Player"),
    ));

    info!(
        "🎮 Player initialized with Avian physics at height {}",
        spawn_height
    );
}

fn player_physics_movement(
    time: Res<Time>,
    input: Res<InputState>,
    mut state: ResMut<PlayerState>,
    mut player_query: Query<(&Transform, &mut LinearVelocity, &PlayerPhysics), With<Player>>,
    camera_query: Query<&Transform, (With<PanoramaCamera>, Without<Player>)>,
) {
    let Ok((player_transform, mut velocity, physics)) = player_query.single_mut() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let dt = time.delta_secs();
    state.prev_pos = state.pos;
    state.pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.z,
    );

    let (yaw, _, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    let forward = Vec3::new(-yaw.sin(), 0.0f32, -yaw.cos());
    let right = Vec3::new(-forward.z, 0.0, forward.x);

    let move_dir = forward * (-input.movement.y) + right * input.movement.x;

    if move_dir.length() > 0.01 {
        let speed = physics.walk_speed;
        let acceleration = 20.0;
        let target_velocity = move_dir.normalize() * speed;

        velocity.x += (target_velocity.x - velocity.x).clamp(-acceleration * dt, acceleration * dt);
        velocity.z += (target_velocity.z - velocity.z).clamp(-acceleration * dt, acceleration * dt);

        let horizontal_speed = Vec2::new(velocity.x, velocity.z).length();
        if horizontal_speed > physics.max_velocity {
            let factor = physics.max_velocity / horizontal_speed;
            velocity.x *= factor;
            velocity.z *= factor;
        }
    } else {
        let deceleration = 10.0;
        velocity.x *= 1.0 - deceleration * dt;
        velocity.z *= 1.0 - deceleration * dt;
    }

    state.velocity = velocity.0;
    state.height = player_transform.translation.y;
}

fn update_camera_from_physics(
    mut camera_query: Query<&mut Transform, With<PanoramaCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<PanoramaCamera>)>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let target_pos = player_transform.translation + Vec3::new(0.0, 1.6, 0.0);
    camera_transform.translation = target_pos;
}

fn handle_player_ground_check(
    spatial_query: SpatialQuery,
    player_entity: Query<Entity, With<Player>>,
    transform_query: Query<&Transform, With<Player>>,
    physics_query: Query<&PlayerPhysics, With<Player>>,
    mut state: ResMut<PlayerState>,
) {
    let Ok(player_entity) = player_entity.single() else {
        return;
    };
    let Ok(transform) = transform_query.single() else {
        return;
    };
    let Ok(_physics) = physics_query.single() else {
        return;
    };

    let capsule_half_height = 0.9;
    let capsule_radius = 0.4;
    let ray_origin = transform.translation;
    let ray_dir = Dir3::NEG_Y;
    let max_distance = capsule_half_height + capsule_radius + 0.2;

    if let Some(hit) = spatial_query.cast_ray(
        ray_origin,
        ray_dir,
        max_distance,
        true,
        &SpatialQueryFilter::default().with_excluded_entities([player_entity]),
    ) {
        state.grounded = hit.distance < max_distance;
    } else {
        state.grounded = false;
    }
}

fn apply_physics_input(
    _input: Res<InputState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut LinearVelocity, &PlayerPhysics), With<Player>>,
    state: Res<PlayerState>,
) {
    let Ok((mut velocity, physics)) = player_query.single_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Space) && state.grounded {
        velocity.0.y = physics.jump_force;
    }
}

#[allow(dead_code)]
pub fn get_player_position(player_query: Query<&Transform, With<Player>>) -> Option<Vec3> {
    player_query.single().ok().map(|t| t.translation)
}

pub fn teleport_player(mut player_query: Query<&mut Transform, With<Player>>, position: Vec3) {
    if let Ok(mut transform) = player_query.single_mut() {
        transform.translation = position;
    }
}

#[allow(dead_code)]
pub fn set_player_physics_enabled(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enabled: bool,
) {
    if let Ok(entity) = player_query.single() {
        if enabled {
            commands.entity(entity).insert(RigidBody::Dynamic);
        } else {
            commands.entity(entity).insert(RigidBody::Static);
        }
    }
}
