use crate::GameState;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct PanoramaPlugin;

impl Plugin for PanoramaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Viewing), setup_panorama)
            .add_systems(
                Update,
                (
                    animate_core_glow,
                    animate_light_orbs,
                    animate_energy_wisps,
                    animate_point_lights,
                )
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

// PanoramaCamera is now GameCamera from camera module
pub use crate::camera::GameCamera as PanoramaCamera;

#[derive(Component)]
pub struct PanoramaSphere;

#[derive(Component)]
pub struct CoreGlow {
    pub layer: u32,
}

#[derive(Component)]
pub struct LightOrb {
    pub angle: f32,
    pub speed: f32,
    pub radius: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct EnergyWisp {
    pub id: u32,
}

#[derive(Component)]
pub struct OrbPointLight {
    pub base: f32,
    pub phase: f32,
}

const ORB_Y: f32 = -0.65;

fn setup_panorama(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // === PURE LIGHT ORB ===

    // Tiny bright core
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.02).mesh().ico(2).unwrap())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: LinearRgba::new(50.0, 45.0, 60.0, 1.0),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, ORB_Y, 0.0),
        CoreGlow { layer: 0 },
    ));

    // Soft glow layers
    for i in 1..=4 {
        let size = 0.03 + i as f32 * 0.04;
        let alpha = 0.15 - i as f32 * 0.03;
        let emission = 20.0 - i as f32 * 4.0;

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(size).mesh().ico(2).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.7, 0.5, 1.0, alpha),
                emissive: LinearRgba::new(emission * 0.8, emission * 0.5, emission, 1.0),
                unlit: true,
                alpha_mode: AlphaMode::Add,
                ..default()
            })),
            Transform::from_xyz(0.0, ORB_Y, 0.0),
            CoreGlow { layer: i },
        ));
    }

    // === ORBITING LIGHT POINTS ===
    let tiny_sphere = meshes.add(Sphere::new(0.008).mesh().ico(1).unwrap());

    for i in 0..16 {
        let angle = (i as f32 / 16.0) * PI * 2.0;
        let radius = 0.08 + (i % 3) as f32 * 0.03;
        let speed = 2.0 + (i % 4) as f32 * 0.5;
        let brightness = 25.0 + (i % 5) as f32 * 5.0;

        commands.spawn((
            Mesh3d(tiny_sphere.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: LinearRgba::new(brightness * 0.9, brightness * 0.7, brightness, 1.0),
                unlit: true,
                ..default()
            })),
            Transform::from_xyz(0.0, ORB_Y, 0.0)
                .with_scale(Vec3::splat(0.5 + (i % 3) as f32 * 0.3)),
            LightOrb {
                angle,
                speed,
                radius,
                phase: i as f32 * 0.4,
            },
        ));
    }

    // === ENERGY WISPS ===
    let wisp_mesh = meshes.add(create_wisp_mesh());

    for i in 0..12 {
        let brightness = 30.0 + (i % 4) as f32 * 8.0;

        commands.spawn((
            Mesh3d(wisp_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.9, 1.0, 0.8),
                emissive: LinearRgba::new(brightness, brightness * 0.8, brightness * 1.2, 1.0),
                unlit: true,
                alpha_mode: AlphaMode::Add,
                ..default()
            })),
            Transform::from_xyz(0.0, ORB_Y, 0.0).with_scale(Vec3::splat(0.3)),
            EnergyWisp { id: i },
        ));
    }

    // === POINT LIGHTS ===
    commands.spawn((
        PointLight {
            color: Color::srgb(0.85, 0.7, 1.0),
            intensity: 50000.0,
            radius: 5.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, ORB_Y, 0.0),
        OrbPointLight {
            base: 50000.0,
            phase: 0.0,
        },
    ));

    for i in 0..4 {
        let angle = (i as f32 / 4.0) * PI * 2.0;
        commands.spawn((
            PointLight {
                color: Color::srgb(0.6, 0.4, 1.0),
                intensity: 8000.0,
                radius: 2.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(angle.cos() * 0.1, ORB_Y, angle.sin() * 0.1),
            OrbPointLight {
                base: 8000.0,
                phase: i as f32 * 1.5,
            },
        ));
    }

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Tonemapping::AcesFitted,
        crate::camera::GameCamera,
    ));

    info!("🔮 Light Orb ready - Look down! Press [B] for Book Reader");
}

