# AGENTS.md - Techno Sutra (Vortex-R3D)

Guidelines for AI coding agents working in this repository.

## Project Overview

Cross-platform immersive VR/360° experience built with **Bevy 0.17** (Rust game engine). Supports Desktop, VR (OpenXR), WebXR (WASM), iOS, and Android.

## Build Commands

```bash
# Desktop (default feature)
cargo build --release --features desktop
cargo run --release

# VR build (requires OpenXR runtime)
cargo build --release --features vr

# WASM/Web build
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve              # Dev server at localhost:8080
trunk build --release   # Production build to dist/

# Android (requires NDK)
cargo build --release --target aarch64-linux-android --no-default-features \
  --features "bevy/bevy_asset,bevy/bevy_core_pipeline,bevy/bevy_pbr,bevy/bevy_render,bevy/bevy_winit,bevy/android-game-activity,bevy/png,bevy/jpeg"

# iOS
cargo build --release --target aarch64-apple-ios --no-default-features \
  --features "bevy/bevy_asset,bevy/bevy_core_pipeline,bevy/bevy_pbr,bevy/bevy_render,bevy/bevy_winit,bevy/png,bevy/jpeg"
```

## Lint/Format Commands

```bash
# Format check (CI enforced)
cargo fmt --check --package techno_sutra

# Apply formatting
cargo fmt

# Clippy (warnings as errors - CI enforced)
cargo clippy --release --features desktop -- -D warnings

# Quick clippy (dev mode)
cargo clippy --features desktop -- -D warnings
```

## Test Commands

```bash
# Run all tests
cargo test --package techno_sutra

# Run a single test module
cargo test --package techno_sutra --lib gaussian_splat::tests

# Run a specific test
cargo test --package techno_sutra --lib test_gaussian_splat_default

# Run tests with output
cargo test --package techno_sutra -- --nocapture

# Run integration tests
cargo test --package techno_sutra --lib integration_tests
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `desktop` | Default - keyboard/mouse, gamepad support |
| `vr` | OpenXR headset support |
| `webxr` | Browser-based WebXR |
| `particles` | GPU particle effects (bevy_hanabi) |

## Code Style Guidelines

### Imports

```rust
// Standard library (if needed)
use std::f32::consts::PI;

// External crates - Bevy first, then others alphabetically
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

// Internal modules (crate-level)
use crate::gaussian_splat::{GaussianSplat, SplatSettings};
use crate::GameState;
```

### Module Structure

- Each domain has its own module directory with `mod.rs`
- Public API exported via `pub use` in module root
- Platform-specific code in submodules gated by `#[cfg(feature = "...")]`
- Tests in `#[cfg(test)]` submodules within the source file

```rust
// src/camera/mod.rs
#[cfg(feature = "desktop")]
mod desktop;
#[cfg(feature = "vr")]
mod vr;

#[cfg(feature = "desktop")]
pub use desktop::DesktopCameraPlugin;
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Types/Structs/Enums | PascalCase | `GaussianSplat`, `GameState` |
| Functions/Methods | snake_case | `spawn_gaussian_cloud`, `update_stability` |
| Variables | snake_case | `splat_count`, `camera_transform` |
| Constants | SCREAMING_SNAKE | `SPLAT_PHYSICS_DENSITY` |
| Modules | snake_case | `gaussian_splat`, `camera` |
| Features | lowercase | `desktop`, `webxr` |

### Component/Resource Pattern

```rust
// Components - derive Component, add Reflect for editor support
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GaussianSplat {
    pub position: Vec3,
    pub opacity: f32,
}

// Resources - derive Resource
#[derive(Resource, Default)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
}

// States - derive States, Default
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Viewing,
}
```

### Plugin Pattern

All major systems use Bevy's Plugin trait:

```rust
pub struct GaussianSplatPlugin;

impl Plugin for GaussianSplatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SplatSettings>()
            .add_systems(Startup, setup_physics_world)
            .add_systems(Update, (
                update_splat_physics,
                handle_splat_collisions,
            ).run_if(in_state(GameState::Viewing)));
    }
}
```

### Builder Pattern for Bundles

```rust
impl SplatPhysicsBundle {
    pub fn dynamic_body() -> Self { /* ... */ }
    
