use std::io::Cursor;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use game::GamePlugin;
use winit::window::Icon;

pub const ORGANISM_LAYER: f32 = 1.0;
pub const CELL_MULT: f32 = 10.;

pub(crate) mod cell;
pub(crate) mod environment;
pub(crate) mod game;
pub(crate) mod load;
pub(crate) mod menu;
pub(crate) mod organism;

pub fn plugin(app: &mut App) {
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Life Engine".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        .add_systems(Startup, set_window_icon);
}

// Sets the icon on windows and X11
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
