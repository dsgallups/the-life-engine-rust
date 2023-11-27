use bevy::prelude::*;

use crate::life_engine::{Organ, Organism, OrganismCell};

use super::{Cell, Drawable, WorldContextResponse, WorldUpdateResponse};

#[derive(Resource)]
pub struct LEWorld {
    settings: WorldSettings,
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
            Organ::new(Producer(0), (-1, 1, 1).into()),
            Organ::new(Mouth, (0, 0, 1).into()),
            Organ::new(Producer(0), (1, -1, 1).into()),
        ];

        let first_organism =
            Organism::new(organs, ((width / 2) as u64, (height / 2) as u64, 1).into());
        LEWorld {
            settings: WorldSettings::default(),
            map,
            width,
            height,
            organisms: vec![first_organism],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
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

    /// world will provide the organism with a request for its context requirements
    /// given the requirements provided by the organism, the world will provide the organism with the information it knows
    /// the organism will then provide the world with a request to update the world
    /// the world will then provide the organism with a response to the request, as its request may not always be fulfilled
    /**
     * Example 1
     *
     * let requested_context = organism.context_request();
     *
     * let OrganismContextRequest { nearest_food } = requested_context;
     *
     * let context_response = if !nearest_food {
     *      WorldContextResponse { nearest_food: None }
     * } else {
     *      let mut nearest_food_loc = I64Vec3::MAX;
     *      let position = organism.origin();
     *      let mut nearest_distance = std::u64::MAX;
     *      for (x, col) in self.map.iter().enumerate() {
     *          for (y, cell) in col.iter().enumerate() {
     *              if let Cell::Inert(InertCell::Food) = cell {
     *                  let distance = (position.x - x as u64).pow(2) + (position.y - y as u64).pow(2);
     *                      if distance < nearest_distance {
     *                          let x = x as i64 - position.x as i64;
     *                          let y = y as i64 - position.y as i64;
     *                          nearest_distance = distance;
     *                          nearest_food_loc = (x, y, 1).into();
     *                      }
     *                  }
     *              }
     *          }
     *     }
     *      WorldContextResponse {
     *          nearest_food: Some(nearest_food_loc),
     *      }
     * };
     *
     * let _requested_update = organism.update_request(context_response);
     *
     * let response = WorldUpdateResponse {};
     *
     * organism.tick(response);
     */
    pub fn tick(&mut self) {
        for organism in self.organisms.iter_mut() {
            let _ctx_req = organism.context_request();
            let response = WorldContextResponse {};
            let _update_req = organism.update_request(response);

            let response = WorldUpdateResponse {};

            while let Some(_update_req) = organism.tick(&response) {
                //Maybe it just ate its own food or something? idk
                //self.update(update_req);
            }
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

pub struct WorldSettings {
    food_spawn_radius: u64,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            food_spawn_radius: 1,
        }
    }
}
