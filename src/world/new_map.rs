pub struct NewWorldMap(FxHashMap<I64Vec2, Cell>);

impl NewWorldMap {
    pub fn new() -> Self {
        Self(FxHashMap::default())
    }

    pub fn insert_organism(&mut self, organism: Organism) {
        todo!();
    }
}
