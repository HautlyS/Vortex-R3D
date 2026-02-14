use bevy::camera::primitives::Aabb;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GaussianSplat {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub color: Color,
    pub opacity: f32,
    pub solidity: f32,
    pub texture_weight: f32,
    pub covariance: Mat3,
    pub spherical_harmonics: [f32; 9],
    pub time_alive: f32,
    pub stability: f32,
    pub cull_distance: f32,
    pub lod_distances: [f32; 3],
    pub physics_enabled: bool,
    pub render_priority: u32,
}

impl Default for GaussianSplat {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE * 0.1,
            color: Color::WHITE,
            opacity: 1.0,
            solidity: 0.5,
            texture_weight: 1.0,
            covariance: Mat3::IDENTITY,
            spherical_harmonics: [0.0; 9],
            time_alive: 0.0,
            stability: 1.0,
            cull_distance: 100.0,
            lod_distances: [5.0, 15.0, 50.0],
            physics_enabled: true,
            render_priority: 0,
        }
    }
}

impl GaussianSplat {
    pub fn new(position: Vec3, color: Color) -> Self {
        Self {
            position,
            color,
            ..default()
        }
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn with_solidity(mut self, solidity: f32) -> Self {
        self.solidity = solidity.clamp(0.0, 1.0);
        self
    }

    pub fn with_texture_weight(mut self, weight: f32) -> Self {
        self.texture_weight = weight.clamp(0.0, 1.0);
        self
    }

    pub fn update_stability(&mut self) {
        self.stability =
            (self.stability * 0.95 + self.calculate_current_stability() * 0.05).clamp(0.0, 1.0);
    }

    fn calculate_current_stability(&self) -> f32 {
        let scale_factor = 1.0 - (self.scale.length() - 0.1).abs();
        let opacity_factor = self.opacity;
        let time_factor = (self.time_alive / 5.0).min(1.0);
        (scale_factor + opacity_factor + time_factor) / 3.0
    }

    pub fn on_collision(&mut self) {
        self.solidity = (self.solidity * 1.1).min(1.0);
        self.render_priority += 1;
    }

    pub fn on_collision_end(&mut self) {
        self.render_priority = self.render_priority.saturating_sub(1);
    }

    pub fn calculate_covariance(&self) -> Mat3 {
        let rot_matrix = Mat3::from_quat(self.rotation);
        let scale_matrix = Mat3::from_diagonal(self.scale);
        rot_matrix * scale_matrix * scale_matrix.transpose() * rot_matrix.transpose()
    }

    pub fn get_world_radius(&self) -> f32 {
        (self.scale.length() * 3.0).max(0.001)
    }

    pub fn get_effective_opacity(&self, view_distance: f32) -> f32 {
        let distance_fade = 1.0 - (view_distance / self.cull_distance).clamp(0.0, 1.0);
        self.opacity * distance_fade * self.texture_weight
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct SplatLOD {
    pub level: u8,
}

impl Default for SplatLOD {
    fn default() -> Self {
        Self { level: 0 }
    }
}

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct SplatCluster {
    pub splats: Vec<Entity>,
    pub bounding_box: Aabb,
    pub cluster_id: u32,
    #[allow(dead_code)]
    pub importance: f32,
}

#[allow(dead_code)]
impl SplatCluster {
    pub fn new(cluster_id: u32) -> Self {
        Self {
            splats: Vec::new(),
            bounding_box: Aabb::from_min_max(Vec3::ZERO, Vec3::ONE),
            cluster_id,
            importance: 1.0,
        }
    }

    pub fn add_splat(&mut self, entity: Entity, position: Vec3, radius: f32) {
        self.splats.push(entity);
        self.update_bounding_box(position, radius);
    }

    fn update_bounding_box(&mut self, position: Vec3, radius: f32) {
        let min = position - Vec3::splat(radius);
        let max = position + Vec3::splat(radius);

        if self.splats.len() == 1 {
            self.bounding_box = Aabb::from_min_max(min, max);
        } else {
            let current_min: Vec3 = self.bounding_box.min().into();
            let current_max: Vec3 = self.bounding_box.max().into();
            let new_min = current_min.min(min);
            let new_max = current_max.max(max);
            self.bounding_box = Aabb::from_min_max(new_min, new_max);
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct SplatSettings {
    pub max_splats_per_frame: usize,
    pub lod_bias: f32,
    pub enable_physics: bool,
    pub enable_culling: bool,
    pub splat_size_multiplier: f32,
    pub texture_quality: TextureQuality,
}

impl Default for SplatSettings {
    fn default() -> Self {
        Self {
            max_splats_per_frame: 10000,
            lod_bias: 1.0,
            enable_physics: true,
            enable_culling: true,
            splat_size_multiplier: 1.0,
            texture_quality: TextureQuality::High,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureQuality {
    Low = 0,
    Medium = 1,
    High = 2,
    Ultra = 3,
}

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct SplatTexture {
    pub base_color: Handle<Image>,
    pub normal_map: Option<Handle<Image>>,
    pub roughness: f32,
    pub metallic: f32,
}

impl Default for SplatTexture {
    fn default() -> Self {
        Self {
            base_color: Handle::default(),
            normal_map: None,
            roughness: 0.5,
            metallic: 0.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct GaussianSplatBundle {
    pub splat: GaussianSplat,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl GaussianSplatBundle {
    pub fn new(position: Vec3, color: Color) -> Self {
        Self {
            splat: GaussianSplat::new(position, color),
            transform: Transform::from_translation(position),
            ..default()
        }
    }
}

#[allow(dead_code)]
#[derive(Message, Debug, Clone)]
pub struct SplatSpawnEvent {
    pub position: Vec3,
    pub color: Color,
    pub scale: Vec3,
    pub count: usize,
    pub spread: f32,
}

impl Default for SplatSpawnEvent {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            color: Color::WHITE,
            scale: Vec3::ONE * 0.1,
            count: 1,
            spread: 0.0,
        }
    }
}

#[allow(dead_code)]
#[derive(Message, Debug, Clone)]
pub struct SplatCollisionEvent {
    pub entity: Entity,
    pub other_entity: Entity,
    pub impact_velocity: Vec3,
    pub impact_force: f32,
}

#[allow(dead_code)]
pub const SPLAT_SIZE_BASE: f32 = 0.05;
#[allow(dead_code)]
pub const SPLAT_MAX_OPACITY: f32 = 1.0;
#[allow(dead_code)]
pub const SPLAT_MIN_OPACITY: f32 = 0.01;
pub const SPLAT_PHYSICS_DENSITY: f32 = 100.0;
pub const SPLAT_PHYSICS_FRICTION: f32 = 0.5;
pub const SPLAT_PHYSICS_RESTITUTION: f32 = 0.3;