    pub fn with_sphere(self, radius: f32) -> Self { /* ... */ }
    pub fn with_density(mut self, density: f32) -> Self { /* ... */ }
    pub fn lock_rotation(mut self) -> Self { /* ... */ }
}

// Usage
let bundle = SplatPhysicsBundle::dynamic_body()
    .with_sphere(0.5)
    .with_density(100.0)
    .lock_rotation();
```

### Error Handling

- Use `Option` and `Result` with `?` operator
- Use `let Ok(x) = ... else { return; }` for early returns
- Use `log::warn!` for recoverable issues, `log::error!` for failures

```rust
fn handle_collision(
    mut splat_query: Query<&mut GaussianSplat>,
) {
    let Ok(mut splat) = splat_query.get_single_mut() else { return };
    splat.on_collision();
}

fn load_asset(
    asset_server: &AssetServer,
) -> Result<Handle<Image>, AssetLoadError> {
    Ok(asset_server.load("panoramas/demo.jpg"))
}
```

### Platform Conditional Compilation

```rust
// Feature-gated modules
#[cfg(feature = "desktop")]
mod desktop;

// Feature-gated code blocks
#[cfg(target_os = "windows")]
fn platform_specific() { /* Windows only */ }

#[cfg(target_arch = "wasm32")]
fn wasm_only() { /* WASM only */ }

// Combined conditions
#[cfg(all(feature = "desktop", not(target_arch = "wasm32")))]
fn desktop_native() { /* Native desktop only */ }
```

### Query Filters

Use type parameters and query filters efficiently:

```rust
// Without<...> to exclude entities
fn setup_physics(
    query: Query<Entity, (With<GaussianSplat>, Without<RigidBody>)>,
) { /* ... */ }

// Changed<...> for optimization
fn update_materials(
    mut query: Query<&mut Material, Changed<GaussianSplat>>,
) { /* ... */ }

// Multiple queries with disjoint filters
fn animate(
    orbs: Query<&mut Transform, With<LightOrb>>,
    wisps: Query<&mut Transform, (With<EnergyWisp>, Without<LightOrb>)>,
) { /* ... */ }
```

### System Scheduling

```rust
// OnEnter for setup
.add_systems(OnEnter(GameState::Viewing), setup_world)

// Update with run conditions
.add_systems(Update, (
    update_physics,
    handle_collisions,
).run_if(in_state(GameState::Viewing)))

// Chain for ordering
.add_systems(Update, (input, process, render).chain())
```

## Project Structure

```
src/
├── main.rs           # Entry point, platform selection
├── lib.rs            # GamePlugin, re-exports, GameState
├── core/             # OS detection, shared types
├── camera/           # Camera controllers (desktop/vr/webxr)
├── input/            # Input abstraction layer
├── platform/         # Platform plugins
├── gaussian_splat/   # Gaussian splatting system
│   ├── mod.rs
│   ├── asset.rs      # Custom asset types
│   ├── render.rs     # Rendering pipeline
│   ├── physics.rs    # Rapier integration
│   ├── world.rs      # Splat world generation
│   ├── splat_types.rs # Data types
│   └── tests.rs      # Unit tests
├── loading/          # Asset loading with bevy_asset_loader
├── panorama/         # 360° panorama rendering
├── player/           # Player state and movement
├── world/            # Room/world setup
├── portals/          # Portal system
├── performance/      # FPS monitor, quality settings
├── upload_room/      # User upload experience
└── book_reader/      # Interactive book UI
```

## Assets

- `assets/panoramas/` - 360° equirectangular images (2:1 ratio, e.g., 4096x2048)
- `assets/models/` - GLB/GLTF 3D models
- `assets/splats/` - PLY Gaussian splat files
- `assets/audio/` - OGG/WAV audio files

## Pre-commit Checklist

1. `cargo fmt` - Format code
2. `cargo clippy --features desktop -- -D warnings` - Fix all warnings
3. `cargo test` - All tests pass
4. Check feature-specific code compiles: `cargo check --features vr`