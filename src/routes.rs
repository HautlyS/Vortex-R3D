//! URL Route Manager
//! Handles paths like /upload to load different experiences

/// App mode based on URL path or CLI args
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum AppMode {
    #[default]
    FullExperience, // / or default - Full experience
    UploadRoom,     // /upload or --upload - User upload room
}

/// Get the current app mode
pub fn get_app_mode() -> AppMode {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            // Check query param (from 404 redirect: ?p=/upload)
            if let Ok(search) = window.location().search() {
                if search.contains("p=%2Fupload") || search.contains("p=/upload") {
                    // Clean up URL after reading
                    let _ = window.history().and_then(|h| {
                        h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("./"))
                    });
                    return AppMode::UploadRoom;
                }
            }

            // Check sessionStorage (set by 404.html or upload/index.html)
            if let Ok(Some(storage)) = window.session_storage() {
                if let Ok(Some(route)) = storage.get_item("route") {
                    let _ = storage.remove_item("route"); // Clean up
                    if route.contains("/upload") {
                        return AppMode::UploadRoom;
                    }
                }
            }

            // Check pathname directly (for direct navigation)
            if let Ok(pathname) = window.location().pathname() {
                if pathname.trim_end_matches('/').ends_with("/upload") {
                    return AppMode::UploadRoom;
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if std::env::args().any(|a| a == "--upload") {
            return AppMode::UploadRoom;
        }
    }

    AppMode::FullExperience
}
