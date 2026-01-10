//! Portals module - Portal doors, crossing logic, render textures

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, TextureFormat};
use bevy::shader::ShaderRef;

use crate::panorama::PanoramaCamera;
use crate::player::PlayerState;
use crate::world::{room_center, TOTAL_ROOMS};
use crate::GameState;

pub struct PortalsPlugin;

impl Plugin for PortalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PortalMaterial>::default())
            .insert_resource(PortalState::default())
            .add_systems(OnEnter(GameState::Viewing), setup_portal_frames)
            .add_systems(
                Update,
                (
                    spawn_portal_views,
                    sync_portal_cameras,
                    update_portal_time,
                    portal_crossing,
                )
                    .chain()
                    .run_if(in_state(GameState::Viewing)),
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
    pub door_index: usize, // 0 = left door, 1 = right door
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

/// Portal material with liquid effect
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
    pub local_pos: Vec2,
    pub rotation: f32,
}

#[derive(Component)]
struct PortalCamera {
    source_room: usize,
    target_room: usize,
    door_rotation: f32,
}

#[derive(Component)]
struct PortalFrame;

fn setup_portal_frames(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<PortalState>,
) {
    state.spawned = false;
    state.frames_waited = 0;

    // Chinese red lacquered wood frame
    let frame_mat = mats.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.02, 0.02),
        perceptual_roughness: 0.4,
        metallic: 0.05,
        ..default()
    });
    // Gold trim accents
    let gold_mat = mats.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.84, 0.0),
        metallic: 0.95,
        perceptual_roughness: 0.15,
        emissive: LinearRgba::new(0.4, 0.28, 0.0, 1.0),
        ..default()
    });
    // Red energy glow
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

            // Pillars
            for x in [-half_w, half_w] {
                cmd.spawn((
                    Mesh3d(pillar.clone()),
                    MeshMaterial3d(frame_mat.clone()),
                    Transform::from_translation(
                        world_pos + rot * Vec3::new(x, PORTAL_HEIGHT / 2.0, 0.0),
                    )
                    .with_rotation(rot),
                    RenderLayers::layer(room),
                    PortalFrame,
                ));
            }
            // Lintel
            cmd.spawn((
                Mesh3d(lintel.clone()),
                MeshMaterial3d(frame_mat.clone()),
                Transform::from_translation(
                    world_pos + rot * Vec3::new(0.0, PORTAL_HEIGHT + 0.08, 0.0),
                )
                .with_rotation(rot),
                RenderLayers::layer(room),
                PortalFrame,
            ));
            // Gold trim
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
            // Glow
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
    info!("🚪 Portal frames created");
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
            ));

            // Portal camera - positioned at target room
            let target_center = room_center(door.target_room);
            cmd.spawn((
                Camera3d::default(),
                Camera {
                    target: rt.clone().into(),
                    order: -10 - (room * 2 + idx) as isize,
                    clear_color: Color::srgb(0.01, 0.005, 0.02).into(),
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
            ));

            // Portal surface
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
                    local_pos: Vec2::new(door.local_pos.x, door.local_pos.z),
                    rotation: door.rotation,
                },
            ));
        }
    }
    info!("🌀 {} portals spawned", TOTAL_ROOMS * 2);
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

        // Mirror the view through the portal
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

fn portal_crossing(
    mut cmd: Commands,
    mut player: ResMut<PlayerState>,
    mut cam_q: Query<(Entity, &mut Transform, Option<&mut RenderLayers>), With<PanoramaCamera>>,
    portals: Query<&PortalDoor>,
) {
    let Ok((cam_entity, mut cam, layers_opt)) = cam_q.single_mut() else {
        return;
    };

    for portal in portals.iter() {
        if portal.room != player.room {
            continue;
        }

        let dist = player.pos.distance(portal.local_pos);
        if dist > 1.5 {
            continue;
        }

        // Door normal
        let normal = Vec2::new(portal.rotation.sin(), -portal.rotation.cos());
        let prev_dot = (player.prev_pos - portal.local_pos).dot(normal);
        let curr_dot = (player.pos - portal.local_pos).dot(normal);

        // Check crossing (sign change)
        if prev_dot * curr_dot >= 0.0 {
            continue;
        }

        // Lateral distance check
        let lateral = ((player.pos - portal.local_pos) - normal * curr_dot).length();
        if lateral > PORTAL_WIDTH * 0.6 {
            continue;
        }

        // Exit at SAME door index in target room
        let target_doors = get_doors(portal.target_room);
        let exit_door = &target_doors[portal.door_index];
        let exit_offset = if curr_dot < 0.0 { 1.2 } else { -1.2 };
        let exit_pos = Vec2::new(exit_door.local_pos.x, exit_door.local_pos.z + exit_offset);

        player.room = portal.target_room;
        player.pos = exit_pos;
        player.prev_pos = exit_pos;
        cam.translation =
            room_center(portal.target_room) + Vec3::new(exit_pos.x, player.height, exit_pos.y);

        if let Some(mut layers) = layers_opt {
            *layers = RenderLayers::layer(portal.target_room);
        } else {
            cmd.entity(cam_entity)
                .insert(RenderLayers::layer(portal.target_room));
        }

        info!(
            "🌀 Portal {} → Room {}",
            portal.door_index + 1,
            portal.target_room + 1
        );
        return;
    }
}
