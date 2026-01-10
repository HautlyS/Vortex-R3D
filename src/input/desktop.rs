//! Desktop input - WASD + mouse look + gamepad

use bevy::input::gamepad::GamepadButton;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use super::{InputEvent, InputState, UiWantsPointer};
use crate::platform::on_desktop;

pub struct DesktopInputPlugin;

impl Plugin for DesktopInputPlugin {
    fn build(&self, app: &mut App) {
        use bevy::ecs::schedule::IntoScheduleConfigs;
        info!("🎮 [DEBUG] DesktopInputPlugin registered");
        app.add_systems(Startup, debug_platform_state).add_systems(
            Update,
            (handle_cursor_grab, read_desktop_input)
                .chain_ignore_deferred()
                .run_if(on_desktop),
        );
    }
}

fn debug_platform_state(platform: Res<crate::platform::Platform>) {
    info!("🎮 [DEBUG] Platform state at startup: {:?}", *platform);
    info!(
        "🎮 [DEBUG] on_desktop would return: {}",
        *platform == crate::platform::Platform::Desktop
    );
}

fn handle_cursor_grab(
    mut cursor_q: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<InputState>,
    ui_wants: Res<UiWantsPointer>,
) {
    let Ok(mut cursor) = cursor_q.get_single_mut() else {
        return;
    };

    // Don't grab cursor if UI wants pointer input
    if mouse.just_pressed(MouseButton::Left) && !ui_wants.0 {
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

    // Look
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

    // Book Reader: Now handled by bevy_egui_kbgp in book_reader module
    // B key and gamepad West button are bound via KbgpSettings
}
