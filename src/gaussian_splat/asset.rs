use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use bevy::camera::primitives::Aabb;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;

pub struct GaussianSplatAssetPlugin;

impl Plugin for GaussianSplatAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GaussianSplatCloud>()
            .init_asset_loader::<PlySplatLoader>()
            .init_asset_loader::<SplatLoader>()
            .register_type::<GaussianSplatCloud>();
    }
}

#[derive(Asset, Debug, Clone, Reflect)]
pub struct GaussianSplatCloud {
    pub splats: Vec<SplatData>,
    pub bounding_box: Aabb,
    pub cluster_count: u32,
    pub metadata: SplatMetadata,
}

impl GaussianSplatCloud {
    pub fn new() -> Self {
        Self {
            splats: Vec::new(),
            bounding_box: Aabb::from_min_max(Vec3::ZERO, Vec3::ONE),
            cluster_count: 0,
            metadata: SplatMetadata::default(),
        }
    }
    
    pub fn from_splats(splats: Vec<SplatData>) -> Self {
        let mut cloud = Self {
            splats,
            ..default()
        };
        cloud.calculate_bounding_box();
        cloud
    }
    
    fn calculate_bounding_box(&mut self) {
        if self.splats.is_empty() {
            return;
        }
        
        let mut min = self.splats[0].position;
        let mut max = self.splats[0].position;
        
        for splat in &self.splats {
            min = min.min(splat.position);
            max = max.max(splat.position);
        }
        
        self.bounding_box = Aabb::from_min_max(min, max);
    }
    
    pub fn cluster_splats(&mut self, max_cluster_size: usize) {
        if self.splats.is_empty() {
            return;
        }
        
        let mut clusters: Vec<Vec<usize>> = Vec::new();
        let mut unassigned: Vec<usize> = (0..self.splats.len()).collect();
        
        while !unassigned.is_empty() {
            let mut cluster = Vec::new();
            let seed = unassigned[0];
            cluster.push(seed);
            unassigned.remove(0);
            
            let seed_pos = self.splats[seed].position;
            
            let mut i = 0;
            while i < unassigned.len() && cluster.len() < max_cluster_size {
                let idx = unassigned[i];
                let dist = self.splats[idx].position.distance(seed_pos);
                
                if dist < 1.0 {
                    cluster.push(idx);
                    unassigned.remove(i);
                } else {
                    i += 1;
                }
            }
            
            clusters.push(cluster);
        }
        
        self.cluster_count = clusters.len() as u32;
        
        for (cluster_id, cluster) in clusters.iter().enumerate() {
            for &idx in cluster {
                self.splats[idx].cluster_id = cluster_id as u32;
            }
        }
    }
}

impl Default for GaussianSplatCloud {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct SplatData {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
    pub color: [u8; 4],
    pub spherical_harmonics: [f32; 9],
    pub opacity: f32,
    pub cluster_id: u32,
}

impl Default for SplatData {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            scale: Vec3::ONE * 0.01,
            rotation: Quat::IDENTITY,
            color: [255, 255, 255, 255],
            spherical_harmonics: [0.0; 9],
            opacity: 1.0,
            cluster_id: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct SplatMetadata {
    pub version: String,
    pub point_count: usize,
    pub has_normals: bool,
    pub has_colors: bool,
    pub format: SplatFormat,
}

impl Default for SplatMetadata {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            point_count: 0,
            has_normals: false,
            has_colors: true,
            format: SplatFormat::Ply,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub enum SplatFormat {
    Ply,
    Splat,
    PlyCompressed,
}

#[derive(Default, TypePath)]
pub struct PlySplatLoader;

impl AssetLoader for PlySplatLoader {
    type Asset = GaussianSplatCloud;
    type Settings = ();
    type Error = io::Error;
    
    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> impl std::future::Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let mut buffer = Vec::new();
            futures_lite::AsyncReadExt::read_to_end(reader, &mut buffer).await?;
            parse_ply_data(&buffer)
        }
    }
    
    fn extensions(&self) -> &[&str] {
        &["ply"]
    }
}

#[derive(Default, TypePath)]
pub struct SplatLoader;

impl AssetLoader for SplatLoader {
    type Asset = GaussianSplatCloud;
    type Settings = ();
    type Error = io::Error;
    
    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> impl std::future::Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let mut buffer = Vec::new();
            futures_lite::AsyncReadExt::read_to_end(reader, &mut buffer).await?;
            parse_splat_data(&buffer)
        }
    }
    
    fn extensions(&self) -> &[&str] {
        &["splat"]
    }
}

