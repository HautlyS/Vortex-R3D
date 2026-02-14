# Tasks - Techno Sutra: Gaussian Splats Implementation Roadmap

## Executive Summary

This roadmap transforms Vortex-R3D from a panoramic skybox application into a fully Gaussian splat-based immersive world with physics, texture solidity (SOLIDEX), and complete "Techno Sutra: Cybermanju e o Bodhisattva" gameplay integration.

**Estimated Timeline:** 12-16 weeks  
**Team Size:** 2-3 developers  
**Critical Path:** Splat Rendering → Physics Integration → Gameplay Systems

---

## Phase 1: Foundation & Splat Infrastructure (Weeks 1-3)

### Week 1: Project Setup & Dependencies

#### Task 1.1: Update Cargo.toml
**Priority:** Critical | **Estimate:** 2 hours | **Assignee:** Backend Lead

**Implementation:**
```toml
[dependencies]
# Existing: bevy 0.17, bevy_kira_audio, etc.

# Physics
bevy_rapier3d = { version = "0.29", features = ["simd-stable", "debug-render"] }

# Splat loading
ply-rs = "0.1"
byteorder = "1.5"
half = "2.4"

# Spatial indexing
rstar = "0.12"

# Texture generation
noise = "0.9"
fast-poisson = "1.0"

# Compression
zstd = "0.13"

[features]
default = ["desktop"]
desktop = ["particles", "bevy/multi_threaded", "rapier-debug"]
particles = ["bevy_hanabi"]
vr = ["bevy_mod_openxr", "bevy_mod_xr", "particles", "bevy/multi_threaded"]
rapier-debug = ["bevy_rapier3d/debug-render"]
```

**Acceptance Criteria:**
- [ ] All dependencies resolve without conflicts
- [ ] Build succeeds on Windows/Linux/macOS
- [ ] Rapier debug visualization available
- [ ] Feature flags work correctly

---

#### Task 1.2: Create Module Structure
**Priority:** Critical | **Estimate:** 4 hours | **Assignee:** Any

**Directory Structure:**
```
src/
├── main.rs
├── lib.rs
├── state.rs
├── splat/
│   ├── mod.rs
│   ├── types.rs
│   ├── loader.rs
│   ├── renderer.rs
│   └── shaders/
├── solidex/
│   ├── mod.rs
│   ├── voxelize.rs
│   ├── meshing.rs
│   └── texture_baker.rs
├── physics/
│   ├── mod.rs
│   ├── character_controller.rs
│   ├── interaction.rs
│   └── collision_layers.rs
├── world/
│   ├── mod.rs
│   ├── room.rs
│   ├── portal.rs
│   └── streaming.rs
├── character/
│   ├── mod.rs
│   ├── cybermanju.rs
│   ├── kalyanamitra.rs
│   └── dialogue.rs
├── challenge/
│   ├── mod.rs
│   ├── dialogue_challenge.rs
│   ├── philosophical.rs
│   ├── key_hunt.rs
│   └── enigma.rs
├── inventory/
│   ├── mod.rs
│   ├── keys.rs
│   └── progression.rs
├── ui/
│   ├── mod.rs
│   ├── book_reader.rs
│   ├── character_sheet.rs
│   ├── hud.rs
│   └── settings.rs
└── audio/
    ├── mod.rs
    ├── spatial.rs
    └── soundtrack.rs
```

**Implementation:**
```rust
// src/lib.rs
pub mod state;
pub mod splat;
pub mod solidex;
pub mod physics;
pub mod world;
pub mod character;
pub mod challenge;
pub mod inventory;
pub mod ui;
pub mod audio;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            splat::SplatPlugin,
            solidex::SolidexPlugin,
            physics::PhysicsPlugin,
            world::WorldPlugin,
            character::CharacterPlugin,
            challenge::ChallengePlugin,
            inventory::InventoryPlugin,
            ui::UiPlugin,
            audio::AudioPlugin,
        ));
    }
}
```

**Acceptance Criteria:**
- [ ] All modules compile
- [ ] No circular dependencies
- [ ] Clear separation of concerns
- [ ] Plugin system functional

---

#### Task 1.3: Splat Data Types & Structures
**Priority:** Critical | **Estimate:** 6 hours | **Assignee:** Graphics Lead

**Implementation:**
```rust
// src/splat/types.rs

use bevy::prelude::*;

/// Single Gaussian splat
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct Splat {
    pub position: [f32; 3],      // World position
    pub scale: [f32; 3],         // Log-space scales
    pub rotation: [f32; 4],      // Quaternion (xyzw)
    pub color: [f32; 3],         // Linear RGB
    pub opacity: f32,            // Alpha (0-1)
}

impl Splat {
    pub const SIZE: usize = 48; // Bytes
    
    pub fn new(position: Vec3, scale: Vec3, rotation: Quat, color: Vec3, opacity: f32) -> Self {
        Self {
            position: position.to_array(),
            scale: scale.to_array(),
            rotation: rotation.to_array(),
            color: color.to_array(),
            opacity,
        }
    }
}

/// Splat with spherical harmonics (higher quality)
#[derive(Clone, Debug)]
pub struct SplatWithSH {
    pub base: Splat,
    pub sh_coeffs: [f32; 9],     // Degree 2 spherical harmonics
}

/// Component holding splat cloud data
#[derive(Component)]
pub struct SplatCloud {
    pub splats: Vec<Splat>,
    pub bounds: Aabb,
    pub gpu_buffer: Option<Buffer>, // Will be populated by renderer
    pub quality: SplatQuality,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SplatQuality {
    #[default]
    Medium,
    Low,
    High,
    Ultra,
    Potato,
}

impl SplatCloud {
    pub fn from_splats(splats: Vec<Splat>) -> Self {
        let bounds = compute_splat_bounds(&splats);
        Self {
            splats,
            bounds,
            gpu_buffer: None,
            quality: SplatQuality::Medium,
        }
    }
    
    pub fn len(&self) -> usize {
        self.splats.len()
    }
}

fn compute_splat_bounds(splats: &[Splat]) -> Aabb {
    if splats.is_empty() {
        return Aabb::default();
    }
    
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    
    for splat in splats {
        let pos = Vec3::from_array(splat.position);
        min = min.min(pos);
        max = max.max(pos);
    }
    
    Aabb::from_min_max(min, max)
}
```

