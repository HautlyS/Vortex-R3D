//! Portals module - Portal doors with Gaussian Splat and Avian physics integration

use avian3d::collision::collision_events::{CollisionEnd, CollisionStart};
use avian3d::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, TextureFormat};
use bevy::shader::ShaderRef;

use crate::gaussian_splat::SplatPhysicsBuilder;
use crate::panorama::PanoramaCamera;
use crate::player::{teleport_player, Player, PlayerState};
use crate::world::{room_center, TOTAL_ROOMS};
use crate::GameState;

pub struct PortalsPlugin;

impl Plugin for PortalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PortalMaterial>::default())
            .insert_resource(PortalState::default())
            .add_message::<PortalCrossingEvent>()
            .add_systems(OnEnter(GameState::Viewing), setup_portal_frames)
            .add_systems(
                Update,
                spawn_portal_views.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                sync_portal_cameras.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                update_portal_time.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                portal_crossing_physics.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                detect_portal_trigger.run_if(in_state(GameState::Viewing)),
            );
    }
}

pub const PORTAL_WIDTH: f32 = 1.0;
pub const PORTAL_HEIGHT: f32 = 2.2;
const FRAME_DEPTH: f32 = 0.15;

#[derive(Resource, Default)]
struct PortalState {
    spawned: bool,
    frames_waited: u32,
}

#[derive(Clone, Copy)]
pub struct DoorConfig {
    pub local_pos: Vec3,
    pub rotation: f32,
    pub target_room: usize,
    pub door_index: usize,
}

pub fn get_doors(room: usize) -> [DoorConfig; 2] {
    let prev = if room == 0 { TOTAL_ROOMS - 1 } else { room - 1 };
    let next = (room + 1) % TOTAL_ROOMS;
    [
        DoorConfig {
            local_pos: Vec3::new(-5.0, 0.0, -5.0),
            rotation: 0.3,
            target_room: prev,
            door_index: 0,
        },
        DoorConfig {
            local_pos: Vec3::new(5.0, 0.0, -5.0),
            rotation: -0.3,
            target_room: next,
            door_index: 1,
        },
    ]
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PortalMaterial {
    #[uniform(0)]
    pub settings: PortalSettings,
    #[texture(1)]
    #[sampler(2)]
    pub view_texture: Handle<Image>,
}

#[derive(Debug, Clone, Copy, Default, bevy::render::render_resource::ShaderType)]
pub struct PortalSettings {
    pub time: f32,
    pub _pad1: f32,
    pub _pad2: f32,
    pub _pad3: f32,
}

impl Material for PortalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/portal_effect.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Component)]
pub struct PortalDoor {
    pub room: usize,
    pub target_room: usize,
    pub door_index: usize,
    pub local_pos: Vec3,
    pub rotation: f32,
    pub is_triggered: bool,
}

#[derive(Component)]
struct PortalCamera {
    source_room: usize,
    target_room: usize,
    door_rotation: f32,
}

#[derive(Component)]
struct PortalFrame;

#[derive(Component)]
struct PortalTrigger;

#[allow(dead_code)]
#[derive(Message, Debug, Clone)]
pub struct PortalCrossingEvent {
    pub from_room: usize,
    pub to_room: usize,
    pub door_index: usize,
}

