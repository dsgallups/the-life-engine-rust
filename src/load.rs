use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::game::GameState;

pub struct LoadingPlugin;

/// Loads initial assets for the game. Most of these assets are only in the Menu.
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        //note: that this is from an extension of App by bevy_asset_loader
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub main_menu_music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub _bevy: Handle<Image>,

    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}
