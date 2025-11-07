#![allow(unused)]

use bevy::prelude::*;

pub mod asset_tracking;

mod interaction_palette;
pub use interaction_palette::*;

mod palette;
pub use palette::*;

pub mod conditions;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((asset_tracking::plugin));
    app.add_plugins(interaction_palette::plugin);
}

#[expect(dead_code)]
pub fn keybind(
    text: impl Into<String>,
    font: Handle<Font>,
    font_size: f32,
    color: impl Into<Color>,
    key: KeyCode,
) -> impl Bundle {
    let color = color.into();
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new(text),
                TextFont {
                    font: font.clone(),
                    font_size,
                    ..default()
                },
                TextColor(color)
            ),
            (
                Text::new(format!("[{key:?}]")),
                TextFont {
                    font,
                    font_size: 10.,
                    ..default()
                },
                TextColor(color)
            )
        ],
    )
}
