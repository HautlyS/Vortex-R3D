//! Light probe data structures with spherical harmonics and environment maps

use bevy::prelude::*;

/// Spherical Harmonics coefficients (L2 - 9 coefficients per channel)
#[derive(Clone, Default)]
pub struct SphericalHarmonics {
    pub coeffs: [[f32; 3]; 9], // RGB for each SH coefficient
}

impl SphericalHarmonics {
    /// Sample ambient color from SH in given direction
    pub fn sample(&self, dir: Vec3) -> Vec3 {
        let d = dir.normalize();
        let (x, y, z) = (d.x, d.y, d.z);

        // SH basis functions (L0, L1, L2)
        let basis = [
            0.282095,                       // Y00
            0.488603 * y,                   // Y1-1
            0.488603 * z,                   // Y10
            0.488603 * x,                   // Y11
            1.092548 * x * y,               // Y2-2
            1.092548 * y * z,               // Y2-1
            0.315392 * (3.0 * z * z - 1.0), // Y20
            1.092548 * x * z,               // Y21
            0.546274 * (x * x - y * y),     // Y22
        ];

        let mut result = Vec3::ZERO;
        for (i, b) in basis.iter().enumerate() {
            result += Vec3::new(self.coeffs[i][0], self.coeffs[i][1], self.coeffs[i][2]) * *b;
        }
        result.max(Vec3::ZERO)
    }
}

#[derive(Resource, Default)]
pub struct IblLightProbe {
    pub dominant_light_dir: Vec3,
    pub dominant_light_color: Color,
    pub dominant_light_intensity: f32,
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub analyzed: bool,
    pub spherical_harmonics: SphericalHarmonics,
    pub env_map: Option<Handle<Image>>,
    pub exposure: f32,
    pub contrast: f32,
}

#[derive(Event, bevy::ecs::message::Message)]
pub struct AnalyzePanoramaEvent;

#[derive(Event, bevy::ecs::message::Message)]
pub struct IblReadyEvent;

#[derive(Component)]
pub struct IblDirectionalLight;

#[derive(Component)]
pub struct IblLitModel;