**Acceptance Criteria:**
- [ ] Splat struct matches GPU layout
- [ ] SplatCloud component functional
- [ ] Bounds calculation correct
- [ ] Memory layout optimized for cache

---

### Week 2: Splat Loading Pipeline

#### Task 2.1: PLY File Loader
**Priority:** Critical | **Estimate:** 8 hours | **Assignee:** Backend Lead

**Implementation:**
```rust
// src/splat/loader.rs

use ply_rs::ply::{self, Property, PropertyAccess};
use std::fs::File;
use std::io::BufReader;

/// Load Gaussian splats from PLY file
pub fn load_splats_from_ply(path: &str) -> Result<Vec<Splat>, SplatLoadError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let ply: ply::Ply = ply::parser::parse_ply(reader)?;
    
    let mut splats = Vec::new();
    
    for element in &ply.elements {
        if element.name == "vertex" {
            for vertex in &element.data {
                let splat = parse_splat_vertex(vertex)?;
                splats.push(splat);
            }
        }
    }
    
    info!("Loaded {} splats from {}", splats.len(), path);
    Ok(splats)
}

fn parse_splat_vertex(vertex: &dyn PropertyAccess) -> Result<Splat, SplatLoadError> {
    // Standard Gaussian splat PLY format
    let x = get_f32(vertex, "x")?;
    let y = get_f32(vertex, "y")?;
    let z = get_f32(vertex, "z")?;
    
    let nx = get_f32(vertex, "nx")?;
    let ny = get_f32(vertex, "ny")?;
    let nz = get_f32(vertex, "nz")?;
    
    // Scale in log space
    let scale_0 = get_f32(vertex, "scale_0")?;
    let scale_1 = get_f32(vertex, "scale_1")?;
    let scale_2 = get_f32(vertex, "scale_2")?;
    
    // Color (SH DC component) with sigmoid activation
    let f_dc_0 = sigmoid(get_f32(vertex, "f_dc_0")?);
    let f_dc_1 = sigmoid(get_f32(vertex, "f_dc_1")?);
    let f_dc_2 = sigmoid(get_f32(vertex, "f_dc_2")?);
    
    // Opacity with sigmoid
    let opacity = sigmoid(get_f32(vertex, "opacity")?);
    
    // Rotation (quaternion, normalized)
    let rot_0 = get_f32(vertex, "rot_0")?;
    let rot_1 = get_f32(vertex, "rot_1")?;
    let rot_2 = get_f32(vertex, "rot_2")?;
    let rot_3 = get_f32(vertex, "rot_3")?;
    let rotation = Quat::from_array([rot_0, rot_1, rot_2, rot_3]).normalize();
    
    Ok(Splat::new(
        Vec3::new(x, y, z),
        Vec3::new(scale_0.exp(), scale_1.exp(), scale_2.exp()),
        rotation,
        Vec3::new(f_dc_0, f_dc_1, f_dc_2),
        opacity,
    ))
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn get_f32(vertex: &dyn PropertyAccess, name: &str) -> Result<f32, SplatLoadError> {
    match vertex.get_property(name) {
        Some(Property::Float(v)) => Ok(*v),
        Some(Property::Double(v)) => Ok(*v as f32),
        _ => Err(SplatLoadError::MissingField(name.to_string())),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SplatLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PLY parse error: {0}")]
    Parse(String),
    #[error("Missing field: {0}")]
    MissingField(String),
}
```

**Acceptance Criteria:**
- [ ] Loads standard Gaussian splat PLY files
- [ ] Handles various PLY variants (Inria, PolyCam, etc.)
- [ ] Proper sigmoid activation for colors/opacity
- [ ] Error handling for malformed files
- [ ] Progress reporting for large files

---

#### Task 2.2: Compressed Splat Loader (.spz)
**Priority:** High | **Estimate:** 6 hours | **Assignee:** Backend Lead

**Implementation:**
```rust
// SPZ format: compressed splats with FP16 positions

pub fn load_spz(path: &str) -> Result<Vec<Splat>, SplatLoadError> {
    let data = std::fs::read(path)?;
    let mut cursor = std::io::Cursor::new(data);
    
    // SPZ header
    let magic = cursor.read_u32::<LittleEndian>()?;
    if magic != 0x53505A00 { // "SPZ\0"
        return Err(SplatLoadError::Parse("Invalid SPZ magic".to_string()));
    }
    
    let version = cursor.read_u16::<LittleEndian>()?;
    let num_splats = cursor.read_u32::<LittleEndian>()? as usize;
    
    let mut splats = Vec::with_capacity(num_splats);
    
    // Positions (FP16)
    for _ in 0..num_splats {
        let x = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        let y = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        let z = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        
        // Scales (log, FP16)
        let sx = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        let sy = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        let sz = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        
        // Colors (8-bit normalized)
        let r = cursor.read_u8()? as f32 / 255.0;
        let g = cursor.read_u8()? as f32 / 255.0;
        let b = cursor.read_u8()? as f32 / 255.0;
        let a = cursor.read_u8()? as f32 / 255.0;
        
        // Rotation (quat, FP16)
        let qx = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        let qy = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        let qz = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        let qw = f16::from_bits(cursor.read_u16::<LittleEndian>()?).to_f32();
        
        splats.push(Splat::new(
            Vec3::new(x, y, z),
            Vec3::new(sx.exp(), sy.exp(), sz.exp()),
            Quat::from_array([qx, qy, qz, qw]).normalize(),
            Vec3::new(r, g, b),
            a,
        ));
    }
    
    Ok(splats)
}
```

**Acceptance Criteria:**
- [ ] Loads SPZ compressed format
- [ ] ~50% memory reduction vs PLY
- [ ] FP16 precision acceptable for positions
- [ ] Faster loading than PLY

---

#### Task 2.3: Asset Loader Integration
**Priority:** High | **Estimate:** 4 hours | **Assignee:** Backend Lead

**Implementation:**
```rust
// Bevy asset loader for splat files

#[derive(Asset, TypePath)]
pub struct SplatAsset {
    pub cloud: SplatCloud,
}

#[derive(Default)]
pub struct SplatAssetLoader;

impl AssetLoader for SplatAssetLoader {
    type Asset = SplatAsset;
    type Settings = ();
    type Error = SplatLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<SplatAsset, SplatLoadError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        
        let extension = _load_context.asset_path().get_extension();
        
        let splats = match extension.as_deref() {
            Some("ply") => load_splats_from_ply_bytes(&bytes)?,
            Some("spz") => load_spz_bytes(&bytes)?,
            Some("splat") => load_splat_bytes(&bytes)?,
            _ => return Err(SplatLoadError::Parse("Unknown format".to_string())),
        };
        
        Ok(SplatAsset {
            cloud: SplatCloud::from_splats(splats),
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ply", "spz", "splat"]
    }
}
```

