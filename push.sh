#!/bin/bash
set -e

export CARGO_INCREMENTAL=1
export CARGO_BUILD_JOBS=$(nproc)

build_with_retry() {
    local cmd="$1"
    local output
    
    if output=$(eval "$cmd" 2>&1); then
        echo "$output"
        return 0
    fi
    
    if echo "$output" | grep -qE "(cannot find -l|undefined reference|linking.*failed|error: could not compile)"; then
        echo "⚠️  Build error, cleaning cache..."
        cargo clean --release -p techno_sutra 2>/dev/null || true
        rm -rf dist 2>/dev/null || true
        
        if eval "$cmd"; then
            return 0
        fi
        
        echo "⚠️  Still failing, full clean..."
        cargo clean --release
        eval "$cmd"
    else
        echo "$output"
        return 1
    fi
}

if [ "$1" = "b" ]; then
    echo "🔧 Running pre-push checks (incremental)..."
    
    echo "📝 cargo fmt --check"
    cargo fmt --check --package techno_sutra
    
    echo "📎 cargo clippy"
    build_with_retry "cargo clippy --release --features desktop -- -D warnings"
    
    echo "🔨 cargo build (desktop)"
    build_with_retry "cargo build --release --features desktop"
    
    echo "🌐 trunk build (wasm)"
    if command -v trunk &> /dev/null; then
        build_with_retry "trunk build --release --public-url '/Vortex-R3D/'"
    else
        echo "⚠️  trunk not installed, skipping wasm build"
    fi
    
    echo "✅ All checks passed!"
    exit 0
fi

git add .
git commit -m "${1:-update}"
git push
