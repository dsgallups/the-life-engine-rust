use crate::genome::PartialNetworkTemplate;

pub enum CellTemplate {
    Eye(EyeGenome),
    Brain(BrainGenome),
    Launcher(LauncherTemplate),
}
impl CellTemplate {
    pub fn template(&self) -> PartialNetworkTemplate {
        match self {
            Self::Brain(b) => b.template(),
            Self::Eye(e) => e.template(),
            Self::Launcher(l) => l.template(),
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
