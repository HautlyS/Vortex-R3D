#import bevy_pbr::{
    forward_io::{Vertex, VertexOutput},
    mesh_functions::get_world_from_local,
    view_transformations::position_world_to_clip,
    mesh_view_bindings::view,
}

struct SplatMaterial {
    base_color: vec4<f32>,
    opacity: f32,
    roughness: f32,
    metallic: f32,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> material: SplatMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    
    let world_from_local = get_world_from_local(0u);
    let world_position = world_from_local * vec4<f32>(vertex.position, 1.0);
    
    out.world_position = world_position;
    out.position = position_world_to_clip(world_position.xyz);
    out.uv = vertex.uv * 2.0 - 1.0;
    out.world_normal = vec3<f32>(0.0, 0.0, 1.0);
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let power = -0.5 * dot(uv, uv);
    let alpha = exp(power) * material.opacity;
    
    if (alpha < 0.01) {
        discard;
    }
    
    return vec4<f32>(material.base_color.rgb, alpha);
}