fn parse_ply_data(data: &[u8]) -> io::Result<GaussianSplatCloud> {
    let mut cloud = GaussianSplatCloud::new();
    let content = String::from_utf8_lossy(data);
    
    let mut vertex_count: usize = 0;
    let mut header_end = 0;
    
    for (idx, line) in content.lines().enumerate() {
        if line.starts_with("element vertex") {
            vertex_count = line.split_whitespace()
                .nth(2)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        } else if line == "end_header" {
            header_end = idx + 1;
            break;
        }
    }
    
    cloud.metadata.point_count = vertex_count;
    cloud.metadata.has_colors = true;
    
    let data_start = content.lines()
        .take(header_end)
        .map(|l| l.len() + 1)
        .sum::<usize>();
    
    let data_slice = &data[data_start..];
    let mut cursor = io::Cursor::new(data_slice);
    
    let max_splats = vertex_count.min(50000);
    
    for _ in 0..max_splats {
        let x = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let y = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let z = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        
        let _nx = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let _ny = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let _nz = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        
        let f_dc_0 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let f_dc_1 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let f_dc_2 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        
        let opacity_raw = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        
        let scale_0 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let scale_1 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let scale_2 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        
        let rot_0 = cursor.read_f32::<LittleEndian>().unwrap_or(1.0);
        let rot_1 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let rot_2 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        let rot_3 = cursor.read_f32::<LittleEndian>().unwrap_or(0.0);
        
        let r = (f_dc_0 * 255.0 + 128.0).clamp(0.0, 255.0) as u8;
        let g = (f_dc_1 * 255.0 + 128.0).clamp(0.0, 255.0) as u8;
        let b = (f_dc_2 * 255.0 + 128.0).clamp(0.0, 255.0) as u8;
        
        let opacity = 1.0 / (1.0 + (-opacity_raw).exp());
        
        let scale_factor = 0.3;
        let sx = scale_0.exp() * scale_factor;
        let sy = scale_1.exp() * scale_factor;
        let sz = scale_2.exp() * scale_factor;
        
        let rot_norm = (rot_0 * rot_0 + rot_1 * rot_1 + rot_2 * rot_2 + rot_3 * rot_3).sqrt();
        let rotation = if rot_norm > 0.0001 {
            Quat::from_xyzw(rot_1 / rot_norm, rot_2 / rot_norm, rot_3 / rot_norm, rot_0 / rot_norm)
        } else {
            Quat::IDENTITY
        };
        
        let splat = SplatData {
            position: Vec3::new(x, y, z),
            scale: Vec3::new(sx.max(0.001), sy.max(0.001), sz.max(0.001)),
            rotation,
            color: [r, g, b, (opacity * 255.0) as u8],
            opacity,
            ..default()
        };
        
        cloud.splats.push(splat);
    }
    
    cloud.calculate_bounding_box();
    cloud.cluster_splats(64);
    
    info!("📊 Loaded PLY with {} splats (from {} total) in {} clusters", cloud.splats.len(), vertex_count, cloud.cluster_count);
    
    Ok(cloud)
}

fn parse_splat_data(data: &[u8]) -> io::Result<GaussianSplatCloud> {
    let mut cloud = GaussianSplatCloud::new();
    let record_size = 32;
    let count = data.len() / record_size;
    
    cloud.metadata.format = SplatFormat::Splat;
    cloud.metadata.point_count = count;
    
    for i in 0..count {
        let offset = i * record_size;
        let chunk = &data[offset..offset + record_size];
        let mut cursor = io::Cursor::new(chunk);
        
        let x = cursor.read_f32::<LittleEndian>()?;
        let y = cursor.read_f32::<LittleEndian>()?;
        let z = cursor.read_f32::<LittleEndian>()?;
        
        let scale_x = cursor.read_f32::<LittleEndian>()?;
        let scale_y = cursor.read_f32::<LittleEndian>()?;
        let scale_z = cursor.read_f32::<LittleEndian>()?;
        
        let rot_w = cursor.read_f32::<LittleEndian>()?;
        let rot_x = cursor.read_f32::<LittleEndian>()?;
        let rot_y = cursor.read_f32::<LittleEndian>()?;
        let rot_z = cursor.read_f32::<LittleEndian>()?;
        
        let r = cursor.read_u8()?;
        let g = cursor.read_u8()?;
        let b = cursor.read_u8()?;
        let a = cursor.read_u8()?;
        
        let splat = SplatData {
            position: Vec3::new(x, y, z),
            scale: Vec3::new(scale_x, scale_y, scale_z),
            rotation: Quat::from_xyzw(rot_x, rot_y, rot_z, rot_w),
            color: [r, g, b, a],
            opacity: a as f32 / 255.0,
            ..default()
        };
        
        cloud.splats.push(splat);
    }
    
    cloud.calculate_bounding_box();
    cloud.cluster_splats(64);
    
    info!("📊 Loaded SPLAT with {} splats in {} clusters", cloud.splats.len(), cloud.cluster_count);
    
    Ok(cloud)
}

