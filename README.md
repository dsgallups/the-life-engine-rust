# the-life-engine-rust

A port of emergent garden's [The Life Engine](https://thelifeengine.net/)

## Issues

Because the "cells" of an organism only need to check the immediate neighbor locations,
I pass around essentially a hashmap that allows one to access an organism entity (or food entity)

to check its neighbors very quickly.

Unfortunately, this hashmap quickly moves out of sync with the ECS, and I'm unsure why.

See `src/environment/locations` for the [`OccupiedLocations`] newtype hashmap

and `src/cell/mover.rs:74` for the area of panic and proof of non-sync

### Questions

- Is it possible for me to keep this method of resource access, or is there some way to query (in the ecs system) for an entity at a particular location?

- If I can only use the newtype hashmap, what is a better way of implementing this check so I don't have to check neighbors in O(n^2) time (where n is cells)?

Genuinely any help is greatly appreciated. Thanks!!
