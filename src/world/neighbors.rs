use bevy::math::I64Vec2;

pub const NEIGHBORS: Neighbors = Neighbors {
    adjacent: [
        I64Vec2::new(0, 1),
        I64Vec2::new(0, -1),
        I64Vec2::new(1, 0),
        I64Vec2::new(-1, 0),
    ],
    all: [
        I64Vec2::new(0, 1),
        I64Vec2::new(0, -1),
        I64Vec2::new(1, 0),
        I64Vec2::new(-1, 0),
        I64Vec2::new(-1, -1),
        I64Vec2::new(1, 1),
        I64Vec2::new(-1, 1),
        I64Vec2::new(1, -1),
    ],
    all_self: [
        I64Vec2::new(0, 1),
        I64Vec2::new(0, -1),
        I64Vec2::new(1, 0),
        I64Vec2::new(-1, 0),
        I64Vec2::new(-1, -1),
        I64Vec2::new(1, 1),
        I64Vec2::new(-1, 1),
        I64Vec2::new(1, -1),
        I64Vec2::new(0, 0),
    ],
    corners: [
        I64Vec2::new(-1, -1),
        I64Vec2::new(1, 1),
        I64Vec2::new(-1, 1),
        I64Vec2::new(1, -1),
    ],
};

pub struct Neighbors {
    pub all: [I64Vec2; 8],
    pub all_self: [I64Vec2; 9],
    pub adjacent: [I64Vec2; 4],
    pub corners: [I64Vec2; 4],
}
