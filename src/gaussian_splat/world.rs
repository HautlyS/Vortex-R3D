use avian3d::prelude::*;
use bevy::camera::primitives::Aabb;
use bevy::prelude::*;

use super::asset::*;
use super::physics::*;
use super::render::*;
use super::splat_types::*;

pub struct GaussianSplatWorldPlugin;

impl Plugin for GaussianSplatWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SplatWorldState>()
            .init_resource::<SplatSettings>()
            .add_message::<RoomTransitionEvent>()
            .add_systems(Startup, setup_splat_world)
            .add_systems(Update, manage_splat_rooms)
            .add_systems(Update, handle_room_transitions)
            .add_systems(Update, update_splat_environment)
            .add_systems(Update, spawn_room_splats)
            .add_systems(Update, optimize_room_rendering);
    }
}

#[allow(dead_code)]
#[derive(Resource, Debug, Clone)]
pub struct SplatWorldState {
    pub current_room: usize,
    pub rooms: Vec<SplatRoom>,
    pub transition_active: bool,
    pub transition_progress: f32,
    pub total_splats: usize,
    pub active_splats: usize,
}

impl Default for SplatWorldState {
    fn default() -> Self {
        Self {
            current_room: 0,
            rooms: Vec::new(),
            transition_active: false,
            transition_progress: 0.0,
            total_splats: 0,
            active_splats: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SplatRoom {
    pub id: usize,
    pub name: String,
    pub bounds: Aabb,
    pub splat_cloud_handle: Handle<GaussianSplatCloud>,
    pub environment_type: SplatEnvironmentType,
    pub portals: Vec<SplatPortalInfo>,
    pub lighting: SplatRoomLighting,
    pub physics_settings: SplatRoomPhysics,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SplatEnvironmentType {
    Indoor,
    Outdoor,
    Mixed,
    Abstract,
    Volumetric,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SplatPortalInfo {
    pub position: Vec3,
    pub rotation: Quat,
    pub target_room: usize,
    pub size: Vec2,
}

#[derive(Debug, Clone)]
pub struct SplatRoomLighting {
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub directional_light: Option<DirectionalLightConfig>,
    pub point_lights: Vec<PointLightConfig>,
}

#[derive(Debug, Clone)]
pub struct DirectionalLightConfig {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Debug, Clone)]
pub struct PointLightConfig {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SplatRoomPhysics {
    pub gravity: Vec3,
    pub air_resistance: f32,
    pub has_floor: bool,
    pub floor_height: f32,
    pub boundaries: Vec<Aabb>,
}

impl Default for SplatRoom {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Unnamed Room".to_string(),
            bounds: Aabb::from_min_max(Vec3::ZERO, Vec3::ONE * 10.0),
            splat_cloud_handle: Handle::default(),
            environment_type: SplatEnvironmentType::Indoor,
            portals: Vec::new(),
            lighting: SplatRoomLighting {
                ambient_color: Color::WHITE,
                ambient_intensity: 0.3,
                directional_light: None,
                point_lights: Vec::new(),
            },
            physics_settings: SplatRoomPhysics {
                gravity: Vec3::new(0.0, -9.81, 0.0),
                air_resistance: 0.0,
                has_floor: true,
                floor_height: 0.0,
                boundaries: Vec::new(),
            },
        }
    }
}

fn setup_splat_world(
    mut commands: Commands,
    mut splat_clouds: ResMut<Assets<GaussianSplatCloud>>,
    mut world_state: ResMut<SplatWorldState>,
) {
    let room1_cloud: Handle<GaussianSplatCloud> =
        splat_clouds.add(generate_procedural_room_splats(
            Vec3::new(20.0, 8.0, 20.0),
            0.8,
            Color::srgb(0.2, 0.15, 0.1),
            0.2,
        ));
    let room2_cloud: Handle<GaussianSplatCloud> =
        splat_clouds.add(generate_procedural_room_splats(
            Vec3::new(24.0, 10.0, 24.0),
            0.6,
            Color::srgb(0.1, 0.1, 0.15),
            0.2,
        ));
    let room3_cloud: Handle<GaussianSplatCloud> =
        splat_clouds.add(generate_procedural_room_splats(
            Vec3::new(30.0, 12.0, 30.0),
            0.5,
            Color::srgb(0.15, 0.05, 0.2),
            0.2,
        ));

    world_state.rooms = vec![
        SplatRoom {
            id: 0,
            name: "Temple Entrance".to_string(),
            bounds: Aabb::from_min_max(Vec3::new(-10.0, 0.0, -10.0), Vec3::new(10.0, 8.0, 10.0)),
            splat_cloud_handle: room1_cloud,
            environment_type: SplatEnvironmentType::Indoor,
            portals: vec![SplatPortalInfo {
                position: Vec3::new(0.0, 2.0, 8.0),
                rotation: Quat::from_rotation_y(0.0),
                target_room: 1,
                size: Vec2::new(2.0, 3.0),
            }],
            lighting: SplatRoomLighting {
                ambient_color: Color::srgb(0.2, 0.15, 0.1),
                ambient_intensity: 0.4,
                directional_light: Some(DirectionalLightConfig {
                    direction: Vec3::new(-0.5, -1.0, -0.3),
                    color: Color::srgb(1.0, 0.9, 0.8),
                    intensity: 1.5,
                }),
                point_lights: vec![PointLightConfig {
                    position: Vec3::new(0.0, 5.0, 0.0),
                    color: Color::srgb(1.0, 0.6, 0.0),
                    intensity: 2.0,
                    range: 15.0,
                }],
            },
            physics_settings: SplatRoomPhysics {
                gravity: Vec3::new(0.0, -9.81, 0.0),
                air_resistance: 0.1,
                has_floor: true,
                floor_height: 0.0,
                boundaries: vec![Aabb::from_min_max(
                    Vec3::new(-10.0, 0.0, -10.0),
                    Vec3::new(10.0, 8.0, 10.0),
                )],
            },
        },
        SplatRoom {
            id: 1,
            name: "Sacred Hall".to_string(),
            bounds: Aabb::from_min_max(Vec3::new(-12.0, 0.0, -12.0), Vec3::new(12.0, 10.0, 12.0)),
            splat_cloud_handle: room2_cloud,
            environment_type: SplatEnvironmentType::Indoor,
            portals: vec![
                SplatPortalInfo {
                    position: Vec3::new(-8.0, 2.0, 0.0),
                    rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0),
                    target_room: 2,
                    size: Vec2::new(2.0, 3.0),
                },
                SplatPortalInfo {
                    position: Vec3::new(0.0, 2.0, -8.0),
                    rotation: Quat::from_rotation_y(std::f32::consts::PI),
                    target_room: 0,
                    size: Vec2::new(2.0, 3.0),
                },
            ],
            lighting: SplatRoomLighting {
                ambient_color: Color::srgb(0.1, 0.1, 0.15),
                ambient_intensity: 0.3,
                directional_light: Some(DirectionalLightConfig {
                    direction: Vec3::new(0.3, -1.0, 0.5),
                    color: Color::srgb(0.8, 0.9, 1.0),
                    intensity: 1.2,
                }),
                point_lights: vec![
                    PointLightConfig {
                        position: Vec3::new(-5.0, 4.0, -5.0),
                        color: Color::srgb(0.0, 1.0, 1.0),
                        intensity: 1.5,
                        range: 10.0,
                    },
                    PointLightConfig {
                        position: Vec3::new(5.0, 4.0, 5.0),
                        color: Color::srgb(1.0, 0.0, 1.0),
                        intensity: 1.5,
                        range: 10.0,
                    },
                ],
            },
            physics_settings: SplatRoomPhysics {
                gravity: Vec3::new(0.0, -9.81, 0.0),
                air_resistance: 0.05,
                has_floor: true,
                floor_height: 0.0,
                boundaries: vec![Aabb::from_min_max(
                    Vec3::new(-12.0, 0.0, -12.0),
                    Vec3::new(12.0, 10.0, 12.0),
                )],
            },
        },
        SplatRoom {
            id: 2,
            name: "Ethereal Chamber".to_string(),
            bounds: Aabb::from_min_max(Vec3::new(-15.0, 0.0, -15.0), Vec3::new(15.0, 12.0, 15.0)),
            splat_cloud_handle: room3_cloud,
            environment_type: SplatEnvironmentType::Volumetric,
            portals: vec![
                SplatPortalInfo {
                    position: Vec3::new(10.0, 2.0, 0.0),
                    rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.0),
                    target_room: 0,
                    size: Vec2::new(2.0, 3.0),
                },
                SplatPortalInfo {
                    position: Vec3::new(0.0, 2.0, 10.0),
                    rotation: Quat::from_rotation_y(0.0),
                    target_room: 1,
                    size: Vec2::new(2.0, 3.0),
                },
            ],
            lighting: SplatRoomLighting {
                ambient_color: Color::srgb(0.15, 0.05, 0.2),
                ambient_intensity: 0.5,
                directional_light: None,
                point_lights: vec![
                    PointLightConfig {
                        position: Vec3::new(0.0, 8.0, 0.0),
                        color: Color::srgb(1.0, 0.5, 0.8),
                        intensity: 3.0,
                        range: 20.0,
                    },
                    PointLightConfig {
                        position: Vec3::new(-8.0, 3.0, -8.0),
                        color: Color::srgb(0.5, 1.0, 0.5),
                        intensity: 1.8,
                        range: 12.0,
                    },
                    PointLightConfig {
                        position: Vec3::new(8.0, 3.0, 8.0),
                        color: Color::srgb(0.5, 0.5, 1.0),
                        intensity: 1.8,
                        range: 12.0,
                    },
                ],
            },
            physics_settings: SplatRoomPhysics {
                gravity: Vec3::new(0.0, -4.0, 0.0),
                air_resistance: 0.3,
                has_floor: true,
                floor_height: 0.0,
                boundaries: vec![Aabb::from_min_max(
                    Vec3::new(-15.0, 0.0, -15.0),
                    Vec3::new(15.0, 12.0, 15.0),
                )],
            },
        },
    ];

    info!(
        "🌍 Gaussian Splat World initialized with {} rooms",
        world_state.rooms.len()
    );

    spawn_room_environment(&mut commands, &world_state.rooms[0]);
}

fn spawn_room_environment(commands: &mut Commands, room: &SplatRoom) {
    if room.physics_settings.has_floor {
        let min = room.bounds.min();
        let max = room.bounds.max();
        let floor_size = max - min;
        create_splat_ground(
            commands,
            Vec3::new(
                (min.x + max.x) / 2.0,
                room.physics_settings.floor_height - 0.1,
                (min.z + max.z) / 2.0,
            ),
            Vec2::new(floor_size.x / 2.0, floor_size.z / 2.0),
        );
    }

    for boundary in &room.physics_settings.boundaries {
        let b_min: Vec3 = boundary.min().into();
        let b_max: Vec3 = boundary.max().into();
        let center: Vec3 = (b_min + b_max) / 2.0;
        let size: Vec3 = (b_max - b_min) / 2.0;

        create_splat_wall(commands, center, size);
    }

    if let Some(dir_light) = &room.lighting.directional_light {
        commands.spawn((
            DirectionalLight {
                color: dir_light.color,
                illuminance: dir_light.intensity * 10000.0,
                ..default()
            },
            Transform::from_translation(Vec3::ZERO).looking_at(dir_light.direction, Vec3::Y),
        ));
    }

    for point_light in &room.lighting.point_lights {
        commands.spawn((
            PointLight {
                color: point_light.color,
                intensity: point_light.intensity * 1000.0,
                range: point_light.range,
                ..default()
            },
            Transform::from_translation(point_light.position),
        ));
    }

    commands.spawn((AmbientLight {
        color: room.lighting.ambient_color,
        brightness: room.lighting.ambient_intensity,
        affects_lightmapped_meshes: true,
    },));
}

fn aabb_contains(aabb: &Aabb, point: Vec3) -> bool {
    let min = aabb.min();
    let max = aabb.max();
    point.x >= min.x
        && point.x <= max.x
        && point.y >= min.y
        && point.y <= max.y
        && point.z >= min.z
        && point.z <= max.z
}

fn manage_splat_rooms(
    mut world_state: ResMut<SplatWorldState>,
    player_query: Query<&Transform, With<crate::player::Player>>,
) {
    if world_state.transition_active {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let player_pos = player_transform.translation;

    let current_room = &world_state.rooms[world_state.current_room];

    if !aabb_contains(&current_room.bounds, player_pos) {
        let new_room_idx = world_state
            .rooms
            .iter()
            .enumerate()
            .find(|(_, room)| aabb_contains(&room.bounds, player_pos))
            .map(|(idx, _)| idx);

        if let Some(idx) = new_room_idx {
            world_state.current_room = idx;
            info!("🏠 Entered room: {}", world_state.rooms[idx].name);
        }
    }
}

fn handle_room_transitions(
    mut world_state: ResMut<SplatWorldState>,
    mut transition_events: MessageWriter<RoomTransitionEvent>,
    time: Res<Time>,
) {
    if world_state.transition_active {
        world_state.transition_progress += time.delta_secs() / 2.0;

        if world_state.transition_progress >= 1.0 {
            world_state.transition_active = false;
            world_state.transition_progress = 0.0;
            transition_events.write(RoomTransitionEvent {
                from_room: world_state.current_room,
                to_room: (world_state.current_room + 1) % world_state.rooms.len(),
                completed: true,
            });
        }
    }
}

fn update_splat_environment(world_state: Res<SplatWorldState>, mut gravity: ResMut<Gravity>) {
    let current_room = &world_state.rooms[world_state.current_room];
    *gravity = Gravity(current_room.physics_settings.gravity);
}

fn spawn_room_splats(
    mut commands: Commands,
    world_state: Res<SplatWorldState>,
    cloud_assets: Res<Assets<GaussianSplatCloud>>,
    mut spawned_rooms: Local<Vec<usize>>,
) {
    for (idx, room) in world_state.rooms.iter().enumerate() {
        if spawned_rooms.contains(&idx) {
            continue;
        }

        if cloud_assets.get(&room.splat_cloud_handle).is_some() {
            spawn_gaussian_cloud(
                &mut commands,
                room.splat_cloud_handle.clone(),
                Transform::IDENTITY,
            );
            spawned_rooms.push(idx);
            info!("🎨 Spawned splats for room: {}", room.name);
        }
    }
}

fn optimize_room_rendering(
    world_state: Res<SplatWorldState>,
    camera_query: Query<&Transform, With<Camera>>,
    mut splat_query: Query<(&mut GaussianSplat, &Transform)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    let camera_pos = camera_transform.translation;

    let current_room = &world_state.rooms[world_state.current_room];

    for (mut splat, transform) in splat_query.iter_mut() {
        let distance = transform.translation.distance(camera_pos);
        let in_room = aabb_contains(&current_room.bounds, transform.translation);

        if !in_room && distance > 5.0 {
            splat.cull_distance = 0.0;
        } else {
            splat.cull_distance = 100.0;
        }
    }
}

#[allow(dead_code)]
#[derive(Message, Debug, Clone)]
pub struct RoomTransitionEvent {
    pub from_room: usize,
    pub to_room: usize,
    pub completed: bool,
}

#[allow(dead_code)]
pub fn trigger_room_transition(world_state: &mut ResMut<SplatWorldState>, target_room: usize) {
    if !world_state.transition_active && target_room < world_state.rooms.len() {
        world_state.transition_active = true;
        world_state.transition_progress = 0.0;
    }
}
