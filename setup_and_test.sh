#!/bin/bash
set -e

echo "🚀 Panorama Viewer Setup & Test Script"
echo "======================================="

cd /home/hautly/StreetView

# 1. Create test panorama (gradient image as placeholder)
echo "📸 Creating test panorama..."
mkdir -p assets/panoramas

# Create a simple test image using ImageMagick if available, otherwise use a solid color PNG
if command -v convert &> /dev/null; then
    convert -size 2048x1024 \
        -define gradient:angle=90 \
        gradient:'#1a1a2e-#00d4ff' \
        -fill white -gravity center -pointsize 48 \
        -annotate +0+0 "360° Panorama Test" \
        assets/panoramas/demo.jpg
    echo "✅ Created gradient panorama with ImageMagick"
else
    # Fallback: create minimal valid JPEG using pure bash + base64
    # This is a 64x32 blue gradient JPEG
    echo "⚠️ ImageMagick not found, creating minimal test image..."
    python3 << 'PYTHON_SCRIPT'
from PIL import Image
import os

# Create 2048x1024 gradient image
width, height = 2048, 1024
img = Image.new('RGB', (width, height))

for y in range(height):
    for x in range(width):
        # Gradient from dark blue to cyan
        r = int(26 + (0 - 26) * y / height)
        g = int(26 + (212 - 26) * y / height)
        b = int(46 + (255 - 46) * y / height)
        img.putpixel((x, y), (r, g, b))

img.save('assets/panoramas/demo.jpg', 'JPEG', quality=85)
print("✅ Created gradient panorama with Python/PIL")
PYTHON_SCRIPT
fi

# 2. Create minimal GLB character (cube placeholder)
echo "🎭 Creating placeholder character model..."
mkdir -p assets/models

# Create minimal valid GLB file (a simple cube)
python3 << 'PYTHON_SCRIPT'
import struct
import json
import base64
import os

# Minimal cube vertices and indices
vertices = [
    # positions (x, y, z) - 8 vertices of a cube
    -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5, -0.5,
    -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5, -0.5,  0.5,  0.5,
]

indices = [
    0, 1, 2, 2, 3, 0,  # front
    1, 5, 6, 6, 2, 1,  # right
    5, 4, 7, 7, 6, 5,  # back
    4, 0, 3, 3, 7, 4,  # left
    3, 2, 6, 6, 7, 3,  # top
    4, 5, 1, 1, 0, 4,  # bottom
]

# Pack binary data
vertex_data = struct.pack(f'{len(vertices)}f', *vertices)
index_data = struct.pack(f'{len(indices)}H', *indices)

# Pad to 4-byte alignment
while len(vertex_data) % 4 != 0:
    vertex_data += b'\x00'
while len(index_data) % 4 != 0:
    index_data += b'\x00'

buffer_data = index_data + vertex_data

# Create glTF JSON
gltf = {
    "asset": {"version": "2.0"},
    "scene": 0,
    "scenes": [{"nodes": [0]}],
    "nodes": [{"mesh": 0, "name": "Character"}],
    "meshes": [{
        "primitives": [{
            "attributes": {"POSITION": 1},
            "indices": 0
        }]
    }],
    "accessors": [
        {
            "bufferView": 0,
            "componentType": 5123,  # UNSIGNED_SHORT
            "count": len(indices),
            "type": "SCALAR"
        },
        {
            "bufferView": 1,
            "componentType": 5126,  # FLOAT
            "count": len(vertices) // 3,
            "type": "VEC3",
            "min": [-0.5, -0.5, -0.5],
            "max": [0.5, 0.5, 0.5]
        }
    ],
    "bufferViews": [
        {"buffer": 0, "byteOffset": 0, "byteLength": len(index_data)},
        {"buffer": 0, "byteOffset": len(index_data), "byteLength": len(vertex_data)}
    ],
    "buffers": [{"byteLength": len(buffer_data)}]
}

json_str = json.dumps(gltf, separators=(',', ':'))
json_data = json_str.encode('utf-8')

# Pad JSON to 4-byte alignment
while len(json_data) % 4 != 0:
    json_data += b' '

# Create GLB
glb_header = struct.pack('<4sII', b'glTF', 2, 12 + 8 + len(json_data) + 8 + len(buffer_data))
json_chunk = struct.pack('<II', len(json_data), 0x4E4F534A) + json_data  # JSON chunk
bin_chunk = struct.pack('<II', len(buffer_data), 0x004E4942) + buffer_data  # BIN chunk

with open('assets/models/character.glb', 'wb') as f:
    f.write(glb_header + json_chunk + bin_chunk)

print("✅ Created placeholder cube GLB model")
PYTHON_SCRIPT

# 3. Create placeholder audio
echo "🔊 Creating placeholder audio..."
mkdir -p assets/audio

# Create minimal valid OGG file (silence)
python3 << 'PYTHON_SCRIPT'
import wave
import struct
import os

# Create a simple WAV first (easier), then we'll use it
# For now, create an empty placeholder that Bevy can skip
with open('assets/audio/dialogue.ogg', 'wb') as f:
    # Minimal OGG header (will cause load error but won't crash)
    # Actually, let's create a valid WAV instead
    pass

# Create valid WAV file (Bevy supports WAV too)
sample_rate = 44100
duration = 0.1  # 100ms of silence
num_samples = int(sample_rate * duration)

with wave.open('assets/audio/dialogue.wav', 'w') as wav:
    wav.setnchannels(1)
    wav.setsampwidth(2)
    wav.setframerate(sample_rate)
    wav.writeframes(struct.pack(f'{num_samples}h', *([0] * num_samples)))

print("✅ Created placeholder WAV audio")
PYTHON_SCRIPT

echo ""
echo "📁 Asset structure:"
find assets -type f -exec ls -lh {} \;

echo ""
echo "🔨 Building project..."
cargo build 2>&1 | tee /tmp/build_output.txt

if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo ""
    echo "❌ BUILD FAILED - Errors:"
    grep -E "^error" /tmp/build_output.txt || cat /tmp/build_output.txt
    exit 1
fi

echo ""
echo "✅ Build successful!"
echo ""
echo "🎮 Running application..."
echo "   Controls:"
echo "   - Click to capture mouse"
echo "   - Mouse/WASD to look around"
echo "   - +/- to adjust FOV"
echo "   - Escape to release mouse"
echo ""

# Run with timeout and capture output
timeout 30 cargo run 2>&1 | tee /tmp/run_output.txt || true

echo ""
echo "📋 Run output saved to /tmp/run_output.txt"

# Check for errors
if grep -q "ERROR\|panic\|FAILED" /tmp/run_output.txt; then
    echo ""
    echo "⚠️ Errors detected during run:"
    grep -E "ERROR|panic|FAILED|error" /tmp/run_output.txt
fi
