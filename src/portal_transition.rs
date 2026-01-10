//! Portal Transition Effects - Seamless visual transitions when crossing portals

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

use crate::GameState;
use crate::portal_doors::RoomState;

pub struct PortalTransitionPlugin;

impl Plugin for PortalTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TransitionOverlayMaterial>::default())
            .init_resource::<TransitionState>()
            .add_systems(Update, (
                detect_portal_approach,
                animate_transition,
            ).chain().run_if(in_state(GameState::Viewing)));
    }
}

#[derive(Resource, Default)]
pub struct TransitionState {
    pub active: bool,
    pub progress: f32,
    pub from_room: usize,
    pub to_room: usize,
    pub fade_in: bool,
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct TransitionOverlayMaterial {
    #[uniform(0)]
    pub progress: f32,
    #[uniform(1)]
    pub direction: f32, // 1.0 = fade out, -1.0 = fade in
}

impl Material for TransitionOverlayMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/portal_transition.wgsl".into()
    }
    
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

fn detect_portal_approach(
    room_state: Res<RoomState>,
    mut trans_state: ResMut<TransitionState>,
) {
    // Detect when player is very close to a portal
    let door_positions = [
        bevy::math::Vec2::new(-4.0, -4.0),
        bevy::math::Vec2::new(4.0, -4.0),
    ];
    
    for door_pos in door_positions {
        let dist = room_state.pos.distance(door_pos);
        if dist < 1.5 && !trans_state.active {
            // Start subtle transition effect
            trans_state.progress = (1.5 - dist) / 1.5;
        }
    }
}

fn animate_transition(
    time: Res<Time>,
    mut trans_state: ResMut<TransitionState>,
) {
    if !trans_state.active { return; }
    
    let dt = time.delta_secs();
    
    if trans_state.fade_in {
        trans_state.progress -= dt * 3.0;
        if trans_state.progress <= 0.0 {
            trans_state.active = false;
            trans_state.progress = 0.0;
        }
    } else {
        trans_state.progress += dt * 3.0;
        if trans_state.progress >= 1.0 {
            trans_state.fade_in = true;
        }
    }
}
