# Practical Solutions & Code Examples

## The Error Explained (Visual)

```
Your Fragment Shader:
  @group(2) @binding(0)
  var<uniform> mesh_data: array<MeshData>;
                 ^^^^^^^ "I expect UNIFORM buffer"

Your Pipeline Layout:
  BindGroupLayoutEntry {
    binding: 0,
    ty: BufferBindingType::Storage { read_only: true }
        ^^^^^^^^^^^^^^^^^^^^^^^^ "I'm providing STORAGE buffer"
  }

Result:
  ❌ MISMATCH → Validation Error
```

---

## Solution 1: Fix Shader Declaration

### Problem Identification

If your error shows:
```
Storage { access: StorageAccess(LOAD) } doesn't match the shader Uniform
```

This means:
- **Pipeline** is declaring: storage buffer (read-only)
- **Shader** is expecting: uniform buffer

### Fix Option A: Change Shader to Storage

**Before (❌)**:
```wgsl
@group(2) @binding(0)
var<uniform> mesh_data: array<MeshData>;
// ❌ Shader expects uniform, but pipeline provides storage
```

**After (✅)**:
```wgsl
@group(2) @binding(0)
var<storage, read> mesh_data: array<MeshData>;
// ✅ Shader expects storage read-only, matches pipeline
```

**When to use**: When you have large data or arrays that don't need to be modified.

### Fix Option B: Change Pipeline to Uniform

**Before (❌)**:
```rust
let entry = wgpu::BindGroupLayoutEntry {
    binding: 0,
    visibility: wgpu::ShaderStages::FRAGMENT,
    ty: wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Storage {
            read_only: true,
        },
        has_dynamic_offset: false,
        min_binding_size: None,
    },
    count: None,
};
```

**After (✅)**:
```rust
let entry = wgpu::BindGroupLayoutEntry {
    binding: 0,
    visibility: wgpu::ShaderStages::FRAGMENT,
    ty: wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: Some(std::num::NonZeroU64::new(
            std::mem::size_of::<MeshData>() as u64
        ).unwrap()),
    },
    count: None,
};
```

**When to use**: When your data is small (<64 KB) and you want optimal performance.

---

## Solution 2: Verify Bind Group Layout Completeness

### Diagnostic Script

```rust
// Add this to your pipeline creation code for debugging:

fn verify_bind_group_layout(
    pipeline_layout: &wgpu::PipelineLayout,
    expected_groups: usize,
) {
    // Unfortunately, wgpu doesn't expose inspect APIs easily
    // But you can add instrumentation logging:
    
    eprintln!("Pipeline created with: {}", pipeline_layout_debug_info);
    eprintln!("Expected bind groups: {}", expected_groups);
    
    // Ensure your shader matches exactly what you create
}

// Or create a "schema" file documenting expected layout:
const EXPECTED_BIND_GROUPS: &str = r#"
Group 0 (View):
  - Binding 0: view_matrix (uniform)
  - Binding 1: lighting_data (uniform)

Group 1 (Material):
  - Binding 0: material_params (uniform)
  - Binding 1: albedo_texture (texture_2d)

Group 2 (Mesh):
  - Binding 0: mesh_data (storage, read)
"#;
```

### Complete Example: Creating Proper Bind Group Layout

```rust
use wgpu::*;
use std::num::NonZeroU64;
use std::mem::size_of;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MeshData {
    vertex_count: u32,
    instance_count: u32,
    // etc...
}

fn create_complete_pipeline_layout(
    device: &Device,
) -> PipelineLayout {
    // Group 0: View data
    let view_layout = device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            label: Some("view_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(
                            size_of::<[f32; 16]>() as u64  // 4x4 matrix
                        ),
                    },
                    count: None,
                },
            ],
        }
    );

    // Group 1: Material data
    let material_layout = device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            label: Some("material_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(16),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        }
    );

    // Group 2: Mesh data (THIS IS CRITICAL)
    let mesh_layout = device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            label: Some("mesh_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage {
                            read_only: true,
                        },
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(
                            size_of::<MeshData>() as u64
                        ),
                    },
                    count: None,
                },
            ],
        }
    );

    // ✅ CRITICAL: Create layout with ALL THREE groups
    device.create_pipeline_layout(
        &PipelineLayoutDescriptor {
            label: Some("complete_pipeline_layout"),
            bind_group_layouts: &[
                &view_layout,      // Group 0
                &material_layout,  // Group 1
                &mesh_layout,      // Group 2 ← MUST INCLUDE
            ],
            push_constant_ranges: &[],
        }
    )
}
```

---

## Solution 3: Create Matching Bind Groups

Once your layout is correct, create bind groups that match:

