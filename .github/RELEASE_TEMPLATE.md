# Techno Sutra {{ .TagName }}

{{ .Body }}

## 📥 Downloads

| Platform | File | Architecture | Notes |
|----------|------|--------------|-------|
| 🐧 Linux | `.tar.gz` | x86_64 | Portable - extract and run |
| 🐧 Linux | `.deb` | x86_64 | Debian/Ubuntu package |
| 🐧 Linux | `.AppImage` | x86_64 | Universal - `chmod +x` and run |
| 🪟 Windows | `.zip` | x86_64 | Portable - extract and run |
| 🍎 macOS | `.zip` | Universal | Intel + Apple Silicon |
| 📱 iOS | `.ipa` | arm64 | Unsigned - use AltStore/Sideloadly |
| 🤖 Android | `.apk` | arm64 | Standard mobile |
| 🥽 Quest | `.apk` | arm64 | Meta Quest VR |
| 🌐 Web | `.tar.gz` | WASM | Host on any web server |

## 💻 System Requirements

### Desktop
- GPU with Vulkan/Metal/DX12 support
- 4GB RAM minimum
- 500MB disk space

### Mobile
- Android 8.0+ / iOS 14+
- OpenGL ES 3.0 / Metal support

### VR (Quest)
- Meta Quest 2/3/Pro
- Developer mode enabled for sideloading

## 📖 Quick Start

### Linux
```bash
# AppImage (recommended)
chmod +x techno-sutra-*.AppImage
./techno-sutra-*.AppImage

# Debian/Ubuntu
sudo dpkg -i techno-sutra_*.deb

# Portable
tar -xzf techno-sutra-linux-*.tar.gz
./techno_sutra/techno_sutra
```

### Windows
1. Extract the `.zip` file
2. Run `techno_sutra.exe`

### macOS
1. Extract the `.zip` file
2. Move `Techno Sutra.app` to Applications
3. Right-click → Open (first time, to bypass Gatekeeper)

### iOS (Unsigned)
Use [AltStore](https://altstore.io/) or [Sideloadly](https://sideloadly.io/):
1. Install AltStore on your device
2. Download the `.ipa` file
3. Open with AltStore to install

### Android / Quest
```bash
# Standard Android
adb install techno-sutra-android-*.apk

# Quest (requires developer mode)
adb install techno-sutra-quest-*.apk
```

### Web
```bash
tar -xzf techno-sutra-web-*.tar.gz
python3 -m http.server 8080
# Open http://localhost:8080
```

## 🎮 Controls

| Input | Action |
|-------|--------|
| **Click** | Capture mouse |
| **Mouse Move** | Look around |
| **WASD / Arrows** | Look around |
| **+/-** | Adjust FOV |
| **Space** | Toggle character audio |
| **Escape** | Release mouse |

---

**Full Changelog**: {{ .PreviousTag }}...{{ .TagName }}
