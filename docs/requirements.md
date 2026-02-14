# Requirements - Techno Sutra: Gaussian Splats Edition

## Project Vision

Transform Vortex-R3D from a panoramic skybox experience into a fully immersive **Gaussian Splatting-based world** where environments, characters, and objects are rendered as 3D Gaussian splats with realistic physics, texture solidity, and interactive gameplay following the "Techno Sutra: Cybermanju e o Bodhisattva" concept.

---

## Technology Stack

### Core Engine
| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| Game Engine | Bevy | 0.17+ | ECS-based rendering, async tasks |
| Language | Rust | 1.82+ | Performance, safety |
| Physics | bevy_rapier3d | 0.29+ | Rigid body, collision, joints |
| Splat Rendering | Custom WGSL | - | Gaussian splat rasterization |
| Splat Formats | .ply, .splat, .spz | - | 3D Gaussian data import |
| Audio | bevy_kira_audio | 0.24+ | Spatial 3D audio |
| Particles | bevy_hanabi | 0.17+ | GPU particle effects (desktop) |
| UI | bevy_ui + bevy_egui | 0.17/0.38 | Native + immediate mode UI |

### Dependencies (Cargo.toml additions)
```toml
[dependencies]
# Physics
bevy_rapier3d = { version = "0.29", features = ["simd-stable", "debug-render"] }

# Gaussian splat loading
ply-rs = "0.1"                    # PLY file parsing
byteorder = "1.5"                 # Binary data reading
half = "2.4"                      # FP16 support for compressed splats

# Spatial indexing for splat sorting
rstar = "0.12"                    # R-tree for spatial queries

# Texture generation for splat solidity
noise = "0.9"                     # Procedural texture generation
fast-poisson = "1.0"              # Poisson disk sampling for solidification

# Asset compression
zstd = "0.13"                     # Splat data compression
```

---

## Functional Requirements

### FR-01: Gaussian Splat Loading
- Load 3D Gaussian splat files in formats: `.ply`, `.splat`, `.spz`
- Support compressed formats (SPZ with FP16 positions)
- Validate splat data integrity (position, scale, rotation, color, opacity)
- Display loading progress for large scenes (>1M splats)
- Memory-efficient streaming for massive scenes

**Specifications:**
- Max splats per scene: 10 million
- Load time target: <5 seconds for 1M splats
- Memory budget: 500MB for splat data (position, covariance, color, opacity)

### FR-02: Real-Time Splat Rendering
- Custom WGSL compute shaders for Gaussian splat rasterization
- Depth-sorting for transparency (every frame for dynamic scenes)
- Tile-based rasterization for GPU efficiency
- Spherical harmonics support for view-dependent color
- Multi-level-of-detail (LOD) for distant splats

**Rendering Pipeline:**
```
Splat Data → Compute Sort (by depth) → Tile Assignment → 
Rasterization (alpha blending) → Final Composite
```

### FR-03: Splat Texture & Solidity (SOLIDEX System)
Each splat must support:
- **Base texture**: Albedo/diffuse color with alpha
- **Normal map**: Surface orientation for lighting
- **Roughness/Metallic**: PBR material properties
- **Solidity**: Physics collision hull generation from dense splat clusters

**SOLIDEX Generation:**
- Convert dense splat regions to signed distance fields (SDF)
- Generate convex hulls or voxel-based collision geometry
- LOD collision meshes for performance
- Material property baking from splat attributes

### FR-04: Physics Integration (Rapier)
- Rigid body simulation for splat-derived collision geometry
- Character controller with physics-based movement
- Splat environment collision (walls, floors, objects)
- Interactive physics objects within splat worlds
- Constraint/joint system for complex interactions

**Physics Features:**
- Kinematic character controller
- Static colliders from splat solidity
- Dynamic rigid bodies for movable objects
- Trigger volumes for gameplay events
- Raycasting for interaction detection

### FR-05: Environment System
- Multi-room splat worlds with seamless transitions
- Portal system between splat environments
- Dynamic lighting affecting splat rendering
- Time-of-day/world state changes
- Persistent world state across sessions

