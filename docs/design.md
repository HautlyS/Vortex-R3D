# Design Document - Techno Sutra: Gaussian Splats Architecture

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           TECHNO SUTRA ENGINE                               │
├─────────────────────────────────────────────────────────────────────────────┤
│  Application Layer                                                           │
│  ├── GameStatePlugin          (state machine: Loading/Menu/Game/Dialogue)   │
│  ├── TechnoSutraPlugin        (main gameplay integration)                   │
│  └── SaveLoadPlugin           (persistence, chapter progress)               │
├─────────────────────────────────────────────────────────────────────────────┤
│  Gameplay Systems                                                            │
│  ├── SplatWorldPlugin         (environment management)                      │
│  │   ├── SplatLoader          (PLY/SPZ/SPLAT loading)                       │
│  │   ├── SplatRenderer        (custom WGSL rasterization)                   │
│  │   └── WorldStreaming       (chunk-based loading)                         │
│  ├── PhysicsPlugin            (Rapier integration)                          │
│  │   ├── SolidexGenerator     (splat → collision hulls)                     │
│  │   ├── CharacterController  (FPS + physics movement)                      │
│  │   └── InteractionSystem    (raycast + physics triggers)                  │
│  ├── CharacterPlugin          (splat-based NPCs)                            │
│  │   ├── SplatCharacter       (animated Gaussian actors)                    │
│  │   ├── CybermanjuSystem     (AI companion logic)                          │
│  │   └── KalyanamitraSystem   (teacher dialogue)                            │
│  ├── ChallengePlugin          (4 challenge types)                           │
│  │   ├── DialogueChallenge    (branching narrative)                         │
│  │   ├── PhilosophicalQuiz    (AI training questions)                       │
│  │   ├── KeyHuntSystem        (hidden collectibles)                         │
│  │   └── EnigmaSystem         (puzzle resolution)                           │
│  └── InventoryPlugin          (keys, progression)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│  Rendering Pipeline                                                          │
│  ├── GaussianSplatPlugin      (core splat rendering)                        │
│  │   ├── SplatSortCompute     (depth sort compute shader)                   │
│  │   ├── TileRasterizer       (tile-based rasterization)                    │
│  │   └── SHLighting           (spherical harmonics)                         │
│  ├── SolidexRenderPlugin      (texture + solidity visualization)            │
│  └── PostProcessPlugin        (tonemapping, effects)                        │
├─────────────────────────────────────────────────────────────────────────────┤
│  Platform Layer                                                              │
│  ├── DesktopPlugin            (Windows/Linux/macOS)                         │
│  ├── WebPlugin                (WASM + WebGL2/WebGPU)                        │
│  └── VRPlugin                 (OpenXR support)                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Gaussian Splat Rendering Pipeline

