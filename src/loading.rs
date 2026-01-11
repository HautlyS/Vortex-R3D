use crate::js_bridge::{hide_loading_overlay, update_loading_progress, JsBridgeState};
use crate::GameState;
use bevy::animation::AnimationPlayer;
use bevy::gltf::{Gltf, GltfExtras, GltfMaterialExtras, GltfMeshExtras, GltfSceneExtras};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GltfExtras>()
            .register_type::<GltfMaterialExtras>()
            .register_type::<GltfMeshExtras>()
            .register_type::<GltfSceneExtras>()
            .register_type::<Name>()
            .register_type::<AnimationPlayer>()
            .register_type::<AnimationTransitions>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Viewing)
                    .load_collection::<PanoramaAssets>()
                    .load_collection::<ModelAssets>(),
            )
            .add_systems(OnEnter(GameState::Loading), on_enter_loading)
            .add_systems(OnEnter(GameState::Viewing), on_enter_viewing);
    }
}

fn on_enter_loading() {
    update_loading_progress("Loading assets...");
}

fn on_enter_viewing(mut bridge: ResMut<JsBridgeState>) {
    if !bridge.loading_hidden {
        hide_loading_overlay();
        bridge.loading_hidden = true;
        info!("🎬 Bevy ready - HTML loading hidden");
    }
}

#[derive(AssetCollection, Resource)]
pub struct PanoramaAssets {
    #[asset(path = "panoramas/demo.jpg")]
    pub demo_panorama: Handle<Image>,

    #[asset(path = "panoramas/demo2.jpg")]
    pub demo2_panorama: Handle<Image>,

    #[asset(path = "panoramas/demo3.jpg")]
    pub demo3_panorama: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ModelAssets {
    #[asset(path = "models/modelo1.glb")]
    pub modelo1: Handle<Gltf>,

    #[asset(path = "models/modelo2.glb")]
    pub modelo2: Handle<Gltf>,

    #[asset(path = "models/modelo3.glb")]
    pub modelo3: Handle<Gltf>,
}
