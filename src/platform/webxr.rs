//! WebXR platform plugin - browser VR support via web-sys bindings

use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use super::Platform;
use crate::camera::{CameraState, GameCamera};
use crate::input::{InputEvent, InputState};

/// WebXR session state
#[derive(Resource, Default)]
pub struct WebXrState {
    pub available: bool,
    pub session_active: bool,
    pub session_requested: bool,
}

/// Shared pose data from JavaScript XR frame callback
#[derive(Default, Clone)]
pub struct XrPoseData {
    pub position: [f32; 3],
    pub orientation: [f32; 4],
    pub valid: bool,
}

/// Thread-safe pose storage updated from JS
#[derive(Resource, Clone)]
pub struct WebXrPose(pub Arc<Mutex<XrPoseData>>);

impl Default for WebXrPose {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(XrPoseData::default())))
    }
}

pub struct WebXrPlatformPlugin;

impl Plugin for WebXrPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Platform::WebXr)
            .init_resource::<WebXrState>()
            .init_resource::<WebXrPose>()
            .add_systems(Startup, setup_webxr)
            .add_systems(
                Update,
                (
                    check_xr_availability,
                    sync_xr_pose_to_camera,
                    handle_xr_input,
                )
                    .chain(),
            );

        info!("🌐 WebXR platform initialized");
    }
}

fn setup_webxr(mut commands: Commands) {
    // Spawn VR enter button UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.6, 1.0, 0.9)),
            Button,
            VrEnterButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("🥽 Enter VR"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

#[derive(Component)]
struct VrEnterButton;

fn check_xr_availability(
    mut state: ResMut<WebXrState>,
    mut button_q: Query<&mut Visibility, With<VrEnterButton>>,
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<VrEnterButton>)>,
    pose: Res<WebXrPose>,
) {
    // Check availability once
    if !state.available {
        if let Some(window) = web_sys::window() {
            // navigator.xr() returns XrSystem directly in newer web-sys
            let _xr = window.navigator().xr();
            state.available = true;
            info!("✅ WebXR available");
        }
    }

    // Hide button if not available or session active
    for mut vis in button_q.iter_mut() {
        *vis = if state.available && !state.session_active {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    // Handle button click to enter VR
    for interaction in interaction_q.iter() {
        if *interaction == Interaction::Pressed && !state.session_requested {
            state.session_requested = true;
            let pose_clone = pose.0.clone();
            spawn_local(request_xr_session(pose_clone));
        }
    }
}

async fn request_xr_session(pose_storage: Arc<Mutex<XrPoseData>>) {
    use web_sys::{XrReferenceSpaceType, XrSessionMode};

    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let navigator = window.navigator();
    let xr = navigator.xr();

    // Check session support
    let supported =
        wasm_bindgen_futures::JsFuture::from(xr.is_session_supported(XrSessionMode::ImmersiveVr))
            .await;

    if !matches!(supported, Ok(ref v) if v.is_truthy()) {
        web_sys::console::warn_1(&"Immersive VR not supported".into());
        return;
    }

    // Request session
    let session_promise = xr.request_session(XrSessionMode::ImmersiveVr);
    let session = match wasm_bindgen_futures::JsFuture::from(session_promise).await {
        Ok(s) => web_sys::XrSession::from(s),
        Err(e) => {
            web_sys::console::error_1(&format!("Session request failed: {:?}", e).into());
            return;
        }
    };

    web_sys::console::log_1(&"🥽 XR Session started!".into());

    // Get reference space
    let ref_space_promise = session.request_reference_space(XrReferenceSpaceType::Local);
    let ref_space = match wasm_bindgen_futures::JsFuture::from(ref_space_promise).await {
        Ok(rs) => web_sys::XrReferenceSpace::from(rs),
        Err(_) => {
            web_sys::console::warn_1(&"Failed to get reference space".into());
            return;
        }
    };

    // Start XR render loop
    start_xr_frame_loop(session, ref_space, pose_storage);
}

fn start_xr_frame_loop(
    session: web_sys::XrSession,
    ref_space: web_sys::XrReferenceSpace,
    pose_storage: Arc<Mutex<XrPoseData>>,
) {
    use std::cell::RefCell;
    use std::rc::Rc;

    let session = Rc::new(session);
    let ref_space = Rc::new(ref_space);
    let pose_storage = Rc::new(pose_storage);

    // Recursive frame callback
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64, web_sys::XrFrame)>>>> =
        Rc::new(RefCell::new(None));
    let g = f.clone();

    let session_clone = session.clone();
    let ref_space_clone = ref_space.clone();
    let pose_clone = pose_storage.clone();

    *g.borrow_mut() = Some(Closure::new(move |_time: f64, frame: web_sys::XrFrame| {
        // Get viewer pose
        if let Some(pose) = frame.get_viewer_pose(&ref_space_clone) {
            let transform = pose.transform();
            let pos = transform.position();
            let ori = transform.orientation();

            // Update shared pose data
            if let Ok(mut data) = pose_clone.lock() {
                data.position = [pos.x() as f32, pos.y() as f32, pos.z() as f32];
                data.orientation = [
                    ori.x() as f32,
                    ori.y() as f32,
                    ori.z() as f32,
                    ori.w() as f32,
                ];
                data.valid = true;
            }
        }

        // Request next frame
        let _ = session_clone
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }));

    // Start the loop
    let _ = session.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
}

fn sync_xr_pose_to_camera(
    pose: Res<WebXrPose>,
    mut state: ResMut<WebXrState>,
    mut camera_state: ResMut<CameraState>,
    mut camera_q: Query<&mut Transform, With<GameCamera>>,
) {
    let data = match pose.0.lock() {
        Ok(d) => d.clone(),
        Err(_) => return,
    };

    if !data.valid {
        return;
    }

    state.session_active = true;

    // Apply XR pose to camera
    let Ok(mut transform) = camera_q.single_mut() else {
        return;
    };

    let quat = Quat::from_xyzw(
        data.orientation[0],
        data.orientation[1],
        data.orientation[2],
        data.orientation[3],
    );

    transform.rotation = quat;

    // Sync to camera state for other systems
    let (yaw, pitch, _) = quat.to_euler(EulerRot::YXZ);
    camera_state.yaw = yaw;
    camera_state.pitch = pitch;

    // Disable motion effects in VR
    camera_state.motion_blur = 0.0;
    camera_state.walk_cycle = 0.0;
}

fn handle_xr_input(
    state: Res<WebXrState>,
    mut input_state: ResMut<InputState>,
    _events: MessageWriter<InputEvent>,
) {
    if !state.session_active {
        return;
    }

    // In WebXR, cursor is always "locked" (no mouse)
    input_state.cursor_locked = true;

    // Controller input would be handled here via XrInputSource
    // For now, head tracking is the primary input
}

/// Marker for entities that follow WebXR head position
#[derive(Component)]
pub struct FollowWebXrHead;
