//! Upload room egui HUD - cross-platform (Desktop, WASM, VR, Android)

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::file_picker::{pick_file, FileKind};
use super::{UploadModel, UploadSphere, UploadState};

#[allow(clippy::too_many_arguments)]
pub fn upload_hud(
    mut ctx: EguiContexts,
    mut state: ResMut<UploadState>,
    mut commands: Commands,
    models: Query<Entity, With<UploadModel>>,
    sphere_mats: Query<&MeshMaterial3d<StandardMaterial>, With<UploadSphere>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    windows: Query<&Window>,
) {
    let Ok(egui_ctx) = ctx.ctx_mut() else { return };

    // Mobile-responsive scaling
    let window = windows.single().ok();
    let (is_mobile, scale) = window
        .map(|w| {
            let min_dim = w.width().min(w.height());
            (min_dim < 600.0, (min_dim / 400.0).clamp(0.8, 1.5))
        })
        .unwrap_or((false, 1.0));

    egui_ctx.set_pixels_per_point(scale);

    let margin = if is_mobile { 8.0 } else { 12.0 };

    // Minimized icon button
    if !state.hud_open {
        egui::Area::new(egui::Id::new("upload_hud_icon"))
            .fixed_pos(egui::pos2(
                margin,
                egui_ctx.viewport_rect().height() - margin - 48.0,
            ))
            .order(egui::Order::Foreground)
            .interactable(true)
            .show(egui_ctx, |ui| {
                let size = if is_mobile { 48.0 } else { 36.0 };
                let btn = egui::Button::new("⚙")
                    .fill(egui::Color32::from_rgba_unmultiplied(30, 30, 50, 220))
                    .corner_radius(size / 2.0);
                if ui.add_sized([size, size], btn).clicked() {
                    state.hud_open = true;
                }
            });
        return;
    }

    // Main panel
    egui::Area::new(egui::Id::new("upload_hud_panel"))
        .fixed_pos(egui::pos2(
            margin,
            egui_ctx.viewport_rect().height() - margin - 280.0,
        ))
        .order(egui::Order::Foreground)
        .interactable(true)
        .show(egui_ctx, |ui| {
            egui::Frame::new()
                .fill(egui::Color32::from_rgba_unmultiplied(15, 15, 25, 220))
                .corner_radius(12.0)
                .inner_margin(if is_mobile { 16.0 } else { 12.0 })
                .show(ui, |ui| {
                    ui.set_min_width(if is_mobile { 160.0 } else { 180.0 });

                    // Header
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("⚙ Scene")
                                .strong()
                                .color(egui::Color32::from_rgb(180, 180, 220)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("✕").clicked() {
                                state.hud_open = false;
                            }
                        });
                    });
                    ui.add_space(6.0);

                    // Upload buttons
                    let btn_size = if is_mobile {
                        egui::vec2(120.0, 40.0)
                    } else {
                        egui::vec2(100.0, 28.0)
                    };

                    if is_mobile {
                        ui.vertical_centered(|ui| {
                            if ui
                                .add_sized(btn_size, egui::Button::new("📷 Panorama"))
                                .clicked()
                            {
                                pick_file(FileKind::Image);
                            }
                            ui.add_space(4.0);
                            if ui
                                .add_sized(btn_size, egui::Button::new("🎭 Model"))
                                .clicked()
                            {
                                pick_file(FileKind::Model);
                            }
                        });
                    } else {
                        ui.horizontal(|ui| {
                            if ui.button("📷 Panorama").clicked() {
                                pick_file(FileKind::Image);
                            }
                            if ui.button("🎭 Model").clicked() {
                                pick_file(FileKind::Model);
                            }
                        });
                    }

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // Skybox controls
                    ui.label(
                        egui::RichText::new("Skybox")
                            .small()
                            .color(egui::Color32::GRAY),
                    );
                    let slider_width = if is_mobile { 140.0 } else { 100.0 };

                    ui.horizontal(|ui| {
                        ui.label("Brightness");
                        if ui
                            .add_sized(
                                [slider_width, 20.0],
                                egui::Slider::new(&mut state.skybox_brightness, 0.1..=2.0)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            if let Ok(mat_h) = sphere_mats.single() {
                                if let Some(mat) = materials.get_mut(&mat_h.0) {
                                    let b = state.skybox_brightness;
                                    mat.base_color = Color::srgb(b, b, b);
                                }
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Refraction");
                        ui.add_sized(
                            [slider_width, 20.0],
                            egui::Slider::new(&mut state.refraction, 0.0..=2.0).show_value(false),
                        );
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // Clear button
                    let clear_btn = if is_mobile {
                        ui.add_sized(
                            [btn_size.x, btn_size.y],
                            egui::Button::new("🗑 Clear Objects"),
                        )
                    } else {
                        ui.button("🗑 Clear 3D Objects")
                    };
                    if clear_btn.clicked() {
                        for e in models.iter() {
                            commands.entity(e).despawn();
                        }
                    }

                    if !is_mobile {
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("P: panorama | M: model").small().weak());
                    }
                });
        });
}

pub fn handle_keyboard_shortcuts(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyP) {
        pick_file(FileKind::Image);
    }
    if keys.just_pressed(KeyCode::KeyM) {
        pick_file(FileKind::Model);
    }
}