fn setup_portal_frames(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<PortalState>,
) {
    state.spawned = false;
    state.frames_waited = 0;

    let frame_mat = mats.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.02, 0.02),
        perceptual_roughness: 0.4,
        metallic: 0.05,
        ..default()
    });
    let gold_mat = mats.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.84, 0.0),
        metallic: 0.95,
        perceptual_roughness: 0.15,
        emissive: LinearRgba::new(0.4, 0.28, 0.0, 1.0),
        ..default()
    });
    let glow_mat = mats.add(StandardMaterial {
        base_color: Color::srgba(0.86, 0.08, 0.24, 0.8),
        emissive: LinearRgba::new(3.0, 0.3, 0.5, 1.0),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let pillar = meshes.add(Cuboid::new(0.12, PORTAL_HEIGHT + 0.3, FRAME_DEPTH));
    let lintel = meshes.add(Cuboid::new(PORTAL_WIDTH + 0.3, 0.15, FRAME_DEPTH));
    let trim_v = meshes.add(Cuboid::new(0.04, PORTAL_HEIGHT + 0.2, FRAME_DEPTH + 0.02));
    let trim_h = meshes.add(Cuboid::new(PORTAL_WIDTH + 0.35, 0.04, FRAME_DEPTH + 0.02));
    let glow_ring = meshes.add(Cuboid::new(0.02, PORTAL_HEIGHT - 0.1, 0.02));

    for room in 0..TOTAL_ROOMS {
        let center = room_center(room);
        for door in get_doors(room) {
            let world_pos = center + door.local_pos;
            let rot = Quat::from_rotation_y(door.rotation);
            let half_w = PORTAL_WIDTH / 2.0 + 0.06;

            for x in [-half_w, half_w] {
                let builder = SplatPhysicsBuilder::static_body().with_box(Vec3::new(
                    0.06,
                    (PORTAL_HEIGHT + 0.3) / 2.0,
                    FRAME_DEPTH / 2.0,
                ));
                let (body, rb, collider, mass, fric, rest, linear_damp, angular_damp, locked) =
                    builder.build();
                cmd.spawn((
                    Mesh3d(pillar.clone()),
                    MeshMaterial3d(frame_mat.clone()),
                    Transform::from_translation(
                        world_pos + rot * Vec3::new(x, PORTAL_HEIGHT / 2.0, 0.0),
                    )
                    .with_rotation(rot),
                    RenderLayers::layer(room),
                    PortalFrame,
                    body,
                    rb,
                    collider,
                    mass,
                    fric,
                    rest,
                    linear_damp,
                    angular_damp,
                    locked,
                ));
            }

            let builder = SplatPhysicsBuilder::static_body().with_box(Vec3::new(
                (PORTAL_WIDTH + 0.3) / 2.0,
                0.075,
                FRAME_DEPTH / 2.0,
            ));
            let (body, rb, collider, mass, fric, rest, linear_damp, angular_damp, locked) =
                builder.build();
            cmd.spawn((
                Mesh3d(lintel.clone()),
                MeshMaterial3d(frame_mat.clone()),
                Transform::from_translation(
                    world_pos + rot * Vec3::new(0.0, PORTAL_HEIGHT + 0.08, 0.0),
                )
                .with_rotation(rot),
                RenderLayers::layer(room),
                PortalFrame,
                body,
                rb,
                collider,
                mass,
                fric,
                rest,
                linear_damp,
                angular_damp,
                locked,
            ));

            for x in [-half_w - 0.01, half_w + 0.01] {
                cmd.spawn((
                    Mesh3d(trim_v.clone()),
                    MeshMaterial3d(gold_mat.clone()),
                    Transform::from_translation(
                        world_pos + rot * Vec3::new(x, PORTAL_HEIGHT / 2.0, 0.02),
                    )
                    .with_rotation(rot),
                    RenderLayers::layer(room),
                    PortalFrame,
                ));
            }
            cmd.spawn((
                Mesh3d(trim_h.clone()),
                MeshMaterial3d(gold_mat.clone()),
                Transform::from_translation(
                    world_pos + rot * Vec3::new(0.0, PORTAL_HEIGHT + 0.1, 0.02),
                )
                .with_rotation(rot),
                RenderLayers::layer(room),
                PortalFrame,
            ));
            for x in [-half_w + 0.08, half_w - 0.08] {
                cmd.spawn((
                    Mesh3d(glow_ring.clone()),
                    MeshMaterial3d(glow_mat.clone()),
                    Transform::from_translation(
                        world_pos + rot * Vec3::new(x, PORTAL_HEIGHT / 2.0, 0.06),
                    )
                    .with_rotation(rot),
                    RenderLayers::layer(room),
                    PortalFrame,
                ));
            }
        }
    }
    info!("🚪 Portal frames created with physics");
}

fn spawn_portal_views(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut portal_mats: ResMut<Assets<PortalMaterial>>,
    mut state: ResMut<PortalState>,
    cam_q: Query<Entity, With<PanoramaCamera>>,
) {
    if state.spawned {
        return;
    }
    state.frames_waited += 1;
    if state.frames_waited < 5 {
        return;
    }
    if cam_q.single().is_err() {
        return;
    }

    state.spawned = true;
    let portal_mesh = meshes.add(Rectangle::new(PORTAL_WIDTH, PORTAL_HEIGHT));

    for room in 0..TOTAL_ROOMS {
        let center = room_center(room);
        for (idx, door) in get_doors(room).iter().enumerate() {
            let world_pos = center + door.local_pos;
            let rot = Quat::from_rotation_y(door.rotation);
            let portal_pos = world_pos + rot * Vec3::new(0.0, PORTAL_HEIGHT / 2.0, 0.08);

            let rt = images.add(Image::new_target_texture(
                512,
                1024,
                TextureFormat::bevy_default(),
                Some(TextureFormat::bevy_default()),
            ));

            let target_center = room_center(door.target_room);
            cmd.spawn((
                Camera3d::default(),
                Camera {
                    order: -10 - (room * 2 + idx) as isize,
                    clear_color: Color::srgb(0.01, 0.005, 0.02).into(),
                    target: RenderTarget::Image(rt.clone().into()),
                    ..default()
                },
                Transform::from_translation(target_center)
                    .looking_at(target_center + Vec3::NEG_Z, Vec3::Y),
                RenderLayers::layer(door.target_room),
                PortalCamera {
                    source_room: room,
                    target_room: door.target_room,
                    door_rotation: door.rotation,
                },
                Name::new(format!("PortalCamera_{}_{}", room, idx)),
            ));

            cmd.spawn((
                Mesh3d(portal_mesh.clone()),
                MeshMaterial3d(portal_mats.add(PortalMaterial {
                    settings: PortalSettings::default(),
                    view_texture: rt,
                })),
                Transform::from_translation(portal_pos).with_rotation(rot),
                RenderLayers::layer(room),
                PortalDoor {
                    room,
                    target_room: door.target_room,
                    door_index: door.door_index,
                    local_pos: world_pos,
                    rotation: door.rotation,
                    is_triggered: false,
                },
                PortalTrigger,
                Sensor,
                Collider::cuboid(PORTAL_WIDTH / 2.0, PORTAL_HEIGHT / 2.0, 0.1),
            ));
        }
    }
    info!(
        "🌀 {} portals spawned with physics triggers",
        TOTAL_ROOMS * 2
    );
}

fn sync_portal_cameras(
    player: Res<PlayerState>,
    main_cam: Query<&Transform, (With<PanoramaCamera>, Without<PortalCamera>)>,
    mut portal_cams: Query<(&mut Transform, &PortalCamera), Without<PanoramaCamera>>,
) {
    let Ok(main_tf) = main_cam.single() else {
        return;
    };
    let (yaw, pitch, _) = main_tf.rotation.to_euler(EulerRot::YXZ);

    for (mut cam_tf, portal) in portal_cams.iter_mut() {
        if portal.source_room != player.room {
            continue;
        }

        let target_center = room_center(portal.target_room);
        cam_tf.translation = target_center + Vec3::Y * 1.7;

        let mirrored_yaw = yaw + std::f32::consts::PI + portal.door_rotation * 2.0;
        cam_tf.rotation = Quat::from_euler(EulerRot::YXZ, mirrored_yaw, pitch, 0.0);
    }
}

fn update_portal_time(
    time: Res<Time>,
    mut portal_mats: ResMut<Assets<PortalMaterial>>,
    portals: Query<&MeshMaterial3d<PortalMaterial>>,
) {
    let t = time.elapsed_secs();
    for mat_handle in portals.iter() {
        if let Some(mat) = portal_mats.get_mut(&mat_handle.0) {
            mat.settings.time = t;
        }
    }
}

fn portal_crossing_physics(
    mut player_state: ResMut<PlayerState>,
    mut event_writer: MessageWriter<PortalCrossingEvent>,
    mut portal_query: Query<&mut PortalDoor>,
    player_query: Query<&mut Transform, With<Player>>,
    _spatial_query: SpatialQuery,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation;

    for mut portal in portal_query.iter_mut() {
        if portal.is_triggered || portal.room != player_state.room {
            continue;
        }

        let portal_pos = portal.local_pos;
        let distance = player_pos.distance(portal_pos);

        if distance < 1.5 {
            let normal = Vec3::new(portal.rotation.sin(), 0.0, -portal.rotation.cos());
            let to_player = (player_pos - portal_pos).normalize();
            let dot = to_player.dot(normal);

            if dot > 0.5 {
                portal.is_triggered = true;

                let target_doors = get_doors(portal.target_room);
                let exit_door = &target_doors[portal.door_index];
                let exit_pos = room_center(portal.target_room)
                    + exit_door.local_pos
                    + Vec3::new(0.0, 1.7, 0.0);

                teleport_player(player_query, exit_pos);

                player_state.room = portal.target_room;
                player_state.pos = Vec2::new(exit_pos.x, exit_pos.z);
                player_state.prev_pos = player_state.pos;

                event_writer.write(PortalCrossingEvent {
                    from_room: portal.room,
                    to_room: portal.target_room,
                    door_index: portal.door_index,
                });

                info!(
                    "🌀 Portal {} → Room {}",
                    portal.door_index + 1,
                    portal.target_room + 1
                );

                return;
            }
        }
    }
}

fn detect_portal_trigger(
    mut collision_started_events: MessageReader<CollisionStart>,
    mut collision_ended_events: MessageReader<CollisionEnd>,
    mut portal_query: Query<&mut PortalDoor>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for event in collision_started_events.read() {
        let portal_entity = if event.collider1 == player_entity {
            event.collider2
        } else if event.collider2 == player_entity {
            event.collider1
        } else {
            continue;
        };

        if let Ok(mut portal) = portal_query.get_mut(portal_entity) {
            portal.is_triggered = true;
        }
    }

    for event in collision_ended_events.read() {
        let portal_entity = if event.collider1 == player_entity {
            event.collider2
        } else if event.collider2 == player_entity {
            event.collider1
        } else {
            continue;
        };

        if let Ok(mut portal) = portal_query.get_mut(portal_entity) {
            portal.is_triggered = false;
        }
    }
}
