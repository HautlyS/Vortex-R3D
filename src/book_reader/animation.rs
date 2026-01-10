//! Spring animation system for book panel

use bevy::prelude::*;

use super::{BookPanel, BookState};

#[derive(Component)]
pub struct AnimatedScale {
    pub current: f32,
    pub velocity: f32,
}

impl Default for AnimatedScale {
    fn default() -> Self {
        Self {
            current: 0.0,
            velocity: 0.0,
        }
    }
}

pub fn animate_panel(
    mut query: Query<
        (&mut Visibility, &mut AnimatedScale, &mut Transform, &mut BackgroundColor),
        With<BookPanel>,
    >,
    state: Res<BookState>,
    time: Res<Time>,
) {
    let Ok((mut vis, mut anim, mut transform, mut bg)) = query.single_mut() else {
        return;
    };

    let target = state.target_scale;
    let dt = time.delta_secs().min(0.05);

    // Spring physics: F = -k*x - d*v
    let stiffness = 180.0;
    let damping = 18.0;
    let displacement = target - anim.current;
    let spring_force = stiffness * displacement - damping * anim.velocity;

    anim.velocity += spring_force * dt;
    anim.current += anim.velocity * dt;

    if displacement.abs() < 0.001 && anim.velocity.abs() < 0.01 {
        anim.current = target;
        anim.velocity = 0.0;
    }

    if anim.current < 0.01 && target == 0.0 {
        *vis = Visibility::Hidden;
        anim.current = 0.0;
    } else {
        *vis = Visibility::Visible;
    }

    let t = anim.current.clamp(0.0, 1.0);
    transform.scale = Vec3::splat(0.85 + t * 0.15);
    *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6 * t));
}

pub fn animate_buttons(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &mut Transform),
        (With<Button>, Without<super::TabButton>, Changed<Interaction>),
    >,
    theme: Res<super::BookTheme>,
) {
    for (interaction, mut bg, mut transform) in &mut buttons {
        let (color, scale) = match interaction {
            Interaction::Pressed => (theme.accent.with_alpha(0.4), 0.95),
            Interaction::Hovered => (theme.surface.with_alpha(0.95), 1.03),
            Interaction::None => (theme.surface, 1.0),
        };
        *bg = BackgroundColor(color);
        transform.scale = Vec3::splat(scale);
    }
}
