#import bevy_pbr::forward_io::VertexOutput

struct PortalSettings {
    time: f32,
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> settings: PortalSettings;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var portal_tex: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var portal_samp: sampler;

const PI: f32 = 3.14159265359;
const TAU: f32 = 6.28318530718;

// Chinese red/gold palette
const RED_DEEP: vec3<f32> = vec3<f32>(0.55, 0.0, 0.0);
const RED_BRIGHT: vec3<f32> = vec3<f32>(0.86, 0.08, 0.24);
const GOLD: vec3<f32> = vec3<f32>(1.0, 0.84, 0.0);
const GOLD_DARK: vec3<f32> = vec3<f32>(0.72, 0.53, 0.04);

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), u.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), u.x), u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    var v = 0.0;
    var a = 0.5;
    var pos = p;
    for (var i = 0; i < 4; i++) {
        v += a * noise(pos);
        pos *= 2.0;
        a *= 0.5;
    }
    return v;
}

// Ink-like liquid flow
fn ink_flow(uv: vec2<f32>, t: f32) -> vec2<f32> {
    var d = vec2<f32>(0.0);
    d.x += sin(uv.y * 5.0 + t * 0.6) * 0.008;
    d.y += cos(uv.x * 5.0 + t * 0.5) * 0.008;
    d += (vec2(fbm(uv * 2.5 + t * 0.12)) - 0.5) * 0.015;
    return d;
}

// Circular energy rings (Chinese motif)
fn energy_rings(uv: vec2<f32>, t: f32) -> f32 {
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(uv - center);
    var rings = 0.0;
    for (var i = 0.0; i < 3.0; i += 1.0) {
        let r = 0.15 + i * 0.12 + sin(t * 0.8 + i) * 0.03;
        rings += smoothstep(0.02, 0.0, abs(dist - r)) * (0.6 - i * 0.15);
    }
    return rings;
}

// Flowing tendrils with Chinese aesthetic
fn red_tendrils(uv: vec2<f32>, edge: f32, t: f32) -> f32 {
    let angle = atan2(uv.y - 0.5, uv.x - 0.5);
    var e = 0.0;
    for (var i = 0.0; i < 3.0; i += 1.0) {
        let phase = t * (0.3 + i * 0.12) + i * PI * 0.4;
        let wave = sin(angle * 8.0 + phase) * 0.5 + 0.5;
        let pulse = sin(t * 1.5 + i * TAU / 3.0) * 0.2 + 0.8;
        e += wave * pulse * smoothstep(0.1, 0.0, edge - i * 0.012);
    }
    return e * 0.3;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = settings.time;
    
    // Edge distance
    let edge_x = min(uv.x, 1.0 - uv.x);
    let edge_y = min(uv.y, 1.0 - uv.y);
    let edge = min(edge_x, edge_y);
    
    let center = vec2<f32>(0.5, 0.5);
    let to_center = uv - center;
    let dist = length(to_center);
    let angle = atan2(to_center.y, to_center.x);
    
    // Ink-like liquid distortion
    let flow = ink_flow(uv, time);
    let edge_factor = smoothstep(0.0, 0.12, edge);
    var sample_uv = uv + flow * edge_factor;
    sample_uv += to_center * (1.0 - edge_factor) * 0.012;
    sample_uv = clamp(sample_uv, vec2(0.005), vec2(0.995));
    
    // Sample with red-shifted chromatic aberration
    let shift = (1.0 - edge_factor) * 0.003;
    let r = textureSample(portal_tex, portal_samp, sample_uv + vec2(shift, 0.0)).r;
    let g = textureSample(portal_tex, portal_samp, sample_uv).g;
    let b = textureSample(portal_tex, portal_samp, sample_uv - vec2(shift * 0.5, 0.0)).b;
    var color = vec3<f32>(r, g, b);
    
    // Red liquid surface highlights
    let highlight = pow(fbm(uv * 6.0 + vec2(time * 0.2, time * 0.15)), 2.0) * 0.1 * edge_factor;
    color += RED_BRIGHT * highlight;
    
    // Border glow - deep red with gold accents
    let border_w = 0.06;
    let border = 1.0 - smoothstep(0.0, border_w, edge);
    let border_phase = time * 1.2 + angle * 2.0;
    let border_col = mix(RED_DEEP, GOLD_DARK, sin(border_phase) * 0.3 + 0.3);
    
    // Red tendrils
    let tend = red_tendrils(uv, edge, time);
    
    // Circular energy rings
    let rings = energy_rings(uv, time);
    
    // Pulse
    let pulse = sin(time * 1.8) * 0.1 + 0.9;
    let glow = border * pulse * 2.0;
    
    // Gold sparkles
    let spark_uv = uv * 40.0 + vec2(time * 1.5, time * 1.2);
    let spark = step(0.97, hash(floor(spark_uv))) * border * 2.0;
    
    // Inner red flow
    let inner = smoothstep(0.3, 0.06, dist) * fbm(uv * 3.5 + vec2(time * 0.3, -time * 0.2)) * 0.06;
    
    // Combine
    var final_color = color;
    final_color = mix(final_color, border_col, glow * 0.4);
    final_color += border_col * glow * 0.4;
    final_color += mix(RED_BRIGHT, GOLD, 0.3) * tend;
    final_color += GOLD * spark;
    final_color += RED_DEEP * inner;
    final_color += mix(RED_BRIGHT, GOLD, 0.5) * rings * 0.15;
    
    // Edge fade
    final_color *= smoothstep(0.0, 0.006, edge);
    
    // Subtle vignette
    final_color *= 1.0 - dist * 0.1;
    
    let alpha = 0.96 + sin(time * 0.9) * 0.04;
    
    return vec4<f32>(final_color, alpha);
}