```rust
fn create_mesh_bind_group(
    device: &Device,
    mesh_layout: &BindGroupLayout,
    mesh_data: &MeshData,
) -> BindGroup {
    // Create buffer
    let buffer = device.create_buffer_init(
        &util::BufferInitDescriptor {
            label: Some("mesh_data_buffer"),
            contents: bytemuck::bytes_of(&mesh_data),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        }
    );

    // Create bind group with entries matching layout
    device.create_bind_group(
        &BindGroupDescriptor {
            label: Some("mesh_bind_group"),
            layout: &mesh_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
            ],
        }
    )
}
```

### Using Bind Groups in Render Pass

```rust
fn render_mesh(
    render_pass: &mut RenderPass,
    view_bind_group: &BindGroup,
    material_bind_group: &BindGroup,
    mesh_bind_group: &BindGroup,
) {
    render_pass.set_pipeline(&your_pipeline);
    
    // Set all three groups
    render_pass.set_bind_group(0, &view_bind_group, &[]);      // Group 0
    render_pass.set_bind_group(1, &material_bind_group, &[]);  // Group 1
    render_pass.set_bind_group(2, &mesh_bind_group, &[]);      // Group 2 ← DON'T FORGET
    
    render_pass.draw(0..vertex_count, 0..1);
}
```

---

## Solution 4: Bevy-Specific: Material Extensions

If using Bevy's material system:

### Problem: Conflicting Material Extensions

```rust
// ❌ WRONG: Trying to bind to group 2 (Bevy's mesh group)
#[derive(AsBindGroup)]
pub struct MyMaterial {
    #[uniform(2)]  // ❌ Group 2 is reserved for mesh!
    pub color: Vec4,
}
```

### Solution: Use Correct Group

```rust
// ✅ CORRECT: Use your own group (e.g., group 3)
#[derive(AsBindGroup)]
pub struct MyMaterial {
    #[uniform(3, 0)]  // ✅ Custom group 3, binding 0
    pub color: Vec4,
    
    #[texture(3, 1)]
    #[sampler(3, 2)]
    pub texture: Handle<Image>,
}

impl Material for MyMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Bevy handles groups 0, 1, 2
        // Your group 3 is added by AsBindGroup
        Ok(())
    }
}
```

### Full Working Example

```rust
use bevy::prelude::*;
use bevy::pbr::*;
use bevy::render::render_resource::*;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CustomMaterial {
    // Group 3, Binding 0
    #[uniform(3)]
    pub tint: LinearRgba,
    
    // Group 3, Binding 1
    #[texture(3, 1)]
    #[sampler(3, 2)]
    pub texture: Handle<Image>,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

// In your shader (custom_material.wgsl):
/*
@group(3) @binding(0)
var<uniform> tint: vec4<f32>;

@group(3) @binding(1)
var texture: texture_2d<f32>;

@group(3) @binding(2)
var tex_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(texture, tex_sampler, in.uv);
    return tex_color * tint;
}
*/
```

---

## Solution 5: Debug with Environment Variables

### Complete Debug Setup

```bash
#!/bin/bash
# debug_bevy.sh

export VERBOSE_SHADER_ERROR=1
export RUST_BACKTRACE=full
export WGPU_DEBUG=1
export RUST_LOG=debug

# Optional: force specific backend
# export WGPU_BACKEND=vulkan

# Run with all debug info
cargo run 2>&1 | tee debug.log

# Extract relevant info
echo "=== SHADER ERRORS ===" && grep -i "shader\|error\|binding\|group" debug.log
```

### Interpreting Debug Output

```
[2026-01-10T07:29:00.117985Z ERROR wgpu::backend::wgpu_core] 
Handling wgpu errors as fatal by default
├─ This means validation failed and pipeline won't be created

wgpu error: Validation Error
├─ Type of error (not a runtime panic)

In Device::create_render_pipeline, label = 'alpha_blend_mesh_pipeline'
├─ Which pipeline: alpha_blend_mesh_pipeline (for transparent objects)

Error matching ShaderStages(FRAGMENT) shader requirements against the pipeline
├─ Fragment shader is the problem
├─ Vertex shader is probably fine

Shader global ResourceBinding { group: 2, binding: 0 } is not available in the pipeline layout
├─ Shader uses: @group(2) @binding(0)
├─ Pipeline layout: doesn't include it!

Storage class Storage { access: StorageAccess(LOAD) } doesn't match the shader Uniform
├─ Pipeline layout: defines it as "storage, read"
├─ Shader expects: "uniform"
├─ MISMATCH!
```

---

## Solution 6: Shader Audit Checklist

