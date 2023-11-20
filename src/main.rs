#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod engine;
pub mod environment;
pub mod organism;
mod startup;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::text::{BreakLineOn, Text2dBounds};
use bevy::window::PrimaryWindow;
use environment::WorldEnvironment;
use organism::anatomy::Anatomy;
use organism::cell::{Cell, CellType};
use organism::Organism;
use rand::prelude::*;
use startup::StartupPlugin;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins((DefaultPlugins, StartupPlugin))
        .init_resource::<WorldEnvironment>()
        .add_systems(Update, print_mouse_pos)
        .run();
}

#[derive(Component)]
pub struct MousePosBox;

pub fn print_mouse_pos(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_pos_box: Query<&mut Text, With<MousePosBox>>,
) {
    let mut text = mouse_pos_box.get_single_mut().unwrap();
    let window = windows.get_single().unwrap();
    let pos = window.cursor_position();
    match pos {
        Some(pos) => text.sections[0].value = format!("({}, {})", pos.x, pos.y),
        None => text.sections[0].value = "(n/a, n/a)".to_string(),
    }
}
