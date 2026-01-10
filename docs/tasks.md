# Tasks - Bevy Panorama Viewer Implementation

## Phase 1: Project Setup & Core Infrastructure

### Task 1.1: Initialize Bevy Project
**Priority**: Critical | **Estimate**: 30 min

```bash
cargo new panorama-viewer
cd panorama-viewer
```

**Cargo.toml Configuration**:
```toml
[package]
name = "panorama-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = "0.15"
image = "0.25"

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
lto = "thin"
```

**Acceptance Criteria**:
- [ ] `cargo run` compiles without errors
- [ ] Empty Bevy window opens
- [ ] Hot reloading works with `cargo watch`

---

### Task 1.2: Create Module Structure
**Priority**: Critical | **Estimate**: 20 min

**Implementation**:
```rust
// src/lib.rs
pub mod plugins;
pub mod components;
pub mod resources;
pub mod systems;

// src/plugins/mod.rs
pub mod panorama;
pub mod camera_controller;
pub mod character;
pub mod hud;

// src/components/mod.rs
pub mod panorama;
pub mod character;
pub mod camera;

// src/resources/mod.rs
pub mod settings;
pub mod state;

// src/systems/mod.rs
pub mod panorama_loader;
pub mod cubemap_converter;
pub mod camera_input;
pub mod character_interaction;
```

**Acceptance Criteria**:
- [ ] All modules compile
- [ ] Clear separation of concerns

---

### Task 1.3: Setup Asset Directory
**Priority**: High | **Estimate**: 15 min

**Directory Structure**:
```
assets/
├── panoramas/
│   └── demo_panorama.jpg    # Download/create 4096x2048 equirect image
├── models/
│   └── character.glb        # Download from Mixamo or Sketchfab
├── audio/
│   └── dialogue.ogg         # Any short audio clip
├── fonts/
│   └── FiraSans-Regular.ttf # Download from Google Fonts
└── shaders/
    └── equirect_to_cubemap.wgsl
```