Create this file to systematically check your shaders:

```wgsl
// shader_audit.wgsl - Template for checking bindings

// AUDIT CHECKLIST:
// ✅ For each @group/@binding in shader:
//    1. Is there a BindGroupLayoutEntry in Rust?
//    2. Does the entry match the storage class? (uniform vs storage)
//    3. Does the entry match the access? (read vs read_write)
//    4. Is visibility set to include this shader stage?
//    5. Is there a buffer/texture created and bound at runtime?

// Example: Checking group 2, binding 0
@group(2) @binding(0)
var<storage, read> mesh_data: array<MeshData>;
// AUDIT:
// ✅ 1. BufferBindingType::Storage { read_only: true } exists?
// ✅ 2. Storage class is "storage, read" ✓
// ✅ 3. Access is read (LOAD) ✓
// ✅ 4. Visibility includes FRAGMENT? ✓
// ✅ 5. Create buffer with STORAGE | COPY_DST usage?

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Can access mesh_data here if group 2 is properly bound
    let data = mesh_data[0];
    return vec4<f32>(1.0);
}
```

---

## Solution 7: Minimal Reproduction Test

When reporting the issue, create this minimal example:

```rust
// minimal_repro.rs - Minimal reproduction of the error

use wgpu::*;

fn main() {
    // Initialize wgpu
    let instance = Instance::new(&InstanceDescriptor::default());
    let adapter = pollster::block_on(
        instance.request_adapter(&RequestAdapterOptions::default())
    ).unwrap();
    let (device, _queue) = pollster::block_on(
        adapter.request_device(&DeviceDescriptor::default(), None)
    ).unwrap();

    // Reproduce the error
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("test_shader"),
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
            r#"
            @group(2) @binding(0)
            var<uniform> mesh_data: vec4<f32>;
            
            @fragment
            fn main() -> @location(0) vec4<f32> {
                return mesh_data;
            }
            "#
        )),
    });

    // Create minimal layout (❌ INTENTIONALLY WRONG to reproduce error)
    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("bad_layout"),
        bind_group_layouts: &[],  // ❌ Missing group 2!
        push_constant_ranges: &[],
    });

    // Try to create pipeline → Should panic with the error
    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("test_pipeline"),
        layout: Some(&layout),
        push_constant_ranges: vec![],
        vertex: VertexState {
            module: &shader,
            entry_point: "main",
            buffers: &[],
        },
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "main",
            targets: &[Some(ColorTargetState {
                format: TextureFormat::Bgra8UnormSrgb,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });

    // If we get here, something went wrong (should have panicked)
    println!("Pipeline created (unexpected!)");
}
```

---

## Quick Reference: Common Fixes

| Error | Cause | Fix |
|-------|-------|-----|
| `group: 2, binding: 0 is not available` | Missing group 2 in layout | Add mesh_layout to bind_group_layouts array |
| `doesn't match the shader Uniform` | Pipeline says Storage, shader says Uniform | Change shader to `var<storage, read>` |
| `doesn't match the shader Storage` | Pipeline says Uniform, shader says Storage | Change pipeline to `BufferBindingType::Storage` |
| `ShaderStages(FRAGMENT)` error | Fragment can't see the binding | Add `ShaderStages::FRAGMENT` to visibility |
| Frame skip with "panic in system" | Async pipeline compilation failed | Check VERBOSE_SHADER_ERROR output |

---

## Testing Your Fix

After applying a solution:

```bash
# 1. Clean build
cargo clean

# 2. Run with full debugging
VERBOSE_SHADER_ERROR=1 RUST_BACKTRACE=full cargo run

# 3. Look for:
#    ✅ No validation errors
#    ✅ Pipeline created successfully
#    ✅ Rendering occurs without panic

# 4. If still failing, check:
#    - Did you update BOTH shader and Rust code?
#    - Are all 3 bind groups included?
#    - Are visibility flags correct?
#    - Did you recompile (cargo clean)?
```

---

## Getting Help

If you're still stuck, gather this information:

```bash
# Create debug dump
cat > debug_info.txt << EOF
# System
uname -a
rustc --version
cargo --version

# Bevy version
grep "^bevy = " Cargo.toml

# wgpu version  
grep "^wgpu = " Cargo.toml

# Full error output
VERBOSE_SHADER_ERROR=1 RUST_BACKTRACE=full cargo run 2>&1

# Your shader code
cat src/my_shader.wgsl

# Relevant Rust code
grep -A 20 "bind_group_layout" src/main.rs
EOF

# Post to:
# 1. Bevy Discord: #rendering
# 2. GitHub: bevyengine/bevy/discussions
# 3. Include debug_info.txt
```
