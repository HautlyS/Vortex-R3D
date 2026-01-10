//! Book Reader Plugin - Modern native bevy_ui with liquid animations

mod animation;
mod content;
mod ui;

use bevy::prelude::*;

use animation::{animate_buttons, animate_panel};
use content::PAGES;
use ui::setup_book_ui;

pub struct BookReaderPlugin;

impl Plugin for BookReaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BookState>()
            .init_resource::<BookTheme>()
            .add_systems(Startup, setup_book_ui)
            .add_systems(
                Update,
                (
                    toggle_book_input,
                    close_book_input,
                    handle_tab_buttons,
                    handle_nav_buttons,
                    update_page_content.run_if(resource_changed::<BookState>),
                    animate_panel,
                    animate_buttons,
                ),
            );
        info!("📚 Book Reader plugin registered");
    }
}

// === Resources ===

#[derive(Resource, Default)]
pub struct BookState {
    pub open: bool,
    pub page: usize,
    pub tab: Tab,
    pub target_scale: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    #[default]
    Book,
    Character,
}

#[derive(Resource)]
pub struct BookTheme {
    pub bg: Color,
    pub surface: Color,
    pub accent: Color,
    pub text: Color,
    pub muted: Color,
}

impl Default for BookTheme {
    fn default() -> Self {
        Self {
            bg: Color::srgba(0.06, 0.05, 0.08, 0.92),
            surface: Color::srgba(0.12, 0.10, 0.16, 0.8),
            accent: Color::srgb(0.54, 0.39, 0.86),
            text: Color::srgb(0.94, 0.92, 0.98),
            muted: Color::srgb(0.55, 0.51, 0.63),
        }
    }
}

// === Components ===

#[derive(Component)]
pub struct BookPanel;

#[derive(Component)]
pub struct TabButton(pub Tab);

#[derive(Component)]
pub struct NavButton(pub NavAction);

#[derive(Clone, Copy)]
pub enum NavAction {
    Prev,
    Next,
}

#[derive(Component)]
pub struct PageTitle;

#[derive(Component)]
pub struct PageChapter;

#[derive(Component)]
pub struct PageContent;

#[derive(Component)]
pub struct PageCounter;

#[derive(Component)]
pub struct BookContent;

#[derive(Component)]
pub struct CharacterContent;

// === Systems ===

fn toggle_book_input(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<BookState>) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        state.open = !state.open;
        state.target_scale = if state.open { 1.0 } else { 0.0 };
    }
}

fn close_book_input(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<BookState>) {
    if keyboard.just_pressed(KeyCode::Escape) && state.open {
        state.open = false;
        state.target_scale = 0.0;
    }
}

fn handle_tab_buttons(
    mut state: ResMut<BookState>,
    mut tabs: Query<
        (&Interaction, &TabButton, &mut BackgroundColor, &Children),
        Changed<Interaction>,
    >,
    mut texts: Query<&mut TextColor>,
    theme: Res<BookTheme>,
) {
    for (interaction, tab_btn, mut bg, children) in &mut tabs {
        if *interaction == Interaction::Pressed {
            state.tab = tab_btn.0;
        }

        let selected = state.tab == tab_btn.0;
        let hovered = *interaction == Interaction::Hovered;

        *bg = if selected || hovered {
            BackgroundColor(theme.surface)
        } else {
            BackgroundColor(Color::NONE)
        };

        if let Some(&child) = children.first() {
            if let Ok(mut color) = texts.get_mut(child) {
                *color = TextColor(if selected { theme.accent } else { theme.muted });
            }
        }
    }
}

fn handle_nav_buttons(
    mut state: ResMut<BookState>,
    buttons: Query<(&Interaction, &NavButton), Changed<Interaction>>,
) {
    for (interaction, nav) in &buttons {
        if *interaction == Interaction::Pressed {
            match nav.0 {
                NavAction::Prev => state.page = state.page.saturating_sub(1),
                NavAction::Next => state.page = (state.page + 1).min(PAGES.len() - 1),
            }
        }
    }
}

fn update_page_content(
    state: Res<BookState>,
    mut book_content: Query<&mut Node, (With<BookContent>, Without<CharacterContent>)>,
    mut char_content: Query<&mut Node, (With<CharacterContent>, Without<BookContent>)>,
    mut chapter: Query<
        &mut Text,
        (
            With<PageChapter>,
            Without<PageTitle>,
            Without<PageContent>,
            Without<PageCounter>,
        ),
    >,
    mut title: Query<
        &mut Text,
        (
            With<PageTitle>,
            Without<PageChapter>,
            Without<PageContent>,
            Without<PageCounter>,
        ),
    >,
    mut content: Query<
        &mut Text,
        (
            With<PageContent>,
            Without<PageChapter>,
            Without<PageTitle>,
            Without<PageCounter>,
        ),
    >,
    mut counter: Query<
        &mut Text,
        (
            With<PageCounter>,
            Without<PageChapter>,
            Without<PageTitle>,
            Without<PageContent>,
        ),
    >,
) {
    if let Ok(mut node) = book_content.single_mut() {
        node.display = if state.tab == Tab::Book {
            Display::Flex
        } else {
            Display::None
        };
    }
    if let Ok(mut node) = char_content.single_mut() {
        node.display = if state.tab == Tab::Character {
            Display::Flex
        } else {
            Display::None
        };
    }

    if state.tab == Tab::Book {
        let page = &PAGES[state.page.min(PAGES.len() - 1)];
        if let Ok(mut t) = chapter.single_mut() {
            t.0 = page.chapter.into();
        }
        if let Ok(mut t) = title.single_mut() {
            t.0 = page.title.into();
        }
        if let Ok(mut t) = content.single_mut() {
            t.0 = page.content.into();
        }
        if let Ok(mut t) = counter.single_mut() {
            t.0 = format!("— {} / {} —", state.page + 1, PAGES.len());
        }
    }
}