**Acceptance Criteria:**
- [ ] Integrates with Bevy asset system
- [ ] Async loading doesn't block
- [ ] Progress tracking for large scenes
- [ ] Hot-reloading support

---

### Week 3: GPU Buffer Management

#### Task 3.1: GPU Splat Buffer Upload
**Priority:** Critical | **Estimate:** 8 hours | **Assignee:** Graphics Lead

**Implementation:**
```rust
// Upload splat data to GPU storage buffer

pub fn upload_splats_to_gpu(
    mut splat_clouds: Query<(&mut SplatCloud, Entity), Added<SplatCloud>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    for (mut cloud, entity) in splat_clouds.iter_mut() {
        if cloud.splats.is_empty() {
            continue;
        }
        
        let buffer_size = (cloud.splats.len() * Splat::SIZE) as u64;
        
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some(&format!("splat_buffer_{:?}", entity)),
            size: buffer_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Upload data
        let data: &[u8] = bytemuck::cast_slice(&cloud.splats);
        render_queue.write_buffer(&buffer, 0, data);
        
        cloud.gpu_buffer = Some(buffer);
        
        info!("Uploaded {} splats to GPU for entity {:?}", cloud.len(), entity);
    }
}
```

**Acceptance Criteria:**
- [ ] Efficient GPU buffer creation
- [ ] Proper memory alignment
- [ ] Multiple splat clouds supported
- [ ] Buffer cleanup on entity removal

---

#### Task 3.2: Splat Sorting System
**Priority:** Critical | **Estimate:** 10 hours | **Assignee:** Graphics Lead

**Implementation:**
```rust
// Depth-based splat sorting for transparency

pub fn sort_splats_by_depth(
    mut splat_clouds: Query<(&mut SplatCloud, &GlobalTransform)>,
    camera: Query<(&GlobalTransform, &Camera), With<PrimaryCamera>>,
) {
    let Ok((camera_transform, _)) = camera.get_single() else { return };
    let camera_pos = camera_transform.translation();
    
    for (mut cloud, transform) in splat_clouds.iter_mut() {
        // Skip if splats haven't moved and camera hasn't moved significantly
        
        // Calculate view-space depth for each splat
        let view_matrix = Mat4::from(camera_transform.compute_matrix().inverse());
        
        // Sort indices by depth (back to front)
        let mut indices: Vec<usize> = (0..cloud.splats.len()).collect();
        indices.sort_by(|&a, &b| {
            let pos_a = Vec3::from_array(cloud.splats[a].position);
            let pos_b = Vec3::from_array(cloud.splats[b].position);
            
            let view_a = view_matrix.transform_point3(pos_a);
            let view_b = view_matrix.transform_point3(pos_b);
            
            // Sort back-to-front (descending Z in view space)
            view_b.z.partial_cmp(&view_a.z).unwrap()
        });
        
        // Reorder splats (or store sorted indices)
        let sorted_splats: Vec<Splat> = indices
            .iter()
            .map(|&i| cloud.splats[i])
            .collect();
        
        cloud.splats = sorted_splats;
        
        // Re-upload to GPU if buffer exists
        if let Some(ref buffer) = cloud.gpu_buffer {
            // Queue re-upload
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Correct depth sorting
- [ ] Back-to-front rendering order
- [ ] Performance: <2ms for 1M splats
- [ ] Only sort when camera moves significantly

---

## Phase 2: Rendering Pipeline (Weeks 4-6)

### Week 4: WGSL Shaders

#### Task 4.1: Vertex Shader - Billboard Generation
**Priority:** Critical | **Estimate:** 8 hours | **Assignee:** Graphics Lead

**Implementation:**
```wgsl
// assets/shaders/splat_vertex.wgsl

#import bevy_render::view::View

struct Splat {
    position: vec3<f32>,
    scale: vec3<f32>,
    rotation: vec4<f32>,
    color: vec3<f32>,
    opacity: f32,
}

@group(0) @binding(0)
var<storage, read> splats: array<Splat>;

@group(0) @binding(1)
var<uniform> view: View;

@group(0) @binding(2)
var<uniform> focal: vec2<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) opacity: f32,
    @location(2) conic: vec3<f32>,
    @location(3) uv: vec2<f32>,
}

// Quaternion to rotation matrix
fn quat_to_mat3(q: vec4<f32>) -> mat3x3<f32> {
    let x = q.x;
    let y = q.y;
    let z = q.z;
    let w = q.w;
    
    return mat3x3<f32>(
        vec3<f32>(1.0 - 2.0*(y*y + z*z), 2.0*(x*y + w*z), 2.0*(x*z - w*y)),
        vec3<f32>(2.0*(x*y - w*z), 1.0 - 2.0*(x*x + z*z), 2.0*(y*z + w*x)),
        vec3<f32>(2.0*(x*z + w*y), 2.0*(y*z - w*x), 1.0 - 2.0*(x*x + y*y))
    );
}

// Compute 3D covariance
fn compute_cov3d(scale: vec3<f32>, rotation: vec4<f32>) -> mat3x3<f32> {
    let S = mat3x3<f32>(
        vec3<f32>(scale.x, 0.0, 0.0),
        vec3<f32>(0.0, scale.y, 0.0),
        vec3<f32>(0.0, 0.0, scale.z)
    );
    let R = quat_to_mat3(rotation);
    return R * S * transpose(S) * transpose(R);
}

// Project to 2D
fn project_cov2d(cov3d: mat3x3<f32>, view: mat4x4<f32>, pos: vec3<f32>, focal: vec2<f32>) -> vec3<f32> {
    let t = (view * vec4<f32>(pos, 1.0)).xyz;
    
    let J = mat3x3<f32>(
        vec3<f32>(focal.x / t.z, 0.0, -(focal.x * t.x) / (t.z * t.z)),
        vec3<f32>(0.0, focal.y / t.z, -(focal.y * t.y) / (t.z * t.z)),
        vec3<f32>(0.0, 0.0, 0.0)
    );
    
    let W = mat3x3<f32>(
        view[0].xyz,
        view[1].xyz,
        view[2].xyz
    );
    
    let T = W * J;
    let cov2d = T * cov3d * transpose(T);
    
    return vec3<f32>(cov2d[0][0] + 0.3, cov2d[0][1], cov2d[1][1] + 0.3);
}

