//! Adaptive Performance System - EMA-based FPS with hysteresis quality control
//! WASM-optimized with async-friendly timing

use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::window;

pub struct PerformancePlugin;

impl Plugin for PerformancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FpsMonitor>()
            .init_resource::<QualitySettings>()
            .add_message::<QualityChanged>()
            .add_systems(Update, (update_fps, auto_adjust_quality));
    }
}

/// Quality levels
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Reflect)]
pub enum QualityLevel {
    Ultra,
    High,
    #[default]
    Medium,
    Low,
    Potato,
}

impl QualityLevel {
    pub fn step_down(&self) -> Self {
        match self {
            Self::Ultra => Self::High,
            Self::High => Self::Medium,
            Self::Medium => Self::Low,
            Self::Low | Self::Potato => Self::Potato,
        }
    }

    pub fn step_up(&self) -> Self {
        match self {
            Self::Potato => Self::Low,
            Self::Low => Self::Medium,
            Self::Medium => Self::High,
            Self::High | Self::Ultra => Self::Ultra,
        }
    }

    /// Spawner rate multiplier for this quality level
    pub fn spawner_rate_mult(&self) -> f32 {
        match self {
            Self::Ultra => 1.0,
            Self::High => 0.7,
            Self::Medium => 0.4,
            Self::Low => 0.2,
            Self::Potato => 0.08,
        }
    }
}

/// Event fired when quality changes
#[derive(bevy::prelude::Message, Clone, Copy)]
pub struct QualityChanged {
    #[allow(dead_code)] // Available for handlers that need previous quality
    pub old: QualityLevel,
    pub new: QualityLevel,
}

/// EMA-based FPS monitor with jitter detection
#[derive(Resource)]
pub struct FpsMonitor {
    /// Short-term EMA (reactive, alpha=0.2)
    pub fps_fast: f32,
    /// Long-term EMA (stable, alpha=0.05)
    pub fps_slow: f32,
    /// Frame time variance (jitter detection)
    pub variance: f32,
    /// Last frame time for variance calc
    last_dt: f32,
    /// Frames since last quality change
    stable_frames: u32,
    /// Cooldown timer (seconds)
    cooldown: f32,
    /// WASM high-precision timestamp
    #[cfg(target_arch = "wasm32")]
    last_perf_now: f64,
}

impl Default for FpsMonitor {
    fn default() -> Self {
        Self {
            fps_fast: 60.0,
            fps_slow: 60.0,
            variance: 0.0,
            last_dt: 0.0167,
            stable_frames: 0,
            cooldown: 2.0, // Initial warmup
            #[cfg(target_arch = "wasm32")]
            last_perf_now: 0.0,
        }
    }
}

/// Quality settings with hysteresis thresholds
#[derive(Resource)]
pub struct QualitySettings {
    pub level: QualityLevel,
    pub particle_multiplier: f32,
    pub effect_intensity: f32,
    pub max_lights: u32,
    pub material_update_hz: f32,
    /// Hysteresis: FPS must drop below this to downgrade
    downgrade_threshold: f32,
    /// Hysteresis: FPS must exceed this to upgrade
    upgrade_threshold: f32,
}

impl Default for QualitySettings {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let level = QualityLevel::Low; // Start conservative on WASM
        #[cfg(not(target_arch = "wasm32"))]
        let level = QualityLevel::Medium;

        let mut s = Self {
            level,
            particle_multiplier: 1.0,
            effect_intensity: 1.0,
            max_lights: 8,
            material_update_hz: 20.0,
            downgrade_threshold: 28.0,
            upgrade_threshold: 55.0,
        };
        s.apply_level(level);
        s
    }
}

impl QualitySettings {
    pub fn apply_level(&mut self, level: QualityLevel) {
        self.level = level;
        match level {
            QualityLevel::Ultra => {
                self.particle_multiplier = 1.0;
                self.effect_intensity = 1.0;
                self.max_lights = 16;
                self.material_update_hz = 60.0;
            }
            QualityLevel::High => {
                self.particle_multiplier = 0.7;
                self.effect_intensity = 1.0;
                self.max_lights = 12;
                self.material_update_hz = 30.0;
            }
            QualityLevel::Medium => {
                self.particle_multiplier = 0.4;
                self.effect_intensity = 0.8;
                self.max_lights = 8;
                self.material_update_hz = 20.0;
            }
            QualityLevel::Low => {
                self.particle_multiplier = 0.2;
                self.effect_intensity = 0.5;
                self.max_lights = 4;
                self.material_update_hz = 10.0;
            }
            QualityLevel::Potato => {
                self.particle_multiplier = 0.08;
                self.effect_intensity = 0.0;
                self.max_lights = 2;
                self.material_update_hz = 5.0;
            }
        }
    }

