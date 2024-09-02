use bevy::prelude::*;

use super::{environment::EnvironmentPlugin, load::LoadingPlugin, menu::MenuPlugin};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

/// # The primary Plugin for the life engine
///
/// This inserts three different plugins:
/// - [`LoadingPlugin`] is used for inserting initial resources, like sound and textures for
///     the main menu
/// - [`MenuPlugin`] is used for rendering and controlling actions on the main menu. This
///     plugin essentially "kicks in" after the [`GameState::Loading`] has transitioned to [`GameState::Menu`].
///     This also spawns a camera.
///
/// - [`EnvironmentPlugin`] is the meat of the game, which loads in systems for organisms and replicating behavior.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((LoadingPlugin, MenuPlugin, EnvironmentPlugin));
    }
}
