use bevy::prelude::*;
use uuid::Uuid;

use crate::CellDetails;

#[derive(Clone)]
pub struct CellGenome {
    id: Uuid,
    kind: CellDetails,
    location: IVec2,
}
impl CellGenome {
    pub fn details(&self) -> &CellDetails {
        &self.kind
    }
    pub fn location(&self) -> IVec2 {
        self.location
    }
}
