//! World module - Room setup with Gaussian Splat environments and physics
//! Loads real splat files from assets/splats/

use avian3d::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::gaussian_splat::{
    create_splat_ground, create_splat_wall, generate_procedural_room_splats, spawn_gaussian_cloud,
    GaussianSplatCloud, SplatCloudInstance,
};
use crate::loading::PanoramaAssets;
use crate::player::PlayerState;
use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldConfig::default())
            .insert_resource(SkyboxRotation::default())
            .insert_resource(SplatSpawnConfig::default())
            .add_systems(OnEnter(GameState::Viewing), setup_world)
            .add_systems(
                Update,
                skybox_rotation_input.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(Update, rotate_skybox.run_if(in_state(GameState::Viewing)))
            .add_systems(
                Update,
                update_splat_world.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                spawn_loaded_splat_clouds.run_if(in_state(GameState::Viewing)),
            )
            .add_systems(
                Update,
                ensure_ground_collision.run_if(in_state(GameState::Viewing)),
            );
    }
}

pub const ROOM_OFFSET: f32 = 500.0;
pub const TOTAL_ROOMS: usize = 3;

#[derive(Resource)]
pub struct WorldConfig {
    pub sky_sphere_radius: f32,
    pub splat_density: f32,
    pub enable_procedural_splats: bool,
    pub enable_real_splats: bool,
    #[allow(dead_code)]
    pub ground_height: f32,
    pub ground_size: Vec2,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            sky_sphere_radius: 80.0,
            splat_density: 2.0,
            enable_procedural_splats: true,
            enable_real_splats: true,
            ground_height: 0.0,
            ground_size: Vec2::new(50.0, 50.0),
        }
    }
}

#[derive(Resource)]
pub struct SplatSpawnConfig {
    pub real_splat_loaded: bool,
    pub procedural_spawned: bool,
    pub ground_created: bool,
}

impl Default for SplatSpawnConfig {
    fn default() -> Self {
        Self {
            real_splat_loaded: false,
            procedural_spawned: false,
            ground_created: false,
        }
    }
}

#[derive(Resource)]
pub struct SkyboxRotation {
    pub enabled: bool,
    pub speed: f32,
    pub angle: f32,
    pub speed_level: u8,
}

impl Default for SkyboxRotation {
    fn default() -> Self {
        Self {
            enabled: false,
            speed: 0.2,
            angle: 0.0,
            speed_level: 1,
        }
    }
}

#[derive(Component)]
pub struct Skybox {
    pub room: usize,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct SplatEnvironment {
    pub room: usize,
    pub cloud_entity: Option<Entity>,
    pub is_real_splat: bool,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct GroundPlane {
    pub room: usize,
}

pub fn room_center(room: usize) -> Vec3 {
    Vec3::new(room as f32 * ROOM_OFFSET, 0.0, 0.0)
}

fn setup_world(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut splat_clouds: ResMut<Assets<GaussianSplatCloud>>,
    asset_server: Res<AssetServer>,
    pano: Res<PanoramaAssets>,
    config: Res<WorldConfig>,
    mut spawn_config: ResMut<SplatSpawnConfig>,
) {
    let panos = [
        &pano.demo_panorama,
        &pano.demo2_panorama,
        &pano.demo3_panorama,
    ];
    let sky_mesh = meshes.add(create_sky_sphere(config.sky_sphere_radius));

    cmd.spawn((
        AmbientLight {
            color: Color::WHITE,
            brightness: 600.0,
            affects_lightmapped_meshes: true,
        },
        RenderLayers::from_layers(&[0, 1, 2]),
    ));

    cmd.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.98, 0.95),
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::from_layers(&[0, 1, 2]),
    ));

    let real_splat_handle: Handle<GaussianSplatCloud> = asset_server.load("splats/techno.ply");

    for (room, pano) in panos.iter().enumerate().take(TOTAL_ROOMS) {
        let center = room_center(room);

        cmd.spawn((
            Mesh3d(sky_mesh.clone()),
            MeshMaterial3d(mats.add(StandardMaterial {
                base_color_texture: Some((*pano).clone()),
                unlit: true,
                double_sided: true,
                cull_mode: None,
                ..default()
            })),
            Transform::from_translation(center),
            RenderLayers::layer(room),
            Skybox { room },
        ));

        create_walkable_ground(
            &mut cmd,
            &mut meshes,
            &mut mats,
            center,
            config.ground_size,
            room,
        );
        spawn_config.ground_created = true;

        if room == 0 && config.enable_real_splats {
            info!("🎨 Loading real Gaussian Splat from assets/splats/techno.ply");

            let cloud_entity = spawn_gaussian_cloud(
                &mut cmd,
                real_splat_handle.clone(),
                Transform::from_translation(center),
            );

            cmd.entity(cloud_entity).insert((
                SplatEnvironment {
                    room,
                    cloud_entity: Some(cloud_entity),
                    is_real_splat: true,
                },
                RenderLayers::layer(room),
                Name::new("RealSplatEnvironment"),
            ));

            info!("🎯 Real splat cloud spawned - will load when asset is ready");
            spawn_config.real_splat_loaded = true;
        }

        if config.enable_procedural_splats && (room > 0 || !config.enable_real_splats) {
            spawn_procedural_room_splats(
                &mut cmd,
                &mut splat_clouds,
                center,
                room,
                config.splat_density,
            );
            spawn_config.procedural_spawned = true;
        }
    }

    info!(
        "🌍 World initialized: Real splats enabled={}, Procedural enabled={}",
        config.enable_real_splats, config.enable_procedural_splats
    );
    info!("📋 Controls: WASD to move, Mouse to look, Space to jump, CTRL+R for camera spin");
}

