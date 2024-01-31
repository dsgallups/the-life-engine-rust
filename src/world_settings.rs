use bevy::ecs::system::Resource;

#[derive(Resource, Debug, Clone)]
pub struct WorldSettings {
    pub producer_probability: u8,
    //every nth tick of an organism being alive, decrease its food consumed by 1
    pub hunger_tick: u64,
    pub spawn_radius: u64,
    pub max_organisms: Option<usize>,
    pub wall_length_half: Option<i64>,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            hunger_tick: 30,
            producer_probability: 5,
            spawn_radius: 15,
            max_organisms: None,
            wall_length_half: None,
        }
    }
}
