# Panorama Viewer

A cross-platform panorama viewer built with Bevy (Rust) that renders equirectangular 360° images as immersive first-person experiences.

## Features

- 🌐 **Equirectangular to Cubemap Conversion** - Automatic GPU-ready conversion
- 🎮 **First-Person Controls** - Mouse look + keyboard navigation
- 🎭 **3D Character Integration** - GLB/GLTF model support
- 🔊 **Spatial Audio** - 3D positional audio for characters
- 🖥️ **Cross-Platform** - Desktop (Windows/macOS/Linux) + Web (WASM)

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target (for web builds)
rustup target add wasm32-unknown-unknown

# Install trunk (for web builds)
cargo install --locked trunk
```

### Run Desktop

```bash
cargo run --release
```

### Run Web

```bash
trunk serve
# Open http://localhost:8080
```

## Controls

| Input | Action |
|-------|--------|
| **Click** | Capture mouse |
| **Mouse Move** | Look around |
| **WASD / Arrows** | Look around |
| **+/-** | Adjust FOV |
| **Space** | Toggle character audio |
| **Escape** | Release mouse |

## Project Structure

```
panorama-viewer/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # GamePlugin + states
│   ├── loading.rs           # Asset loading
│   ├── panorama.rs          # Equirect→Cubemap conversion
│   ├── camera_controller.rs # First-person controls
│   └── character.rs         # GLB + spatial audio
├── assets/
│   ├── panoramas/demo.jpg   # 4096x2048 equirectangular
│   ├── models/character.glb # 3D character
│   └── audio/dialogue.ogg   # Character audio
├── Cargo.toml
├── index.html               # WASM template
└── Trunk.toml               # WASM build config
```

## Adding Your Own Panorama

1. Place a 2:1 aspect ratio image (e.g., 4096x2048) in `assets/panoramas/`
2. Update `src/loading.rs` to point to your image:
   ```rust
   #[asset(path = "panoramas/your_panorama.jpg")]
   pub demo_panorama: Handle<Image>,
   ```
3. Run `cargo run`

## License

MIT / Apache-2.0