const QUAD = array<vec2<f32>, 4>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0, -1.0),
    vec2<f32>(-1.0,  1.0),
    vec2<f32>( 1.0,  1.0)
);

@vertex
fn vertex(@builtin(vertex_index) vert_idx: u32,
          @builtin(instance_index) instance_idx: u32) -> VertexOutput {
    let splat = splats[instance_idx];
    let quad = QUAD[vert_idx];
    
    let cov3d = compute_cov3d(splat.scale, splat.rotation);
    let cov2d = project_cov2d(cov3d, view.view_proj, splat.position, focal);
    
    // Invert covariance
    let det = cov2d.x * cov2d.z - cov2d.y * cov2d.y;
    let conic = vec3<f32>(cov2d.z / det, -cov2d.y / det, cov2d.x / det);
    
    // Compute extent
    let extent = vec2<f32>(sqrt(cov2d.x * 3.0), sqrt(cov2d.z * 3.0));
    
    // Project position
    let clip = view.view_proj * vec4<f32>(splat.position, 1.0);
    let offset = quad * extent * clip.w;
    
    var out: VertexOutput;
    out.clip_position = clip + vec4<f32>(offset, 0.0, 0.0);
    out.color = splat.color;
    out.opacity = splat.opacity;
    out.conic = conic;
    out.uv = quad;
    
    return out;
}
```

**Acceptance Criteria:**
- [ ] Correct covariance projection
- [ ] Proper billboard generation
- [ ] Clipping handles edge cases
- [ ] Performance: 2M+ splats @ 60fps

---

#### Task 4.2: Fragment Shader - Gaussian Evaluation
**Priority:** Critical | **Estimate:** 6 hours | **Assignee:** Graphics Lead

**Implementation:**
```wgsl
// assets/shaders/splat_fragment.wgsl

struct FragmentInput {
    @location(0) color: vec3<f32>,
    @location(1) opacity: f32,
    @location(2) conic: vec3<f32>,
    @location(3) uv: vec2<f32>,
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    // Evaluate 2D Gaussian: exp(-0.5 * x^T * Sigma^-1 * x)
    let x = input.uv;
    let conic = input.conic;
    
    let power = -0.5 * (conic.x * x.x * x.x + conic.z * x.y * x.y) - conic.y * x.x * x.y;
    
    if (power > 0.0) {
        discard;
    }
    
    let alpha = input.opacity * exp(power);
    
    // Gamma correction (splat colors are linear)
    let color = pow(input.color, vec3<f32>(2.2));
    
    return vec4<f32>(color * alpha, alpha);
}
```

**Acceptance Criteria:**
- [ ] Correct Gaussian falloff
- [ ] Proper alpha blending
- [ ] Discard for transparent pixels
- [ ] Gamma correction applied

---

### Week 5: Render Pipeline Integration

#### Task 5.1: Bevy Render Node
**Priority:** Critical | **Estimate:** 10 hours | **Assignee:** Graphics Lead

**Implementation:**
```rust
// Custom render node for Gaussian splats

pub struct SplatRenderNode {
    query: QueryState<&'static SplatCloud, With<ExtractedSplat>>,
}

impl Node for SplatRenderNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let splat_clouds = self.query.iter_manual(world);
        
        for cloud in splat_clouds {
            if let Some(ref buffer) = cloud.gpu_buffer {
                // Bind splat buffer
                // Draw instanced quads
                // One instance per splat
                render_context.render_pass().draw_indexed(
                    0..6,  // Quad indices
                    0,
                    0..cloud.len() as u32,
                );
            }
        }
        
        Ok(())
    }
}
```

**Acceptance Criteria:**
- [ ] Integrates with Bevy render graph
- [ ] Proper render phases
- [ ] Transparency sorting
- [ ] HDR/tonemapping compatible

---

#### Task 5.2: Tile-Based Rasterization
**Priority:** High | **Estimate:** 8 hours | **Assignee:** Graphics Lead

**Implementation:**
```rust
// Compute shader for tile assignment

