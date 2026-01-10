use bevy::prelude::*;
use bevy::gltf::{Gltf, GltfExtras, GltfMaterialExtras, GltfMeshExtras, GltfSceneExtras};
use bevy::animation::AnimationPlayer;
use bevy_asset_loader::prelude::*;
use crate::GameState;

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
            );
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
