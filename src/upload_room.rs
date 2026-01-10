//! Upload Room - Cross-platform file upload for panoramas and 3D models
//! Works on Desktop (rfd), Web (FileReader API), and Mobile (touch UI)

use bevy::prelude::*;
use bevy::gltf::Gltf;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui_kbgp::prelude::*;
use std::f32::consts::PI;
use std::sync::Mutex;
use crate::GameState;

pub struct UploadRoomPlugin;

impl Plugin for UploadRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin::default(), KbgpPlugin))
            .init_resource::<UploadState>()
            .add_systems(Startup, go_to_viewing)
            .add_systems(OnEnter(GameState::Viewing), setup_upload_room)
            .add_systems(Update, (
                handle_keyboard_shortcuts,
                poll_file_data,
                load_pending_model,
                rotate_ambient_light,
                upload_hud,
            ).run_if(in_state(GameState::Viewing)));
    }
}

fn go_to_viewing(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Viewing);
}

static PENDING_IMAGE: Mutex<Option<Vec<u8>>> = Mutex::new(None);
static PENDING_GLB: Mutex<Option<Vec<u8>>> = Mutex::new(None);

#[derive(Resource, Default)]
struct UploadState {
    model_handle: Option<Handle<Gltf>>,
}

#[derive(Component)]
pub struct UploadSphere;

#[derive(Component)]
pub struct UploadModel;

#[derive(Component)]
struct AmbientOrb;

fn setup_upload_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sky sphere with gradient
    let sphere = meshes.add(create_sphere(50.0, 64, 32));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.05, 0.08, 0.15),
        unlit: true,
        cull_mode: None,
        ..default()
    });
    commands.spawn((Mesh3d(sphere), MeshMaterial3d(mat), UploadSphere));

    // Floor grid
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(20.0)));
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.1, 0.15, 0.25, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    commands.spawn((Mesh3d(floor), MeshMaterial3d(floor_mat), Transform::from_xyz(0.0, -2.0, 0.0)));

    // Ambient orb (glowing center)
    let orb = meshes.add(Sphere::new(0.3));
    let orb_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.6, 1.0),
        emissive: LinearRgba::new(0.5, 0.8, 1.5, 1.0),
        ..default()
    });
    commands.spawn((Mesh3d(orb), MeshMaterial3d(orb_mat), Transform::from_xyz(0.0, 0.5, -3.0), AmbientOrb));

    // Lights
    commands.spawn((PointLight { intensity: 300000.0, color: Color::srgb(0.6, 0.8, 1.0), shadows_enabled: true, ..default() }, Transform::from_xyz(0.0, 4.0, 0.0)));
    commands.spawn((PointLight { intensity: 100000.0, color: Color::srgb(0.8, 0.5, 1.0), ..default() }, Transform::from_xyz(-3.0, 2.0, -2.0)));
    commands.spawn((PointLight { intensity: 100000.0, color: Color::srgb(0.5, 1.0, 0.8), ..default() }, Transform::from_xyz(3.0, 2.0, -2.0)));

    // Camera
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 0.0), bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted, crate::camera::GameCamera));

    info!("📤 Upload Room ready");
}

fn rotate_ambient_light(time: Res<Time>, mut orbs: Query<&mut Transform, With<AmbientOrb>>) {
    for mut t in &mut orbs {
        t.translation.y = 0.5 + (time.elapsed_secs() * 0.5).sin() * 0.2;
        t.rotate_y(time.delta_secs() * 0.3);
    }
}

fn upload_hud(mut ctx: EguiContexts) {
    let Ok(egui_ctx) = ctx.ctx_mut() else { return };
    
    egui::Window::new("Upload Room")
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -20.0])
        .resizable(false)
        .collapsible(false)
        .show(egui_ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🌐 Panorama Viewer");
                ui.add_space(8.0);
                
                if ui.button("📷 Upload Panorama (P)").kbgp_navigation().clicked() {
                    pick_file(FileKind::Image);
                }
                if ui.button("🎭 Upload 3D Model (M)").kbgp_navigation().clicked() {
                    pick_file(FileKind::Model);
                }
                
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                
                ui.label(egui::RichText::new("⌨️ Controls").strong());
                egui::Grid::new("controls").show(ui, |ui| {
                    ui.label("P"); ui.label("Upload panorama"); ui.end_row();
                    ui.label("M"); ui.label("Upload 3D model"); ui.end_row();
                    ui.label("Mouse"); ui.label("Look around"); ui.end_row();
                    ui.label("WASD"); ui.label("Look around"); ui.end_row();
                    ui.label("+/-"); ui.label("Adjust FOV"); ui.end_row();
                    ui.label("Ctrl+R"); ui.label("Spin effect"); ui.end_row();
                    ui.label("Esc"); ui.label("Release mouse"); ui.end_row();
                });
            });
        });
}

fn handle_keyboard_shortcuts(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyP) { pick_file(FileKind::Image); }
    if keys.just_pressed(KeyCode::KeyM) { pick_file(FileKind::Model); }
}

#[derive(Clone, Copy)]
enum FileKind { Image, Model }

