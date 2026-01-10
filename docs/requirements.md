# Requirements - Bevy Panorama Viewer

## Project Overview

A cross-platform panorama viewer built with Bevy (Rust) that renders equirectangular 360° images as immersive first-person POV experiences with 3D character integration and spatial audio.

---

## Technology Stack

### Core Engine
| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| Game Engine | Bevy | 0.15+ | ECS-based rendering, audio, input |
| Language | Rust | 1.75+ | Performance, safety, cross-platform |
| Graphics API | wgpu | via Bevy | WebGPU/Vulkan/Metal/DX12 abstraction |
| Asset Format | GLTF/GLB | 2.0 | 3D models with animations |
| Image Format | PNG/JPG/HDR | - | Panorama source images |
| Audio Format | OGG/MP3/WAV | - | Spatial audio for characters |

### Dependencies (Cargo.toml)
```toml
[package]
name = "panorama-viewer"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = { version = "0.15", features = ["dynamic_linking"] }
image = "0.25"                    # Image processing
bevy_egui = "0.31"               # Debug UI (optional)

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

---

## Functional Requirements

### FR-01: Panorama Loading
- Load equirectangular images (2:1 aspect ratio)
- Support formats: PNG, JPG, JPEG, HDR
- Validate image dimensions (min 2048x1024, max 16384x8192)
- Display loading progress indicator

### FR-02: Equirectangular to Cubemap Conversion
- Convert equirectangular projection to 6-face cubemap
- GPU-accelerated conversion via compute shader
- Support both runtime and pre-baked conversion
- Output cubemap faces: +X, -X, +Y, -Y, +Z, -Z

### FR-03: Skybox Rendering
- Render cubemap as immersive skybox
- First-person POV camera at world origin
- Smooth camera rotation (mouse/keyboard/gamepad)
- Field of view adjustment (60°-120°)

### FR-04: 3D Character Integration
- Load GLB/GLTF models into panorama scene
- Position characters at specific world coordinates
- Support skeletal animations
- Billboard-style HUD attached to characters

### FR-05: Spatial Audio
- 3D positional audio for character dialogue
- Audio attenuation based on distance
- Support MP3/OGG/WAV formats
- Play/pause/stop controls

### FR-06: User Interface
- Minimal HUD with character info
- Settings panel (FOV, sensitivity, audio volume)
- File browser for loading panoramas
- Keyboard shortcuts overlay

---

## Non-Functional Requirements

### NFR-01: Performance
- Target 60 FPS on mid-range hardware
- Panorama load time < 3 seconds (4K image)
- Memory usage < 512MB for single panorama
- GPU memory < 256MB for cubemap + models

### NFR-02: Compatibility
| Platform | Status | Notes |
|----------|--------|-------|
| Windows 10/11 | Primary | Vulkan/DX12 |
| macOS 12+ | Supported | Metal |
| Linux (X11/Wayland) | Supported | Vulkan |
| Web (WASM) | Future | WebGPU required |

### NFR-03: Image Quality
- Cubemap face resolution: source_width / 4
- Bilinear filtering for sampling
- HDR support for environment lighting
- No visible seams at cubemap edges

---

## Input Requirements

### Keyboard
| Key | Action |
|-----|--------|
| W/A/S/D | Camera rotation |
| Q/E | Roll camera (optional) |
| +/- | Adjust FOV |
| Space | Play/pause character audio |
| Escape | Open settings menu |
| F11 | Toggle fullscreen |

### Mouse
| Input | Action |
|-------|--------|
| Move | Look around (when captured) |
| Left Click | Interact with character |
| Right Click | Toggle mouse capture |
| Scroll | Zoom (FOV adjustment) |

### Gamepad
| Input | Action |
|-------|--------|
| Right Stick | Look around |
| A/X | Interact |
| Start | Settings menu |

---

## Asset Requirements

### Demo Assets (Included)
```
assets/
├── panoramas/
│   └── demo_panorama.jpg      # 4096x2048 equirectangular
├── models/
│   └── character.glb          # Animated humanoid
├── audio/
│   └── dialogue.ogg           # Character speech
└── fonts/
    └── FiraSans-Regular.ttf   # UI font
```

### Panorama Specifications
- Format: Equirectangular projection
- Aspect ratio: 2:1 (width = 2 × height)
- Recommended resolution: 4096×2048 or 8192×4096
- Color space: sRGB (or linear for HDR)

### Character Model Specifications
- Format: GLB (binary GLTF)
- Max polygons: 50,000
- Max bones: 128
- Animations: Idle, Talk (optional)
- Scale: 1 unit = 1 meter

---

## Development Environment

### Required Tools
```bash
# Rust toolchain
rustup default stable
rustup component add clippy rustfmt

# Build dependencies (Linux)
sudo apt install libasound2-dev libudev-dev pkg-config

# Build dependencies (macOS)
xcode-select --install
```

### Build Commands
```bash
# Development (fast compile)
cargo run

# Release (optimized)
cargo run --release

# Check code
cargo clippy
cargo fmt --check
```

---

## Success Criteria

### MVP (Desktop Demo)
- [ ] Load any equirectangular JPG/PNG
- [ ] Convert to cubemap and display as skybox
- [ ] First-person camera with mouse look
- [ ] Load and display one GLB character
- [ ] Play spatial audio from character position
- [ ] Basic HUD showing character name

### Future Enhancements
- [ ] Multiple panorama scenes with transitions
- [ ] VR support via bevy_oxr
- [ ] Web deployment (WASM + WebGPU)
- [ ] Mobile touch controls
- [ ] Scene editor for placing characters
