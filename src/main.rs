#![allow(dead_code)]

use std::{thread, time};

pub mod cell;
//pub mod messages;
pub mod organism;
pub mod world;

mod render;
use render::begin_ticking;

use bevy::{
    app::{App, FixedUpdate, Update},
    render::color::Color,
    time::{Fixed, Time},
};
pub use cell::*;
//pub use messages::*;
pub use organism::*;
pub use world::*;

pub trait Drawable {
    fn color(&self) -> Color;
}

fn main() {
    //println!("ozymandias");
    let mut world = LEWorld::new();
    world.add_simple_organism((0, 0, 0).into());

    begin_ticking(world);
}
