use crate::world::Skybox;
use crate::GameState;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

pub struct VortexTransitionPlugin;

impl Plugin for VortexTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<VortexMaterial>::default())
            .init_resource::<TransitionState>()
            .add_systems(
                Update,
                (handle_transition_input, animate_transition)
                    .chain()
                    .run_if(in_state(GameState::Viewing)),
            );
    }
}

#[derive(Resource)]
pub struct TransitionState {
    current_index: usize,
    paths: Vec<&'static str>,
    transitioning: bool,
    progress: f32,
    current_texture: Option<Handle<Image>>,
    next_texture: Option<Handle<Image>>,
    loading: bool,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            current_index: 0,
            paths: vec![
                "panoramas/demo.jpg",
                "panoramas/demo2.jpg",
                "panoramas/demo3.jpg",
            ],
            transitioning: false,
            progress: 0.0,
            current_texture: None,
            next_texture: None,
            loading: false,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct VortexMaterial {
    #[texture(0)]
    #[sampler(2)]
    pub texture_a: Handle<Image>,
    #[texture(1)]
    pub texture_b: Handle<Image>,
    #[uniform(3)]
    pub progress: f32,
}

impl Material for VortexMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/vortex_transition.wgsl".into()
    }
}

#[derive(Component)]
pub struct VortexSphere;

fn handle_transition_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<TransitionState>,
    asset_server: Res<AssetServer>,
    sphere: Query<&MeshMaterial3d<StandardMaterial>, With<Skybox>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    if shift && keyboard.just_pressed(KeyCode::Space) && !state.transitioning && !state.loading {
        // Capture current texture from skybox
        if state.current_texture.is_none() {
            if let Ok(mat_handle) = sphere.single() {
                if let Some(mat) = materials.get(&mat_handle.0) {
                    state.current_texture = mat.base_color_texture.clone();
                }
            }
        }

        let next = (state.current_index + 1) % state.paths.len();
        let path = state.paths[next];

        state.next_texture = Some(asset_server.load(path));
        state.loading = true;
        state.current_index = next;

        info!("🌀 Loading panorama {}: {}", next, path);
    }
}

#[allow(clippy::too_many_arguments)]
fn animate_transition(
    time: Res<Time>,
    mut state: ResMut<TransitionState>,
    asset_server: Res<AssetServer>,
    sphere_std: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        (With<Skybox>, Without<VortexSphere>),
    >,
    sphere_vortex: Query<(Entity, &MeshMaterial3d<VortexMaterial>), With<VortexSphere>>,
    mut commands: Commands,
    mut vortex_materials: ResMut<Assets<VortexMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    player: Option<Res<crate::player::PlayerState>>,
) {
    let current_room = player.map(|p| p.room).unwrap_or(0);

    // Handle loading state
    if state.loading {
        let next_handle = state.next_texture.clone();
        if let Some(handle) = next_handle {
            match asset_server.get_load_state(&handle) {
                Some(bevy::asset::LoadState::Loaded) => {
                    state.loading = false;
                    state.transitioning = true;
                    state.progress = 0.0;

                    // Find current room's skybox and swap to vortex material
                    for (entity, std_mat_handle) in sphere_std.iter() {
                        let current = if let Some(mat) = std_materials.get(&std_mat_handle.0) {
                            mat.base_color_texture.clone().unwrap_or_else(|| handle.clone())
                        } else {
                            handle.clone()
                        };

                        state.current_texture = Some(current.clone());

                        let vortex = vortex_materials.add(VortexMaterial {
                            texture_a: current,
                            texture_b: handle.clone(),
                            progress: 0.0,
                        });

                        commands
                            .entity(entity)
                            .remove::<MeshMaterial3d<StandardMaterial>>()
                            .insert((MeshMaterial3d(vortex), VortexSphere));
                        break; // Only transition first matching skybox
                    }

                    info!("✨ Starting vortex transition in room {}", current_room);
                }
                Some(bevy::asset::LoadState::Failed(_)) => {
                    warn!("⚠️ Failed to load panorama, looping to start");
                    state.loading = false;
                    state.next_texture = None;
                    state.current_index = 0;
                }
                _ => {}
            }
        }
        return;
    }

    if !state.transitioning {
        return;
    }

    state.progress += time.delta_secs() * 0.9;

    // Update vortex material progress
    for (_, vortex_handle) in sphere_vortex.iter() {
        if let Some(mat) = vortex_materials.get_mut(&vortex_handle.0) {
            mat.progress = state.progress.min(1.0);
        }
    }

    // Complete transition
    if state.progress >= 1.0 {
        for (entity, _) in sphere_vortex.iter() {
            let new_mat = std_materials.add(StandardMaterial {
                base_color_texture: state.next_texture.clone(),
                unlit: true,
                double_sided: true,
                cull_mode: None,
                ..default()
            });

            commands
                .entity(entity)
                .remove::<MeshMaterial3d<VortexMaterial>>()
                .remove::<VortexSphere>()
                .insert(MeshMaterial3d(new_mat));
        }

        state.current_texture = state.next_texture.take();
        state.transitioning = false;
        info!("✅ Vortex transition complete");
    }
}
