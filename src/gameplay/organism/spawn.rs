use bevy::prelude::*;

use crate::gameplay::genome::Genome;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnOrganism>();

    //todo
}
fn spawn_organisms(mut events: EventReader<SpawnOrganism>, mut commands: Commands) {
    //todo
}

#[derive(Event)]
pub struct SpawnOrganism {
    genome: Handle<Genome>,
    location: IVec2,
}
impl SpawnOrganism {
    pub fn new(genome: Handle<Genome>, location: IVec2) -> Self {
        Self { genome, location }
    }
}
