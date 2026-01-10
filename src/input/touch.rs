//! Touch input for mobile web - drag to look, pinch to zoom

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{InputEvent, InputState};

pub struct TouchInputPlugin;

impl Plugin for TouchInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TouchState>()
            .add_systems(Update, handle_touch_input);
    }
}

#[derive(Resource, Default)]
struct TouchState {
    last_pos: Option<Vec2>,
    last_pinch_dist: Option<f32>,
}

fn handle_touch_input(
    touches: Res<Touches>,
    mut state: ResMut<TouchState>,
    mut input_state: ResMut<InputState>,
    mut events: MessageWriter<InputEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else { return };
    let scale = window.width().min(window.height()) / 800.0;

    let active: Vec<_> = touches.iter().collect();

    match active.len() {
        0 => {
            state.last_pos = None;
            state.last_pinch_dist = None;
            input_state.look_delta = Vec2::ZERO;
        }
        1 => {
            // Single touch - look around
            state.last_pinch_dist = None;
            let touch = active[0];
            let pos = touch.position();

            if let Some(last) = state.last_pos {
                let delta = pos - last;
                if delta.length() > 0.5 {
                    let sensitivity = 0.4 / scale.max(0.5);
                    let look = delta * sensitivity;
                    input_state.look_delta = look;
                    events.write(InputEvent::Look(look));
                }
            }
            state.last_pos = Some(pos);
        }
        _ => {
            // Multi-touch - pinch zoom
            state.last_pos = None;
            let p1 = active[0].position();
            let p2 = active[1].position();
            let dist = p1.distance(p2);

            if let Some(last_dist) = state.last_pinch_dist {
                let delta = (last_dist - dist) * 0.1;
                if delta.abs() > 0.5 {
                    events.write(InputEvent::AdjustFov(delta));
                }
            }
            state.last_pinch_dist = Some(dist);
        }
    }

    // Double tap to interact
    for touch in touches.iter_just_pressed() {
        if touches.iter().count() == 1 {
            events.write(InputEvent::Interact);
        }
        let _ = touch; // suppress warning
    }
}