fn create_walkable_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mats: &mut ResMut<Assets<StandardMaterial>>,
    center: Vec3,
    size: Vec2,
    room: usize,
) {
    let ground_y = 0.0;

    let ground_mesh = meshes.add(Cuboid::new(size.x * 2.0, 0.2, size.y * 2.0));
    let ground_mat = mats.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.15),
        ..default()
    });

    commands.spawn((
        Mesh3d(ground_mesh),
        MeshMaterial3d(ground_mat),
        Transform::from_translation(center + Vec3::new(0.0, ground_y - 0.1, 0.0)),
        RenderLayers::layer(room),
        GroundPlane { room },
        Name::new(format!("Ground_Room_{}", room)),
    ));

    create_splat_ground(commands, center + Vec3::new(0.0, ground_y - 0.1, 0.0), size);

    let wall_height = 5.0;
    let half_size = Vec3::new(size.x, wall_height / 2.0, 1.0);

    create_splat_wall(
        commands,
        center + Vec3::new(0.0, wall_height / 2.0, -size.y),
        half_size,
    );

    create_splat_wall(
        commands,
        center + Vec3::new(0.0, wall_height / 2.0, size.y),
        half_size,
    );

    create_splat_wall(
        commands,
        center + Vec3::new(size.x, wall_height / 2.0, 0.0),
        Vec3::new(1.0, wall_height / 2.0, size.y),
    );

    create_splat_wall(
        commands,
        center + Vec3::new(-size.x, wall_height / 2.0, 0.0),
        Vec3::new(1.0, wall_height / 2.0, size.y),
    );

    info!(
        "✅ Walkable ground created for room {} at y={}",
        room, ground_y
    );
}

fn spawn_procedural_room_splats(
    commands: &mut Commands,
    splat_clouds: &mut ResMut<Assets<GaussianSplatCloud>>,
    center: Vec3,
    room: usize,
    density: f32,
) {
    let room_configs = [
        (Vec3::new(20.0, 8.0, 20.0), Color::srgb(0.2, 0.15, 0.1), 0.5),
        (
            Vec3::new(24.0, 10.0, 24.0),
            Color::srgb(0.1, 0.1, 0.15),
            0.4,
        ),
        (
            Vec3::new(30.0, 12.0, 30.0),
            Color::srgb(0.15, 0.05, 0.2),
            0.3,
        ),
    ];

    let (room_size, base_color, density_mult) = room_configs[room];

    let room_cloud =
        generate_procedural_room_splats(room_size, density * density_mult, base_color, 0.2);

    let cloud_handle = splat_clouds.add(room_cloud);
    let cloud_entity =
        spawn_gaussian_cloud(commands, cloud_handle, Transform::from_translation(center));

    commands.entity(cloud_entity).insert((
        SplatEnvironment {
            room,
            cloud_entity: Some(cloud_entity),
            is_real_splat: false,
        },
        RenderLayers::layer(room),
        Name::new(format!("ProceduralSplatEnvironment_Room_{}", room)),
    ));
}

