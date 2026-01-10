//! File picker - cross-platform file upload (Desktop rfd, Web FileReader)

use bevy::asset::RenderAssetUsages;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use std::sync::Mutex;

use super::{UploadModel, UploadSphere, UploadState};

pub static PENDING_IMAGE: Mutex<Option<Vec<u8>>> = Mutex::new(None);
pub static PENDING_GLB: Mutex<Option<Vec<u8>>> = Mutex::new(None);

#[derive(Clone, Copy)]
pub enum FileKind {
    Image,
    Model,
}

pub fn pick_file(kind: FileKind) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            let filter = match kind {
                FileKind::Image => ("Images", &["jpg", "jpeg", "png"][..]),
                FileKind::Model => ("GLB", &["glb"][..]),
            };
            if let Some(path) = rfd::FileDialog::new()
                .add_filter(filter.0, filter.1)
                .pick_file()
            {
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

        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let document = match window.document() {
            Some(d) => d,
            None => return,
        };

        let input: web_sys::HtmlInputElement = match document.create_element("input") {
            Ok(el) => match el.dyn_into() {
                Ok(i) => i,
                Err(_) => return,
            },
            Err(_) => return,
        };

        input.set_type("file");
        input.set_accept(match kind {
            FileKind::Image => "image/jpeg,image/png",
            FileKind::Model => ".glb",
        });

        let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
            let input: web_sys::HtmlInputElement = match e.target() {
                Some(t) => match t.dyn_into() {
                    Ok(i) => i,
                    Err(_) => return,
                },
                None => return,
            };
            let files = match input.files() {
                Some(f) => f,
                None => return,
            };
            let file = match files.get(0) {
                Some(f) => f,
                None => return,
            };

            let reader = match web_sys::FileReader::new() {
                Ok(r) => r,
                Err(_) => return,
            };
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

pub fn poll_file_data(
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
                bevy::render::render_resource::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
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

pub fn load_pending_model(
    mut commands: Commands,
    gltfs: Res<Assets<Gltf>>,
    mut state: ResMut<UploadState>,
    existing: Query<Entity, With<UploadModel>>,
) {
    let handle = match &state.model_handle {
        Some(h) => h,
        None => return,
    };
    let gltf = match gltfs.get(handle) {
        Some(g) => g,
        None => return,
    };

    for e in existing.iter() {
        commands.entity(e).despawn();
    }

    let scene = gltf
        .default_scene
        .clone()
        .unwrap_or_else(|| gltf.scenes[0].clone());
    commands.spawn((
        SceneRoot(scene),
        Transform::from_xyz(0.0, -1.0, -3.0),
        UploadModel,
    ));

    state.model_handle = None;
    info!("✅ Model loaded");
}
