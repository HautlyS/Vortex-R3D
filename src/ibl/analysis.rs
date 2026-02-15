//! Async panorama analysis for IBL with spherical harmonics extraction

use super::{
    AnalyzePanoramaEvent, IblDirectionalLight, IblLightProbe, IblReadyEvent, SphericalHarmonics,
};
use crate::loading::PanoramaAssets;
use async_channel::{bounded, Receiver, Sender};
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Resource)]
pub struct IblAnalysisChannel {
    pub rx: Receiver<IblAnalysisResult>,
}

pub struct IblAnalysisResult {
    pub dominant_dir: Vec3,
    pub dominant_color: Color,
    pub dominant_intensity: f32,
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub sh: SphericalHarmonics,
    pub exposure: f32,
    pub contrast: f32,
}

pub fn analyze_panorama_system(
    mut events: MessageReader<AnalyzePanoramaEvent>,
    mut light_probe: ResMut<IblLightProbe>,
    panorama_assets: Res<PanoramaAssets>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
) {
    for _ in events.read() {
        if light_probe.analyzed {
            continue;
        }

        let Some(image) = images.get(&panorama_assets.demo_panorama) else {
            continue;
        };
        let Some(data) = &image.data else { continue };

        let (tx, rx): (Sender<IblAnalysisResult>, Receiver<IblAnalysisResult>) = bounded(1);
        commands.insert_resource(IblAnalysisChannel { rx });

        let width = image.width() as usize;
        let height = image.height() as usize;
        let bpp = data.len() / (width * height);
        let data_clone = data.clone();

        // Spawn async analysis task
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            let result = analyze_image(&data_clone, width, height, bpp);
            let _ = tx.send_blocking(result);
        });

        #[cfg(target_arch = "wasm32")]
        {
            let result = analyze_image(&data_clone, width, height, bpp);
            let _ = tx.try_send(result);
        }

        light_probe.env_map = Some(panorama_assets.demo_panorama.clone());
        info!("🔄 IBL analysis started async...");
    }
}

pub fn receive_analysis_results(
    channel: Option<Res<IblAnalysisChannel>>,
    mut light_probe: ResMut<IblLightProbe>,
    mut ready_events: MessageWriter<IblReadyEvent>,
) {
    let Some(channel) = channel else { return };

    if let Ok(result) = channel.rx.try_recv() {
        light_probe.dominant_light_dir = result.dominant_dir;
        light_probe.dominant_light_color = result.dominant_color;
        light_probe.dominant_light_intensity = result.dominant_intensity;
        light_probe.ambient_color = result.ambient_color;
        light_probe.ambient_intensity = result.ambient_intensity;
        light_probe.spherical_harmonics = result.sh;
        light_probe.exposure = result.exposure;
        light_probe.contrast = result.contrast;
        light_probe.analyzed = true;

        ready_events.write(IblReadyEvent);
        info!(
            "✅ IBL: dir={:?}, intensity={:.0}, exposure={:.2}",
            light_probe.dominant_light_dir,
            light_probe.dominant_light_intensity,
            light_probe.exposure
        );
    }
}

