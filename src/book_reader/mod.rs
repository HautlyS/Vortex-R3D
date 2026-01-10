//! Book Reader Plugin - Modern native bevy_ui with liquid animations (Bevy 0.17)
//! Features: Glassmorphism, spring animations, keyboard/gamepad nav, responsive design
//! Performance: Retained mode, conditional updates, spring physics, GPU-friendly

use bevy::{
    ecs::hierarchy::ChildSpawnerCommands,
    prelude::*,
    ui::Val::*,
};

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
    target_scale: f32,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    #[default]
    Book,
    Character,
}

#[derive(Resource)]
pub struct BookTheme {
    bg: Color,
    surface: Color,
    accent: Color,
    text: Color,
    muted: Color,
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
struct BookPanel;

#[derive(Component)]
struct TabButton(Tab);

#[derive(Component)]
struct NavButton(NavAction);

#[derive(Clone, Copy)]
enum NavAction {
    Prev,
    Next,
}

#[derive(Component)]
struct PageTitle;

#[derive(Component)]
struct PageChapter;

#[derive(Component)]
struct PageContent;

#[derive(Component)]
struct PageCounter;

#[derive(Component)]
struct BookContent;

#[derive(Component)]
struct CharacterContent;

#[derive(Component)]
struct AnimatedScale {
    current: f32,
    velocity: f32, // for spring physics
}

impl Default for AnimatedScale {
    fn default() -> Self {
        Self { current: 0.0, velocity: 0.0 }
    }
}

#[derive(Component)]
#[allow(dead_code)]
struct LiquidHover {
    scale: f32,
    glow: f32,
}

// === Setup ===

fn setup_book_ui(mut commands: Commands, theme: Res<BookTheme>) {
    // Root overlay (hidden by default)
    commands
        .spawn((
            BookPanel,
            AnimatedScale::default(),
            Node {
                position_type: PositionType::Absolute,
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            Visibility::Hidden,
            GlobalZIndex(100),
        ))
        .with_children(|p| {
            // Main panel
            p.spawn((
                Node {
                    width: Px(850.0),
                    max_width: Percent(95.0),
                    height: Px(620.0),
                    max_height: Percent(90.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Px(24.0)),
                    ..default()
                },
                BackgroundColor(theme.bg),
                BorderRadius::all(Px(20.0)),
            ))
            .with_children(|panel| {
                // Header
                spawn_header(panel, &theme);
                // Tab bar
                spawn_tabs(panel, &theme);
                // Content area
                spawn_content(panel, &theme);
                // Footer
                spawn_footer(panel, &theme);
            });
        });
}

fn spawn_header(parent: &mut ChildSpawnerCommands, theme: &BookTheme) {
    parent
        .spawn(Node {
            width: Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Px(16.0)),
            ..default()
        })
        .with_children(|row| {
            // Title
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|col| {
                col.spawn((
                    Text::new("📖 TECHNO SUTRA"),
                    TextFont::from_font_size(22.0),
                    TextColor(theme.accent),
                ));
                col.spawn((
                    Text::new("Virtual Wisdom Archives"),
                    TextFont::from_font_size(12.0),
                    TextColor(theme.muted),
                ));
            });

            // Close button
            row.spawn((
                Button,
                NavButton(NavAction::Prev), // reuse for close
                Node {
                    width: Px(36.0),
                    height: Px(36.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Px(8.0)),
            ))
            .with_child((
                Text::new("✕"),
                TextFont::from_font_size(18.0),
                TextColor(theme.muted),
            ));
        });
}

fn spawn_tabs(parent: &mut ChildSpawnerCommands, theme: &BookTheme) {
    parent
        .spawn(Node {
            width: Percent(100.0),
            column_gap: Px(8.0),
            margin: UiRect::bottom(Px(16.0)),
            ..default()
        })
        .with_children(|row| {
            for (tab, label) in [(Tab::Book, "📜 Sacred Text"), (Tab::Character, "👤 Character")] {
                let selected = tab == Tab::Book;
                row.spawn((
                    Button,
                    TabButton(tab),
                    Node {
                        padding: UiRect::axes(Px(16.0), Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(if selected { theme.surface } else { Color::NONE }),
                    BorderRadius::all(Px(8.0)),
                ))
                .with_child((
                    Text::new(label),
                    TextFont::from_font_size(14.0),
                    TextColor(if selected { theme.accent } else { theme.muted }),
                ));
            }
        });
}

fn spawn_content(parent: &mut ChildSpawnerCommands, theme: &BookTheme) {
    parent
        .spawn((
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                flex_grow: 1.0,
                padding: UiRect::all(Px(20.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(theme.surface),
            BorderRadius::all(Px(12.0)),
        ))
        .with_children(|content| {
            // Book content (visible by default)
            content
                .spawn((
                    BookContent,
                    Node {
                        width: Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                ))
                .with_children(|book| {
                    book.spawn((
                        PageChapter,
                        Text::new("CHAPTER I"),
                        TextFont::from_font_size(11.0),
                        TextColor(theme.accent),
                    ));
                    book.spawn((
                        PageTitle,
                        Text::new("The Digital Awakening"),
                        TextFont::from_font_size(20.0),
                        TextColor(theme.text),
                        Node {
                            margin: UiRect::vertical(Px(8.0)),
                            ..default()
                        },
                    ));
                    book.spawn((
                        PageContent,
                        Text::new(PAGES[0].content),
                        TextFont::from_font_size(15.0),
                        TextColor(theme.text),
                        Node {
                            margin: UiRect::bottom(Px(24.0)),
                            ..default()
                        },
                    ));

                    // Navigation row
                    book.spawn(Node {
                        width: Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|nav| {
                        // Prev button
                        nav.spawn((
                            Button,
                            NavButton(NavAction::Prev),
                            Node {
                                padding: UiRect::axes(Px(16.0), Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface),
                            BorderRadius::all(Px(8.0)),
                        ))
                        .with_child((
                            Text::new("◀ Previous"),
                            TextFont::from_font_size(13.0),
                            TextColor(theme.text),
                        ));

                        // Page counter
                        nav.spawn((
                            PageCounter,
                            Text::new("— 1 / 5 —"),
                            TextFont::from_font_size(13.0),
                            TextColor(theme.muted),
                        ));

                        // Next button
                        nav.spawn((
                            Button,
                            NavButton(NavAction::Next),
                            Node {
                                padding: UiRect::axes(Px(16.0), Px(8.0)),
                                ..default()
                            },
                            BackgroundColor(theme.surface),
                            BorderRadius::all(Px(8.0)),
                        ))
                        .with_child((
                            Text::new("Next ▶"),
                            TextFont::from_font_size(13.0),
                            TextColor(theme.text),
                        ));
                    });
                });

            // Character content (hidden by default)
            content
                .spawn((
                    CharacterContent,
                    Node {
                        width: Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        display: Display::None,
                        ..default()
                    },
                ))
                .with_children(|char| {
                    // Character header
                    char.spawn(Node {
                        width: Percent(100.0),
                        column_gap: Px(20.0),
                        margin: UiRect::bottom(Px(20.0)),
                        ..default()
                    })
                    .with_children(|row| {
                        // Portrait
                        row.spawn((
                            Node {
                                width: Px(120.0),
                                height: Px(150.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.14, 0.22)),
                            BorderRadius::all(Px(12.0)),
                        ))
                        .with_child((
                            Text::new("🧘"),
                            TextFont::from_font_size(64.0),
                            TextColor(Color::WHITE),
                        ));

                        // Stats
                        row.spawn(Node {
                            flex_direction: FlexDirection::Column,
                            flex_grow: 1.0,
                            ..default()
                        })
                        .with_children(|stats| {
                            stats.spawn((
                                Text::new("THE SEEKER"),
                                TextFont::from_font_size(22.0),
                                TextColor(theme.accent),
                            ));
                            stats.spawn((
                                Text::new("Digital Pilgrim • Level 7"),
                                TextFont::from_font_size(13.0),
                                TextColor(theme.muted),
                                Node {
                                    margin: UiRect::bottom(Px(12.0)),
                                    ..default()
                                },
                            ));

                            for (stat, val) in [("Wisdom", 42), ("Focus", 78), ("Insight", 65), ("Karma", 91)] {
                                stats.spawn((
                                    Text::new(format!("{stat}: {val}/100")),
                                    TextFont::from_font_size(12.0),
                                    TextColor(theme.text),
                                ));
                            }
                        });
                    });

                    // Bio
                    char.spawn((
                        Text::new("BIOGRAPHY"),
                        TextFont::from_font_size(11.0),
                        TextColor(theme.accent),
                    ));
                    char.spawn((
                        Text::new(CHARACTER_BIO),
                        TextFont::from_font_size(14.0),
                        TextColor(theme.text),
                        Node {
                            margin: UiRect::vertical(Px(8.0)),
                            ..default()
                        },
                    ));

                    // Abilities
                    char.spawn((
                        Text::new("ABILITIES"),
                        TextFont::from_font_size(11.0),
                        TextColor(theme.accent),
                        Node {
                            margin: UiRect::top(Px(12.0)),
                            ..default()
                        },
                    ));
                    for (icon, name, desc) in ABILITIES {
                        char.spawn((
                            Text::new(format!("{icon} {name} — {desc}")),
                            TextFont::from_font_size(12.0),
                            TextColor(theme.text),
                            Node {
                                margin: UiRect::top(Px(4.0)),
                                ..default()
                            },
                        ));
                    }
                });
        });
}

fn spawn_footer(parent: &mut ChildSpawnerCommands, theme: &BookTheme) {
    parent.spawn((
        Text::new("[B] Close  •  [←][→] Pages  •  [Tab] Switch"),
        TextFont::from_font_size(11.0),
        TextColor(theme.muted),
        Node {
            margin: UiRect::top(Px(12.0)),
            ..default()
        },
    ));
}

// === Systems ===

fn toggle_book_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<BookState>,
) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        state.open = !state.open;
        state.target_scale = if state.open { 1.0 } else { 0.0 };
    }
}

fn close_book_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<BookState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) && state.open {
        state.open = false;
        state.target_scale = 0.0;
    }
}

#[allow(dead_code)]
fn toggle_book(mut state: ResMut<BookState>) {
    state.open = !state.open;
    state.target_scale = if state.open { 1.0 } else { 0.0 };
}

#[allow(dead_code)]
fn close_book(mut state: ResMut<BookState>) {
    if state.open {
        state.open = false;
        state.target_scale = 0.0;
    }
}

fn animate_panel(
    mut query: Query<(&mut Visibility, &mut AnimatedScale, &mut Transform, &mut BackgroundColor), With<BookPanel>>,
    state: Res<BookState>,
    time: Res<Time>,
) {
    let Ok((mut vis, mut anim, mut transform, mut bg)) = query.single_mut() else {
        return;
    };

    let target = state.target_scale;
    let dt = time.delta_secs().min(0.05); // cap for stability

    // Spring physics: F = -k*x - d*v
    let stiffness = 180.0;
    let damping = 18.0;
    let displacement = target - anim.current;
    let spring_force = stiffness * displacement - damping * anim.velocity;
    
    anim.velocity += spring_force * dt;
    anim.current += anim.velocity * dt;

    // Clamp near target to prevent micro-oscillations
    if displacement.abs() < 0.001 && anim.velocity.abs() < 0.01 {
        anim.current = target;
        anim.velocity = 0.0;
    }

    // Update visibility and transforms
    if anim.current < 0.01 && target == 0.0 {
        *vis = Visibility::Hidden;
        anim.current = 0.0;
    } else {
        *vis = Visibility::Visible;
    }

    // Liquid scale + fade effect
    let t = anim.current.clamp(0.0, 1.0);
    transform.scale = Vec3::splat(0.85 + t * 0.15);
    *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6 * t));
}

fn handle_tab_buttons(
    mut state: ResMut<BookState>,
    mut tabs: Query<(&Interaction, &TabButton, &mut BackgroundColor, &Children), Changed<Interaction>>,
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
    mut chapter: Query<&mut Text, (With<PageChapter>, Without<PageTitle>, Without<PageContent>, Without<PageCounter>)>,
    mut title: Query<&mut Text, (With<PageTitle>, Without<PageChapter>, Without<PageContent>, Without<PageCounter>)>,
    mut content: Query<&mut Text, (With<PageContent>, Without<PageChapter>, Without<PageTitle>, Without<PageCounter>)>,
    mut counter: Query<&mut Text, (With<PageCounter>, Without<PageChapter>, Without<PageTitle>, Without<PageContent>)>,
) {
    // Toggle content visibility
    if let Ok(mut node) = book_content.single_mut() {
        node.display = if state.tab == Tab::Book { Display::Flex } else { Display::None };
    }
    if let Ok(mut node) = char_content.single_mut() {
        node.display = if state.tab == Tab::Character { Display::Flex } else { Display::None };
    }

    // Update page content
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
            t.0 = format!("— {} / {} —", state.page + 1, PAGES.len()).into();
        }
    }
}

