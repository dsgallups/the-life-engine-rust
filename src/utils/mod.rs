use std::borrow::Cow;

use bevy::{ecs::system::IntoObserverSystem, prelude::*};

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

pub fn simple_button<E: EntityEvent, B: Bundle, M, I: IntoObserverSystem<E, B, M>>(
    text: impl Into<String>,
    text_color: impl Into<Color>,
    action: I,
    bundle: impl Bundle,
) -> impl Bundle {
    let text = text.into();
    let text_color = text_color.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Button,
                    Pickable::default(),
                    children![(Text(text), TextColor(text_color))],
                ))
                .insert(bundle)
                .observe(action);
        })),
    )
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

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(24.0),
        TextColor(LABEL_TEXT),
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0),
        TextColor(HEADER_TEXT),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: px(380.0),
                height: px(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette::default(),
                    Pickable::default(),
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(40.0),
                        TextColor(BUTTON_TEXT),
                        // Don't bubble picking events from the text up to the button.
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}
