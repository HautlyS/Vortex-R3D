#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> settings: SpinBlurSettings;
@group(2) @binding(1) var screen_tex: texture_2d<f32>;
@group(2) @binding(2) var screen_samp: sampler;

struct SpinBlurSettings {
    intensity: f32,
    time: f32,
    center: vec2<f32>,
}

const BLUR_SAMPLES: i32 = 12;
const PI: f32 = 3.14159265359;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = settings.center;
    let intensity = settings.intensity;
    
    if intensity < 0.01 {
        return textureSample(screen_tex, screen_samp, uv);
    }
    
    let to_center = uv - center;
    let dist = length(to_center);
    let angle = atan2(to_center.y, to_center.x);
    
    // Radial + rotational blur
    var color = vec4<f32>(0.0);
    let blur_strength = intensity * 0.08;
    
    for (var i = 0; i < BLUR_SAMPLES; i++) {
        let t = f32(i) / f32(BLUR_SAMPLES - 1) - 0.5;
        
        // Radial blur (zoom effect)
        let radial_offset = to_center * t * blur_strength;
        
        // Rotational blur (spin effect)
        let angle_offset = t * blur_strength * 2.0;
        let rot_dist = dist + t * blur_strength * 0.5;
        let rot_uv = center + vec2(
            cos(angle + angle_offset) * rot_dist,
            sin(angle + angle_offset) * rot_dist
        );
        
        let sample_uv = mix(uv + radial_offset, rot_uv, 0.5);
        let clamped_uv = clamp(sample_uv, vec2(0.001), vec2(0.999));
        
        color += textureSample(screen_tex, screen_samp, clamped_uv);
    }
    
    color /= f32(BLUR_SAMPLES);
    
    // Chromatic aberration during spin
    let chroma = intensity * 0.015;
    let r = textureSample(screen_tex, screen_samp, uv + to_center * chroma).r;
    let b = textureSample(screen_tex, screen_samp, uv - to_center * chroma).b;
    color.r = mix(color.r, r, intensity * 0.5);
    color.b = mix(color.b, b, intensity * 0.5);
    
    // Vignette intensifies during spin
    let vignette = 1.0 - dist * intensity * 0.8;
    color.rgb *= vignette;
    
    return color;
}
