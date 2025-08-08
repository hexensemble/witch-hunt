use crate::components::*;
use crate::physics::*;
use crate::world::grid::*;
use hecs::{Bundle, World};
use rapier3d::prelude::*;
use raylib::prelude::*;

// Generate blocks
pub fn generate_blocks(ecs_world: &mut World, physics_world: &mut PhysicsWorld, grid: &Grid) {
    for row in &grid.tiles {
        for tile in row {
            match tile.kind {
                TileType::Grass => {
                    generate_block(ecs_world, physics_world, grid, tile, |body_handle| {
                        let grass = Block {
                            width: grid.tile_size,
                            height: grid.tile_size,
                            color: Color::LIMEGREEN,
                        };

                        let body_handle = BodyHandle { body_handle };

                        (grass, body_handle)
                    });
                }
                TileType::Stone => {
                    generate_block(ecs_world, physics_world, grid, tile, |body_handle| {
                        let stone = Block {
                            width: grid.tile_size,
                            height: grid.tile_size,
                            color: Color::DARKGRAY,
                        };

                        let body_handle = BodyHandle { body_handle };

                        (stone, body_handle)
                    });
                }
                _ => {}
            }
        }
    }
}

// Generate block
fn generate_block<F, B>(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    grid: &Grid,
    tile: &Tile,
    mut generator_function: F,
) where
    F: FnMut(RigidBodyHandle) -> B,
    B: Bundle,
{
    // Create body
    let body = RigidBodyBuilder::fixed()
        .translation(tile.coord.to_rapier3d_vec(grid.tile_size))
        .build();

    // Insert body into physics world and get body handle
    let body_handle = physics_world.bodies.insert(body);

    // Create collider
    let collider = ColliderBuilder::cuboid(
        grid.tile_size / 2.0,
        grid.tile_size / 2.0,
        grid.tile_size / 2.0,
    )
    .build();

    // Insert collider into physics world, attach it to body, and get collider handle
    let collider_handle = physics_world.colliders.insert_with_parent(
        collider,
        body_handle,
        &mut physics_world.bodies,
    );

    // Create entity component bundle
    // Exposes 'body_handle' variable out to the closure
    let bundle = generator_function(body_handle);

    // Spawn entity into the world
    let entity = ecs_world.spawn(bundle);

    // Set collider's user_data field to ECS entity ID
    if let Some(collider) = physics_world.colliders.get_mut(collider_handle) {
        collider.user_data = entity.to_bits().get() as u128;
    }
}
