# Complete Guide: Bevy 0.17/wgpu-26 Shader Validation Error

**Error Code**: `Shader global ResourceBinding { group: 2, binding: 0 } is not available in the pipeline layout`

**Version Context**: Bevy 0.17.3 with wgpu-26.0.1

---

## Executive Summary

This error occurs when the **fragment shader's bind group layout expectations don't match the render pipeline's actual bind group layout configuration**. Specifically, group 2, binding 0 is declared in the shader but either:

1. Isn't defined in the pipeline layout
2. Has a mismatched storage class (Storage vs Uniform)
3. Is declared with incompatible access patterns

This is a **validation error**, not a runtime bug—the GPU driver is correctly rejecting an invalid pipeline configuration.

---

## Understanding the Error

### Error Breakdown

```
In Device::create_render_pipeline, label = 'alpha_blend_mesh_pipeline'
  Error matching ShaderStages(FRAGMENT) shader requirements against the pipeline
    Shader global ResourceBinding { group: 2, binding: 0 } is not available in the pipeline layout
      Storage class Storage { access: StorageAccess(LOAD) } doesn't match the shader Uniform
```

**What this means**:
- The **fragment shader** references `@group(2) @binding(0)`
- The shader expects it to be a **uniform buffer** (`var<uniform>`)
- The **pipeline layout** declares it as a **storage buffer** with read-only access (`var<storage, read>`)
- These types are **incompatible**—the driver can't reconcile them

### Why This Happens in Bevy's Rendering Pipeline

Bevy's 3D mesh rendering uses a **hierarchical bind group structure**:

| Bind Group | Purpose | Content |
|-----------|---------|---------|
| **0** | View-level | Camera matrix, lighting data, view uniforms |
| **1** | Material-level | Textures, material parameters |
| **2** | Mesh-level | Mesh data, vertex colors, per-instance data |
| **3+** | Custom | Material extensions, custom data |

The `alpha_blend_mesh_pipeline` is Bevy's specialized render pipeline for **transparent and blended meshes**. It requires specific bind groups to be present for the shader to compile correctly.

---

## Root Causes & Solutions

### Root Cause #1: Missing or Incomplete Bind Group Layout in Pipeline

**Problem**: The pipeline is created without including all bind group layouts that the shader references.

**Diagnostic**:
- Check where `alpha_blend_mesh_pipeline` is created
- Verify it includes group 0, 1, and 2 bind group layouts
- Group 2 might be missing entirely or incomplete

**Solution**:

```rust
// ❌ WRONG: Pipeline created without complete layout
let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("my_pipeline_layout"),
    bind_group_layouts: &[
        &group_0_layout,
        &group_1_layout,
        // ❌ Missing group 2!
    ],
    push_constant_ranges: &[],
});

// ✅ CORRECT: All required groups included
let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some("my_pipeline_layout"),
    bind_group_layouts: &[
        &group_0_layout,
        &group_1_layout,
        &group_2_layout,  // ✅ Must include mesh bindings
    ],
    push_constant_ranges: &[],
});
```

---

### Root Cause #2: Storage vs. Uniform Buffer Type Mismatch

**Problem**: The shader declares `var<uniform>` but the pipeline layout defines it as `var<storage, read>`.

**In Shader** (WGSL):
```wgsl
// ❌ Declares as uniform buffer
@group(2) @binding(0)
var<uniform> mesh_data: MeshData;

// ✅ Correct if declared as storage (read-only)
@group(2) @binding(0)
var<storage, read> mesh_data: MeshData;
```

**In Rust Pipeline Code** (wgpu/Bevy):
```rust
// ❌ WRONG: Declaring storage buffer when shader expects uniform
let bind_group_layout_entries = &[
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage {
                read_only: true,  // ❌ This is storage, not uniform!
            },
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(std::mem::size_of::<MeshData>() as u64),
        },
        count: None,
    },
];

// ✅ CORRECT: Uniform buffer (if shader uses var<uniform>)
let bind_group_layout_entries = &[
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,  // ✅ Correct type
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(std::mem::size_of::<MeshData>() as u64),
        },
        count: None,
    },
];
```

**Storage vs. Uniform: Key Differences**:

| Property | Uniform Buffer | Storage Buffer |
|----------|--------|--------|
| **Max Size** | 16-64 KB typical | Megabytes typical |
| **Read/Write** | Read-only | Read-write (configurable) |
| **Alignment** | 16-byte (std140) | 4-byte or 8-byte |
| **Performance** | Optimized for small data | Better for large data |
| **Use Case** | Camera matrix, lighting | Large arrays, mesh data |

