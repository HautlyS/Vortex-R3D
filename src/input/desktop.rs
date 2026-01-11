//! Desktop input - WASD + mouse look + gamepad

use bevy::input::gamepad::GamepadButton;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use super::{InputEvent, InputState, UiWantsPointer};
use crate::platform::on_desktop;
use crate::GameState;

pub struct DesktopInputPlugin;

impl Plugin for DesktopInputPlugin {
    fn build(&self, app: &mut App) {
        use bevy::ecs::schedule::IntoScheduleConfigs;
        app.init_resource::<CursorGrabDelay>().add_systems(
            Update,
            (update_grab_delay, handle_cursor_grab, read_desktop_input)
                .chain_ignore_deferred()
                .run_if(on_desktop)
                .run_if(in_state(GameState::Viewing)),
        );
    }
}

/// Delay cursor grab after state transitions to let UI settle
#[derive(Resource, Default)]
pub struct CursorGrabDelay {
    frames_since_viewing: u32,
}

const GRAB_DELAY_FRAMES: u32 = 3;

fn update_grab_delay(mut delay: ResMut<CursorGrabDelay>) {
    delay.frames_since_viewing = delay.frames_since_viewing.saturating_add(1);
}

fn handle_cursor_grab(
    mut cursor_q: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<InputState>,
    ui_wants: Res<UiWantsPointer>,
    delay: Res<CursorGrabDelay>,
) {
    let Ok(mut cursor) = cursor_q.single_mut() else {
        return;
    };

    // Don't grab cursor if:
    // 1. UI wants pointer input
    // 2. Not enough frames since state transition
    let can_grab = !ui_wants.0 && delay.frames_since_viewing >= GRAB_DELAY_FRAMES;

    if mouse.just_pressed(MouseButton::Left) && can_grab {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
        state.cursor_locked = true;
    }

    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
        state.cursor_locked = false;
    }
}

fn read_desktop_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    _gamepad_buttons: Option<Res<ButtonInput<GamepadButton>>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut state: ResMut<InputState>,
    mut events: MessageWriter<InputEvent>,
) {
    // Movement
    let mut movement = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        movement.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }

    state.movement = if movement != Vec2::ZERO {
        movement.normalize()
    } else {
        Vec2::ZERO
    };
    if state.movement != Vec2::ZERO {
        events.write(InputEvent::Move(state.movement));
    }

    // Look - only when cursor is locked
    if state.cursor_locked && mouse_motion.delta != Vec2::ZERO {
        state.look_delta = mouse_motion.delta;
        events.write(InputEvent::Look(mouse_motion.delta));
    } else {
        state.look_delta = Vec2::ZERO;
    }

    // Actions
    if mouse.just_pressed(MouseButton::Left) && state.cursor_locked {
        events.write(InputEvent::Interact);
    }
    if keys.pressed(KeyCode::ShiftLeft) && keys.just_pressed(KeyCode::Space) {
        events.write(InputEvent::NextRoom);
    }
    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::NumpadAdd) {
        events.write(InputEvent::AdjustFov(-2.0));
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::NumpadSubtract) {
        events.write(InputEvent::AdjustFov(2.0));
    }
    if keys.just_pressed(KeyCode::Escape) {
        events.write(InputEvent::ToggleMenu);
    }
}
