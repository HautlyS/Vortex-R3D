//! Player module - FPS movement, position, room state

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;

use crate::input::InputState;
use crate::panorama::PanoramaCamera;
use crate::world::room_center;
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_systems(OnEnter(GameState::Viewing), init_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Viewing)));
    }
}

#[derive(Resource)]
pub struct PlayerState {
    pub room: usize,
    pub pos: Vec2, // XZ position in room-local coords
    pub prev_pos: Vec2,
    pub height: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            room: 0,
            pos: Vec2::ZERO,
            prev_pos: Vec2::ZERO,
            height: 1.7,
        }
    }
}

fn init_player(
    mut cmd: Commands,
    cam_q: Query<Entity, (With<PanoramaCamera>, Without<RenderLayers>)>,
) {
    for entity in cam_q.iter() {
        cmd.entity(entity).insert(RenderLayers::layer(0));
    }
}

fn player_movement(
    time: Res<Time>,
    input: Res<InputState>,
    mut state: ResMut<PlayerState>,
    mut cam_q: Query<&mut Transform, With<PanoramaCamera>>,
) {
    let Ok(mut cam) = cam_q.single_mut() else {
        return;
    };
    let dt = time.delta_secs();
    state.prev_pos = state.pos;

    // Get camera yaw for movement direction
    let (yaw, _, _) = cam.rotation.to_euler(EulerRot::YXZ);

    // Forward direction in XZ plane (yaw=0 means looking at -Z)
    let forward = Vec2::new(-yaw.sin(), -yaw.cos());
    // Right direction (perpendicular to forward)
    let right = Vec2::new(-forward.y, forward.x);

    // Input mapping:
    // W (y=-1) -> move forward
    // S (y=+1) -> move backward
    // A (x=-1) -> move left
    // D (x=+1) -> move right
    let move_dir = forward * (-input.movement.y) + right * input.movement.x;

    if move_dir.length() > 0.01 {
        let speed = 3.0;
        let new_pos = state.pos + move_dir * speed * dt;

        // Character barrier
        let char_pos = Vec2::new(0.0, -10.0);
        let to_char = new_pos - char_pos;
        let char_dist = to_char.length();
        let min_char_dist = 4.0;

        let bounded_pos = if char_dist < min_char_dist && char_dist > 0.01 {
            char_pos + to_char.normalize() * min_char_dist
        } else {
            new_pos
        };

        state.pos.x = bounded_pos.x.clamp(-8.0, 8.0);
        state.pos.y = bounded_pos.y.clamp(-14.0, 3.0);
    }

    cam.translation = room_center(state.room) + Vec3::new(state.pos.x, state.height, state.pos.y);
}