    pub fn particle_count(&self, base: usize) -> usize {
        ((base as f32 * self.particle_multiplier).ceil() as usize).max(1)
    }

    pub fn should_update_materials(&self, elapsed: f32, last_update: f32) -> bool {
        elapsed - last_update >= 1.0 / self.material_update_hz
    }
}

fn update_fps(time: Res<Time>, mut monitor: ResMut<FpsMonitor>) {
    // Get delta time - use WASM performance.now() when available
    #[cfg(target_arch = "wasm32")]
    let dt = {
        if let Some(perf) = window().and_then(|w| w.performance()) {
            let now = perf.now();
            let dt = if monitor.last_perf_now > 0.0 {
                ((now - monitor.last_perf_now) / 1000.0) as f32
            } else {
                time.delta_secs()
            };
            monitor.last_perf_now = now;
            dt.max(0.001)
        } else {
            time.delta_secs()
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let dt = time.delta_secs();

    if dt <= 0.0 || dt > 1.0 {
        return;
    }

    let fps = 1.0 / dt;

    // EMA updates: fast (alpha=0.2) and slow (alpha=0.05)
    monitor.fps_fast = monitor.fps_fast * 0.8 + fps * 0.2;
    monitor.fps_slow = monitor.fps_slow * 0.95 + fps * 0.05;

    // Variance tracking (jitter detection)
    let dt_diff = (dt - monitor.last_dt).abs();
    monitor.variance = monitor.variance * 0.9 + dt_diff * 0.1;
    monitor.last_dt = dt;

    // Cooldown tick
    if monitor.cooldown > 0.0 {
        monitor.cooldown -= dt;
    }

    // Stable frame counting
    if monitor.fps_fast > 45.0 && monitor.variance < 0.01 {
        monitor.stable_frames = monitor.stable_frames.saturating_add(1);
    } else {
        monitor.stable_frames = 0;
    }
}

fn auto_adjust_quality(
    mut monitor: ResMut<FpsMonitor>,
    mut settings: ResMut<QualitySettings>,
    mut events: MessageWriter<QualityChanged>,
) {
    if monitor.cooldown > 0.0 {
        return;
    }

    let fps = monitor.fps_fast;
    let jitter = monitor.variance > 0.015; // High frame time variance
    let current = settings.level;

    // Downgrade: FPS below threshold OR high jitter
    if fps < settings.downgrade_threshold || (jitter && fps < 40.0) {
        let new_level = current.step_down();
        if new_level != current {
            settings.apply_level(new_level);
            monitor.cooldown = 3.0; // Longer cooldown after downgrade
            monitor.stable_frames = 0;
            events.write(QualityChanged {
                old: current,
                new: new_level,
            });
            info!(
                "⚡ Quality ↓ {:?}→{:?} (fps:{:.0})",
                current, new_level, fps
            );
        }
    }
    // Upgrade: stable high FPS for 4+ seconds
    else if monitor.stable_frames > 240 && fps > settings.upgrade_threshold {
        let new_level = current.step_up();
        if new_level != current {
            settings.apply_level(new_level);
            monitor.cooldown = 5.0; // Longer cooldown after upgrade
            monitor.stable_frames = 0;
            events.write(QualityChanged {
                old: current,
                new: new_level,
            });
            info!(
                "⚡ Quality ↑ {:?}→{:?} (fps:{:.0})",
                current, new_level, fps
            );
        }
    }
}

// Debug overlay
#[derive(Component)]
pub struct FpsOverlay;

pub fn spawn_fps_overlay(mut commands: Commands) {
    commands.spawn((
        Text::new("FPS: --"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgba(0.0, 1.0, 0.5, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        FpsOverlay,
    ));
}

pub fn update_fps_overlay(
    monitor: Res<FpsMonitor>,
    quality: Res<QualitySettings>,
    mut query: Query<&mut Text, With<FpsOverlay>>,
) {
    for mut text in query.iter_mut() {
        **text = format!(
            "FPS:{:.0} | {:?} | j:{:.3}",
            monitor.fps_fast, quality.level, monitor.variance
        );
    }
}