### FR-06: Character System (Cybermanju & Kalyanamitra)
- Characters as animated Gaussian splat models
- Lip-sync capable splat characters
- Billboard HUD attached to splat characters
- Interactive dialogue system

**Character Types:**
1. **Cybermanju**: AI companion with UI overlay
2. **Kalyanamitra**: Chapter teachers/guides
3. **Environment entities**: Interactive splat objects

### FR-07: Gameplay Mechanics (from idea.md)

#### 4 Challenge Types:

**1. Dialogue Choices (Sutile Preference Tests)**
- Branching narrative through dialogue
- Tests: Bravery, Love, Compassion, Anxiety, Anguish, Jealousy
- Affect relationship with Cybermanju
- Stored in player profile

**2. Philosophical Questions**
- User trains AI on Buddhist philosophy
- Questions about human-machine relationship
- Opinions on chapter themes
- Contributes to AI learning model

**3. Physical Key Hunt**
- Hidden keys in walkable splat world
- Visual indicator: twinkling light
- Interaction: Press ENTER when nearby
- Key reveals text/challenge for next Kalyanamitra

**4. Enigma Resolution**
- Password/key to unlock teacher
- Related to chat/dialogue content
- Progresses story to next chapter

### FR-08: Inventory & Progression
- **Keys**: Collected items with inscribed text
- **Character Sheet**: Collapsible UI overlay
  - Wisdom, Focus, Insight, Karma stats
  - Dialogue history
  - Unlocked chapters
- **Cybermanju Status**: Fixed UI with AI description
  - Evolution based on collected keys
  - Knowledge improvements

### FR-09: Audio System
- Spatial audio positioned in splat world
- Room-based soundtrack with crossfade
- Character dialogue with lip-sync
- Ambient audio tied to splat environment
- Narration system

### FR-10: User Interface
- **Book Reader**: Sacred text interface (bevy_ui)
  - Multi-page navigation
  - Chapter selection
  - Character stats display
- **Character HUD**: World-space UI above splat characters
- **Settings Panel**: Cross-platform options
  - Graphics quality (splat LOD, effects)
  - Audio levels
  - Physics debug visualization
  - Splat rendering options

---

## Non-Functional Requirements

### NFR-01: Performance
| Metric | Target | Notes |
|--------|--------|-------|
| FPS | 60+ on mid-range | RTX 3060 / RX 6600 |
| Splat render | 2M+ splats @ 60fps | With sorting |
| Load time | <5s for 1M splats | From SSD |
| Memory | <1GB GPU | Splat + physics + audio |
| Physics step | 60Hz stable | Deterministic |

### NFR-02: Quality Levels
```rust
pub enum SplatQuality {
    Ultra,    // 4M splats, full SH, 1m collision detail
    High,     // 2M splats, SH level 1, 0.5m collision
    Medium,   // 1M splats, no SH, 1m collision
    Low,      // 500K splats, basic color, 2m collision
    Potato,   // 200K splats, no transparency sort
}
```

### NFR-03: Platform Support
| Platform | Status | Splat Rendering | Physics |
|----------|--------|-----------------|---------|
| Windows 10/11 | Primary | Full | Full |
| macOS 12+ | Supported | Metal compute | Full |
| Linux | Supported | Vulkan compute | Full |
| Web (WASM) | Limited | Software sort | Basic |
| VR (OpenXR) | Future | Foveated | Full |

### NFR-04: Accessibility
- Adjustable text size (14px - 32px)
- Colorblind-friendly UI
- Subtitles for all dialogue
- Reduced motion option
- Physics difficulty settings

---

## Asset Requirements

### Splat Asset Specifications
```
assets/splats/
├── environments/
│   ├── room_01_garden.ply      # 2-5M splats, outdoor
│   ├── room_02_temple.ply      # 1-3M splats, indoor
│   ├── room_03_void.ply        # 500K splats, abstract
│   └── *.spz                   # Compressed versions
├── characters/
│   ├── cybermanju.ply          # Animated splat character
│   ├── kalyanamitra_01.ply     # Teacher 1
│   ├── kalyanamitra_02.ply     # Teacher 2
│   └── *.splat                 # Alternative format
└── objects/
    ├── key_artefact.ply        # Collectible keys
    ├── portal_frame.ply        # Portal structures
    └── furniture_*.ply         # Interactive objects
```

