//! JS Bridge - WASM ↔ HTML communication for loading state

use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Call JS to hide the HTML loading overlay
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(inline_js = "
export function hide_loading_overlay() {
    const loading = document.getElementById('loading');
    if (loading) {
        loading.style.transition = 'opacity 0.3s';
        loading.style.opacity = '0';
        setTimeout(() => loading.style.display = 'none', 300);
    }
    const controls = document.getElementById('controls');
    if (controls) controls.style.display = 'block';
}

export function update_loading_progress(msg) {
    const progress = document.getElementById('progress');
    if (progress) progress.textContent = msg;
}
")]
extern "C" {
    pub fn hide_loading_overlay();
    pub fn update_loading_progress(msg: &str);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn hide_loading_overlay() {}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_loading_progress(_msg: &str) {}

/// Resource to track if we've signaled ready to JS
#[derive(Resource, Default)]
pub struct JsBridgeState {
    pub loading_hidden: bool,
}

pub struct JsBridgePlugin;

impl Plugin for JsBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JsBridgeState>();
    }
}
