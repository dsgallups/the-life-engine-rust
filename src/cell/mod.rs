use bevy::{ecs::component::Component, render::color::Color};

pub trait Drawable {
    fn color(&self) -> Color;
}

#[derive(Component)]
pub struct Food;

impl Drawable for Food {
    fn color(&self) -> Color {
        Color::BLUE
    }
}

#[derive(Component)]
pub struct Wall;
