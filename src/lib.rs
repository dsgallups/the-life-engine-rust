use std::{io::Cursor, time::Duration};

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::PrimaryWindow,
    winit::WinitWindows,
};
use bevy_spatial::{kdtree::KDTree2, AutomaticUpdate, SpatialStructure, TransformMode};
use camera::{spawn_camera, update_camera};
use cell::CellType;
use game::GamePlugin;
use winit::window::Icon;

pub const ORGANISM_LAYER: f32 = 1.0;

pub(crate) mod camera;
pub(crate) mod cell;
pub(crate) mod environment;
pub(crate) mod game;
pub(crate) mod load;
pub(crate) mod menu;
pub(crate) mod neighbor;
pub(crate) mod organism;

pub type CellTree = KDTree2<CellType>;

/// Entry point for the bin
///
/// See [`GamePlugin`] for detailed information about how the systems work
pub fn plugin(app: &mut App) {
    // borrowed mostly from the bevy game template
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Life Engine".to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::ERROR,
                    ..Default::default()
                }),
        )
        .add_plugins(
            AutomaticUpdate::<CellType>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(1))
                .with_transform(TransformMode::GlobalTransform),
        )
        .add_plugins(GamePlugin)
        .add_systems(Startup, (set_window_icon, spawn_camera))
        .add_systems(Update, update_camera);
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
