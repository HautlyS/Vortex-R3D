# Design Document - Bevy Techno Sutra DEMO

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        BEVY APP                                 │
├─────────────────────────────────────────────────────────────────┤
│  Plugins                                                        │
│  ├── PanoramaPlugin        (panorama loading & conversion)      │
│  ├── SkyboxPlugin          (cubemap rendering)                  │
│  ├── CharacterPlugin       (GLB loading, animation, audio)      │
│  ├── CameraControllerPlugin (first-person controls)             │
│  └── HudPlugin             (UI overlay)                         │
├─────────────────────────────────────────────────────────────────┤
│  Resources                                                      │
│  ├── PanoramaState         (current panorama, loading status)   │
│  ├── CameraSettings        (FOV, sensitivity, invert Y)         │
│  └── AudioSettings         (master volume, spatial scale)       │
├─────────────────────────────────────────────────────────────────┤
│  ECS Components                                                 │
│  ├── Panorama              (skybox reference)                   │
│  ├── Character             (name, dialogue, position)           │
│  ├── CharacterHud          (billboard UI)                       │
│  ├── SpatialAudioSource    (3D audio emitter)                   │
│  └── FirstPersonCamera     (player camera marker)               │
└─────────────────────────────────────────────────────────────────┘
```

---

## Component Design

### 1. Panorama Component
```rust
#[derive(Component)]
pub struct Panorama {
    pub source_path: String,
    pub cubemap_handle: Handle<Image>,
    pub resolution: UVec2,
}

#[derive(Resource)]
pub struct PanoramaState {
    pub current: Option<Entity>,
    pub loading: bool,
    pub progress: f32,
}
```

### 2. Character Component
```rust
#[derive(Component)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub dialogue_audio: Handle<AudioSource>,
    pub is_talking: bool,
}

#[derive(Component)]
pub struct CharacterHud {
    pub offset: Vec3,        // Offset from character position
    pub visible: bool,
    pub fade_distance: f32,  // Distance at which HUD fades
}
```

### 3. Camera Controller Component
```rust
#[derive(Component)]
pub struct FirstPersonCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub fov: f32,
}

#[derive(Resource)]
pub struct CameraSettings {
    pub sensitivity: f32,    // 0.1 - 2.0
    pub fov: f32,            // 60.0 - 120.0
    pub invert_y: bool,
    pub smoothing: f32,      // 0.0 - 1.0
}
```

---

## Shader Design

### Equirectangular to Cubemap Conversion Shader

**Purpose**: Convert 2:1 equirectangular image to 6-face cubemap on GPU.

**Algorithm**:
1. For each cubemap face, generate UV coordinates
2. Convert UV to 3D direction vector based on face
3. Convert 3D direction to spherical coordinates (θ, φ)
4. Sample equirectangular texture at (θ/2π, φ/π)

```wgsl
// equirect_to_cubemap.wgsl

@group(0) @binding(0) var equirect_texture: texture_2d<f32>;
@group(0) @binding(1) var equirect_sampler: sampler;
@group(0) @binding(2) var<storage, read_write> cubemap_face: array<vec4<f32>>;
@group(0) @binding(3) var<uniform> face_index: u32;

const PI: f32 = 3.14159265359;
const TWO_PI: f32 = 6.28318530718;

// Convert UV + face index to 3D direction
fn uv_to_direction(uv: vec2<f32>, face: u32) -> vec3<f32> {
    let u = uv.x * 2.0 - 1.0;
    let v = uv.y * 2.0 - 1.0;
    
    switch face {
        case 0u: { return normalize(vec3<f32>( 1.0,   -v,   -u)); } // +X
        case 1u: { return normalize(vec3<f32>(-1.0,   -v,    u)); } // -X
        case 2u: { return normalize(vec3<f32>(   u,  1.0,    v)); } // +Y
        case 3u: { return normalize(vec3<f32>(   u, -1.0,   -v)); } // -Y
        case 4u: { return normalize(vec3<f32>(   u,   -v,  1.0)); } // +Z
        case 5u: { return normalize(vec3<f32>(  -u,   -v, -1.0)); } // -Z
        default: { return vec3<f32>(0.0); }
    }
}

