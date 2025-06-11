use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CellType>()
        .register_type::<Cell>()
        .register_type::<Genome>();
    app.init_asset::<Genome>();
    //todo
}

#[derive(Component)]
pub struct GenomeHandle(pub Handle<Genome>);

#[derive(Asset, Reflect, Clone, Debug, PartialEq)]
pub struct Genome {
    cells: Vec<Cell>,
    mutation_rate: f64,
}

impl Genome {
    fn new(cells: Vec<Cell>, mutation_rate: f64) -> Self {
        Self {
            cells,
            mutation_rate,
        }
    }
    pub fn first_organism() -> Self {
        Self::new(
            vec![
                Cell::new(CellType::Producer, IVec2::new(-1, -1)),
                Cell::new(CellType::Mouth, IVec2::new(0, 0)),
                Cell::new(CellType::Producer, IVec2::new(1, 1)),
            ],
            50.,
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
struct Cell {
    cell_type: CellType,
    /// this is relative
    location: IVec2,
}
impl Cell {
    pub fn new(cell_type: CellType, location: IVec2) -> Self {
        Self {
            cell_type,
            location,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
enum CellType {
    Armor,
    Eye,
    Killer,
    Mover,
    Producer,
    Mouth,
}