### Splat Data Format (per splat)
| Attribute | Type | Size | Description |
|-----------|------|------|-------------|
| Position | vec3<f32> | 12 bytes | XYZ world position |
| Scale | vec3<f32> | 12 bytes | XYZ scale (log space) |
| Rotation | vec4<f32> | 16 bytes | Quaternion |
| Color | vec3<f32> | 12 bytes | RGB (0-1) |
| Opacity | f32 | 4 bytes | Alpha (0-1) |
| SH Coeffs | f32[9] | 36 bytes | Spherical harmonics (optional) |
| **Total** | - | **92 bytes** | Per splat uncompressed |

### Physics Asset Specifications
- Collision hulls: Generated from splat density
- Material properties: Embedded in splat attributes
- Mass: Proportional to volume for dynamic objects

---

## Input Requirements

### Keyboard
| Key | Action |
|-----|--------|
| W/A/S/D | Character movement |
| Space | Jump (physics-based) |
| E / Enter | Interact with object/character |
| B | Toggle book/character sheet |
| Tab | Switch book tabs |
| Escape | Menu/Release cursor |
| F11 | Toggle fullscreen |
| N | Trigger narration |
| P | Pick up key (when nearby) |

### Mouse
| Input | Action |
|-------|--------|
| Move | Look around (FPS style) |
| Left Click | Interact/Select |
| Right Click | Hold for cursor mode |
| Scroll | Zoom (when in UI) |

### Gamepad
| Input | Action |
|-------|--------|
| Left Stick | Move |
| Right Stick | Look |
| A/Cross | Jump |
| B/Circle | Interact |
| X/Square | Open book |
| Start | Menu |

---

## Development Environment

### Required Tools
```bash
# Rust toolchain
rustup default stable
rustup component add clippy rustfmt

# Splat preprocessing tools
# (Python scripts for PLY optimization)
pip install numpy open3d plyfile

# Build dependencies (Linux)
sudo apt install libasound2-dev libudev-dev pkg-config clang

# Physics debug (optional)
cargo install bevy_rapier3d --features debug-render
```

### Build Commands
```bash
# Development (fast compile)
cargo run

# Release (optimized splat rendering)
cargo run --release

# With physics debug
cargo run --features rapier-debug

# Web build
cargo build --target wasm32-unknown-unknown --release
```

---

## Success Criteria

### MVP (Desktop Gaussian Splat)
- [ ] Load and render .ply/.splat files
- [ ] Depth-sorted transparency
- [ ] Basic physics collision from splats
- [ ] FPS movement with Rapier character controller
- [ ] One complete splat room with walkable areas
- [ ] Collectible key system
- [ ] Dialogue with splat character

### Full Release (Techno Sutra Complete)
- [ ] 3+ splat environments with portals
- [ ] 3+ Kalyanamitra splat characters
- [ ] Cybermanju AI companion with progression
- [ ] Complete 4 challenge types
- [ ] Chapter-based progression system
- [ ] "Sudhana was found" ending sequence
- [ ] VR support for splat worlds

### Future Enhancements
- [ ] Real-time splat editing/sculpting
- [ ] Multiplayer splat worlds
- [ ] AI-generated splat environments
- [ ] Neural radiance fields (NeRF) integration
- [ ] Haptic feedback for splat collision

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Splat sorting performance | High | Tile-based rasterization, LOD |
| Large file sizes | Medium | SPZ compression, streaming |
| Physics complexity | Medium | LOD collision, static optimization |
| VR performance | High | Foveated rendering, async reprojection |
| Cross-platform shaders | Medium | Comprehensive shader testing |

---

*Document Version: 1.0*
*Target: Vortex-R3D Gaussian Splats Transformation*
*Based on: idea.md (Techno Sutra: Cybermanju e o Bodhisattva)*
