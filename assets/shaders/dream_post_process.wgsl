// Dream-like Post-Processing Shader - 2026 Modern Quality
// Advanced bloom, chromatic aberration, film grain, vignette, color grading

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct DreamSettings {
    blur_amount: f32,
    chromatic_strength: f32,
    vignette_intensity: f32,
    bloom_threshold: f32,
    time: f32,
    environment_warmth: f32,
    light_intensity: f32,
    _padding: f32,
}

@group(0) @binding(2) var<uniform> settings: DreamSettings;

const PI: f32 = 3.14159265359;

// High-quality gaussian blur kernel
fn gaussian_blur(uv: vec2<f32>, radius: f32) -> vec3<f32> {
    let texel = 1.0 / vec2<f32>(1920.0, 1080.0);
    var color = vec3<f32>(0.0);
    var total = 0.0;
    
    let samples = 9;
    let weights = array<f32, 9>(0.0162, 0.0540, 0.1216, 0.1945, 0.2270, 0.1945, 0.1216, 0.0540, 0.0162);
    
    for (var i = 0; i < samples; i++) {
        let offset = (f32(i) - 4.0) * radius;
        color += textureSample(screen_texture, texture_sampler, uv + vec2(offset, 0.0) * texel).rgb * weights[i];
        color += textureSample(screen_texture, texture_sampler, uv + vec2(0.0, offset) * texel).rgb * weights[i];
        total += weights[i] * 2.0;
    }
    return color / total;
}

// Kawase blur for efficient bloom
fn kawase_blur(uv: vec2<f32>, iteration: f32) -> vec3<f32> {
    let texel = 1.0 / vec2<f32>(1920.0, 1080.0);
    let offset = (iteration + 0.5) * texel;
    
    var color = textureSample(screen_texture, texture_sampler, uv + vec2(-offset.x, -offset.y)).rgb;
    color += textureSample(screen_texture, texture_sampler, uv + vec2(offset.x, -offset.y)).rgb;
    color += textureSample(screen_texture, texture_sampler, uv + vec2(-offset.x, offset.y)).rgb;
    color += textureSample(screen_texture, texture_sampler, uv + vec2(offset.x, offset.y)).rgb;
    
    return color * 0.25;
}

// Advanced chromatic aberration with barrel distortion
fn chromatic_aberration(uv: vec2<f32>, strength: f32) -> vec3<f32> {
    let center = vec2<f32>(0.5);
    let dir = uv - center;
    let dist = length(dir);
    let dist_sq = dist * dist;
    
    // Barrel distortion factor
    let barrel = 1.0 + dist_sq * 0.1;
    let offset = dir * dist_sq * strength * barrel;
    
    let r = textureSample(screen_texture, texture_sampler, uv + offset * 1.2).r;
    let g = textureSample(screen_texture, texture_sampler, uv + offset * 0.0).g;
    let b = textureSample(screen_texture, texture_sampler, uv - offset * 1.2).b;
    
    return vec3<f32>(r, g, b);
}

// Cinematic vignette with soft falloff
fn vignette(uv: vec2<f32>, intensity: f32, softness: f32) -> f32 {
    let center = vec2<f32>(0.5);
    let dist = length((uv - center) * vec2<f32>(1.0, 0.75)); // Aspect ratio correction
    let vig = 1.0 - smoothstep(0.4 * softness, 0.9, dist * intensity);
    return vig * vig; // Squared for smoother falloff
}

// Physically-based bloom extraction
fn extract_bloom(color: vec3<f32>, threshold: f32) -> vec3<f32> {
    let luminance = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    let soft_threshold = threshold * 0.8;
    let knee = smoothstep(soft_threshold, threshold + 0.2, luminance);
    return color * knee * knee;
}

// Multi-pass bloom simulation
fn bloom(uv: vec2<f32>, base_color: vec3<f32>, threshold: f32) -> vec3<f32> {
    var bloom_color = vec3<f32>(0.0);
    
    // Multiple blur passes at different scales
    bloom_color += kawase_blur(uv, 1.0) * 0.4;
    bloom_color += kawase_blur(uv, 2.0) * 0.3;
    bloom_color += kawase_blur(uv, 4.0) * 0.2;
    bloom_color += kawase_blur(uv, 8.0) * 0.1;
    
    // Extract bright areas
    let bright = extract_bloom(bloom_color, threshold);
    
    // Additive blend with energy conservation
    return base_color + bright * 0.6;
}

