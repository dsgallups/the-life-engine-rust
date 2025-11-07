use bevy::camera::visibility::RenderLayers;
use bitflags::bitflags;

bitflags! {
    pub struct RenderLayer: u32 {
        /// Implicitly used without identifying a render layer component
        const DEFAULT = 0b0000_0001;
        /// Used specifically for a camera that renders a UI texture
        const CELL_VISUAL = 0b0000_0010;
        /// Gizmos (debug info)
        const GIZMO = 0b0000_0100;
    }
}

impl From<RenderLayer> for RenderLayers {
    fn from(value: RenderLayer) -> Self {
        // Render layers are vectors of ints, so each active bit is converted to an int.
        RenderLayers::from_iter(value.iter().map(|l| (l.bits() >> 1) as usize))
    }
}
