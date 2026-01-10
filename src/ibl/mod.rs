//! Image-Based Lighting (IBL) Plugin
//! 
//! Analyzes panoramic images to extract lighting information and applies
//! it to 3D objects for seamless integration into the scene.

mod analysis;
mod light_probe;

pub use analysis::*;
pub use light_probe::*;

use bevy::prelude::*;
use crate::GameState;

pub struct IblPlugin;

impl Plugin for IblPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IblLightProbe>()
            .add_message::<AnalyzePanoramaEvent>()
            .add_message::<IblReadyEvent>()
            .add_systems(OnEnter(GameState::Viewing), trigger_analysis)
            .add_systems(Update, (
                analyze_panorama_system,
                receive_analysis_results,
                apply_ibl_lighting_system,
                apply_ibl_to_models,
            ).chain().run_if(in_state(GameState::Viewing)));
    }
}

fn trigger_analysis(mut events: MessageWriter<AnalyzePanoramaEvent>) {
    events.write(AnalyzePanoramaEvent);
}

/// Apply IBL-aware materials to marked models
fn apply_ibl_to_models(
    mut events: MessageReader<IblReadyEvent>,
    light_probe: Res<IblLightProbe>,
    models: Query<Entity, With<IblLitModel>>,
    children: Query<&Children>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mat_handles: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    for _ in events.read() {
        for entity in models.iter() {
            apply_ibl_recursive(entity, &light_probe, &children, &mut materials, &mat_handles);
        }
        info!("🎨 IBL materials applied to {} models", models.iter().count());
    }
}

fn apply_ibl_recursive(
    entity: Entity,
    light_probe: &IblLightProbe,
    children: &Query<&Children>,
    materials: &mut Assets<StandardMaterial>,
    mat_handles: &Query<&MeshMaterial3d<StandardMaterial>>,
) {
    // Apply to this entity's material
    if let Ok(mat_handle) = mat_handles.get(entity) {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            // Enhance material with IBL-derived properties
            let sh_sample = light_probe.spherical_harmonics.sample(Vec3::Y);
            let avg_brightness = (sh_sample.x + sh_sample.y + sh_sample.z) / 3.0;
            
            // Subtle emissive tint from environment
            mat.emissive = LinearRgba::new(
                sh_sample.x * 0.02,
                sh_sample.y * 0.02,
                sh_sample.z * 0.02,
                1.0,
            );
            
            // Adjust roughness based on scene contrast
            if light_probe.contrast > 2.0 {
                mat.perceptual_roughness = (mat.perceptual_roughness * 0.9).max(0.1);
            }
            
            // Boost reflectance for high-brightness environments
            if avg_brightness > 0.5 {
                mat.reflectance = (mat.reflectance + 0.1).min(1.0);
            }
        }
    }
    
    // Recurse to children
    if let Ok(kids) = children.get(entity) {
        for child in kids.iter() {
            apply_ibl_recursive(child, light_probe, children, materials, mat_handles);
        }
    }
}
