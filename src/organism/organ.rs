use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec3,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{direction::Direction, map::WorldLocation, Drawable};

#[derive(Clone, Component, Debug, Default, PartialEq)]
pub struct Producer {
    pub food_produced: u8,
    pub counter: u8,
}

impl Producer {
    pub fn new() -> Producer {
        Producer {
            food_produced: 0,
            counter: 0,
        }
    }
}

#[derive(Component)]
pub struct Mouth;

#[derive(Component)]
pub struct Mover;

#[derive(Component)]
pub struct Killer;

#[derive(Component)]
pub struct Armor;

#[derive(Component)]
pub struct Eye(Direction);

impl Drawable for Producer {
    fn color(&self) -> Color {
        Color::GREEN
    }
}

impl Drawable for Mouth {
    fn color(&self) -> Color {
        Color::ORANGE
    }
}

impl Drawable for Mover {
    fn color(&self) -> Color {
        Color::AQUAMARINE
    }
}

impl Drawable for Killer {
    fn color(&self) -> Color {
        Color::RED
    }
}

impl Drawable for Armor {
    fn color(&self) -> Color {
        Color::PURPLE
    }
}

impl Drawable for Eye {
    fn color(&self) -> Color {
        Color::SALMON
    }
}

#[derive(Clone, Bundle)]
pub struct OrganBundle<T: Component> {
    pub sprite: SpriteBundle,
    pub organ_type: T,
    pub relative_location: WorldLocation,
}

impl<T: Component + Drawable> OrganBundle<T> {
    pub fn new(organ_type: T, relative_location: impl Into<WorldLocation>) -> Self {
        let relative_location: WorldLocation = relative_location.into();
        OrganBundle {
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    relative_location.x() as f32,
                    relative_location.y() as f32,
                    0.,
                )),
                sprite: Sprite {
                    color: organ_type.color(),
                    ..default()
                },
                ..default()
            },
            organ_type,
            relative_location,
        }
    }
}
