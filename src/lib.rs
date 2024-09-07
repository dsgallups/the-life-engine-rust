use std::{io::Cursor, time::Duration};

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PrimaryWindow, winit::WinitWindows,
};
use bevy_spatial::{kdtree::KDTree2, AutomaticUpdate, SpatialStructure, TransformMode};
use camera::{spawn_camera, update_camera};
use cell::CellType;
use environment::EnvironmentPlugin;
use load::LoadingPlugin;
use menu::MenuPlugin;
use winit::window::Icon;

pub const ORGANISM_LAYER: f32 = 1.0;

pub(crate) mod camera;
pub(crate) mod cell;
pub(crate) mod environment;
pub(crate) mod fps;
pub(crate) mod load;
pub(crate) mod menu;
pub(crate) mod neighbor;
pub(crate) mod organism;

pub type CellTree = KDTree2<CellType>;

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
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                EnvironmentPlugin,
                FrameTimeDiagnosticsPlugin,
            ))
            .add_systems(Startup, (set_window_icon, spawn_camera))
            .add_systems(Update, update_camera)
            .add_plugins(
                AutomaticUpdate::<CellType>::new()
                    .with_spatial_ds(SpatialStructure::KDTree2)
                    .with_frequency(Duration::from_millis(1))
                    .with_transform(TransformMode::GlobalTransform),
            );

        app.add_systems(Startup, fps::setup_fps_counter);
        app.add_systems(
            Update,
            (fps::fps_text_update_system, fps::fps_counter_showhide),
        );
    }
}

// Sets the icon on windows and X11. Borrowed from the bevy game template
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
