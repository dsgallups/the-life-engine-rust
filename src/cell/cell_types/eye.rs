use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    cell::{CellOf, Cells},
    cpu_net::Cell,
    food::Food,
    organism::{Organism, OrganismSet},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_outputs.in_set(OrganismSet::ProcessInput));
}

#[derive(Component, Default)]
pub struct Eye {}

// fn eye_shape_caster() -> ShapeCaster {
//     let mut caster = ShapeCaster::new(Collider::circle(50.), Vec2::ZERO, 0., Dir2::X);

//     caster.ignore_self = true;

//     todo!()
// }

fn update_outputs(
    eyes: Query<(&Eye, &CellOf, &Cell, &GlobalTransform)>,
    spatial_query: SpatialQuery,
    food: Query<&Transform, With<Food>>,
    organisms: Query<&Cells>,
    transforms: Query<&Transform>,
    cell_transforms: Query<&Transform, With<CellOf>>,
) {
    for (eye, cell_of, cell, transform) in eyes {
        let transform = transform.compute_transform();

        let cells = organisms.get(cell_of.0).unwrap();

        let mut filter = SpatialQueryFilter::DEFAULT;
        for cell in cells.cells() {
            filter.excluded_entities.insert(*cell);
        }

        let collisions = spatial_query.shape_intersections(
            &Collider::circle(50.),
            Vec2::new(transform.translation.x, transform.translation.y),
            0.,
            &filter,
        );

        let mut x = 0.;
        let mut y = 0.;
        let mut t_val = 0.;
        // x
        // y
        // -1 if thing
        // 1 if food
        // Note: this isn't ordered by distance :)
        for collision in collisions {
            if let Ok(trns) = transforms.get(collision) {
                let maybe_x = trns.translation.x - transform.translation.x;
                let maybe_y = trns.translation.y - transform.translation.y;

                if maybe_x * maybe_y < x * y {
                    x = maybe_x;
                    y = maybe_y;

                    if food.get(collision).is_ok() {
                        t_val = 1.;
                    } else if cell_transforms.get(collision).is_ok() {
                        t_val = -1.;
                    }
                }
            }
        }

        cell.set(0, x);
        cell.set(1, y);
        cell.set(2, t_val);
    }

    //todo
}
