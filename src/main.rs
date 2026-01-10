// Disable console on Windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

fn main() {
    #[cfg(all(feature = "desktop", not(feature = "vr")))]
    run_desktop();

    #[cfg(feature = "vr")]
    run_vr();

    #[cfg(all(feature = "webxr", target_arch = "wasm32"))]
    run_webxr();
}

#[cfg(all(feature = "desktop", not(feature = "vr")))]
fn run_desktop() {
    use techno_sutra::GamePlugin;

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)));

    #[allow(unused_mut)]
    let mut default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "🕉️ Techno Sutra: Virtual Wisdom".into(),
                resolution: (1280, 720).into(),
                canvas: Some("#bevy".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true, // Better touch handling
                ..default()
            }),
            ..default()
        })
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        })
        .set(ImagePlugin::default_nearest()); // Faster texture sampling

    // Mobile/WASM performance settings
    #[cfg(target_arch = "wasm32")]
    let default_plugins = {
        use bevy::render::{settings::{WgpuSettings, PowerPreference}, RenderPlugin};
        default_plugins.set(RenderPlugin {
            render_creation: WgpuSettings {
                power_preference: PowerPreference::LowPower,
                ..default()
            }.into(),
            ..default()
        })
    };

    app.add_plugins(default_plugins)
        .add_plugins(GamePlugin)
        .run();
}

#[cfg(feature = "vr")]
fn run_vr() {
    use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
    use bevy_mod_openxr::{add_xr_plugins, resources::OxrSessionConfig};
    use openxr::EnvironmentBlendMode;
    use techno_sutra::VrGamePlugin;

    App::new()
        .add_plugins(add_xr_plugins(
            DefaultPlugins
                .build()
                .disable::<PipelinedRenderingPlugin>()
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        ))
        .insert_resource(OxrSessionConfig {
            blend_mode_preference: vec![
                EnvironmentBlendMode::OPAQUE,
                EnvironmentBlendMode::ALPHA_BLEND,
            ],
            ..default()
        })
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins(VrGamePlugin)
        .run();
}

#[cfg(all(feature = "webxr", target_arch = "wasm32"))]
fn run_webxr() {
    use techno_sutra::WebXrGamePlugin;

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🕉️ Techno Sutra: Virtual Wisdom".into(),
                        canvas: Some("#bevy".into()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(WebXrGamePlugin)
        .run();
}