fn analyze_image(data: &[u8], width: usize, height: usize, bpp: usize) -> IblAnalysisResult {
    let step = (width / 64).max(1);

    let mut brightest = 0.0f32;
    let mut brightest_dir = Vec3::Y;
    let mut brightest_color = Color::WHITE;
    let mut total_color = Vec3::ZERO;
    let mut total_weight = 0.0f32;
    let mut lum_sum = 0.0f32;
    let mut count = 0u32;
    let mut sh = SphericalHarmonics::default();

    for y in (0..height).step_by(step) {
        for x in (0..width).step_by(step) {
            let idx = (y * width + x) * bpp;
            if idx + 2 >= data.len() {
                continue;
            }

            let r = data[idx] as f32 / 255.0;
            let g = data[idx + 1] as f32 / 255.0;
            let b = data[idx + 2] as f32 / 255.0;
            let lum = 0.299 * r + 0.587 * g + 0.114 * b;

            lum_sum += lum;
            count += 1;

            // Equirectangular to direction
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let theta = u * 2.0 * PI;
            let phi = v * PI;
            let dir = Vec3::new(phi.sin() * theta.sin(), phi.cos(), phi.sin() * theta.cos());

            // Solid angle weight for equirectangular
            let solid_angle = phi.sin().max(0.001);

            // Accumulate SH coefficients
            accumulate_sh(&mut sh, dir, Vec3::new(r, g, b), solid_angle);

            if lum > brightest {
                brightest = lum;
                brightest_dir = dir;
                brightest_color = Color::srgb(r, g, b);
            }

            let w = (1.0 - lum * 0.5).max(0.1) * solid_angle;
            total_color += Vec3::new(r, g, b) * w;
            total_weight += w;
        }
    }

    // Normalize SH
    let norm = 4.0 * PI / count.max(1) as f32;
    for coeff in &mut sh.coeffs {
        coeff[0] *= norm;
        coeff[1] *= norm;
        coeff[2] *= norm;
    }

    let avg_lum = lum_sum / count.max(1) as f32;
    let ambient = total_color / total_weight.max(1.0);
    let contrast = brightest / avg_lum.max(0.01);

    let intensity = if contrast > 3.0 {
        50000.0 + brightest * 50000.0
    } else if contrast > 1.5 {
        10000.0 + brightest * 40000.0
    } else {
        1000.0 + brightest * 9000.0
    };

    let exposure = (0.5 / avg_lum.max(0.01)).clamp(0.5, 3.0);

    IblAnalysisResult {
        dominant_dir: brightest_dir.normalize(),
        dominant_color: brightest_color,
        dominant_intensity: intensity,
        ambient_color: Color::srgb(
            ambient.x.clamp(0.0, 1.0),
            ambient.y.clamp(0.0, 1.0),
            ambient.z.clamp(0.0, 1.0),
        ),
        ambient_intensity: avg_lum * 500.0,
        sh,
        exposure,
        contrast,
    }
}

fn accumulate_sh(sh: &mut SphericalHarmonics, dir: Vec3, color: Vec3, weight: f32) {
    let d = dir.normalize();
    let (x, y, z) = (d.x, d.y, d.z);

    let basis = [
        0.282095,
        0.488603 * y,
        0.488603 * z,
        0.488603 * x,
        1.092548 * x * y,
        1.092548 * y * z,
        0.315392 * (3.0 * z * z - 1.0),
        1.092548 * x * z,
        0.546274 * (x * x - y * y),
    ];

    for (i, b) in basis.iter().enumerate() {
        let w = *b * weight;
        sh.coeffs[i][0] += color.x * w;
        sh.coeffs[i][1] += color.y * w;
        sh.coeffs[i][2] += color.z * w;
    }
}

pub fn apply_ibl_lighting_system(
    mut commands: Commands,
    light_probe: Res<IblLightProbe>,
    existing: Query<Entity, With<IblDirectionalLight>>,
) {
    if !light_probe.analyzed || !light_probe.is_changed() {
        return;
    }

    for e in existing.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn((
        DirectionalLight {
            color: light_probe.dominant_light_color,
            illuminance: light_probe.dominant_light_intensity,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(-light_probe.dominant_light_dir, Vec3::Y),
        IblDirectionalLight,
    ));

    // Sample SH for ambient from multiple directions
    let up_color = light_probe.spherical_harmonics.sample(Vec3::Y);
    let ambient = Color::srgb(
        up_color.x.clamp(0.0, 1.0),
        up_color.y.clamp(0.0, 1.0),
        up_color.z.clamp(0.0, 1.0),
    );

    commands.insert_resource(AmbientLight {
        color: ambient,
        brightness: light_probe.ambient_intensity,
        affects_lightmapped_meshes: true,
    });
}
