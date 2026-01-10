#import bevy_pbr::forward_io::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> progress: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> direction: f32;

const PI: f32 = 3.14159265359;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = vec2<f32>(0.5, 0.5);
    let to_center = uv - center;
    let dist = length(to_center);
    let angle = atan2(to_center.y, to_center.x);
    
    let p = smoothstep(0.0, 1.0, progress);
    
    // Radial wipe
    let wipe_radius = select(p * 1.2, (1.0 - p) * 1.2, direction > 0.0);
    let wipe = smoothstep(wipe_radius - 0.15, wipe_radius + 0.05, dist);
    
    // Swirling energy
    let swirl = sin(angle * 6.0 + p * PI * 4.0 * direction) * 0.5 + 0.5;
    
    // Edge glow
    let edge_glow = smoothstep(wipe_radius - 0.08, wipe_radius, dist) * 
                    smoothstep(wipe_radius + 0.15, wipe_radius, dist);
    
    // Sparkles
    let sparkle = step(0.96, hash(uv * 30.0 + vec2(p * 20.0, 0.0))) * edge_glow * 4.0;
    
    // Energy color
    let energy = mix(vec3<f32>(0.15, 0.08, 0.5), vec3<f32>(0.5, 0.35, 0.95), swirl);
    
    let base_alpha = (1.0 - wipe) * p * 0.85;
    let glow_alpha = edge_glow * swirl * 0.6;
    let total_alpha = clamp(base_alpha + glow_alpha + sparkle * 0.3, 0.0, 0.92);
    
    let final_color = energy + vec3<f32>(0.9, 0.8, 1.0) * sparkle;
    
    return vec4<f32>(final_color, total_alpha);
}