// Filmic tonemapping (ACES approximation)
fn aces_tonemap(color: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return saturate((color * (a * color + b)) / (color * (c * color + d) + e));
}

// Color grading based on environment
fn color_grade(color: vec3<f32>, warmth: f32, intensity: f32) -> vec3<f32> {
    // Lift-Gamma-Gain style grading
    let shadows = vec3<f32>(0.0, 0.02, 0.05) * (1.0 - warmth);
    let midtones = mix(vec3<f32>(0.95, 0.98, 1.05), vec3<f32>(1.08, 1.0, 0.92), warmth);
    let highlights = vec3<f32>(1.0 + warmth * 0.1, 1.0, 1.0 - warmth * 0.05);
    
    let luma = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    
    // Apply grading based on luminance
    var graded = color;
    graded = graded + shadows * (1.0 - luma);
    graded = graded * mix(vec3<f32>(1.0), midtones, 0.5);
    graded = graded * mix(vec3<f32>(1.0), highlights, luma);
    
    // Subtle saturation boost
    let sat_boost = 1.0 + intensity * 0.15;
    graded = mix(vec3<f32>(luma), graded, sat_boost);
    
    return graded;
}

// Film grain with temporal variation
fn film_grain(uv: vec2<f32>, time: f32, strength: f32) -> f32 {
    let noise_uv = uv * 500.0 + vec2<f32>(time * 100.0, time * 73.0);
    let n1 = fract(sin(dot(noise_uv, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    let n2 = fract(sin(dot(noise_uv + 0.5, vec2<f32>(39.346, 11.135))) * 43758.5453);
    return (n1 + n2 - 1.0) * strength;
}

// Lens flare simulation
fn lens_flare(uv: vec2<f32>, light_pos: vec2<f32>, intensity: f32) -> vec3<f32> {
    let dir = uv - light_pos;
    let dist = length(dir);
    
    // Main flare
    let flare = exp(-dist * 8.0) * intensity;
    
    // Anamorphic streak
    let streak = exp(-abs(dir.y) * 20.0) * exp(-abs(dir.x) * 3.0) * intensity * 0.3;
    
    // Color fringing
    let flare_color = vec3<f32>(1.0, 0.9, 0.7) * flare + vec3<f32>(0.7, 0.8, 1.0) * streak;
    
    return flare_color;
}

// Dream-like soft glow
fn dream_glow(uv: vec2<f32>, base_color: vec3<f32>, amount: f32) -> vec3<f32> {
    let blurred = gaussian_blur(uv, 3.0 * amount);
    let glow = max(blurred - base_color * 0.5, vec3<f32>(0.0));
    return base_color + glow * amount * 0.8;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    
    // Base color with chromatic aberration
    var color = chromatic_aberration(uv, settings.chromatic_strength);
    
    // Dream glow effect
    if (settings.blur_amount > 0.01) {
        color = dream_glow(uv, color, settings.blur_amount);
    }
    
    // Bloom
    color = bloom(uv, color, settings.bloom_threshold);
    
    // Lens flare for bright areas (subtle)
    let light_pos = vec2<f32>(0.5, 0.3);
    color += lens_flare(uv, light_pos, settings.light_intensity * 0.05);
    
    // Color grading
    color = color_grade(color, settings.environment_warmth, settings.light_intensity);
    
    // Tonemapping
    color = aces_tonemap(color);
    
    // Vignette
    let vig = vignette(uv, settings.vignette_intensity, 0.8);
    color = color * vig;
    
    // Film grain
    let grain = film_grain(uv, settings.time, 0.025);
    color = color + grain;
    
    // Final gamma correction
    color = pow(color, vec3<f32>(1.0 / 2.2));
    
    return vec4<f32>(saturate(color), 1.0);
}
