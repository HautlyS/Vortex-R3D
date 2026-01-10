#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var texture_a: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var texture_b: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var texture_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> progress: f32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    let center = vec2<f32>(0.5, 0.5);
    let to_center = uv - center;
    let dist = length(to_center);
    let angle = atan2(to_center.y, to_center.x);
    
    // Vortex parameters
    let t = progress;
    let spiral_strength = sin(t * 3.14159) * 4.0;
    let radial_warp = sin(t * 3.14159) * 0.3;
    
    // Spiral distortion
    let new_angle = angle + spiral_strength * (1.0 - dist) * dist;
    let new_dist = dist * (1.0 + radial_warp * sin(dist * 10.0 - t * 8.0));
    
    // Chromatic aberration during transition
    let chroma = sin(t * 3.14159) * 0.02;
    
    let warped_uv = center + vec2<f32>(cos(new_angle), sin(new_angle)) * new_dist;
    let uv_r = mix(uv, warped_uv, 1.0) + vec2<f32>(chroma, 0.0);
    let uv_g = mix(uv, warped_uv, 1.0);
    let uv_b = mix(uv, warped_uv, 1.0) - vec2<f32>(chroma, 0.0);
    
    // Sample both textures with chromatic split
    let col_a_r = textureSample(texture_a, texture_sampler, uv_r).r;
    let col_a_g = textureSample(texture_a, texture_sampler, uv_g).g;
    let col_a_b = textureSample(texture_a, texture_sampler, uv_b).b;
    let col_a = vec4<f32>(col_a_r, col_a_g, col_a_b, 1.0);
    
    let col_b_r = textureSample(texture_b, texture_sampler, uv_r).r;
    let col_b_g = textureSample(texture_b, texture_sampler, uv_g).g;
    let col_b_b = textureSample(texture_b, texture_sampler, uv_b).b;
    let col_b = vec4<f32>(col_b_r, col_b_g, col_b_b, 1.0);
    
    // Smooth blend with vortex-shaped mask
    let blend_mask = smoothstep(0.0, 1.0, t + (dist - 0.5) * sin(t * 3.14159) * 0.5);
    var color = mix(col_a, col_b, blend_mask);
    
    // Add energy glow at vortex center during peak
    let glow = exp(-dist * 8.0) * sin(t * 3.14159) * 0.4;
    color = color + vec4<f32>(0.3, 0.5, 1.0, 0.0) * glow;
    
    return color;
}