---

### Root Cause #3: Fragment Shader Visibility Restriction

**Problem**: The binding is defined but marked invisible to fragment shaders.

**In Rust**:
```rust
// ❌ WRONG: Binding invisible to fragment stage
let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
    binding: 0,
    visibility: wgpu::ShaderStages::VERTEX,  // ❌ Fragment shader can't see it!
    ty: wgpu::BindingType::Buffer { /* ... */ },
    count: None,
};

// ✅ CORRECT: Made visible to fragment shader
let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
    binding: 0,
    visibility: wgpu::ShaderStages::FRAGMENT,  // ✅ Fragment can access
    ty: wgpu::BindingType::Buffer { /* ... */ },
    count: None,
};

// ✅ OR: Visible to both stages
let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
    binding: 0,
    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
    ty: wgpu::BindingType::Buffer { /* ... */ },
    count: None,
};
```

---

### Root Cause #4: Material Extension Binding Conflict (Bevy-specific)

**Problem**: Custom `MaterialExtension` doesn't properly declare all bindings.

**Diagnosis**:

If you're using Bevy's material system with extensions:

```rust
// Check your material extension implementation
impl MaterialExtension for MyMaterial {
    fn bind_group_layout(render_device: &RenderDevice) -> wgpu::BindGroupLayout {
        // ❌ WRONG: Incomplete layout
        render_device.create_bind_group_layout(
            Some("my_material"),
            &[
                // Missing binding 0 or incomplete definitions
            ],
        )
    }

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
    ) -> Result<PreparedBindGroup, AsBindGroupError> {
        // ❌ WRONG: Entries don't match layout
        let mut entries = vec![];
        // Only binding 1, but layout expects 0 and 1
        entries.push(/* binding 1 only */);
        render_device.create_bind_group(Some("my_material"), layout, &entries)
    }
}
```

**Solution**:

```rust
impl MaterialExtension for MyMaterial {
    fn bind_group_layout(render_device: &RenderDevice) -> wgpu::BindGroupLayout {
        // ✅ CORRECT: Complete layout matching all shader bindings
        render_device.create_bind_group_layout(
            Some("my_material"),
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(size_of::<MyData>().try_into().unwrap()),
                    },
                    count: None,
                },
            ],
        )
    }

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
    ) -> Result<PreparedBindGroup, AsBindGroupError> {
        // ✅ CORRECT: Create buffer and bind group with all entries
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("my_material_buffer"),
            contents: bytemuck::bytes_of(&self.data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let entries = vec![
            BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            },
        ];

        Ok(render_device.create_bind_group(
            Some("my_material"),
            layout,
            &entries,
        ))
    }
}
```

---

## API Changes in wgpu-26 & Bevy 0.17

### wgpu-26 Breaking Changes

**1. Binding Array Restrictions**

If your pipeline uses binding arrays:

```rust
// ❌ NEW RESTRICTION: Cannot mix binding arrays with dynamic offsets
BindGroupLayoutEntry {
    binding: 0,
    visibility: ShaderStages::FRAGMENT,
    ty: BindingType::Buffer {
        ty: BufferBindingType::Uniform,
        has_dynamic_offset: true,  // ❌ FORBIDDEN with binding arrays!
        min_binding_size: None,
    },
    count: Some(NonZeroU32::new(10).unwrap()),  // Binding array
}

// ✅ CORRECT: Remove dynamic offsets when using binding arrays
BindGroupLayoutEntry {
    binding: 0,
    visibility: ShaderStages::FRAGMENT,
    ty: BindingType::Buffer {
        ty: BufferBindingType::Uniform,
        has_dynamic_offset: false,  // ✅ Must be false
        min_binding_size: None,
    },
    count: Some(NonZeroU32::new(10).unwrap()),
}
```

**2. Storage Access Validation**

wgpu-26 strictly validates that storage buffer access (`LOAD`, `STORE`, or both) matches the shader:

```wgsl
// ❌ SHADER: Declares read-write
@group(2) @binding(0)
var<storage, read_write> data: DataType;
```

```rust
// ❌ RUST: But pipeline only allows read
ty: BindingType::Buffer {
    ty: wgpu::BufferBindingType::Storage {
        read_only: true,  // ❌ Mismatch!
    },
    // ...
}

// ✅ RUST: Must allow write
ty: BindingType::Buffer {
    ty: wgpu::BufferBindingType::Storage {
        read_only: false,  // ✅ Allows both read and write
    },
    // ...
}
```

