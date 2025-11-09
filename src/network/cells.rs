use crate::network::{CellTemplate, PartialNetworkTemplate};

#[derive(Default)]
pub struct EyeTemplate {}

impl CellTemplate for EyeTemplate {
    /// The eye feeds 3 data points into the network
    /// 1 - enemy is around
    /// 2 - enemy position x
    /// 3 - enemy position y
    fn template(&self) -> PartialNetworkTemplate {
        PartialNetworkTemplate::new(3, 0)
    }
}

#[derive(Default)]
pub struct BrainTemplate {}
impl CellTemplate for BrainTemplate {
    /// the brain feeds 2 data points into the network
    /// 1 - position x
    /// 2 - position y
    fn template(&self) -> PartialNetworkTemplate {
        PartialNetworkTemplate::new(2, 0)
    }
}

#[derive(Default)]
pub struct LauncherTemplate {}

impl CellTemplate for LauncherTemplate {
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
