//! 3D scene setup for upload room

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::f32::consts::PI;

use super::{UploadSphere, UploadModel};

#[derive(Component)]
pub struct AmbientOrb;

pub fn setup_upload_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sky sphere
    let sphere = meshes.add(create_sphere(50.0, 64, 32));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.08, 0.15),
        unlit: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn((Mesh3d(sphere), MeshMaterial3d(mat), UploadSphere));

    // Floor grid
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(20.0)));
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.1, 0.15, 0.25, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn((
        Mesh3d(floor),
        MeshMaterial3d(floor_mat),
        Transform::from_xyz(0.0, -2.0, 0.0),
    ));

    // Ambient orb
    let orb = meshes.add(Sphere::new(0.3));
    let orb_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.6, 1.0),
        emissive: LinearRgba::new(0.5, 0.8, 1.5, 1.0),
        ..default()
    });
    commands.spawn((
        Mesh3d(orb),
        MeshMaterial3d(orb_mat),
        Transform::from_xyz(0.0, 0.5, -3.0),
        AmbientOrb,
    ));

    // Lights
    commands.spawn((
        PointLight {
            intensity: 300000.0,
            color: Color::srgb(0.6, 0.8, 1.0),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));
    commands.spawn((
        PointLight {
            intensity: 100000.0,
            color: Color::srgb(0.8, 0.5, 1.0),
            ..default()
        },
        Transform::from_xyz(-3.0, 2.0, -2.0),
    ));
    commands.spawn((
        PointLight {
            intensity: 100000.0,
            color: Color::srgb(0.5, 1.0, 0.8),
            ..default()
        },
        Transform::from_xyz(3.0, 2.0, -2.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
        crate::camera::GameCamera,
    ));

    info!("📤 Upload Room ready");
}

pub fn rotate_ambient_light(time: Res<Time>, mut orbs: Query<&mut Transform, With<AmbientOrb>>) {
    for mut t in &mut orbs {
        t.translation.y = 0.5 + (time.elapsed_secs() * 0.5).sin() * 0.2;
        t.rotate_y(time.delta_secs() * 0.3);
    }
}

fn create_sphere(radius: f32, sectors: u32, stacks: u32) -> Mesh {
    let mut pos = Vec::new();
    let mut norm = Vec::new();
    let mut uv = Vec::new();
    let mut idx = Vec::new();

    for i in 0..=stacks {
        let v = i as f32 / stacks as f32;
        let phi = PI * v;
        for j in 0..=sectors {
            let u = j as f32 / sectors as f32;
            let theta = 2.0 * PI * u;
            let (x, y, z) = (
                radius * phi.sin() * theta.cos(),
                radius * phi.cos(),
                radius * phi.sin() * theta.sin(),
            );
            pos.push([x, y, z]);
            norm.push([x / radius, y / radius, z / radius]);
            uv.push([1.0 - u, v]);
        }
    }

    for i in 0..stacks {
        for j in 0..sectors {
            let a = i * (sectors + 1) + j;
            let b = a + sectors + 1;
            idx.extend([a, a + 1, b, b, a + 1, b + 1]);
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, norm)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
        .with_inserted_indices(Indices::U32(idx))
}
