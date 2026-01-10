#!/bin/bash
# Final test script for Techno Sutra DEMO

echo "🎮 Techno Sutra DEMO - Final Test"
echo "================================"

cd /home/hautly/StreetView

# Check assets exist
echo ""
echo "📁 Checking assets..."
if [ -f "assets/panoramas/demo.jpg" ]; then
    echo "✅ Panorama: $(ls -lh assets/panoramas/demo.jpg | awk '{print $5}')"
else
    echo "❌ Missing: assets/panoramas/demo.jpg"
    exit 1
fi

if [ -f "assets/models/character.glb" ]; then
    echo "✅ Character: $(ls -lh assets/models/character.glb | awk '{print $5}')"
else
    echo "❌ Missing: assets/models/character.glb"
    exit 1
fi

# Build
echo ""
echo "🔨 Building..."
cargo build --release 2>&1 | tail -5

if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi
echo "✅ Build successful"

# Run test
echo ""
echo "🚀 Running application (15 seconds)..."
echo ""
echo "   CONTROLS:"
echo "   ─────────────────────────────────"
echo "   Click        → Capture mouse"
echo "   Mouse        → Look around"
echo "   WASD/Arrows  → Look around"
echo "   +/-          → Adjust FOV"
echo "   Escape       → Release mouse"
echo "   ─────────────────────────────────"
echo ""

timeout 15 cargo run --release 2>&1 | tee /tmp/final_test.txt

# Analyze results
echo ""
echo "📊 Test Results:"
echo "─────────────────────────────────"

if grep -q "Panorama skybox initialized" /tmp/final_test.txt; then
    echo "✅ Panorama loaded and converted"
else
    echo "❌ Panorama failed to load"
fi

if grep -q "Character spawned" /tmp/final_test.txt; then
    echo "✅ Character spawned"
else
    echo "❌ Character failed to spawn"
fi

if grep -q "Cubemap created" /tmp/final_test.txt; then
    echo "✅ Cubemap conversion successful"
else
    echo "❌ Cubemap conversion failed"
fi

# Check FPS
FPS=$(grep "fps" /tmp/final_test.txt | tail -1 | grep -oP '\d+\.\d+' | head -1)
if [ ! -z "$FPS" ]; then
    echo "✅ Performance: ~${FPS} FPS"
fi

# Check for errors
if grep -q "ERROR\|panic" /tmp/final_test.txt; then
    echo ""
    echo "⚠️ Errors found:"
    grep -E "ERROR|panic" /tmp/final_test.txt
else
    echo "✅ No errors detected"
fi

echo ""
echo "🎉 Test complete!"
echo ""
echo "To run manually: cargo run --release"