fn spawn_loaded_splat_clouds(
    cloud_assets: Res<Assets<GaussianSplatCloud>>,
    query: Query<(&SplatCloudInstance, &SplatEnvironment)>,
) {
    for (instance, env) in query.iter() {
        if instance.spawned {
            continue;
        }

        if let Some(cloud) = cloud_assets.get(&instance.cloud_handle) {
            info!(
                "📦 Splat cloud loaded with {} splats in {} clusters for room {}",
                cloud.splats.len(),
                cloud.cluster_count,
                env.room
            );
        }
    }
}

fn ensure_ground_collision(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    player_query: Query<(Entity, &Transform), With<crate::player::Player>>,
    config: Res<SplatSpawnConfig>,
) {
    if !config.ground_created {
        return;
    }

    for (player_entity, transform) in player_query.iter() {
        let ray_origin = transform.translation;
        let ray_dir = Dir3::NEG_Y;
        let max_distance = 5.0;

        match spatial_query.cast_ray(
            ray_origin,
            ray_dir,
            max_distance,
            true,
            &SpatialQueryFilter::default().with_excluded_entities([player_entity]),
        ) {
            Some(hit) => if hit.distance < 2.0 {},
            None => {
                warn!("⚠️ Player not detecting ground! Creating emergency ground...");
                create_splat_ground(
                    &mut commands,
                    Vec3::new(transform.translation.x, -0.5, transform.translation.z),
                    Vec2::new(10.0, 10.0),
                );
            }
        }
    }
}

fn skybox_rotation_input(keys: Res<ButtonInput<KeyCode>>, mut rotation: ResMut<SkyboxRotation>) {
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

    if ctrl && keys.just_pressed(KeyCode::KeyT) {
        rotation.enabled = !rotation.enabled;
        info!(
            "🌀 Skybox rotation: {}",
            if rotation.enabled { "ON" } else { "OFF" }
        );
    }

    if ctrl {
        if keys.just_pressed(KeyCode::Digit1) {
            rotation.speed_level = 1;
            rotation.speed = 0.1;
            info!("🌀 Rotation speed: 1 (slow)");
        }
        if keys.just_pressed(KeyCode::Digit2) {
            rotation.speed_level = 2;
            rotation.speed = 0.3;
            info!("🌀 Rotation speed: 2 (medium)");
        }
        if keys.just_pressed(KeyCode::Digit3) {
            rotation.speed_level = 3;
            rotation.speed = 0.6;
            info!("🌀 Rotation speed: 3 (fast)");
        }
        if keys.just_pressed(KeyCode::Digit4) {
            rotation.speed_level = 4;
            rotation.speed = 1.2;
            info!("🌀 Rotation speed: 4 (very fast)");
        }
    }
}

fn rotate_skybox(
    time: Res<Time>,
    mut rotation: ResMut<SkyboxRotation>,
    player: Res<PlayerState>,
    mut skyboxes: Query<(&mut Transform, &Skybox)>,
) {
    if !rotation.enabled {
        return;
    }

    rotation.angle += rotation.speed * time.delta_secs();
    if rotation.angle > PI * 2.0 {
        rotation.angle -= PI * 2.0;
    }

    for (mut transform, skybox) in skyboxes.iter_mut() {
        if skybox.room == player.room {
            let center = room_center(skybox.room);
            transform.translation = center;
            transform.rotation = Quat::from_rotation_y(rotation.angle);
        }
    }
}

fn update_splat_world(
    player: Res<PlayerState>,
    mut environments: Query<(&SplatEnvironment, &mut Visibility)>,
) {
    for (env, mut visibility) in environments.iter_mut() {
        if env.room == player.room {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn create_sky_sphere(r: f32) -> Mesh {
    let (sec, stk) = (96u32, 48u32);
    let mut pos = Vec::new();
    let mut nrm = Vec::new();
    let mut uv = Vec::new();
    let mut idx = Vec::new();

    for i in 0..=stk {
        let v = i as f32 / stk as f32;
        let phi = PI * v;
        for j in 0..=sec {
            let u = j as f32 / sec as f32;
            let th = 2.0 * PI * u;
            let (x, y, z) = (
                r * phi.sin() * th.cos(),
                r * phi.cos(),
                r * phi.sin() * th.sin(),
            );
            pos.push([x, y, z]);
            nrm.push([-x / r, -y / r, -z / r]);
            uv.push([1.0 - u, v]);
        }
    }
    for i in 0..stk {
        for j in 0..sec {
            let a = i * (sec + 1) + j;
            let b = a + sec + 1;
            idx.extend([a, a + 1, b, b, a + 1, b + 1]);
        }
    }
    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nrm)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
        .with_inserted_indices(Indices::U32(idx))
}
