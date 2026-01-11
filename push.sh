#!/bin/bash
set -e

# Incremental build settings
export CARGO_INCREMENTAL=1
export CARGO_BUILD_JOBS=$(nproc)

# Retry build with cache cleanup on linking errors
build_with_retry() {
    local cmd="$1"
    local output
    
    if output=$($cmd 2>&1); then
        echo "$output"
        return 0
    fi
    
    # Check for linking errors
    if echo "$output" | grep -qE "(cannot find -l|undefined reference|linking with .* failed|error: could not compile)"; then
        echo "⚠️  Linking error detected, cleaning crate cache..."
        cargo clean --release -p techno_sutra 2>/dev/null || true
        
        if $cmd; then
            return 0
        fi
        
        echo "⚠️  Still failing, full clean..."
        cargo clean --release
        $cmd
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
    
    echo "🔨 cargo build"
    build_with_retry "cargo build --release --features desktop"
    
    echo "✅ All checks passed!"
    exit 0
fi

git add .
git commit -m "${1:-update}"
git push