fn pick_file(kind: FileKind) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            let filter = match kind {
                FileKind::Image => ("Images", &["jpg", "jpeg", "png"][..]),
                FileKind::Model => ("GLB", &["glb"][..]),
            };
            if let Some(path) = rfd::FileDialog::new().add_filter(filter.0, filter.1).pick_file() {
                if let Ok(data) = std::fs::read(&path) {
                    match kind {
                        FileKind::Image => *PENDING_IMAGE.lock().unwrap() = Some(data),
                        FileKind::Model => *PENDING_GLB.lock().unwrap() = Some(data),
                    }
                    info!("Loaded: {:?}", path);
                }
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        
        let window = match web_sys::window() { Some(w) => w, None => return };
        let document = match window.document() { Some(d) => d, None => return };
        
        let input: web_sys::HtmlInputElement = match document.create_element("input") {
            Ok(el) => match el.dyn_into() { Ok(i) => i, Err(_) => return },
            Err(_) => return,
        };
        
        input.set_type("file");
        input.set_accept(match kind { FileKind::Image => "image/jpeg,image/png", FileKind::Model => ".glb" });

        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            let input: web_sys::HtmlInputElement = match e.target() {
                Some(t) => match t.dyn_into() { Ok(i) => i, Err(_) => return },
                None => return,
            };
            let files = match input.files() { Some(f) => f, None => return };
            let file = match files.get(0) { Some(f) => f, None => return };
            
            let reader = match web_sys::FileReader::new() { Ok(r) => r, Err(_) => return };
            let reader2 = reader.clone();
            let k = kind;
            
            let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Ok(result) = reader2.result() {
                    let arr = js_sys::Uint8Array::new(&result);
                    let data = arr.to_vec();
                    match k {
                        FileKind::Image => *PENDING_IMAGE.lock().unwrap() = Some(data),
                        FileKind::Model => *PENDING_GLB.lock().unwrap() = Some(data),
                    }
                }
            }) as Box<dyn FnMut(_)>);
            
            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
            let _ = reader.read_as_array_buffer(&file);
        }) as Box<dyn FnMut(_)>);

        input.set_onchange(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        input.click();
    }
}

fn poll_file_data(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    sphere_q: Query<&MeshMaterial3d<StandardMaterial>, With<UploadSphere>>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<UploadState>,
) {
    if let Some(data) = PENDING_IMAGE.lock().unwrap().take() {
        if let Ok(img) = image::load_from_memory(&data) {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            let tex = Image::new(
                bevy::render::render_resource::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                bevy::render::render_resource::TextureDimension::D2,
                rgba.into_raw(),
                bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::RENDER_WORLD,
            );
            let handle = images.add(tex);
            
            if let Ok(mat_h) = sphere_q.single() {
                if let Some(mat) = materials.get_mut(&mat_h.0) {
                    mat.base_color_texture = Some(handle);
                    mat.base_color = Color::WHITE;
                    info!("✅ Panorama applied");
                }
            }
        }
    }

    if let Some(data) = PENDING_GLB.lock().unwrap().take() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = std::env::temp_dir().join("_upload.glb");
            if std::fs::write(&path, data).is_ok() {
                state.model_handle = Some(asset_server.load(path.display().to_string()));
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let arr = js_sys::Uint8Array::from(&data[..]);
            let parts = js_sys::Array::new();
            parts.push(&arr.buffer());
            if let Ok(blob) = web_sys::Blob::new_with_u8_array_sequence(&parts) {
                if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                    state.model_handle = Some(asset_server.load(url));
                }
            }
        }
    }
}

fn load_pending_model(
    mut commands: Commands,
    gltfs: Res<Assets<Gltf>>,
    mut state: ResMut<UploadState>,
    existing: Query<Entity, With<UploadModel>>,
) {
    let handle = match &state.model_handle { Some(h) => h, None => return };
    let gltf = match gltfs.get(handle) { Some(g) => g, None => return };
    
    for e in existing.iter() { commands.entity(e).despawn(); }
    
    let scene = gltf.default_scene.clone().unwrap_or_else(|| gltf.scenes[0].clone());
    commands.spawn((SceneRoot(scene), Transform::from_xyz(0.0, -1.0, -3.0), UploadModel));
    
    state.model_handle = None;
    info!("✅ Model loaded");
}

fn create_sphere(radius: f32, sectors: u32, stacks: u32) -> Mesh {
    let mut pos = Vec::new();
    let mut norm = Vec::new();
    let mut uv = Vec::new();
    let mut idx = Vec::new();

    for i in 0..=stacks {
        let v = i as f32 / stacks as f32;
        let phi = PI * v;
        for j in 0..=sectors {
            let u = j as f32 / sectors as f32;
            let theta = 2.0 * PI * u;
            let (x, y, z) = (radius * phi.sin() * theta.cos(), radius * phi.cos(), radius * phi.sin() * theta.sin());
            pos.push([x, y, z]);
            norm.push([x / radius, y / radius, z / radius]);
            uv.push([1.0 - u, v]);
        }
    }

    for i in 0..stacks {
        for j in 0..sectors {
            let a = i * (sectors + 1) + j;
            let b = a + sectors + 1;
            idx.extend([a, a + 1, b, b, a + 1, b + 1]);
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList, default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, norm)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
        .with_inserted_indices(Indices::U32(idx))
}
