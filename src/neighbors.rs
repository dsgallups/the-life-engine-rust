use crate::map::WorldLocation;

pub const NEIGHBORS: Neighbors = Neighbors {
    adjacent: [
        WorldLocation::new(0, 1),
        WorldLocation::new(0, -1),
        WorldLocation::new(1, 0),
        WorldLocation::new(-1, 0),
    ],
    all: [
        WorldLocation::new(0, 1),
        WorldLocation::new(0, -1),
        WorldLocation::new(1, 0),
        WorldLocation::new(-1, 0),
        WorldLocation::new(-1, -1),
        WorldLocation::new(1, 1),
        WorldLocation::new(-1, 1),
        WorldLocation::new(1, -1),
    ],
    all_self: [
        WorldLocation::new(0, 1),
        WorldLocation::new(0, -1),
        WorldLocation::new(1, 0),
        WorldLocation::new(-1, 0),
        WorldLocation::new(-1, -1),
        WorldLocation::new(1, 1),
        WorldLocation::new(-1, 1),
        WorldLocation::new(1, -1),
        WorldLocation::new(0, 0),
    ],
    corners: [
        WorldLocation::new(-1, -1),
        WorldLocation::new(1, 1),
        WorldLocation::new(-1, 1),
        WorldLocation::new(1, -1),
    ],
};

pub struct Neighbors {
    pub all: [WorldLocation; 8],
    pub all_self: [WorldLocation; 9],
    pub adjacent: [WorldLocation; 4],
    pub corners: [WorldLocation; 4],
}