// Convert 3D direction to equirectangular UV
fn direction_to_equirect_uv(dir: vec3<f32>) -> vec2<f32> {
    let theta = atan2(dir.z, dir.x);          // Longitude: -π to π
    let phi = asin(clamp(dir.y, -1.0, 1.0));  // Latitude: -π/2 to π/2
    
    let u = (theta + PI) / TWO_PI;            // 0 to 1
    let v = (phi + PI * 0.5) / PI;            // 0 to 1
    
    return vec2<f32>(u, 1.0 - v);             // Flip V for texture coords
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let face_size = 1024u; // Configurable
    if (id.x >= face_size || id.y >= face_size) { return; }
    
    let uv = vec2<f32>(f32(id.x) + 0.5, f32(id.y) + 0.5) / f32(face_size);
    let direction = uv_to_direction(uv, face_index);
    let equirect_uv = direction_to_equirect_uv(direction);
    
    let color = textureSampleLevel(equirect_texture, equirect_sampler, equirect_uv, 0.0);
    
    let pixel_index = id.y * face_size + id.x;
    cubemap_face[pixel_index] = color;
}
```

### Skybox Rendering (Bevy Built-in)

Bevy's `Skybox` component handles cubemap rendering. Custom shader not needed unless adding effects.

**Optional: Skybox Rotation Shader** (for animated transitions)
```wgsl
// skybox_rotate.wgsl - Fragment shader modification

@group(1) @binding(0) var<uniform> rotation: mat3x3<f32>;

fn sample_skybox(direction: vec3<f32>) -> vec4<f32> {
    let rotated_dir = rotation * direction;
    return textureSample(skybox_texture, skybox_sampler, rotated_dir);
}
```

---

## UI/UX Design

### Screen Layout
```
┌─────────────────────────────────────────────────────────────────┐
│ [Settings ⚙]                              [Fullscreen ⛶] [?] │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│                                                                 │
│                    ┌─────────────────┐                          │
│                    │   CHARACTER     │                          │
│                    │   ┌─────────┐   │                          │
│                    │   │  Name   │   │  ← Billboard HUD         │
│                    │   │ [▶ Play]│   │                          │
│                    │   └─────────┘   │                          │
│                    │      👤         │  ← 3D Character          │
│                    └─────────────────┘                          │
│                                                                 │
│                         ╋                ← Crosshair            │
│                                                                 │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ FOV: 90° | Pos: (0, 0, 0) | FPS: 60                            │
└─────────────────────────────────────────────────────────────────┘
```

### HUD Components

#### 1. Character Billboard HUD
- **Position**: Floats above character head (world-space)
- **Behavior**: Always faces camera (billboard)
- **Content**:
  - Character name (bold, white text)
  - Audio play/pause button
  - Distance indicator (fades with distance)

```
┌──────────────────────┐
│  ★ Character Name    │
│  ──────────────────  │
│  "Click to interact" │
│      [▶ Play]        │
└──────────────────────┘
```

#### 2. Settings Panel (Modal)
```
┌─────────────────────────────────────┐
│         ⚙ SETTINGS                  │
├─────────────────────────────────────┤
│  Camera                             │
│  ├─ FOV:         [====●====] 90°    │
│  ├─ Sensitivity: [==●======] 0.5    │
│  └─ Invert Y:    [ ] Off            │
│                                     │
│  Audio                              │
│  ├─ Master:      [======●==] 80%    │
│  └─ Spatial:     [●] On             │
│                                     │
│  Display                            │
│  └─ Fullscreen:  [ ] Off            │
│                                     │
│         [Apply]  [Cancel]           │
└─────────────────────────────────────┘
```

#### 3. Debug Overlay (Dev Mode)
```
┌─────────────────────────┐
│ FPS: 60.0               │
│ Frame: 16.67ms          │
│ Entities: 42            │
│ Draw Calls: 8           │
│ GPU Mem: 128MB          │
│ Camera: (0.0, 1.6, 0.0) │
│ Yaw: 45° Pitch: -10°    │
└─────────────────────────┘
```

---

## Visual Design

### Color Palette
| Element | Color | Hex |
|---------|-------|-----|
| HUD Background | Dark transparent | `#000000AA` |
| HUD Text | White | `#FFFFFF` |
| HUD Accent | Cyan | `#00D4FF` |
| Button Hover | Light blue | `#4DA6FF` |
| Crosshair | White 50% | `#FFFFFF80` |
| Warning | Orange | `#FF9500` |
| Error | Red | `#FF3B30` |

### Typography
| Element | Font | Size | Weight |
|---------|------|------|--------|
| Character Name | Fira Sans | 18px | Bold |
| HUD Text | Fira Sans | 14px | Regular |
| Button Text | Fira Sans | 14px | Medium |
| Debug Text | Fira Mono | 12px | Regular |

### Animations & Transitions

