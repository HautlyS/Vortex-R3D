use bevy::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::pbr::Material;
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, FrontFace, PrimitiveState, SpecializedMeshPipelineError,
};
use bevy::shader::ShaderRef;
use bevy_mesh::MeshVertexBufferLayoutRef;

use super::asset::*;
use super::splat_types::*;

pub struct GaussianSplatRenderPlugin;

impl Plugin for GaussianSplatRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_splat_clouds,
                update_splat_meshes,
                apply_splat_materials,
                cull_distant_splats,
            ),
        )
        .init_resource::<SplatMeshCache>()
        .register_type::<SplatMeshConfig>();
    }
}

#[derive(Resource, Default)]
pub struct SplatMeshCache {
    pub base_mesh: Option<Handle<Mesh>>,
    pub lod_meshes: Vec<Handle<Mesh>>,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct SplatMeshConfig {
    pub billboard_size: f32,
    pub enable_sh: bool,
    pub cutoff_alpha: f32,
    pub depth_sort: bool,
}

impl Default for SplatMeshConfig {
    fn default() -> Self {
        Self {
            billboard_size: 1.0,
            enable_sh: true,
            cutoff_alpha: 0.01,
            depth_sort: true,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SplatMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub opacity: f32,
    #[uniform(0)]
    pub roughness: f32,
    #[uniform(0)]
    pub metallic: f32,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub normal_map: Option<Handle<Image>>,
}

impl Default for SplatMaterial {
    fn default() -> Self {
        Self {
            base_color: LinearRgba::WHITE,
            opacity: 1.0,
            roughness: 0.5,
            metallic: 0.0,
            color_texture: None,
            normal_map: None,
        }
    }
}

impl Material for SplatMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/gaussian_splat.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/gaussian_splat.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: bevy::render::render_resource::PolygonMode::Fill,
            conservative: false,
        };
        Ok(())
    }
}

#[derive(Component)]
pub struct SplatCloudInstance {
    pub cloud_handle: Handle<GaussianSplatCloud>,
    pub spawned: bool,
    pub splat_entities: Vec<Entity>,
}

fn spawn_splat_clouds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SplatMaterial>>,
    cloud_assets: Res<Assets<GaussianSplatCloud>>,
    query: Query<(Entity, &SplatCloudInstance, &Transform), Without<SplatMeshConfig>>,
    mut mesh_cache: ResMut<SplatMeshCache>,
) {
    if mesh_cache.base_mesh.is_none() {
        let base_mesh = create_billboard_mesh(&mut meshes);
        mesh_cache.base_mesh = Some(base_mesh);

        for lod in 0..4 {
            let lod_mesh = create_lod_billboard_mesh(&mut meshes, lod);
            mesh_cache.lod_meshes.push(lod_mesh);
        }
    }

    for (entity, instance, cloud_transform) in query.iter() {
        if instance.spawned {
            continue;
        }

        if let Some(cloud) = cloud_assets.get(&instance.cloud_handle) {
            let mut splat_entities = Vec::new();

            let batch_size = 200;
            let batches = cloud.splats.chunks(batch_size);
            let cloud_center = cloud_transform.translation;

            for batch in batches {
                for splat_data in batch {
                    let color = LinearRgba::new(
                        splat_data.color[0] as f32 / 255.0,
                        splat_data.color[1] as f32 / 255.0,
                        splat_data.color[2] as f32 / 255.0,
                        splat_data.opacity,
                    );

                    let material = materials.add(SplatMaterial {
                        base_color: color,
                        opacity: splat_data.opacity,
                        roughness: 0.5,
                        metallic: 0.0,
                        ..default()
                    });

                    let world_pos = cloud_center + splat_data.position;
                    let render_scale = splat_data.scale * 5.0;

                    let splat_entity = commands
                        .spawn((
                            GaussianSplatBundle {
                                splat: GaussianSplat::new(world_pos, color.into())
                                    .with_scale(render_scale)
                                    .with_opacity(splat_data.opacity),
                                transform: Transform {
                                    translation: world_pos,
                                    rotation: splat_data.rotation,
                                    scale: render_scale,
                                },
                                ..default()
                            },
                            Mesh3d(mesh_cache.base_mesh.clone().unwrap()),
                            MeshMaterial3d(material),
                            SplatMeshConfig::default(),
                        ))
                        .id();

                    splat_entities.push(splat_entity);
                }
            }

            let count = splat_entities.len();
            commands.entity(entity).insert(SplatCloudInstance {
                cloud_handle: instance.cloud_handle.clone(),
                spawned: true,
                splat_entities,
            });

            info!("🎨 Spawned {} splats from cloud", count);
        }
    }
}

fn create_billboard_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());

    let vertices: Vec<[f32; 3]> = vec![
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.5, 0.5, 0.0],
        [-0.5, 0.5, 0.0],
    ];

    let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    let normals: Vec<[f32; 3]> = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];

    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));

    meshes.add(mesh)
}

fn create_lod_billboard_mesh(meshes: &mut ResMut<Assets<Mesh>>, lod: u32) -> Handle<Mesh> {
    let size = 0.5 * (lod as f32 + 1.0);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());

    let vertices: Vec<[f32; 3]> = vec![
        [-size, -size, 0.0],
        [size, -size, 0.0],
        [size, size, 0.0],
        [-size, size, 0.0],
    ];

    let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    let normals: Vec<[f32; 3]> = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];

    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));

    meshes.add(mesh)
}

fn update_splat_meshes(
    mut commands: Commands,
    splat_query: Query<(Entity, &GaussianSplat, &SplatLOD, &Mesh3d), Changed<SplatLOD>>,
    mesh_cache: Res<SplatMeshCache>,
) {
    for (entity, _splat, lod, _current_mesh) in splat_query.iter() {
        let lod_idx = (lod.level as usize).min(mesh_cache.lod_meshes.len() - 1);
        if let Some(mesh_handle) = mesh_cache.lod_meshes.get(lod_idx) {
            commands.entity(entity).insert(Mesh3d(mesh_handle.clone()));
        }
    }
}

fn apply_splat_materials(
    mut materials: ResMut<Assets<SplatMaterial>>,
    splat_query: Query<(&GaussianSplat, &MeshMaterial3d<SplatMaterial>), Changed<GaussianSplat>>,
) {
    for (splat, material_handle) in splat_query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.opacity = splat.opacity;
            material.base_color = LinearRgba::from(splat.color);
            material.roughness = 1.0 - splat.solidity;
        }
    }
}

fn cull_distant_splats(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    splat_query: Query<(Entity, &Transform, &GaussianSplat), Without<Camera>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    let camera_pos = camera_transform.translation;

    for (entity, transform, splat) in splat_query.iter() {
        let distance = transform.translation.distance(camera_pos);

        if distance > splat.cull_distance {
            commands.entity(entity).insert(Visibility::Hidden);
        } else {
            commands.entity(entity).insert(Visibility::Visible);
        }
    }
}

pub fn spawn_gaussian_cloud(
    commands: &mut Commands,
    cloud_handle: Handle<GaussianSplatCloud>,
    transform: Transform,
) -> Entity {
    commands
        .spawn((
            SplatCloudInstance {
                cloud_handle,
                spawned: false,
                splat_entities: Vec::new(),
            },
            transform,
            GlobalTransform::from(transform),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id()
}