fn animate_core_glow(
    time: Res<Time>,
    mut cores: Query<(&mut Transform, &CoreGlow)>,
    cam_q: Query<&Transform, (With<PanoramaCamera>, Without<CoreGlow>)>,
) {
    let t = time.elapsed_secs();

    // Get camera yaw for subtle orb rotation response
    let cam_yaw = cam_q
        .single()
        .map(|c| {
            let (yaw, _, _) = c.rotation.to_euler(EulerRot::YXZ);
            yaw
        })
        .unwrap_or(0.0);

    for (mut transform, core) in cores.iter_mut() {
        let phase = t * 3.0 + core.layer as f32 * 0.5;
        let pulse = 1.0 + phase.sin() * 0.15;
        let micro_pulse = 1.0 + (t * 12.0).sin() * 0.05;

        transform.scale = Vec3::splat(pulse * micro_pulse);

        // Subtle drift that responds to camera rotation
        let drift_x = (t * 1.5 + core.layer as f32).sin() * 0.003 + cam_yaw.sin() * 0.002;
        let drift_z = (t * 1.3 + core.layer as f32).sin() * 0.003 + cam_yaw.cos() * 0.002;

        transform.translation.x = drift_x;
        transform.translation.y = ORB_Y + (t * 1.8 + core.layer as f32).cos() * 0.003;
        transform.translation.z = drift_z;
    }
}

fn animate_light_orbs(
    time: Res<Time>,
    mut orbs: Query<(&mut Transform, &mut LightOrb)>,
    cam_q: Query<&Transform, (With<PanoramaCamera>, Without<LightOrb>)>,
) {
    let t = time.elapsed_secs();
    let dt = time.delta_secs();

    // Get camera forward direction for orb orientation
    let cam_yaw = cam_q
        .single()
        .map(|c| {
            let (yaw, _, _) = c.rotation.to_euler(EulerRot::YXZ);
            yaw
        })
        .unwrap_or(0.0);

    for (mut transform, mut orb) in orbs.iter_mut() {
        // Orbit follows camera rotation
        orb.angle += orb.speed * dt;
        let adjusted_angle = orb.angle + cam_yaw * 0.3; // Partial follow

        let wobble = (t * 4.0 + orb.phase).sin() * 0.02;
        let vertical = (t * 2.0 + orb.phase * 2.0).sin() * 0.04;

        transform.translation = Vec3::new(
            adjusted_angle.cos() * (orb.radius + wobble),
            ORB_Y + vertical,
            adjusted_angle.sin() * (orb.radius + wobble),
        );

        let twinkle = 0.6 + (t * 8.0 + orb.phase * 3.0).sin().abs() * 0.4;
        transform.scale = Vec3::splat(twinkle);
    }
}

fn animate_energy_wisps(
    time: Res<Time>,
    mut wisps: Query<(&mut Transform, &mut Visibility, &EnergyWisp)>,
) {
    let t = time.elapsed_secs();

    for (mut transform, mut vis, wisp) in wisps.iter_mut() {
        let id = wisp.id as f32;
        let cycle = (t * 1.5 + id * 0.5) % 3.0;

        if !(0.1..=2.5).contains(&cycle) {
            *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Visible;

            let progress = (cycle - 0.1) / 2.4;
            let angle = id * 0.52 + (t * 0.3).sin() * 0.5;
            let dist = progress * 0.25;

            transform.translation = Vec3::new(
                angle.cos() * dist,
                ORB_Y + (id * 0.3).sin() * 0.05,
                angle.sin() * dist,
            );

            let stretch = 1.0 + progress * 2.0;
            let fade = 1.0 - progress * 0.7;
            transform.scale = Vec3::new(fade * 0.3, fade * 0.3, stretch * 0.5);
            transform.look_to(Vec3::new(angle.cos(), 0.0, angle.sin()), Vec3::Y);
        }
    }
}

fn animate_point_lights(time: Res<Time>, mut lights: Query<(&mut PointLight, &OrbPointLight)>) {
    let t = time.elapsed_secs();
    for (mut light, orb) in lights.iter_mut() {
        let pulse = 1.0 + (t * 3.0 + orb.phase).sin() * 0.3;
        light.intensity = orb.base * pulse;
    }
}

fn create_wisp_mesh() -> Mesh {
    let positions = vec![[0.0, 0.01, 0.0], [0.0, -0.01, 0.0], [0.0, 0.0, 0.05]];
    let normals = vec![[0.0, 0.0, -1.0]; 3];
    let indices = vec![0, 1, 2];

    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}

#[allow(dead_code)]
fn create_panorama_sphere(radius: f32, sectors: u32, stacks: u32) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=stacks {
        let v = i as f32 / stacks as f32;
        let phi = PI * v;
        for j in 0..=sectors {
            let u = j as f32 / sectors as f32;
            let theta = 2.0 * PI * u;
            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.cos();
            let z = radius * phi.sin() * theta.sin();
            positions.push([x, y, z]);
            normals.push([-x / radius, -y / radius, -z / radius]);
            uvs.push([1.0 - u, v]);
        }
    }

    for i in 0..stacks {
        for j in 0..sectors {
            let first = i * (sectors + 1) + j;
            let second = first + sectors + 1;
            indices.push(first);
            indices.push(first + 1);
            indices.push(second);
            indices.push(second);
            indices.push(first + 1);
            indices.push(second + 1);
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}