@compute @workgroup_size(256)
fn assign_tiles(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let splat_idx = global_id.x;
    if (splat_idx >= arrayLength(&splats)) {
        return;
    }
    
    let splat = splats[splat_idx];
    
    // Project to screen
    let clip = view.view_proj * vec4<f32>(splat.position, 1.0);
    let ndc = clip.xyz / clip.w;
    let screen = (ndc.xy * 0.5 + 0.5) * viewport_size;
    
    // Calculate screen extent
    let extent = calculate_screen_extent(splat);
    
    // Assign to tiles
    let min_tile = vec2<u32>(max(vec2<i32>(0), vec2<i32>((screen - extent) / tile_size)));
    let max_tile = vec2<u32>(min(vec2<i32>(num_tiles), vec2<i32>((screen + extent) / tile_size)));
    
    for (var y = min_tile.y; y < max_tile.y; y++) {
        for (var x = min_tile.x; x < max_tile.x; x++) {
            let tile_idx = y * num_tiles.x + x;
            // Append splat to tile list (atomic)
            let list_idx = atomicAdd(&tile_counts[tile_idx], 1u);
            tile_splats[tile_idx * max_per_tile + list_idx] = splat_idx;
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Efficient tile assignment
- [ ] No tile overflow
- [ ] Performance improvement vs naive
- [ ] Handles edge cases

---

### Week 6: Quality & Optimization

#### Task 6.1: Level-of-Detail System
**Priority:** High | **Estimate:** 6 hours | **Assignee:** Graphics Lead

**Implementation:**
```rust
// LOD based on distance to camera

pub fn select_splat_lod(
    splat_cloud: &SplatCloud,
    camera_pos: Vec3,
) -> Vec<&Splat> {
    let distance = splat_cloud.bounds.center.distance(camera_pos);
    
    // Select LOD based on distance
    let lod_factor = match distance {
        d if d < 10.0 => 1.0,      // Full detail
        d if d < 30.0 => 0.5,      // Half splats
        d if d < 100.0 => 0.25,    // Quarter splats
        _ => 0.1,                   // 10% splats
    };
    
    let target_count = (splat_cloud.len() as f32 * lod_factor) as usize;
    
    // Select every Nth splat (or use importance sampling)
    let step = splat_cloud.len() / target_count;
    
    splat_cloud.splats.iter()
        .step_by(step.max(1))
        .collect()
}
```

**Acceptance Criteria:**
- [ ] Smooth LOD transitions
- [ ] Distance-based selection
- [ ] Preserves visual quality
- [ ] Performance scaling

---

#### Task 6.2: Frustum Culling
**Priority:** High | **Estimate:** 4 hours | **Assignee:** Graphics Lead

**Implementation:**
```rust
// Cull splats outside view frustum

pub fn frustum_cull_splats(
    splats: &[Splat],
    frustum: &Frustum,
) -> Vec<usize> {
    splats.iter()
        .enumerate()
        .filter(|(_, splat)| {
            let pos = Vec3::from_array(splat.position);
            // Approximate with bounding sphere
            let radius = Vec3::from_array(splat.scale).length();
            frustum.intersects_sphere(&Sphere::new(pos, radius))
        })
        .map(|(idx, _)| idx)
        .collect()
}
```

**Acceptance Criteria:**
- [ ] Correct culling
- [ ] Conservative bounds
- [ ] Significant performance gain
- [ ] No visible popping

---

## Phase 3: Physics Integration (Weeks 7-9)

### Week 7: Rapier Setup

#### Task 7.1: Rapier Configuration
**Priority:** Critical | **Estimate:** 4 hours | **Assignee:** Physics Lead

**Implementation:**
```rust
// src/physics/mod.rs

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<PhysicsLayers>::default())
           .insert_resource(RapierConfiguration {
               gravity: Vec3::new(0.0, -9.81, 0.0),
               timestep_mode: TimestepMode::Fixed { dt: 1.0 / 60.0, substeps: 4 },
               ..default()
           })
           .add_systems(Startup, setup_physics)
           .add_systems(FixedUpdate, (
               character_controller_system,
               sync_splat_physics,
           ));
    }
}

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
    Default = 0,
    Player = 1,
    Environment = 2,
    Key = 3,
    Portal = 4,
    Character = 5,
}
```

**Acceptance Criteria:**
- [ ] Rapier initialized
- [ ] Correct timestep
- [ ] Layer collision matrix
- [ ] Debug rendering available

---

#### Task 7.2: Character Controller
**Priority:** Critical | **Estimate:** 8 hours | **Assignee:** Physics Lead

**Implementation:**
```rust
// FPS character with physics

#[derive(Component)]
pub struct CharacterController {
    pub speed: f32,
    pub jump_force: f32,
    pub max_slope_angle: f32,
}

pub fn spawn_player(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    commands.spawn((
        Player,
        FirstPersonCamera::default(),
        CharacterController {
            speed: 5.0,
            jump_force: 8.0,
            max_slope_angle: 45.0_f32.to_radians(),
        },
        Transform::from_translation(position),
        RigidBody::KinematicPositionBased,
        Collider::capsule(Vec3::new(0.0, 0.9, 0.0), Vec3::new(0.0, -0.9, 0.0), 0.3),
        KinematicCharacterController {
            max_slope_climb_angle: 45.0_f32.to_radians(),
            min_slope_slide_angle: 30.0_f32.to_radians(),
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            ..default()
        },
        Ccd::enabled(),
        ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
        ActiveEvents::COLLISION_EVENTS,
    )).id()
}

pub fn character_controller_system(
    mut controllers: Query<(
        &mut KinematicCharacterController,
        &mut KinematicCharacterControllerOutput,
        &CharacterController,
        &Transform,
    )>,
    input: Res<InputState>,
    time: Res<Time>,
) {
    for (mut controller, output, char_ctrl, transform) in controllers.iter_mut() {
        let grounded = output.grounded;
        
        // Movement
        let mut movement = Vec3::ZERO;
        let forward = transform.forward();
        let right = transform.right();
        
        if input.forward { movement += forward; }
        if input.backward { movement -= forward; }
        if input.left { movement -= right; }
        if input.right { movement += right; }
        
        if movement.length_squared() > 0.0 {
            movement = movement.normalize() * char_ctrl.speed * time.delta_secs();
        }
        
        // Gravity
        movement.y -= 9.81 * time.delta_secs();
        
        // Jump
        if input.jump && grounded {
            movement.y = char_ctrl.jump_force * time.delta_secs();
        }
        
        controller.translation = Some(movement);
    }
}
```

**Acceptance Criteria:**
- [ ] Smooth FPS movement
- [ ] Proper collision response
- [ ] Slope handling
- [ ] Jump physics
- [ ] No clipping through walls

---

### Week 8: SOLIDEX System

#### Task 8.1: Voxelization & SDF Generation
**Priority:** Critical | **Estimate:** 10 hours | **Assignee:** Physics Lead

**Implementation:**
```rust
// src/solidex/voxelize.rs

/// Generate SDF from splat cloud
pub fn voxelize_splats(
    splats: &[Splat],
    voxel_size: f32,
) -> SdfGrid {
    let bounds = compute_splat_bounds(splats);
    let size = ((bounds.max - bounds.min) / voxel_size).ceil().as_ivec3();
    
    let mut grid = SdfGrid::new(size, bounds.min, voxel_size);
    
    // For each voxel, compute minimum distance to any splat
    for x in 0..size.x {
        for y in 0..size.y {
            for z in 0..size.z {
                let voxel_pos = bounds.min + Vec3::new(x as f32, y as f32, z as f32) * voxel_size;
                
                let mut min_dist = f32::INFINITY;
                let mut max_opacity = 0.0f32;
                
                for splat in splats {
                    let splat_pos = Vec3::from_array(splat.position);
                    let dist = voxel_pos.distance(splat_pos);
                    let splat_radius = Vec3::from_array(splat.scale).length();
                    
                    if dist < min_dist {
                        min_dist = dist;
                    }
                    
                    if dist < splat_radius && splat.opacity > 0.5 {
                        max_opacity = max_opacity.max(splat.opacity);
                    }
                }
                
                // Signed distance: negative inside solid region
                let sdf_value = if max_opacity > 0.5 {
                    -min_dist
                } else {
                    min_dist
                };
                
                grid.set(IVec3::new(x, y, z), sdf_value);
            }
        }
    }
    
    grid
}

/// Sparse SDF grid for memory efficiency
pub struct SdfGrid {
    pub size: IVec3,
    pub origin: Vec3,
    pub voxel_size: f32,
    pub data: Vec<f32>,
}
```

**Acceptance Criteria:**
- [ ] Accurate SDF generation
- [ ] Sparse storage for large scenes
- [ ] Configurable voxel size
- [ ] Performance: <1s for 1M splats

---

#### Task 8.2: Mesh Generation (Marching Cubes)
**Priority:** Critical | **Estimate:** 8 hours | **Assignee:** Physics Lead

**Implementation:**
```rust
// src/solidex/meshing.rs

/// Generate mesh from SDF using marching cubes
pub fn marching_cubes(
    sdf: &SdfGrid,
    iso_value: f32,
) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    
    // Standard marching cubes lookup tables
    const EDGE_TABLE: [i32; 256] = [/* ... */];
    const TRI_TABLE: [[i8; 16]; 256] = [/* ... */];
    
    for x in 0..sdf.size.x - 1 {
        for y in 0..sdf.size.y - 1 {
            for z in 0..sdf.size.z - 1 {
                let corners = [
                    sdf.get(IVec3::new(x, y, z)),
                    sdf.get(IVec3::new(x + 1, y, z)),
                    sdf.get(IVec3::new(x + 1, y + 1, z)),
                    sdf.get(IVec3::new(x, y + 1, z)),
                    sdf.get(IVec3::new(x, y, z + 1)),
                    sdf.get(IVec3::new(x + 1, y, z + 1)),
                    sdf.get(IVec3::new(x + 1, y + 1, z + 1)),
                    sdf.get(IVec3::new(x, y + 1, z + 1)),
                ];
                
                // Compute cube index
                let mut cube_index = 0;
                for i in 0..8 {
                    if corners[i] < iso_value {
                        cube_index |= 1 << i;
                    }
                }
                
                if cube_index == 0 || cube_index == 255 {
                    continue;
                }
                
                // Generate triangles for this cube
                let edge_flags = EDGE_TABLE[cube_index];
                
                // Interpolate vertices along edges
                // ... (standard marching cubes implementation)
                
                // Add triangles
                let tri_edges = &TRI_TABLE[cube_index];
                for i in (0..16).step_by(3) {
                    if tri_edges[i] < 0 {
                        break;
                    }
                    
                    let base_idx = vertices.len() as u32;
                    
                    for j in 0..3 {
                        let edge_idx = tri_edges[i + j] as usize;
                        // Get interpolated vertex position
                        let vert = interpolate_vertex(edge_idx, corners, x, y, z, sdf);
                        vertices.push(vert);
                    }
                    
                    indices.extend_from_slice(&[base_idx, base_idx + 1, base_idx + 2]);
                }
            }
        }
    }
    
    // Compute normals from vertices
    compute_normals(&vertices, &indices)
}
```

**Acceptance Criteria:**
- [ ] Watertight mesh
- [ ] Correct normals
- [ ] LOD variants
- [ ] Performance: <2s for 100³ voxels

---

### Week 9: Collision Integration

#### Task 9.1: Splat → Collider System
**Priority:** Critical | **Estimate:** 6 hours | **Assignee:** Physics Lead

**Implementation:**
```rust
// Auto-generate collision from splats

pub fn generate_splat_collision(
    mut commands: Commands,
    splat_query: Query<(Entity, &SplatCloud, &Transform), Without<SolidexHull>>,
    config: Res<SolidexConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, cloud, transform) in splat_query.iter() {
        if cloud.len() < 100 {
            continue; // Skip small clouds
        }
        
        // Generate SDF
        let sdf = voxelize_splats(&cloud.splats, config.voxel_size);
        
        // Generate mesh at multiple LODs
        let lod_meshes: Vec<_> = (0..config.lod_levels)
            .map(|lod| {
                let voxel_scale = 2.0_f32.powi(lod as i32);
                let mesh = marching_cubes(&sdf, 0.0);
                meshes.add(mesh)
            })
            .collect();
        
        // Generate collision (simplified mesh)
        let collision_mesh = simplify_mesh(&lod_meshes[0], config.max_collision_tris);
        let collider = Collider::from_bevy_mesh(&collision_mesh, 
            &ComputedColliderShape::ConvexDecomposition(VHACDParameters::default())
        ).unwrap();
        
        commands.entity(entity).insert((
            SolidexHull {
                lod_meshes,
                collision: collider.clone(),
                material: Default::default(),
            },
            RigidBody::Fixed,
            collider,
            CollisionGroups::new(Group::ENVIRONMENT, Group::ALL),
        ));
    }
}
```

**Acceptance Criteria:**
- [ ] Automatic collision generation
- [ ] LOD collision meshes
- [ ] Performance optimized
- [ ] No manual collider placement needed

---

#### Task 9.2: Interaction System
**Priority:** High | **Estimate:** 6 hours | **Assignee:** Physics Lead

**Implementation:**
```rust
// Raycasting for interactions

pub fn interact_system(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<(&Transform, &Camera), With<Player>>,
    rapier_context: Res<RapierContext>,
    interactable_query: Query<(Entity, &Transform, Option<&KeyItem>, Option<&Character>)>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) && !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    
    let Ok((player_transform, camera)) = player_query.get_single() else { return };
    
    // Ray from camera center
    let ray_origin = player_transform.translation();
    let ray_dir = player_transform.forward();
    let max_distance = 5.0;
    
    if let Some((entity, hit)) = rapier_context.cast_ray(
        ray_origin,
        ray_dir,
        max_distance,
        true,
        QueryFilter::default().exclude_colliders(&[]),
    ) {
        if let Ok((_, _, key_item, character)) = interactable_query.get(entity) {
            if let Some(key) = key_item {
                // Collect key
                commands.insert_resource(KeyCollectedEvent(key.key_id.clone()));
                commands.entity(entity).despawn();
            }
            
            if let Some(char) = character {
                // Start dialogue
                commands.insert_resource(StartDialogueEvent(entity));
            }
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Raycasting for interaction
- [ ] Key collection works
- [ ] Dialogue triggers
- [ ] Physics-based reach

---

## Phase 4: Gameplay Systems (Weeks 10-12)

### Week 10: Challenge System

#### Task 10.1: Dialogue Challenges
**Priority:** High | **Estimate:** 8 hours | **Assignee:** Gameplay Lead

**Implementation:**
```rust
// src/challenge/dialogue_challenge.rs

#[derive(Component)]
pub struct DialogueChallenge {
    pub npc_name: String,
    pub current_node: String,
    pub dialogue_tree: DialogueTree,
    pub test_type: PersonalityTest,
}

pub enum PersonalityTest {
    Bravery,
    Love,
    Compassion,
    Anxiety,
    Anguish,
    Jealousy,
}

pub struct DialogueNode {
    pub text: String,
    pub speaker: Speaker,
    pub choices: Vec<DialogueChoice>,
}

pub struct DialogueChoice {
    pub text: String,
    pub next_node: String,
    pub trait_effect: Option<(PersonalityTest, i32)>, // Test type and score delta
    pub unlocks_key: Option<String>,
}

pub fn dialogue_system(
    mut dialogue_query: Query<&mut DialogueChallenge>,
    input: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<UiState>,
) {
    for mut challenge in dialogue_query.iter_mut() {
        let current = challenge.dialogue_tree.nodes.get(&challenge.current_node);
        
        if let Some(node) = current {
            // Show dialogue UI
            ui_state.show_dialogue = true;
            ui_state.dialogue_text = node.text.clone();
            ui_state.dialogue_speaker = format!("{:?}", node.speaker);
            ui_state.dialogue_choices = node.choices.iter().map(|c| c.text.clone()).collect();
            
            // Handle choice selection (1-4 keys)
            for (i, choice) in node.choices.iter().enumerate() {
                if input.just_pressed([KeyCode::Digit1, KeyCode::Digit2, 
                                       KeyCode::Digit3, KeyCode::Digit4][i]) {
                    // Apply trait effects
                    if let Some((test, delta)) = choice.trait_effect {
                        // Update player stats
                    }
                    
                    // Transition to next node
                    challenge.current_node = choice.next_node.clone();
                }
            }
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Branching dialogue works
- [ ] Personality tests tracked
- [ ] Choice UI functional
- [ ] State transitions smooth

---

#### Task 10.2: Key Hunt System
**Priority:** High | **Estimate:** 6 hours | **Assignee:** Gameplay Lead

**Implementation:**
```rust
// Hidden keys in splat world

pub fn spawn_hidden_keys(
    mut commands: Commands,
    room_query: Query<&Room>,
    asset_server: Res<AssetServer>,
) {
    for room in room_query.iter() {
        for key_data in &room.keys {
            let position = key_data.position;
            
            commands.spawn((
                KeyItem {
                    key_id: key_data.id.clone(),
                    challenge_text: key_data.challenge_text.clone(),
                    unlocks_kalyanamitra: key_data.unlocks,
                    twinkle: true,
                },
                SplatCloud::from_splats(generate_key_splats()),
                Transform::from_translation(position),
                // Visual twinkle effect
                TwinkleEffect {
                    base_intensity: 1.0,
                    pulse_speed: 2.0,
                    color: Color::srgb(1.0, 0.8, 0.2), // Golden twinkle
                },
                // Physics sensor (no collision, just detection)
                Sensor,
                Collider::sphere(0.5),
                CollisionGroups::new(Group::KEY, Group::PLAYER),
            ));
        }
    }
}

pub fn key_twinkle_system(
    mut key_query: Query<(&mut SplatCloud, &TwinkleEffect, &KeyItem)>,
    time: Res<Time>,
) {
    for (mut cloud, twinkle, key) in key_query.iter_mut() {
        if !key.twinkle {
            continue;
        }
        
        let pulse = (time.elapsed_secs() * twinkle.pulse_speed).sin() * 0.5 + 0.5;
        let intensity = twinkle.base_intensity + pulse * 0.5;
        
        // Modify splat colors for twinkle effect
        for splat in cloud.splats.iter_mut() {
            let base_color = twinkle.color.to_linear().to_vec3();
            splat.color = (base_color * intensity).to_array();
        }
    }
}

pub fn key_collection_system(
    mut commands: Commands,
    mut player_inventory: ResMut<PlayerInventory>,
    key_query: Query<(Entity, &KeyItem, &Transform)>,
    player_query: Query<&Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    
    for (entity, key_item, key_transform) in key_query.iter() {
        let distance = player_transform.translation.distance(key_transform.translation);
        
        if distance < 2.0 && keys.just_pressed(KeyCode::Enter) {
            // Collect key
            player_inventory.keys.push(key_item.clone());
            
            // Show key text
            commands.insert_resource(ShowKeyTextEvent(key_item.challenge_text.clone()));
            
            // Despawn key
            commands.entity(entity).despawn();
            
            info!("Collected key: {}", key_item.key_id);
        }
    }
}
```

**Acceptance Criteria:**
- [ ] Keys hidden in world
- [ ] Twinkle effect visible
- [ ] Collection by proximity + Enter
- [ ] Text displayed on collect

---

### Week 11: UI & Progression

#### Task 11.1: Character Sheet UI
**Priority:** High | **Estimate:** 8 hours | **Assignee:** UI Lead

**Implementation:**
```rust
// src/ui/character_sheet.rs

pub fn character_sheet_ui(
    mut contexts: EguiContexts,
    player: Res<PlayerInventory>,
    ui_state: Res<UiState>,
) {
    if !ui_state.show_character_sheet {
        return;
    }
    
    egui::Window::new("Character Sheet")
        .collapsible(true)
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            // Stats
            ui.heading("Stats");
            ui.horizontal(|ui| {
                ui.label("Wisdom:");
                ui.label(player.stats.wisdom.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Focus:");
                ui.label(player.stats.focus.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Insight:");
                ui.label(player.stats.insight.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Karma:");
                ui.label(player.stats.karma.to_string());
            });
            
            ui.separator();
            
            // Keys
            ui.heading("Collected Keys");
            for key in &player.keys {
                ui.collapsing(&key.key_id, |ui| {
                    ui.label(&key.challenge_text);
                });
            }
            
            ui.separator();
            
            // Unlocked teachers
            ui.heading("Kalyanamitras Unlocked");
            for teacher_id in &player.unlocked_teachers {
                ui.label(format!("Teacher {}", teacher_id));
            }
        });
}
```

**Acceptance Criteria:**
- [ ] Stats displayed
- [ ] Keys list with details
- [ ] Unlocked teachers shown
- [ ] Toggle with B key

---

#### Task 11.2: Progression System
**Priority:** High | **Estimate:** 6 hours | **Assignee:** Gameplay Lead

**Implementation:**
```rust
// Chapter progression

#[derive(Resource)]
pub struct GameProgress {
    pub current_chapter: u8,
    pub unlocked_kalyanamitras: Vec<u8>,
    pub collected_keys: Vec<String>,
    pub sudhana_found: bool,
}

pub fn check_chapter_progression(
    mut progress: ResMut<GameProgress>,
    mut events: EventReader<KeyCollectedEvent>,
    kalyanamitra_query: Query<&Kalyanamitra>,
) {
    for event in events.read() {
        // Find which kalyanamitra this key unlocks
        if let Some(key_data) = KEY_DATABASE.get(&event.0) {
            if !progress.unlocked_kalyanamitras.contains(&key_data.unlocks) {
                progress.unlocked_kalyanamitras.push(key_data.unlocks);
                info!("Unlocked Kalyanamitra {}", key_data.unlocks);
            }
        }
    }
    
    // Check if all chapters complete
    if progress.unlocked_kalyanamitras.len() >= 3 && !progress.sudhana_found {
        progress.sudhana_found = true;
        // Trigger ending sequence
    }
}

pub fn ending_sequence(
    progress: Res<GameProgress>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<UiState>,
) {
    if progress.sudhana_found {
        ui_state.show_ending = true;
        ui_state.ending_text = format!(
            "ALARME DE EMERGÊNCIA: SUDHANA FOI ENCONTRADO!\n\n\
             Missão concluída - Sudhana foi reconhecido como o usuário do device.\n\n\
             VOZ FINAL:\n\
             Bem vindo de volta, Sudhana!"
        );
        
        // Transition to ending state after delay
        // ...
    }
}
```

**Acceptance Criteria:**
- [ ] Chapters unlock correctly
- [ ] Keys tracked
- [ ] Ending triggers properly
- [ ] State saved/loaded

---

### Week 12: Polish & Audio

#### Task 12.1: Spatial Audio Integration
**Priority:** Medium | **Estimate:** 6 hours | **Assignee:** Audio Lead

**Implementation:**
```rust
// 3D positioned audio for splat world

pub fn setup_spatial_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Ambient room audio
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/room_ambience.ogg"),
            settings: PlaybackSettings::LOOP.with_spatial(true),
        },
        SpatialAudioBundle {
            spatial: SpatialAudio {
                emitter: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            },
        },
    ));
}

pub fn character_audio_system(
    mut commands: Commands,
    character_query: Query<(Entity, &Transform, &Character), Changed<Character>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, character) in character_query.iter() {
        if character.is_talking {
            commands.entity(entity).insert(
                AudioBundle {
                    source: asset_server.load(format!("audio/dialogue_{}.ogg", character.name)),
                    settings: PlaybackSettings::DESPAWN.with_spatial(true),
                }
            );
        }
    }
}
```

**Acceptance Criteria:**
- [ ] 3D audio positioning
- [ ] Character dialogue spatial
- [ ] Ambient room audio
- [ ] Crossfade between rooms

---

#### Task 12.2: Cybermanju UI
**Priority:** Medium | **Estimate:** 6 hours | **Assignee:** UI Lead

**Implementation:**
```rust
// Fixed AI companion UI

pub fn cybermanju_ui(
    mut contexts: EguiContexts,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::InGame {
        return;
    }
    
    // Fixed position panel (top-right)
    egui::SidePanel::right("cybermanju_panel")
        .resizable(false)
        .default_width(250.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Cybermanju");
            ui.label("IA Companheira");
            ui.separator();
            ui.label("Status: Ativa");
            ui.label("Conhecimento: Crescente");
            ui.separator();
            ui.label("\"Estou aqui para guiá-lo através \\ da jornada de Sudhana.\"");
        });
}
```

**Acceptance Criteria:**
- [ ] Fixed position UI
- [ ] Always visible during gameplay
- [ ] Shows AI status
- [ ] Themed appropriately

---

## Phase 5: Integration & Testing (Weeks 13-14)

### Week 13: System Integration

#### Task 13.1: Full System Integration
**Priority:** Critical | **Estimate:** 20 hours | **Assignee:** All

**Tasks:**
- [ ] Connect all gameplay systems
- [ ] Wire up progression tracking
- [ ] Integrate UI with game state
- [ ] Test portal transitions
- [ ] Verify save/load system
- [ ] Ensure audio sync
- [ ] Check physics stability

---

#### Task 13.2: Asset Pipeline
**Priority:** High | **Estimate:** 10 hours | **Assignee:** Art/Backend

**Create sample assets:**
- 3 splat environments (garden, temple, void)
- 3 Kalyanamitra characters
- 1 Cybermanju character
- 3 keys with challenges
- Audio tracks for each room

---

### Week 14: Testing & Optimization

#### Task 14.1: Performance Testing
**Priority:** Critical | **Estimate:** 10 hours | **Assignee:** Graphics Lead

**Test scenarios:**
- [ ] 1M splats @ 60fps (RTX 3060)
- [ ] 2M splats @ 60fps (RTX 4070)
- [ ] Physics with 100+ objects
- [ ] Memory usage <1GB
- [ ] Load time <5s per room

---

#### Task 14.2: Gameplay Testing
**Priority:** High | **Estimate:** 10 hours | **Assignee:** All

**Test all 4 challenge types:**
- [ ] Dialogue choices affect stats
- [ ] Philosophical questions recorded
- [ ] Keys findable and collectable
- [ ] Enigmas solvable
- [ ] Progression to ending works

---

## Quick Reference

### Build Commands
```bash
# Development
cargo run

# Release
cargo run --release

# With physics debug
cargo run --features rapier-debug

# Web
cargo build --target wasm32-unknown-unknown --release
```

### Key Bindings
| Key | Action |
|-----|--------|
| W/A/S/D | Move |
| Space | Jump |
| E/Enter | Interact |
| B | Character sheet |
| Tab | Switch tabs |
| Escape | Menu |

### File Locations
```
assets/
├── splats/
│   ├── environments/
│   ├── characters/
│   └── objects/
├── audio/
└── fonts/
```

---

*Document Version: 1.0*  
*Total Estimated Hours: 480-600 hours*  
*Timeline: 12-16 weeks*  
*Team: 2-3 developers*