#### 1. Panorama Transition (Future)
```
Current Panorama ──[Fade Out 0.5s]──> Black ──[Fade In 0.5s]──> New Panorama

Timeline:
0.0s ─────── 0.5s ─────── 1.0s
[Current]   [Black]    [New]
Alpha: 1.0   0.0        1.0
```

#### 2. Character HUD Fade
```rust
// Distance-based opacity
fn calculate_hud_opacity(distance: f32, fade_start: f32, fade_end: f32) -> f32 {
    1.0 - ((distance - fade_start) / (fade_end - fade_start)).clamp(0.0, 1.0)
}

// Example: fade_start = 5m, fade_end = 15m
// At 5m: opacity = 1.0
// At 10m: opacity = 0.5
// At 15m+: opacity = 0.0
```

#### 3. Camera Smoothing
```rust
// Exponential smoothing for camera rotation
fn smooth_rotation(current: f32, target: f32, smoothing: f32, dt: f32) -> f32 {
    let t = 1.0 - (-smoothing * dt).exp();
    current + (target - current) * t
}
```

---

## Data Flow

### Panorama Loading Pipeline
```
User selects file
        │
        ▼
┌───────────────────┐
│ Validate Image    │ ← Check 2:1 ratio, format, size
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Load to GPU       │ ← Create texture, upload pixels
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Run Compute Shader│ ← Equirect → Cubemap conversion
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Create Cubemap    │ ← Assemble 6 faces into cubemap
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Apply to Skybox   │ ← Update Skybox component
└───────────────────┘
```

### Character Interaction Flow
```
User clicks on character
        │
        ▼
┌───────────────────┐
│ Raycast from      │
│ camera to world   │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Check collision   │ ← Character bounding box
│ with characters   │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Trigger dialogue  │ ← Play audio, show HUD
└───────────────────┘
```

---

## File Structure

```
techno-sutra-demo/
├── Cargo.toml
├── src/
│   ├── main.rs                 # App entry point
│   ├── lib.rs                  # Library exports
│   ├── plugins/
│   │   ├── mod.rs
│   │   ├── panorama.rs         # Panorama loading & conversion
│   │   ├── skybox.rs           # Skybox rendering
│   │   ├── character.rs        # Character management
│   │   ├── camera_controller.rs# First-person controls
│   │   └── hud.rs              # UI overlay
│   ├── components/
│   │   ├── mod.rs
│   │   ├── panorama.rs
│   │   ├── character.rs
│   │   └── camera.rs
│   ├── resources/
│   │   ├── mod.rs
│   │   ├── settings.rs
│   │   └── state.rs
│   ├── systems/
│   │   ├── mod.rs
│   │   ├── panorama_loader.rs
│   │   ├── cubemap_converter.rs
│   │   ├── camera_input.rs
│   │   ├── character_interaction.rs
│   │   └── audio_spatial.rs
│   └── shaders/
│       ├── equirect_to_cubemap.wgsl
│       └── billboard.wgsl
├── assets/
│   ├── panoramas/
│   │   └── demo_panorama.jpg
│   ├── models/
│   │   └── character.glb
│   ├── audio/
│   │   └── dialogue.ogg
│   ├── fonts/
│   │   └── FiraSans-Regular.ttf
│   └── shaders/
│       └── (compiled shaders)
└── docs/
    ├── requirements.md
    ├── design.md
    └── tasks.md
```

---

## State Machine

### Application States
```
                    ┌─────────┐
                    │ Loading │
                    └────┬────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
    ┌─────────┐    ┌──────────┐    ┌─────────┐
    │  Menu   │◄──►│  Viewer  │◄──►│Settings │
    └─────────┘    └──────────┘    └─────────┘
         │               │
         ▼               ▼
    ┌─────────┐    ┌──────────┐
    │  Exit   │    │ Dialogue │
    └─────────┘    └──────────┘
```

```rust
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    Viewer,
    Settings,
    Dialogue,
}
```

---

## Performance Considerations

### GPU Memory Budget
| Asset | Memory | Notes |
|-------|--------|-------|
| Equirect Source (4K) | ~32MB | Temporary, freed after conversion |
| Cubemap (1024/face) | ~24MB | 6 faces × 1024² × 4 bytes |
| Character Model | ~10MB | Depends on complexity |
| Audio Buffers | ~5MB | Streaming for long audio |
| **Total** | **~70MB** | Well under 256MB budget |

### Optimization Strategies
1. **Async Loading**: Load panoramas in background thread
2. **Texture Streaming**: Load lower mip levels first
3. **LOD for Characters**: Reduce poly count at distance
4. **Frustum Culling**: Don't render off-screen characters
5. **Audio Culling**: Mute audio beyond threshold distance
