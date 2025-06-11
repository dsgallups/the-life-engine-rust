use bevy::prelude::*;

use crate::{load::TextureAssets, GameState};

use super::actions::{ChangeState, OpenLink};

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.24, 0.25, 0.25),
        }
    }
}

pub struct Buttons;

impl Buttons {
    /// Creates a play button
    pub fn play_button(children_builder: &mut ChildBuilder) {
        let button_colors = ButtonColors::default();

        children_builder
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(140.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: button_colors.normal.into(),
                    ..Default::default()
                },
                button_colors,
                ChangeState(GameState::Playing),
            ))
            .with_children(|cb| {
                cb.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font_size: 40.,
                        color: Color::linear_rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                ));
            });
    }

    /// Creates a button for github
    pub fn github_button(children_builder: &mut ChildBuilder, textures: Res<TextureAssets>) {
        children_builder
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..Default::default()
                },
                ButtonColors {
                    normal: Color::NONE,
                    hovered: Color::linear_rgb(0.25, 0.25, 0.25),
                },
                OpenLink("https://github.com/dsgallups"),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Open source",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::linear_rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
                parent.spawn(ImageBundle {
                    image: textures.github.clone().into(),
                    style: Style {
                        width: Val::Px(32.),
                        ..default()
                    },
                    ..default()
                });
            });
    }
    pub fn mute_button(children_builder: &mut ChildBuilder) {
        children_builder
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        ..Default::default()
                    },
                    background_color: Color::linear_rgb(1., 0., 0.).into(),
                    ..Default::default()
                },
                ButtonColors::default(),
                Mute(false),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Mute",
                    TextStyle {
                        font_size: 15.0,
                        color: Color::linear_rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
    }
}

#[derive(Component, Debug)]
pub struct Mute(pub bool);
