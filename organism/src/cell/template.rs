use bevy::math::IVec2;
use uuid::Uuid;

use crate::{CellKind, genome::PartialNetworkTemplate};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PartialCellGenome {
    id: Uuid,
    kind: CellKind,
    position: IVec2,
}

impl PartialCellGenome {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn kind(&self) -> CellKind {
        self.kind
    }
    pub fn position(&self) -> IVec2 {
        self.position
    }
}
pub struct CellTemplate {
    position: IVec2,
    template_kind: CellTemplateKind,
}

impl CellTemplate {
    pub fn template(&self) -> PartialNetworkTemplate {
        self.template_kind.template()
    }
    pub fn into_genome(self) -> PartialCellGenome {
        let id = Uuid::new_v4();
        PartialCellGenome {
            id,
            kind: self.template_kind.into_kind(),
            position: self.position,
        }
    }
}

pub enum CellTemplateKind {
    Eye(EyeGenome),
    Brain(BrainGenome),
    Launcher(LauncherTemplate),
}
impl CellTemplateKind {
    pub fn template(&self) -> PartialNetworkTemplate {
        match self {
            Self::Brain(b) => b.template(),
            Self::Eye(e) => e.template(),
            Self::Launcher(l) => l.template(),
        }
    }

    pub fn into_kind(self) -> CellKind {
        match self {
            Self::Brain(_) => CellKind::Brain,
            Self::Eye(_) => CellKind::Eye,
            Self::Launcher(_) => CellKind::Launcher,
        }
    }
}

#[derive(Default)]
pub struct EyeGenome {}

impl EyeGenome {
    /// The eye feeds 3 data points into the network
    /// 1 - enemy is around
    /// 2 - enemy position x
    /// 3 - enemy position y
    fn template(&self) -> PartialNetworkTemplate {
        PartialNetworkTemplate::new(3, 0)
    }
}

#[derive(Default)]
pub struct BrainGenome {}

impl BrainGenome {
    /// the brain feeds 2 data points into the network
    /// 1 - position x
    /// 2 - position y
    fn template(&self) -> PartialNetworkTemplate {
        PartialNetworkTemplate::new(2, 0)
    }
}

#[derive(Default)]
pub struct LauncherTemplate {}

impl LauncherTemplate {
    /// the launcher doesn't feed any data into the network.
    ///
    /// the launcher receives 3 data points from the network:
    /// 1 - cosine
    /// 2 - sine
    /// 3 - Fire (threshold)
    fn template(&self) -> PartialNetworkTemplate {
        PartialNetworkTemplate::new(0, 3)
    }
}