### Bevy 0.17 StandardMaterial Changes

**StandardMaterial Bind Groups Hierarchy** (Bevy 0.17):

```rust
// In bevy_pbr::mesh_pipeline:
// Group 0: View (camera, lighting)
// Group 1: Material (StandardMaterial params, textures)
// Group 2: Mesh (vertex data)

// When extending StandardMaterial:
impl Material for MyMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Group layout [0, 1] are provided by StandardMaterial
        // Group 2 is managed by mesh pipeline
        // If adding Group 3+, must extend layouts explicitly
        
        // ✅ Correct: Don't override groups 0-2, add beyond
        if let Some(ref mut layout) = descriptor.layout.get_mut(0) {
            // Extend, don't replace
        }
    }
}
```

---

## Debugging Steps

### Step 1: Enable Verbose Shader Errors

```bash
VERBOSE_SHADER_ERROR=1 RUST_BACKTRACE=full cargo run 2>&1 | tee error.log
```

This will show:
- Exact shader compilation errors
- Binding mismatch details
- Which bindings the shader requires

### Step 2: Inspect the Shader Code

Find the shader file being compiled for `alpha_blend_mesh_pipeline`:

```bash
# In Bevy source:
grep -r "alpha_blend_mesh_pipeline" ~/.cargo/registry/src/*/bevy-*/

# Or in your project:
grep -r "alpha_blend_mesh_pipeline" src/
```

Look at the WGSL shader and find the `@group(2) @binding(0)` declaration:

```wgsl
// This is what the shader declares
@group(2) @binding(0)
var<uniform> my_data: MyDataType;  // or var<storage, read>
```

### Step 3: Verify Pipeline Layout Creation

```rust
// In bevy_render::render_resource::pipeline_cache
// Add debug logging:

#[cfg(debug_assertions)]
{
    eprintln!("Pipeline layout groups: {}", pipeline_layout.bind_group_layouts.len());
    for (i, layout) in pipeline_layout.bind_group_layouts.iter().enumerate() {
        eprintln!("  Group {}: entries = ?", i);  // Can't easily inspect wgpu layouts
    }
}
```

### Step 4: Compare Declarations

Create a test file that shows what Bevy expects:

```rust
// test_bind_groups.rs
use bevy::prelude::*;
use bevy::pbr::*;

#[test]
fn print_material_bind_groups() {
    // Attempt to create material pipeline and inspect bind groups
    // This is exploratory - may require unsafe or internal API access
}
```

### Step 5: Check for Custom Shader Modifications

If using custom shaders:

```bash
# Search for any shader files that might be overriding alpha_blend:
find . -name "*.wgsl" -exec grep -l "alpha_blend\|group(2)" {} \;
find . -name "*.wgsl" -exec grep -l "@binding(0).*@group(2)" {} \;
```

---

## Common Scenarios & Solutions

### Scenario A: Extending StandardMaterial

**Symptom**: Error appears after adding a `MaterialExtension`

```rust
// ❌ WRONG APPROACH
#[derive(AsBindGroup, Debug, Clone)]
pub struct MyExtension {
    #[uniform(2, 0)]  // ❌ Trying to bind group 2, which is mesh's job
    pub data: MyData,
}

// ✅ CORRECT APPROACH
#[derive(AsBindGroup, Debug, Clone)]
pub struct MyExtension {
    #[uniform(2, 1)]  // ✅ Use binding 1 in your own group
    pub data: MyData,
}

// OR use group 3 if Bevy supports it:
#[derive(AsBindGroup, Debug, Clone)]
pub struct MyExtension {
    #[uniform(3, 0)]  // ✅ Custom group
    pub data: MyData,
}
```

### Scenario B: Custom Mesh Pipeline

**Symptom**: Creating a custom pipeline from scratch

```rust
// ✅ CORRECT: Include all standard bind groups
fn create_render_pipeline(
    device: &Device,
    view_layout: &BindGroupLayout,      // Group 0
    material_layout: &BindGroupLayout,  // Group 1
    mesh_layout: &BindGroupLayout,      // Group 2
) -> RenderPipeline {
    let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        bind_group_layouts: &[
            &view_layout,
            &material_layout,
            &mesh_layout,      // ✅ MUST include
        ],
        // ...
    });
    
    device.create_render_pipeline(&RenderPipelineDescriptor {
        layout: Some(&layout),
        // ...
    })
}
```

### Scenario C: Accessing Mesh Data in Fragment Shader

**Symptom**: Need to read mesh properties in fragment shader