**Demo Asset Sources**:
- Panoramas: [Poly Haven](https://polyhaven.com/hdris) (free HDRIs)
- Characters: [Mixamo](https://www.mixamo.com/) (free with Adobe account)
- Audio: [Freesound](https://freesound.org/) (CC0 audio)

---

## Phase 2: Panorama Loading & Conversion

### Task 2.1: Implement Panorama Resource & Component
**Priority**: Critical | **Estimate**: 45 min

**File**: `src/components/panorama.rs`
```rust
use bevy::prelude::*;

/// Marker component for the panorama skybox entity
#[derive(Component)]
pub struct Panorama {
    pub source_path: String,
    pub original_resolution: UVec2,
}

/// Resource tracking panorama loading state
#[derive(Resource, Default)]
pub struct PanoramaState {
    pub loading: bool,
    pub current_handle: Option<Handle<Image>>,
    pub cubemap_handle: Option<Handle<Image>>,
}
```

**File**: `src/resources/settings.rs`
```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraSettings {
    pub sensitivity: f32,
    pub fov: f32,
    pub invert_y: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.5,
            fov: 90.0,
            invert_y: false,
        }
    }
}
```

---

### Task 2.2: Implement Equirectangular Image Loader
**Priority**: Critical | **Estimate**: 1 hour

**File**: `src/systems/panorama_loader.rs`
```rust
use bevy::prelude::*;
use crate::resources::PanoramaState;

/// System to load equirectangular image from file
pub fn load_panorama_image(
    asset_server: Res<AssetServer>,
    mut panorama_state: ResMut<PanoramaState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Press 'L' to load demo panorama (temporary, replace with file picker)
    if keyboard.just_pressed(KeyCode::KeyL) && !panorama_state.loading {
        panorama_state.loading = true;
        panorama_state.current_handle = Some(
            asset_server.load("panoramas/demo_panorama.jpg")
        );
        info!("Loading panorama...");
    }
}

/// System to check if panorama finished loading
pub fn check_panorama_loaded(
    mut panorama_state: ResMut<PanoramaState>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(handle) = &panorama_state.current_handle {
        if asset_server.is_loaded_with_dependencies(handle) {
            if let Some(image) = images.get(handle) {
                let size = image.size();
                // Validate 2:1 aspect ratio
                if size.x == size.y * 2 {
                    info!("Panorama loaded: {}x{}", size.x, size.y);
                    // Trigger conversion (Task 2.3)
                } else {
                    error!("Invalid aspect ratio: expected 2:1, got {}:{}", size.x, size.y);
                }
                panorama_state.loading = false;
            }
        }
    }
}
```

**Acceptance Criteria**:
- [ ] Loads JPG/PNG images from assets folder
- [ ] Validates 2:1 aspect ratio
- [ ] Reports image dimensions in console

---

### Task 2.3: Implement CPU-based Equirectangular to Cubemap Converter
**Priority**: Critical | **Estimate**: 2 hours

**Approach**: Start with CPU conversion for simplicity, optimize to GPU later.

**File**: `src/systems/cubemap_converter.rs`
```rust
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
use std::f32::consts::PI;

/// Convert equirectangular image to cubemap (CPU implementation)
pub fn convert_equirect_to_cubemap(
    source: &Image,
    face_size: u32,
) -> Image {
    let src_width = source.size().x;
    let src_height = source.size().y;
    let src_data = &source.data;
    
    // 6 faces stacked vertically
    let mut cubemap_data = vec![0u8; (face_size * face_size * 6 * 4) as usize];
    
    for face in 0..6u32 {
        for y in 0..face_size {
            for x in 0..face_size {
                // Normalize UV to [-1, 1]
                let u = (x as f32 + 0.5) / face_size as f32 * 2.0 - 1.0;
                let v = (y as f32 + 0.5) / face_size as f32 * 2.0 - 1.0;
                
                // Get 3D direction for this face
                let dir = face_uv_to_direction(face, u, v);
                
                // Convert direction to equirectangular UV
                let (eq_u, eq_v) = direction_to_equirect_uv(dir);
                
                // Sample source image (bilinear)
                let color = sample_bilinear(src_data, src_width, src_height, eq_u, eq_v);
                
                // Write to cubemap
                let dst_idx = ((face * face_size * face_size + y * face_size + x) * 4) as usize;
                cubemap_data[dst_idx..dst_idx + 4].copy_from_slice(&color);
            }
        }
    }
    
    let mut cubemap = Image::new(
        Extent3d {
            width: face_size,
            height: face_size,
            depth_or_array_layers: 6,
        },
        TextureDimension::D2,
        cubemap_data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    );
    
    cubemap.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    
    cubemap
}

fn face_uv_to_direction(face: u32, u: f32, v: f32) -> Vec3 {
    match face {
        0 => Vec3::new( 1.0,   -v,   -u).normalize(), // +X
        1 => Vec3::new(-1.0,   -v,    u).normalize(), // -X
        2 => Vec3::new(   u,  1.0,    v).normalize(), // +Y
        3 => Vec3::new(   u, -1.0,   -v).normalize(), // -Y
        4 => Vec3::new(   u,   -v,  1.0).normalize(), // +Z
        5 => Vec3::new(  -u,   -v, -1.0).normalize(), // -Z
        _ => Vec3::ZERO,
    }
}

fn direction_to_equirect_uv(dir: Vec3) -> (f32, f32) {
    let theta = dir.z.atan2(dir.x);           // Longitude
    let phi = dir.y.clamp(-1.0, 1.0).asin();  // Latitude
    
    let u = (theta + PI) / (2.0 * PI);
    let v = (phi + PI * 0.5) / PI;
    
    (u, 1.0 - v)
}

fn sample_bilinear(data: &[u8], width: u32, height: u32, u: f32, v: f32) -> [u8; 4] {
    let x = (u * width as f32).rem_euclid(width as f32);
    let y = (v * height as f32).clamp(0.0, height as f32 - 1.0);
    
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = (x0 + 1) % width;
    let y1 = (y0 + 1).min(height - 1);
    
    let fx = x.fract();
    let fy = y.fract();
    
    let sample = |px: u32, py: u32| -> [f32; 4] {
        let idx = ((py * width + px) * 4) as usize;
        [
            data[idx] as f32,
            data[idx + 1] as f32,
            data[idx + 2] as f32,
            data.get(idx + 3).copied().unwrap_or(255) as f32,
        ]
    };
    
    let c00 = sample(x0, y0);
    let c10 = sample(x1, y0);
    let c01 = sample(x0, y1);
    let c11 = sample(x1, y1);
    
    let mut result = [0u8; 4];
    for i in 0..4 {
        let top = c00[i] * (1.0 - fx) + c10[i] * fx;
        let bottom = c01[i] * (1.0 - fx) + c11[i] * fx;
        result[i] = (top * (1.0 - fy) + bottom * fy) as u8;
    }
    result
}
```

**Acceptance Criteria**:
- [ ] Converts 4096x2048 equirect to 1024x1024x6 cubemap
- [ ] No visible seams at face edges
- [ ] Conversion completes in < 2 seconds

---

### Task 2.4: Apply Cubemap to Skybox
**Priority**: Critical | **Estimate**: 30 min

**File**: `src/plugins/panorama.rs`
```rust
use bevy::prelude::*;
use bevy::core_pipeline::Skybox;
use crate::systems::{panorama_loader::*, cubemap_converter::*};
use crate::resources::PanoramaState;

pub struct PanoramaPlugin;

impl Plugin for PanoramaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PanoramaState>()
            .add_systems(Update, (
                load_panorama_image,
                check_panorama_loaded,
                apply_cubemap_to_skybox,
            ).chain());
    }
}

fn apply_cubemap_to_skybox(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    panorama_state: Res<PanoramaState>,
    mut skybox_query: Query<&mut Skybox>,
    // Trigger when cubemap is ready
) {
    // Implementation: Update Skybox.image with new cubemap handle
}
```

---

## Phase 3: Camera Controller

### Task 3.1: Implement First-Person Camera Component
**Priority**: Critical | **Estimate**: 1 hour

**File**: `src/components/camera.rs`
```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct FirstPersonCamera {
    pub yaw: f32,    // Horizontal rotation (radians)
    pub pitch: f32,  // Vertical rotation (radians)
}

impl Default for FirstPersonCamera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}
```

**File**: `src/systems/camera_input.rs`
```rust
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use crate::components::camera::FirstPersonCamera;
use crate::resources::settings::CameraSettings;

pub fn camera_mouse_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut Transform, &mut FirstPersonCamera)>,
    settings: Res<CameraSettings>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    if window.cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }
    
    let mut delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        delta += event.delta;
    }
    
    if delta == Vec2::ZERO {
        return;
    }
    
    for (mut transform, mut fp_camera) in camera_query.iter_mut() {
        let sensitivity = settings.sensitivity * 0.001;
        
        fp_camera.yaw -= delta.x * sensitivity;
        fp_camera.pitch -= delta.y * sensitivity * if settings.invert_y { -1.0 } else { 1.0 };
        
        // Clamp pitch to prevent flipping
        fp_camera.pitch = fp_camera.pitch.clamp(-1.5, 1.5);
        
        // Apply rotation
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            fp_camera.yaw,
            fp_camera.pitch,
            0.0,
        );
    }
}

pub fn toggle_cursor_grab(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut window = window.single_mut();
    
    if mouse_button.just_pressed(MouseButton::Left) {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
    
    if keyboard.just_pressed(KeyCode::Escape) {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}
```

**Acceptance Criteria**:
- [ ] Mouse look works when cursor is captured
- [ ] Click to capture, Escape to release
- [ ] Pitch clamped to prevent camera flip
- [ ] Sensitivity setting affects rotation speed

---

### Task 3.2: Implement FOV Control
**Priority**: Medium | **Estimate**: 30 min

```rust
pub fn camera_fov_control(
    mut camera_query: Query<&mut Projection, With<FirstPersonCamera>>,
    mut settings: ResMut<CameraSettings>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll: EventReader<MouseWheel>,
) {
    let mut fov_delta = 0.0;
    
    // Keyboard
    if keyboard.pressed(KeyCode::Equal) { fov_delta -= 1.0; }
    if keyboard.pressed(KeyCode::Minus) { fov_delta += 1.0; }
    
    // Mouse scroll
    for event in scroll.read() {
        fov_delta -= event.y * 5.0;
    }
    
    if fov_delta != 0.0 {
        settings.fov = (settings.fov + fov_delta).clamp(60.0, 120.0);
        
        for mut projection in camera_query.iter_mut() {
            if let Projection::Perspective(ref mut persp) = *projection {
                persp.fov = settings.fov.to_radians();
            }
        }
    }
}
```

---

## Phase 4: Character Integration

### Task 4.1: Load GLB Character Model
**Priority**: High | **Estimate**: 45 min

**File**: `src/plugins/character.rs`
```rust
use bevy::prelude::*;
use crate::components::character::Character;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_demo_character);
    }
}

fn spawn_demo_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/character.glb")
        )),
        Transform::from_xyz(0.0, 0.0, -5.0)
            .with_scale(Vec3::splat(1.0)),
        Character {
            name: "Demo Character".to_string(),
            description: "A friendly NPC".to_string(),
            is_talking: false,
        },
    ));
}
```

**File**: `src/components/character.rs`
```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub is_talking: bool,
}
```

**Acceptance Criteria**:
- [ ] GLB model loads and renders
- [ ] Model positioned in front of camera
- [ ] Model scale appropriate for scene

---

### Task 4.2: Add Spatial Audio to Character
**Priority**: High | **Estimate**: 45 min

```rust
fn spawn_character_with_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb")),
        Transform::from_xyz(0.0, 0.0, -5.0),
        Character { /* ... */ },
        // Spatial audio
        AudioPlayer::new(asset_server.load("audio/dialogue.ogg")),
        PlaybackSettings::PAUSED.with_spatial(true),
    ));
}

// Add listener to camera
fn setup_audio_listener(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FirstPersonCamera::default(),
        SpatialListener::new(0.3),
    ));
}
```

---

### Task 4.3: Implement Character HUD Billboard
**Priority**: Medium | **Estimate**: 1.5 hours

**Approach**: Use Bevy UI with world-space positioning.

```rust
use bevy::prelude::*;

#[derive(Component)]
pub struct CharacterHud {
    pub target_character: Entity,
}

fn spawn_character_hud(
    mut commands: Commands,
    characters: Query<(Entity, &Character, &Transform), Added<Character>>,
) {
    for (entity, character, _transform) in characters.iter() {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            CharacterHud { target_character: entity },
            children![
                (
                    Text::new(&character.name),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ),
            ],
        ));
    }
}

fn update_hud_position(
    mut hud_query: Query<(&mut Node, &CharacterHud)>,
    character_query: Query<&GlobalTransform, With<Character>>,
    camera_query: Query<(&GlobalTransform, &Camera), With<FirstPersonCamera>>,
) {
    let Ok((camera_transform, camera)) = camera_query.get_single() else { return };
    
    for (mut node, hud) in hud_query.iter_mut() {
        let Ok(char_transform) = character_query.get(hud.target_character) else { continue };
        
        // Project 3D position to screen
        let world_pos = char_transform.translation() + Vec3::Y * 2.0; // Above head
        if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
            node.left = Val::Px(screen_pos.x - 50.0);
            node.top = Val::Px(screen_pos.y);
        }
    }
}
```

---

## Phase 5: Main Application Assembly

### Task 5.1: Create Main Entry Point
**Priority**: Critical | **Estimate**: 30 min

**File**: `src/main.rs`
```rust
use bevy::prelude::*;
use panorama_viewer::plugins::{
    panorama::PanoramaPlugin,
    camera_controller::CameraControllerPlugin,
    character::CharacterPlugin,
    hud::HudPlugin,
};
use panorama_viewer::resources::settings::CameraSettings;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Panorama Viewer".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CameraSettings>()
        .add_plugins((
            PanoramaPlugin,
            CameraControllerPlugin,
            CharacterPlugin,
            HudPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.6, 0.0), // Eye height
        panorama_viewer::components::camera::FirstPersonCamera::default(),
        SpatialListener::new(0.3),
    ));
    
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });
}
```

---

### Task 5.2: Add Debug Overlay
**Priority**: Low | **Estimate**: 30 min

```rust
fn debug_overlay(
    mut commands: Commands,
    diagnostics: Res<DiagnosticsStore>,
    camera_query: Query<(&Transform, &FirstPersonCamera)>,
) {
    // Display FPS, camera position, etc.
}
```

---

## Phase 6: Testing & Polish

### Task 6.1: Test with Various Panorama Sizes
**Priority**: High | **Estimate**: 1 hour

**Test Cases**:
- [ ] 2048x1024 (minimum)
- [ ] 4096x2048 (standard)
- [ ] 8192x4096 (high quality)
- [ ] 16384x8192 (maximum)
- [ ] Non-2:1 ratio (should error gracefully)
- [ ] Corrupted image (should error gracefully)

---

### Task 6.2: Performance Profiling
**Priority**: Medium | **Estimate**: 1 hour

```bash
# Run with Tracy profiler
cargo run --release --features bevy/trace_tracy
```

**Metrics to Verify**:
- [ ] 60 FPS sustained
- [ ] Panorama load < 3 seconds
- [ ] Memory < 512MB

---

### Task 6.3: Cross-Platform Build Test
**Priority**: Medium | **Estimate**: 2 hours

```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

---

## Implementation Order Summary

```
Week 1:
├── Task 1.1: Initialize project ✓
├── Task 1.2: Module structure ✓
├── Task 1.3: Asset setup ✓
├── Task 2.1: Panorama components
├── Task 2.2: Image loader
└── Task 2.3: Cubemap converter (CPU)

Week 2:
├── Task 2.4: Apply to skybox
├── Task 3.1: Camera controller
├── Task 3.2: FOV control
├── Task 4.1: Load GLB character
└── Task 4.2: Spatial audio

Week 3:
├── Task 4.3: Character HUD
├── Task 5.1: Main assembly
├── Task 5.2: Debug overlay
├── Task 6.1: Testing
├── Task 6.2: Profiling
└── Task 6.3: Cross-platform builds
```

---

## Quick Start Commands

```bash
# Clone and setup
git clone <repo>
cd panorama-viewer

# Download demo assets
mkdir -p assets/panoramas assets/models assets/audio assets/fonts
# Add demo_panorama.jpg, character.glb, dialogue.ogg, FiraSans-Regular.ttf

# Run development build
cargo run

# Controls:
# - Click to capture mouse
# - Move mouse to look around
# - Press 'L' to load panorama
# - Press Escape to release mouse
# - +/- or scroll to adjust FOV
```