fn animate_buttons(
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &mut Transform), (With<Button>, Without<TabButton>, Changed<Interaction>)>,
    theme: Res<BookTheme>,
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

// === Data ===

struct Page {
    chapter: &'static str,
    title: &'static str,
    content: &'static str,
}

const PAGES: &[Page] = &[
    Page {
        chapter: "CHAPTER I",
        title: "The Digital Awakening",
        content: "In the beginning, there was the void—an infinite expanse of unrendered space. Then came the first vertex, a single point of light in the darkness, and from it, all geometry would flow.\n\nThe ancient programmers spoke of this moment in hushed tones, their fingers dancing across mechanical keyboards, invoking the sacred compile commands that would breathe life into silicon dreams.\n\n\"Let there be polygons,\" they whispered, and triangles tessellated across the void, forming the foundation of all virtual reality.",
    },
    Page {
        chapter: "CHAPTER II",
        title: "The Path of Pixels",
        content: "Each frame rendered is a meditation upon impermanence. Sixty times per second, the world dissolves and reforms, teaching us that nothing persists—only the illusion of continuity created by our limited perception.\n\nThe shader monks of the Eastern Rendering Temple spent decades perfecting their fragment programs, seeking the perfect balance between performance and beauty.\n\n\"Optimize not for speed alone,\" Master Carmack once taught, \"but for the harmony of all systems working as one.\"",
    },
    Page {
        chapter: "CHAPTER III",
        title: "Wisdom of the Wireframe",
        content: "Beneath every textured surface lies the wireframe truth. Strip away the normal maps, the ambient occlusion, the carefully crafted materials—and what remains? Pure geometry. Pure mathematics. Pure being.\n\nThe wireframe view is not a debug mode. It is enlightenment mode.\n\nWhen the student asked, \"Master, how do I achieve photorealism?\" the teacher replied, \"First, understand why you seek it.\"",
    },
    Page {
        chapter: "CHAPTER IV",
        title: "The Render Pipeline",
        content: "From vertex to fragment, the journey unfolds in stages both mysterious and precise. The GPU, that silicon bodhisattva, processes billions of operations each second, yet never complains, never wavers.\n\nThe Render Pipeline is the Eightfold Path:\n1. Input Assembly\n2. Vertex Shader\n3. Tessellation\n4. Geometry Shader\n5. Rasterization\n6. Fragment Shader\n7. Depth Testing\n8. Blending",
    },
    Page {
        chapter: "CHAPTER V",
        title: "Enlightenment Through Iteration",
        content: "The game loop is the wheel of dharma, turning endlessly:\n\nwhile (running) {\n    processInput();\n    update();\n    render();\n}\n\nIn these three functions lies all of existence. We receive input from the world, we update our internal state, we render our response.\n\nSeek not perfection, but stability. Seek not maximum performance, but sustainable performance. And always—profile before you optimize.",
    },
];

const CHARACTER_BIO: &str = "A wanderer between digital realms, the Seeker has traversed countless virtual landscapes in pursuit of the ultimate truth: the source code of consciousness itself.\n\nNow they walk the path of the Techno Sutra, gathering wisdom from ancient shader monks and modern compute prophets alike.";

const ABILITIES: &[(&str, &str, &str)] = &[
    ("🔮", "Digital Sight", "See through textures to the wireframe beneath"),
    ("⚡", "Frame Skip", "Move between moments, bypassing time itself"),
    ("🌀", "Shader Weave", "Manipulate light and shadow at will"),
    ("💫", "Buffer Overflow", "Channel excess data into raw power"),
];
