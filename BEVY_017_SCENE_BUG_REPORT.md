# Bevy 0.17.3 Scene Spawning Bug - Technical Report

## Issue Summary
**Two major Bevy 0.17 compatibility issues found and fixed:**

1. **SceneRoot/SceneSpawner panics** - Type registration bug (WORKAROUND APPLIED)
2. **Custom shader binding errors** - `@group(2)` → `@group(#{MATERIAL_BIND_GROUP})` (FIXED)

## Shader Binding Fix (RESOLVED)

### Problem
Custom material shaders using hardcoded `@group(2)` bindings fail with:
```
wgpu error: Validation Error
Shader global ResourceBinding { group: 2, binding: 0 } is not available in the pipeline layout
```

### Solution
Use Bevy's preprocessor variable `#{MATERIAL_BIND_GROUP}` instead of hardcoded group numbers:

**Before (broken):**
```wgsl
@group(2) @binding(0) var texture: texture_2d<f32>;
```

**After (working):**
```wgsl
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture: texture_2d<f32>;
```

### Files Fixed
- `assets/shaders/vortex_transition.wgsl`
- `examples/bevy_basic_portals/src/assets/portal.wgsl`

---

## Scene Spawning Bug (WORKAROUND)

## Error Chain
Each type registration reveals another missing type:
1. `bevy_transform::components::transform::Transform`
2. `bevy_transform::components::transform::TransformTreeChanged`
3. `bevy_camera::visibility::Visibility`
4. `bevy_ecs::hierarchy::Children`
5. ... (continues indefinitely)

## Root Cause
Bevy 0.17's new **"Reflect Auto Registration"** feature (using `inventory` crate) is not working correctly for internal Bevy types. The scene spawner requires all component types to be registered in the `TypeRegistry`, but many core types are missing.

## Affected Code Path
```
bevy_scene::scene_spawner::scene_spawner_system
  → SceneSpawner::spawn_queued_scenes()
  → Scene::write_to_world_with()
  → TypeRegistry lookup fails for unregistered types
```

## Attempted Workarounds

### 1. Manual Type Registration (Partial Success)
```rust
app.register_type::<Transform>()
   .register_type::<GlobalTransform>()
   .register_type::<TransformTreeChanged>()
   .register_type::<Visibility>()
   // ... endless chain
```
**Result**: Each registration reveals another missing type.

### 2. SceneSpawner Direct API (Failed)
```rust
scene_spawner.spawn(scene_handle);
```
**Result**: Same error - uses same internal scene system.

### 3. Direct GLTF Mesh Extraction (Current Workaround)
```rust
// Extract meshes/materials directly from Gltf asset
if let Some(gltf_mesh) = gltf_meshes.get(handle) {
    for primitive in &gltf_mesh.primitives {
        cmd.spawn((
            Mesh3d(primitive.mesh.clone()),
            MeshMaterial3d(primitive.material.clone()),
            transform,
        ));
    }
}
```
**Result**: Works but loses hierarchy, animations, and complex scene structure.

## Research Links

### Official Bevy Resources
- **Bevy 0.17 Release Notes**: https://bevy.org/news/bevy-0-17/
- **Migration Guide 0.16→0.17**: https://bevy.org/learn/migration-guides/0-16-to-0-17/
- **Bevy Discord**: https://discord.gg/bevy (check #help and #rendering channels)

### Relevant GitHub Issues/PRs
Search these on https://github.com/bevyengine/bevy:
- `scene unregistered type`
- `SceneSpawner panic`
- `reflect auto registration`
- `TypeRegistry scene`
- PR #15030 (Reflect Auto Registration)

### Key Documentation
- **bevy_scene crate**: https://docs.rs/bevy_scene/0.17.3/bevy_scene/
- **SceneSpawner**: https://docs.rs/bevy_scene/0.17.3/bevy_scene/struct.SceneSpawner.html
- **TypeRegistry**: https://docs.rs/bevy_reflect/0.17.3/bevy_reflect/struct.TypeRegistry.html
- **Gltf Asset**: https://docs.rs/bevy_gltf/0.17.3/bevy_gltf/struct.Gltf.html

### Bevy 0.17 API Changes to Investigate
1. **Event → Message rename**: `EventReader` → `MessageReader`, `EventWriter` → `MessageWriter`
2. **Scene system changes**: Check if `SceneRoot` usage changed
3. **Hierarchy changes**: `Children` component may have new requirements
4. **Reflect derive changes**: Check `#[reflect(no_auto_register)]` attribute

## Potential Solutions to Research

### 1. Check for Missing Plugin
There may be a plugin that registers all scene-related types:
```rust
// Try adding these if they exist:
app.add_plugins(ScenePlugin);
app.add_plugins(HierarchyPlugin);
app.add_plugins(TransformPlugin);
```

### 2. Use GltfSceneHandle Instead
Check if there's a new API for GLTF scenes:
```rust
// Bevy 0.17 may have new GLTF spawning API
commands.spawn(GltfSceneHandle(gltf.default_scene.clone()));
```

### 3. DynamicScene vs Scene
Try using `DynamicScene` which may have different registration requirements.

### 4. Check bevy_gltf Examples
Look at official Bevy 0.17 examples:
- https://github.com/bevyengine/bevy/blob/v0.17.3/examples/3d/load_gltf.rs
- https://github.com/bevyengine/bevy/blob/v0.17.3/examples/animation/animated_fox.rs

## Current Workaround Implementation

The project currently uses direct GLTF mesh extraction in `src/portal_doors.rs`:

```rust
fn spawn_gltf_model(
    cmd: &mut Commands,
    gltf: &Gltf,
    gltf_meshes: &Assets<GltfMesh>,
    transform: Transform,
    marker: impl Bundle,
) {
    if let Some(gltf_mesh_handle) = gltf.meshes.first() {
        if let Some(gltf_mesh) = gltf_meshes.get(gltf_mesh_handle) {
            for primitive in &gltf_mesh.primitives {
                cmd.spawn((
                    Mesh3d(primitive.mesh.clone()),
                    MeshMaterial3d(primitive.material.clone().unwrap_or_default()),
                    transform,
                    marker,
                ));
            }
        }
    }
}
```

### Limitations of Current Workaround
- ❌ Loses scene hierarchy (parent-child relationships)
- ❌ No animation support
- ❌ Only spawns first mesh (multi-mesh models incomplete)
- ❌ No skeleton/armature support
- ✅ Materials preserved
- ✅ No panics

## Recommended Next Steps

1. **Check Bevy Discord** for known issues with 0.17.3 scene spawning
2. **Search GitHub Issues** for similar reports
3. **Test with Bevy 0.17.0** vs 0.17.3 to see if it's a regression
4. **Review official examples** to see correct 0.17 scene spawning pattern
5. **Consider downgrade** to Bevy 0.16.x if blocking

## Environment
- Bevy: 0.17.3
- Rust: 1.88 (Edition 2024)
- OS: Linux (Pop!_OS)
- GPU: NVIDIA GeForce GTX 1650

---
*Report generated: 2026-01-10*
