use bevy::{ecs::component::Component, render::color::Color};

pub trait Drawable {
    fn color(&self) -> Color;
}

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Wall;