```wgsl
// ✅ CORRECT APPROACH: Use group 2 (mesh bindings)
// But verify it's available - may need layout(set = 2, binding = 0, ...)

@group(2) @binding(0)
var<storage, read> mesh_data: array<MeshData>;

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let data = mesh_data[in.mesh_index];
    // Use mesh data
    return vec4<f32>(1.0);
}
```

---

## Environment Variables for Debugging

```bash
# Enable all debug features
VERBOSE_SHADER_ERROR=1 RUST_BACKTRACE=full WGPU_DEBUG=1 cargo run

# Disable validation (temporary, for testing):
WGPU_VALIDATION=0 cargo run

# Use specific adapter:
WGPU_ADAPTER_NAME="Intel" cargo run

# Force specific backend:
WGPU_BACKEND=vulkan cargo run  # or: dx12, metal, gl

# Dump shader information:
VERBOSE_SHADER_ERROR=1 cargo run 2>&1 | grep -i "group\|binding\|storage\|uniform"
```

---

## Prevention: Best Practices

### 1. **Always Match Shader to Rust Declarations**

For every `@group(X) @binding(Y)` in WGSL, have corresponding Rust code:

```rust
// Checklist:
// ✅ Binding declared in shader with @group/@binding
// ✅ BindGroupLayoutEntry created with matching binding number
// ✅ Storage type (Uniform vs Storage) matches
// ✅ Visibility includes the shader stages that use it
// ✅ Actual bind group entries include the buffer/texture
```

### 2. **Use Helper Macros**

```rust
// Create macro for consistency:
macro_rules! uniform_binding {
    ($binding:expr, $stage:expr, $type_size:expr) => {
        wgpu::BindGroupLayoutEntry {
            binding: $binding,
            visibility: $stage,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: std::num::NonZeroU64::new($type_size),
            },
            count: None,
        }
    };
}
```

### 3. **Document Bind Group Structure**

```rust
// In your pipeline module:
//
// Bind Group Structure for MyPipeline:
// ================================
// Group 0: View-level data
//   - Binding 0: Camera matrix (uniform)
//   - Binding 1: Lighting data (uniform)
//
// Group 1: Material data
//   - Binding 0: Material parameters (uniform)
//   - Binding 1: Albedo texture (texture_2d)
//   - Binding 2: Normal texture (texture_2d)
//
// Group 2: Mesh data (Bevy standard)
//   - Binding 0: Mesh vertex data (storage, read)
```

### 4. **Test with Simple Example**

Before complex pipelines, verify basic setup:

```rust
// Simple test pipeline with group 2
let simple_layout = device.create_bind_group_layout(
    Some("test_group_2"),
    &[BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,  // Start with uniform
            has_dynamic_offset: false,
            min_binding_size: Some(size_of::<u32>().try_into().unwrap()),
        },
        count: None,
    }],
);
// Test pipeline creation before adding complexity
```

---

## References & Further Reading

### Official Documentation
- [wgpu Validation Guide](https://docs.rs/wgpu/latest/wgpu/)
- [Bevy Rendering Architecture](https://bevyengine.org/learn/book/getting-started/)
- [WGSL Specification](https://www.w3.org/TR/WGSL/)

### Bevy Material System
- `bevy_pbr::Material` trait documentation
- `bevy_pbr::StandardMaterial` implementation
- `bevy_render::render_resource::BindGroup` documentation

### Common Issues
- [GitHub Issue #2617: Pipeline validation errors not clear](https://github.com/gfx-rs/wgpu/issues/2617)
- [Learn wgpu: Bind Groups Tutorial](https://sotrh.github.io/learn-wgpu/)

---

## Quick Checklist

When you see this error, verify:

- [ ] Fragment shader has `@group(2) @binding(0)`?
- [ ] Pipeline layout includes 3 bind group layouts (groups 0, 1, 2)?
- [ ] Group 2's binding 0 is declared as either `uniform` OR `storage, read`?
- [ ] Rust code matches the shader declaration exactly?
- [ ] Binding visibility includes `ShaderStages::FRAGMENT`?
- [ ] No dynamic offsets if using binding arrays?
- [ ] All bind group entries are actually created and passed to pipeline?
- [ ] Bevy version and wgpu version are compatible?

---

## Contact & Support

If this guide doesn't resolve your issue:

1. **Check Bevy Discord**: #rendering channel
2. **GitHub Issue**: Post minimal reproduction with version info
3. **Generate more logs**: Use `VERBOSE_SHADER_ERROR=1` and include full output