### Data Flow

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                        SPLAT RENDERING PIPELINE                              │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  SPLAT ASSET                                                                  │
│  ┌─────────────┐                                                              │
│  │ .ply/.spz   │──Load──┐                                                     │
│  │ Position    │        │                                                     │
│  │ Scale       │        ▼                                                     │
│  │ Rotation    │   ┌─────────────┐                                            │
│  │ Color       │   │ SplatBuffer │  GPU Storage Buffer                        │
│  │ Opacity     │──►│ (positions, │  (positions, covariances, colors)          │
│  │ SH coeffs   │   │  covariances│                                             │
│  └─────────────┘   └─────────────┘                                            │
│                           │                                                   │
│                           ▼                                                   │
│                    ┌──────────────┐                                           │
│                    │ Depth Sort   │  Compute Shader                           │
│                    │ (per frame)  │  - View-space transform                   │
│                    │              │  - Key generation (depth)                 │
│                    └──────────────┘  - Radix/Bitonic sort                     │
│                           │                                                   │
│                           ▼                                                   │
│                    ┌──────────────┐                                           │
│                    │ Tile Assign  │  Compute Shader                           │
│                    │              │  - Screen tiling (16x16)                  │
│                    └──────────────┘  - Splat-to-tile mapping                  │
│                           │                                                   │
│                           ▼                                                   │
│  ┌──────────────┐  ┌──────────────┐                                           │
│  │ Vertex Shader│  │ Fragment     │                                           │
│  │ - Billboard  │  │ Shader       │                                           │
│  │   quad gen   │  │ - Gaussian   │                                           │
│  │ - Covariance │  │   evaluation │                                           │
│  │   projection │  │ - Alpha      │                                           │
│  └──────────────┘  │   blending   │                                           │
│                    └──────────────┘                                           │
│                           │                                                   │
│                           ▼                                                   │
│                    ┌──────────────┐                                           │
│                    │ Final Image  │  Render Target                            │
│                    └──────────────┘                                           │
│                                                                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

### WGSL Shader Design

#### 1. Splat Data Structure (Storage Buffer)
```wgsl
// splat_data.wgsl

struct Splat {
    position: vec3<f32>,      // World position
    scale: vec3<f32>,         // Log-space scales
    rotation: vec4<f32>,      // Quaternion
    color: vec3<f32>,         // Base color
    opacity: f32,             // Alpha (0-1)
    sh_coeffs: array<f32, 9>, // Spherical harmonics (optional)
}

@group(0) @binding(0)
var<storage, read> splats: array<Splat>;

@group(0) @binding(1)
var<uniform> view_proj: mat4x4<f32>;

@group(0) @binding(2)
var<uniform> camera_pos: vec3<f32>;
```

#### 2. Covariance Matrix Calculation
```wgsl
// covariance.wgsl

fn quat_to_rotation_matrix(q: vec4<f32>) -> mat3x3<f32> {
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

fn compute_covariance_3d(scale: vec3<f32>, rotation: vec4<f32>) -> mat3x3<f32> {
    let S = mat3x3<f32>(
        vec3<f32>(exp(scale.x), 0.0, 0.0),
        vec3<f32>(0.0, exp(scale.y), 0.0),
        vec3<f32>(0.0, 0.0, exp(scale.z))
    );
    
    let R = quat_to_rotation_matrix(rotation);
    return R * S * S * transpose(R);
}

fn project_covariance_2d(cov3d: mat3x3<f32>, view_matrix: mat4x4<f32>, 
                         position: vec3<f32>, focal: vec2<f32>, 
                         viewport: vec2<f32>) -> vec3<f32> {
    let t = (view_matrix * vec4<f32>(position, 1.0)).xyz;
    
    let J = mat3x3<f32>(
        vec3<f32>(focal.x / t.z, 0.0, -(focal.x * t.x) / (t.z * t.z)),
        vec3<f32>(0.0, focal.y / t.z, -(focal.y * t.y) / (t.z * t.z)),
        vec3<f32>(0.0, 0.0, 0.0)
    );
    
    let W = mat3x3<f32>(
        view_matrix[0].xyz,
        view_matrix[1].xyz,
        view_matrix[2].xyz
    );
    
    let T = W * J;
    let cov2d = T * cov3d * transpose(T);
    
    // Return (cov_xx, cov_xy, cov_yy)
    return vec3<f32>(
        cov2d[0][0] + 0.3,  // Small regularization
        cov2d[0][1],
        cov2d[1][1] + 0.3
    );
}
```

#### 3. Vertex Shader (Billboard Generation)
```wgsl
// splat_vertex.wgsl

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) alpha: f32,
    @location(2) conic: vec3<f32>,
    @location(3) uv: vec2<f32>,
}

const QUAD_VERTICES = array<vec2<f32>, 4>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0, -1.0),
    vec2<f32>(-1.0,  1.0),
    vec2<f32>( 1.0,  1.0)
);

@vertex
fn main(@builtin(vertex_index) vert_idx: u32,
        @builtin(instance_index) instance_idx: u32) -> VertexOutput {
    let splat = splats[instance_idx];
    let quad_vert = QUAD_VERTICES[vert_idx];
    
    // Project to view space
    let view_pos = (view_matrix * vec4<f32>(splat.position, 1.0)).xyz;
    
    // Compute projected covariance
    let cov2d = project_covariance_2d(
        compute_covariance_3d(splat.scale, splat.rotation),
        view_matrix,
        splat.position,
        focal_lengths,
        viewport_size
    );
    
    // Invert covariance for conic
    let det = cov2d.x * cov2d.z - cov2d.y * cov2d.y;
    let conic = vec3<f32>(
        cov2d.z / det,
        -cov2d.y / det,
        cov2d.x / det
    );
    
    // Compute screen-space billboard extent
    let extent = vec2<f32>(
        sqrt(cov2d.x * 3.0),  // 3 sigma
        sqrt(cov2d.z * 3.0)
    );
    
    let screen_pos = project(view_pos);
    let offset = quad_vert * extent;
    
    var output: VertexOutput;
    output.position = vec4<f32>(screen_pos.xy + offset, screen_pos.zw);
    output.color = splat.color;
    output.alpha = splat.opacity;
    output.conic = conic;
    output.uv = quad_vert;
    
    return output;
}
```

#### 4. Fragment Shader (Gaussian Evaluation)
```wgsl
// splat_fragment.wgsl

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Evaluate 2D Gaussian
    let d = -0.5 * (input.conic.x * input.uv.x * input.uv.x + 
                    input.conic.z * input.uv.y * input.uv.y) - 
              input.conic.y * input.uv.x * input.uv.y;
    
    if (d > 0.0) {
        discard;
    }
    
    let alpha = input.alpha * exp(d);
    
    // Apply spherical harmonics (if enabled)
    var final_color = input.color;
    if (use_sh > 0u) {
        final_color = evaluate_sh(input.color, sh_coeffs, view_dir);
    }
    
    // Gamma correction
    final_color = pow(final_color, vec3<f32>(2.2));
    
    return vec4<f32>(final_color * alpha, alpha);
}
```

---

## SOLIDEX System: Texture & Solidity

### Overview
SOLIDEX (Solid Extraction) converts dense Gaussian splat clusters into textured collision geometry for physics interaction.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                          SOLIDEX PIPELINE                                    │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  INPUT: Dense Splat Region                                                    │
│  ┌─────────────────────────────────────┐                                      │
│  │ Splat Cloud (100K+ splats)          │                                      │
│  │ - Positions                         │                                      │
│  │ - Scales                            │                                      │
│  │ - Colors                            │                                      │
│  │ - Opacities                         │                                      │
│  └─────────────┬───────────────────────┘                                      │
│                │                                                              │
│                ▼                                                              │
│  ┌─────────────────────────────────────┐                                      │
│  │ 1. VOXELIZATION                     │                                      │
│  │    - Grid size: 0.1m - 1.0m         │                                      │
│  │    - Density threshold for solid    │                                      │
│  │    - Signed Distance Field (SDF)    │                                      │
│  └─────────────┬───────────────────────┘                                      │
│                │                                                              │
│                ▼                                                              │
│  ┌─────────────────────────────────────┐                                      │
│  │ 2. MESH GENERATION                  │                                      │
│  │    - Marching Cubes / Dual Contour  │                                      │
│  │    - LOD levels (3-5)               │                                      │
│  │    - Convex decomposition           │                                      │
│  └─────────────┬───────────────────────┘                                      │
│                │                                                              │
│                ▼                                                              │
│  ┌─────────────────────────────────────┐                                      │
│  │ 3. TEXTURE BAKING                   │                                      │
│  │    - Albedo from splat colors       │                                      │
│  │    - Normal from density gradient   │                                      │
│  │    - Roughness from scale variance  │                                      │
│  └─────────────┬───────────────────────┘                                      │
│                │                                                              │
│                ▼                                                              │
│  OUTPUT: Textured Collision Geometry                                          │
│  ┌─────────────────────────────────────┐                                      │
│  │ - Collision Mesh (for Rapier)       │                                      │
│  │ - Visual Mesh (fallback rendering)  │                                      │
│  │ - Material (PBR textures)           │                                      │
│  │ - LOD variants                      │                                      │
│  └─────────────────────────────────────┘                                      │
│                                                                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Implementation

```rust
// src/solidex/mod.rs

pub struct SolidexPlugin;

impl Plugin for SolidexPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SolidexCache>()
           .add_systems(Update, (
               generate_solidex_from_splats,
               update_lod_collision,
               bake_splat_textures,
           ));
    }
}

/// Configuration for SOLIDEX generation
#[derive(Resource)]
pub struct SolidexConfig {
    /// Voxel size in meters (smaller = more detail)
    pub voxel_size: f32,
    /// Minimum density to consider voxel as solid
    pub density_threshold: f32,
    /// Number of LOD levels
    pub lod_levels: u32,
    /// Maximum triangles per collision mesh
    pub max_collision_tris: usize,
}

impl Default for SolidexConfig {
    fn default() -> Self {
        Self {
            voxel_size: 0.1,
            density_threshold: 0.5,
            lod_levels: 3,
            max_collision_tris: 10000,
        }
    }
}

/// Generated collision and visual data from splats
#[derive(Component)]
pub struct SolidexHull {
    pub lod_meshes: Vec<Handle<Mesh>>,
    pub collision_shape: Collider,
    pub material: Handle<StandardMaterial>,
    pub bounds: Aabb,
}

/// System to generate SOLIDEX from dense splat regions
fn generate_solidex_from_splats(
    mut commands: Commands,
    splat_query: Query<(&SplatCloud, &Transform, Entity), Without<SolidexHull>>,
    config: Res<SolidexConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (splat_cloud, transform, entity) in splat_query.iter() {
        // Only process dense regions
        if splat_cloud.splats.len() < 1000 {
            continue;
        }
        
        // Generate SDF from splats
        let sdf = voxelize_splats(&splat_cloud.splats, config.voxel_size);
        
        // Generate meshes at different LODs
        let lod_meshes = (0..config.lod_levels)
            .map(|lod| {
                let lod_voxel_size = config.voxel_size * (2.0_f32.powi(lod as i32));
                let mesh = marching_cubes(&sdf, lod_voxel_size, config.density_threshold);
                meshes.add(mesh)
            })
            .collect();
        
        // Generate convex decomposition for collision
        let collision_shape = generate_collision_shape(&sdf, config.max_collision_tris);
        
        // Bake textures from splat colors
        let material = bake_splat_material(&splat_cloud.splats, &mut materials);
        
        commands.entity(entity).insert(SolidexHull {
            lod_meshes,
            collision_shape,
            material,
            bounds: calculate_splat_bounds(&splat_cloud.splats),
        });
    }
}

/// Generate Rapier collider from SDF
fn generate_collision_shape(sdf: &SdfGrid, max_tris: usize) -> Collider {
    // Extract surface mesh
    let mesh = marching_cubes(sdf, sdf.voxel_size, 0.0);
    
    // Simplify if too complex
    let simplified = if mesh.indices().len() > max_tris * 3 {
        simplify_mesh(&mesh, max_tris)
    } else {
        mesh
    };
    
    // Decompose into convex parts for stability
    Collider::from_bevy_mesh(&simplified, &ComputedColliderShape::ConvexDecomposition(
        VHACDParameters::default()
    )).unwrap()
}
```

---

## Physics Integration Architecture

### Rapier Component Mapping

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                      PHYSICS COMPONENT MAPPING                               │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  SPLAT ENTITY                                                                 │
│  ┌───────────────────────────────────────────────────────────────────────┐    │
│  │  Transform         (position, rotation, scale)                        │    │
│  │  SplatCloud        (visual gaussian data)                             │    │
│  │  SolidexHull       (generated collision + texture)                    │    │
│  │  ─────────────────────────────────────────────────────────────────    │    │
│  │  RigidBody         (Rapier: Dynamic/Static/Kinematic)                 │    │
│  │  Collider          (Rapier: shape from SolidexHull)                   │    │
│  │  Velocity          (Rapier: linear/angular)                           │    │
│  │  MassProperties    (Rapier: mass, inertia)                            │    │
│  │  Friction          (Rapier: coefficient, combine rule)                │    │
│  │  Restitution       (Rapier: bounciness)                               │    │
│  └───────────────────────────────────────────────────────────────────────┘    │
│                                                                               │
│  PLAYER ENTITY                                                                │
│  ┌───────────────────────────────────────────────────────────────────────┐    │
│  │  Transform                                                            │    │
│  │  FirstPersonCamera                                                    │    │
│  │  CharacterController     (custom: input processing)                   │    │
│  │  ─────────────────────────────────────────────────────────────────    │    │
│  │  RigidBody               (KinematicPositionBased)                     │    │
│  │  Collider                (Capsule: height 1.8m, radius 0.3m)          │    │
│  │  Velocity                                                             │    │
│  │  Ccd                     (Continuous collision detection)             │    │
│  └───────────────────────────────────────────────────────────────────────┘    │
│                                                                               │
│  KEY ENTITY (Collectible)                                                     │
│  ┌───────────────────────────────────────────────────────────────────────┐    │
│  │  Transform                                                            │    │
│  │  KeyItem                 (gameplay: challenge text, unlocks)          │    │
│  │  SplatCloud              (visual only, low density)                   │    │
│  │  ─────────────────────────────────────────────────────────────────    │    │
│  │  Sensor                  (Rapier: trigger collider)                   │    │
│  │  Collider                (Sphere: 0.2m radius)                        │    │
│  │  TwinkleEffect           (visual feedback)                            │    │
│  └───────────────────────────────────────────────────────────────────────┘    │
│                                                                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Character Controller System

```rust
// src/physics/character_controller.rs

#[derive(Component)]
pub struct CharacterController {
    pub speed: f32,
    pub jump_force: f32,
    pub max_slope_angle: f32,
    pub grounded: bool,
}

pub fn character_movement_system(
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
        // Check grounded state
        let grounded = output.grounded;
        
        // Calculate movement direction
        let forward = transform.forward();
        let right = transform.right();
        
        let mut movement = Vec3::ZERO;
        if input.forward { movement += forward; }
        if input.backward { movement -= forward; }
        if input.left { movement -= right; }
        if input.right { movement += right; }
        
        // Normalize and apply speed
        if movement.length_squared() > 0.0 {
            movement = movement.normalize() * char_ctrl.speed * time.delta_secs();
        }
        
        // Add jump if grounded
        if input.jump && grounded {
            controller.translation = Some(Vec3::Y * char_ctrl.jump_force * time.delta_secs());
        }
        
        // Apply movement
        controller.translation = Some(movement);
    }
}
```

---

## Entity-Component Design

### Core Components

```rust
// src/components/mod.rs

// ==================== SPLAT COMPONENTS ====================

/// Gaussian splat cloud data
#[derive(Component)]
pub struct SplatCloud {
    pub splats: Vec<Splat>,
    pub bounds: Aabb,
    pub quality: SplatQuality,
}

/// Single Gaussian splat
#[derive(Clone, Copy, Debug)]
pub struct Splat {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
    pub color: Vec3,
    pub opacity: f32,
    pub sh_coeffs: Option<[f32; 9]>,
}

/// Splat rendering configuration
#[derive(Component)]
pub struct SplatRenderConfig {
    pub enabled: bool,
    pub lod_bias: f32,
    pub alpha_threshold: f32,
    pub use_sh: bool,
}

// ==================== PHYSICS COMPONENTS ====================

/// Generated collision from splats (SOLIDEX)
#[derive(Component)]
pub struct SolidexHull {
    pub collision: Collider,
    pub visual_mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

/// Interactive physics object
#[derive(Component)]
pub struct PhysicsObject {
    pub mass: f32,
    pub can_be_pushed: bool,
    pub can_be_picked_up: bool,
}

// ==================== GAMEPLAY COMPONENTS ====================

/// Player marker
#[derive(Component)]
pub struct Player;

/// Character entity (NPC)
#[derive(Component)]
pub struct Character {
    pub name: String,
    pub character_type: CharacterType,
    pub dialogue_state: DialogueState,
}

pub enum CharacterType {
    Cybermanju,      // AI companion
    Kalyanamitra(u8), // Teacher 1-3
    Environment,     // Interactive object
}

/// Key item for progression
#[derive(Component)]
pub struct KeyItem {
    pub key_id: String,
    pub challenge_text: String,
    pub unlocks_kalyanamitra: u8,
    pub twinkle: bool,
}

/// Portal between rooms
#[derive(Component)]
pub struct PortalDoor {
    pub target_room: u8,
    pub target_position: Vec3,
}

/// Challenge component
#[derive(Component)]
pub struct Challenge {
    pub challenge_type: ChallengeType,
    pub completed: bool,
    pub chapter: u8,
}

pub enum ChallengeType {
    Dialogue { options: Vec<DialogueOption> },
    Philosophical { question: String },
    KeyHunt { key_entity: Entity },
    Enigma { answer: String },
}

// ==================== UI COMPONENTS ====================

/// World-space UI attached to entity
#[derive(Component)]
pub struct WorldSpaceUi {
    pub offset: Vec3,
    pub always_face_camera: bool,
}

/// Character sheet/inventory UI
#[derive(Component)]
pub struct CharacterSheet {
    pub stats: CharacterStats,
    pub keys: Vec<KeyItem>,
    pub dialogue_history: Vec<String>,
}

#[derive(Default)]
pub struct CharacterStats {
    pub wisdom: u32,
    pub focus: u32,
    pub insight: u32,
    pub karma: i32,
}
```

---

## State Machine

```
                    ┌─────────────┐
                    │   Loading   │
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
           ▼               ▼               ▼
    ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
    │  Main Menu  │ │  Game Init  │ │  Error      │
    └──────┬──────┘ └──────┬──────┘ └─────────────┘
           │               │
           │               ▼
           │        ┌─────────────┐
           │        │  InGame     │◄──────────────────┐
           │        └──────┬──────┘                   │
           │               │                          │
     ┌─────┴─────┬─────────┼─────────┬─────────┐      │
     │           │         │         │         │      │
     ▼           ▼         ▼         ▼         ▼      │
┌─────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐│
│Settings │ │Dialogue│ │Key Hunt│ │ Enigma │ │ Portal ││
└────┬────┘ └───┬────┘ └───┬────┘ └───┬────┘ └───┬────┘│
     │          │          │          │          │     │
     └──────────┴──────────┴──────────┴──────────┘     │
                        │                              │
                        ▼                              │
                 ┌─────────────┐                       │
                 │   Return    │───────────────────────┘
                 └─────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Ending    │
                    │  (Sudhana)  │
                    └─────────────┘
```

```rust
// src/state.rs

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    GameInit,
    InGame,
    Dialogue,
    KeyHunt,
    Enigma,
    PortalTransition,
    Settings,
    Ending,
}

#[derive(Resource)]
pub struct GameProgress {
    pub current_chapter: u8,
    pub unlocked_kalyanamitras: Vec<u8>,
    pub collected_keys: Vec<String>,
    pub sudhana_found: bool,
}
```

---

## File Structure

```
src/
├── main.rs                          # Entry point
├── lib.rs                           # GamePlugin
├── state.rs                         # GameState, GameProgress
│
├── splat/                           # Gaussian splat system
│   ├── mod.rs                       # SplatPlugin
│   ├── loader.rs                    # PLY/SPZ/SPLAT loading
│   ├── types.rs                     # Splat, SplatCloud structs
│   ├── renderer.rs                  # Bevy render pipeline
│   ├── shaders/
│   │   ├── splat_vertex.wgsl
│   │   ├── splat_fragment.wgsl
│   │   ├── splat_sort.wgsl
│   │   └── splat_tile.wgsl
│   └── compute/                     # Compute shader utilities
│
├── solidex/                         # SOLIDEX: texture + solidity
│   ├── mod.rs                       # SolidexPlugin
│   ├── voxelize.rs                  # SDF generation from splats
│   ├── meshing.rs                   # Marching cubes / dual contour
│   ├── texture_baker.rs             # Material generation
│   └── lod.rs                       # Level-of-detail system
│
├── physics/                         # Rapier integration
│   ├── mod.rs                       # PhysicsPlugin
│   ├── character_controller.rs      # FPS movement with physics
│   ├── interaction.rs               # Raycasting, triggers
│   └── collision_layers.rs          # Physics layers
│
├── world/                           # Environment system
│   ├── mod.rs                       # WorldPlugin
│   ├── room.rs                      # Room/splat world management
│   ├── portal.rs                    # Portal system
│   └── streaming.rs                 # Chunk-based loading
│
├── character/                       # NPC system
│   ├── mod.rs                       # CharacterPlugin
│   ├── cybermanju.rs                # AI companion logic
│   ├── kalyanamitra.rs              # Teacher system
│   ├── dialogue.rs                  # Dialogue state machine
│   └── animation.rs                 # Splat animation
│
├── challenge/                       # 4 challenge types
│   ├── mod.rs                       # ChallengePlugin
│   ├── dialogue_challenge.rs        # Branching narrative
│   ├── philosophical.rs             # AI training questions
│   ├── key_hunt.rs                  # Hidden keys
│   └── enigma.rs                    # Puzzle system
│
├── inventory/                       # Progression system
│   ├── mod.rs                       # InventoryPlugin
│   ├── keys.rs                      # Key items
│   └── progression.rs               # Chapter unlocking
│
├── ui/                              # User interface
│   ├── mod.rs                       # UiPlugin
│   ├── book_reader.rs               # Sacred text interface
│   ├── character_sheet.rs           # Stats/keys display
│   ├── hud.rs                       # World-space HUD
│   └── settings.rs                  # Options panel
│
├── audio/                           # Spatial audio
│   ├── mod.rs                       # AudioPlugin
│   ├── spatial.rs                   # 3D positioning
│   └── soundtrack.rs                # Room music
│
└── platform/                        # Platform abstraction
    ├── mod.rs
    ├── desktop.rs
    ├── web.rs
    └── vr.rs
```

---

## Performance Optimization

### Splat Rendering Optimizations

```rust
// Optimization strategies

pub struct SplatOptimization {
    /// Frustum culling before sorting
    pub frustum_cull: bool,
    
    /// Distance-based LOD selection
    pub lod_distance: Vec<f32>,  // [10.0, 30.0, 100.0]
    
    /// Skip sorting for distant splats
    pub sort_distance_threshold: f32,
    
    /// Tile size for rasterization
    pub tile_size: UVec2,  // (16, 16)
    
    /// Max splats to render per frame
    pub max_splats: usize,
}

impl Default for SplatOptimization {
    fn default() -> Self {
        Self {
            frustum_cull: true,
            lod_distance: vec![10.0, 30.0, 100.0],
            sort_distance_threshold: 200.0,
            tile_size: UVec2::new(16, 16),
            max_splats: 4_000_000,
        }
    }
}
```

### Physics Optimizations

```rust
/// Rapier configuration for splat worlds
pub fn configure_rapier_physics() -> RapierConfiguration {
    RapierConfiguration {
        gravity: Vec3::new(0.0, -9.81, 0.0),
        timestep_mode: TimestepMode::Fixed { dt: 1.0 / 60.0, substeps: 4 },
        scaled_shape_subdivision: 5,
        force_update_from_transform_changes: true,
        ..default()
    }
}

/// Static collision optimization
pub fn optimize_static_colliders(
    mut static_bodies: Query<&mut Collider, (With<RigidBody>, Without<PhysicsObject>)>,
) {
    for mut collider in static_bodies.iter_mut() {
        // Enable sleeping for static objects
        collider.set_active_collision_types(ActiveCollisionTypes::default());
    }
}
```

---

## Integration Points

### Bevy Plugin Integration

```rust
// src/lib.rs

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
           .init_resource::<GameProgress>()
           
           // Core systems
           .add_plugins((
               SplatPlugin,
               SolidexPlugin,
               PhysicsPlugin,
               WorldPlugin,
               CharacterPlugin,
               ChallengePlugin,
               InventoryPlugin,
               UiPlugin,
               AudioPlugin,
           ))
           
           // Platform-specific
           .add_plugins(PlatformPlugin);
    }
}

/// Platform abstraction
pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        app.add_plugins(WebPlugin);
        
        #[cfg(all(not(target_arch = "wasm32"), feature = "vr"))]
        app.add_plugins(VrPlugin);
        
        #[cfg(all(not(target_arch = "wasm32"), not(feature = "vr")))]
        app.add_plugins(DesktopPlugin);
    }
}
```

### Rapier Integration

```rust
// src/physics/mod.rs

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<PhysicsLayers>::default())
           .insert_resource(configure_rapier_physics())
           .add_systems(Startup, setup_physics)
           .add_systems(Update, (
               character_movement_system,
               handle_interactions,
               sync_splats_to_physics,
           ));
    }
}

#[derive(PhysicsLayer)]
pub enum PhysicsLayers {
    Default,
    Player,
    Environment,
    Key,
    Portal,
    Character,
}
```

---

*Document Version: 1.0*
*Architecture: Gaussian Splats + Rapier Physics*
*Based on: Techno Sutra (idea.md)*
