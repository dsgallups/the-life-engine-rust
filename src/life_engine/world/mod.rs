use bevy::{prelude::*, utils::Uuid};

use crate::life_engine::{Organ, Organism, OrganismCell};

use super::{Cell, Drawable, TickResponse};

#[derive(Resource)]
pub struct LEWorld {
    map: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    organisms: Vec<Organism>,
}

impl Default for LEWorld {
    fn default() -> Self {
        let default_width = 40;
        let default_height = 40;
        LEWorld::new(default_width, default_height)
    }
}

impl LEWorld {
    pub fn new(width: usize, height: usize) -> LEWorld {
        let map = vec![vec![Cell::default(); width]; height];

        pub use OrganismCell::*;

        let organs = vec![
            Organ::new(Producer, (-1, 1, 1).into()),
            Organ::new(Mouth, (0, 0, 1).into()),
            Organ::new(Producer, (1, -1, 1).into()),
        ];

        let first_organism =
            Organism::new(organs, ((width / 2) as u64, (height / 2) as u64, 1).into());
        LEWorld {
            map,
            width,
            height,
            organisms: vec![first_organism],
        }
    }

    pub fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn organisms(&self) -> &[Organism] {
        &self.organisms
    }

    pub fn refresh_map(&mut self) {
        for organism in self.organisms.iter() {
            let position = organism.origin();
            for organ in organism.organs() {
                println!("organ_position = {:?}", organ.position());
                println!("position = {:?}", position);
                let position = (*organ.position() + (*position).as_i64vec3()).as_u64vec3();
                println!("position = {:?}", position);
                self.map[position.x as usize][position.y as usize] = Cell::Organism(organ.cell());
            }
        }
    }

    pub fn tick(&mut self) {
        for organism in self.organisms.iter_mut() {
            let request = organism.update_request();
            let response = TickResponse {};
            organism.tick_response(response);
        }
    }

    pub fn draw(&self, commands: &mut Commands) {
        let map = &self.map;

        for (x, col) in map.iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                let x = x as f32;
                let y = y as f32;
                let color = cell.color();

                commands.spawn(SpriteBundle {
                    sprite: Sprite { color, ..default() },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                });
            }
        }
    }
}

#[derive(Component)]
pub enum ItemType {
    Organism(Uuid),
}
