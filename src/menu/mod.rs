use bevy::{audio::Volume, prelude::*};
use button::Buttons;
use systems::click_button;

use super::{
    game::GameState,
    load::{AudioAssets, TextureAssets},
};

mod actions;

mod button;

mod systems;

pub struct MenuPlugin;

/// # The main menu
///
/// Sets up the ui elements for the main menu and cleans itself when the [`GameState::Menu`] is
/// exited.
///
/// Additionally, it spawns a camera in [`setup_menu`].
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            Menu,
        ))
        .with_children(Buttons::play_button);

    commands.spawn((
        AudioBundle {
            source: audio_assets.main_menu_music.clone(),
            settings: PlaybackSettings {
                volume: Volume::new(0.),
                ..PlaybackSettings::LOOP
            },
        },
        Menu,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|child_builder| {
            Buttons::github_button(child_builder, textures);
            Buttons::mute_button(child_builder);
        });
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        // also despawns the children (buttons and whatnot)
        commands.entity(entity).despawn_recursive();
    }
}
