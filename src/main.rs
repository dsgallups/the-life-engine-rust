mod organism;
mod world;
use world::LEWorld;
mod startup;

use bevy::prelude::*;
use organism::Organism;

fn main() {
    //println!("ozymandias");
    let world = LEWorld::default();

    App::new()
        .insert_resource(world)
        .add_plugins((DefaultPlugins, bevy::diagnostic::FrameTimeDiagnosticsPlugin))
        .add_systems(Update, draw_world);
}

#[derive(Clone, Debug, Default)]
pub struct Vec2d {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Vec2d {
    fn from((x, y): (isize, isize)) -> Self {
        Vec2d { x, y }
    }
}

fn draw_world(mut commands: Commands, world: Res<LEWorld>) {
    todo!();
}
