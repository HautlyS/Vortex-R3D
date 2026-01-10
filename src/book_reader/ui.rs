//! Book reader UI setup

use bevy::{ecs::hierarchy::ChildSpawnerCommands, prelude::*, ui::Val::*};

use super::{
    animation::AnimatedScale,
    content::{ABILITIES, CHARACTER_BIO, PAGES},
    BookContent, BookPanel, BookTheme, CharacterContent, NavAction, NavButton, PageChapter,
    PageContent, PageCounter, PageTitle, Tab, TabButton,
};

pub fn setup_book_ui(mut commands: Commands, theme: Res<BookTheme>) {
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
                spawn_header(panel, &theme);
                spawn_tabs(panel, &theme);
                spawn_content(panel, &theme);
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

            row.spawn((
                Button,
                NavButton(NavAction::Prev),
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
            spawn_book_content(content, theme);
            spawn_character_content(content, theme);
        });
}

fn spawn_book_content(content: &mut ChildSpawnerCommands, theme: &BookTheme) {
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

            // Navigation
            book.spawn(Node {
                width: Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|nav| {
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

                nav.spawn((
                    PageCounter,
                    Text::new("— 1 / 5 —"),
                    TextFont::from_font_size(13.0),
                    TextColor(theme.muted),
                ));

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
}

fn spawn_character_content(content: &mut ChildSpawnerCommands, theme: &BookTheme) {
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
            // Header with portrait
            char.spawn(Node {
                width: Percent(100.0),
                column_gap: Px(20.0),
                margin: UiRect::bottom(Px(20.0)),
                ..default()
            })
            .with_children(|row| {
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