pub fn generate_procedural_room_splats(
    room_size: Vec3,
    density: f32,
    base_color: Color,
    variation: f32,
) -> GaussianSplatCloud {
    use rand::Rng;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    
    let mut rng = StdRng::seed_from_u64(42);
    let mut cloud = GaussianSplatCloud::new();
    
    let count = (room_size.x * room_size.y * room_size.z * density) as usize;
    
    let base_srgba: Srgba = LinearRgba::from(base_color).into();
    
    for _ in 0..count {
        let position = Vec3::new(
            rng.gen::<f32>() * room_size.x - room_size.x / 2.0,
            rng.gen::<f32>() * room_size.y,
            rng.gen::<f32>() * room_size.z - room_size.z / 2.0,
        );
        
        let color_var = rng.gen::<f32>() * variation;
        let color = LinearRgba::new(
            (base_srgba.red + color_var).clamp(0.0, 1.0),
            (base_srgba.green + color_var).clamp(0.0, 1.0),
            (base_srgba.blue + color_var).clamp(0.0, 1.0),
            rng.gen::<f32>() * 0.5 + 0.5,
        );
        
        let scale = Vec3::splat(rng.gen::<f32>() * 0.05 + 0.02);
        
        let splat = SplatData {
            position,
            scale,
            rotation: Quat::from_euler(
                bevy::math::EulerRot::XYZ,
                rng.gen::<f32>() * std::f32::consts::PI,
                rng.gen::<f32>() * std::f32::consts::PI,
                rng.gen::<f32>() * std::f32::consts::PI,
            ),
            color: [
                (color.red * 255.0) as u8,
                (color.green * 255.0) as u8,
                (color.blue * 255.0) as u8,
                (color.alpha * 255.0) as u8,
            ],
            opacity: color.alpha,
            ..default()
        };
        
        cloud.splats.push(splat);
    }
    
    cloud.calculate_bounding_box();
    cloud.cluster_splats(64);
    
    info!("🎨 Generated {} procedural splats in {} clusters", cloud.splats.len(), cloud.cluster_count);
    
    cloud
}

#[allow(dead_code)]
pub fn generate_wall_splats(
    start: Vec3,
    end: Vec3,
    height: f32,
    density: f32,
    color: Color,
) -> GaussianSplatCloud {
    use rand::Rng;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    
    let mut rng = StdRng::seed_from_u64(123);
    let mut cloud = GaussianSplatCloud::new();
    
    let direction = end - start;
    let length = direction.length();
    let steps = (length * density) as usize;
    
    let base_srgba: Srgba = LinearRgba::from(color).into();
    
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let base_pos = start + direction * t;
        
        for _ in 0..((height * density) as usize) {
            let h = rng.gen::<f32>() * height;
            let offset = Vec3::new(
                rng.gen::<f32>() * 0.2 - 0.1,
                h,
                rng.gen::<f32>() * 0.2 - 0.1,
            );
            
            let position = base_pos + offset;
            let scale = Vec3::splat(rng.gen::<f32>() * 0.03 + 0.01);
            
            let splat = SplatData {
                position,
                scale,
                color: [
                    (base_srgba.red * 255.0) as u8,
                    (base_srgba.green * 255.0) as u8,
                    (base_srgba.blue * 255.0) as u8,
                    (rng.gen::<f32>() * 100.0 + 155.0) as u8,
                ],
                opacity: rng.gen::<f32>() * 0.5 + 0.5,
                ..default()
            };
            
            cloud.splats.push(splat);
        }
    }
    
    cloud.calculate_bounding_box();
    cloud.cluster_splats(32);
    
    cloud
}

#[allow(dead_code)]
#[derive(Message, Debug, Clone)]
pub struct SplatCloudLoadedEvent {
    pub handle: Handle<GaussianSplatCloud>,
    pub entity_count: usize,
}
