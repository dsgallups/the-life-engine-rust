mod organism;
mod world;
use startup::StartupPlugin;
use world::LEWorld;
mod startup;

use bevy::prelude::*;

fn main() {
    //println!("ozymandias");
    let world = LEWorld::default();

    App::new()
        .insert_resource(world)
        .add_plugins((DefaultPlugins, StartupPlugin))
        .run();
}
