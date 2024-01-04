# tile_collision_example_xpbd

![image](https://github.com/richchurcher/tile_collision_example_xpbd/assets/171905/ccd68b41-c6ee-4abc-9fe0-81c4615d743d)

Demonstrates use of colliders with [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap), as discussed in [this issue](https://github.com/StarArawn/bevy_ecs_tilemap/issues/504). Note that tile entities must be children of the tilemap entity for this to work as expected. The [`fill_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap/blob/b08a5d997867d5e5a760e296dc38e435c6b268cc/src/helpers/filling.rs#L11) helper does this, but does not itself allow the created entities to be supplemented with e.g. `RigidBody`, colliders and transforms.
