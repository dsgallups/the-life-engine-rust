use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::{I64Vec2, Vec3},
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};
use rand::Rng;
use uuid::Uuid;

use crate::{direction::Direction, map::WorldLocation, world_settings::WorldSettings, Drawable};

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Component)]
pub enum OrganType {
    Mouth,
    Producer(Producer),
    Mover,
    Killer,
    Armor,
    Eye(Direction),
}

impl OrganType {
    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=5) {
            0 => Self::Mouth,
            1 => Self::Producer(Producer::default()),
            2 => Self::Mover,
            3 => Self::Killer,
            4 => Self::Armor,
            5 => Self::Eye(Direction::rand()),
            _ => panic!(),
        }
    }

    pub fn new_producer() -> Self {
        Self::Producer(Producer::default())
    }
}

impl Default for OrganType {
    fn default() -> Self {
        OrganType::Producer(Producer::default())
    }
}

impl Drawable for OrganType {
    fn color(&self) -> Color {
        match self {
            OrganType::Producer(_) => Color::GREEN,
            OrganType::Mouth => Color::ORANGE,
            OrganType::Mover => Color::AQUAMARINE,
            OrganType::Killer => Color::RED,
            OrganType::Armor => Color::PURPLE,
            OrganType::Eye(_) => Color::SALMON,
        }
    }
}

#[derive(Clone, Bundle)]
pub struct OrganBundle {
    pub sprite: SpriteBundle,
    pub organ_type: OrganType,
    pub relative_location: WorldLocation,
}

impl OrganBundle {
    pub fn new(organ_type: OrganType, relative_location: impl Into<WorldLocation>) -> Self {
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Organ {
    pub id: Uuid,
    pub r#type: OrganType,
    pub relative_location: I64Vec2,
}

/// The vectors in here are relatively positioned to the center of the organism
pub enum OrganEvent {
    MakeFoodAround(I64Vec2),
    EatFoodAround(I64Vec2),
    KillAround(I64Vec2),
}

impl Organ {
    pub fn new(r#type: OrganType, relative_location: I64Vec2) -> Organ {
        Organ {
            id: Uuid::new_v4(),
            r#type,
            relative_location,
        }
    }

    pub fn new_rand(relative_location: I64Vec2) -> Organ {
        Organ {
            id: Uuid::new_v4(),
            r#type: OrganType::new_rand(),
            relative_location,
        }
    }

    pub fn mutate(&mut self) {
        self.r#type = OrganType::new_rand();
    }

    pub fn organ_type(&self) -> &OrganType {
        &self.r#type
    }
    pub fn color(&self) -> Color {
        self.r#type.color()
    }

    pub fn tick(&mut self, world_settings: &WorldSettings) -> Option<OrganEvent> {
        let mut rng = rand::thread_rng();
        match self.r#type {
            OrganType::Producer(ref mut producer) => {
                if rng.gen_range(0..=100) < world_settings.producer_probability {
                    producer.counter += 1;
                    return Some(OrganEvent::MakeFoodAround(self.relative_location));
                }

                None
            }
            OrganType::Mouth => Some(OrganEvent::EatFoodAround(self.relative_location)),

            OrganType::Killer => Some(OrganEvent::KillAround(self.relative_location)),
            _ => None,
        }
    }
}
