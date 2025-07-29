use crate::components::*;
use crate::physics::*;
use crate::settings::*;
use hecs::{Bundle, World};
use rapier3d::prelude::*;
use raylib::prelude::*;

// Generate terrain
pub fn generate_terrain(ecs_world: &mut World, physics_world: &mut PhysicsWorld) {
    // Generate ground
    let ground_position = Vector3::new(GROUND_POS_X, -GROUND_POS_Y, GROUND_POS_Z);
    let ground_size = Vector3::new(GROUND_SIZE_X, GROUND_SIZE_Y, GROUND_SIZE_Z);
    generate_terrain_piece(
        ecs_world,
        physics_world,
        ground_position,
        ground_size,
        |body_handle| {
            let ground = Ground { body_handle };

            (ground, Nothing)
        },
    );

    // Generate walls
    let walls = [
        // West
        (
            Vector3::new(-WALL_POS_Z, WALL_POS_Y, WALL_POS_X),
            Vector3::new(WALL_SIZE_X, WALL_SIZE_Y, WALL_SIZE_Z),
        ),
        // East
        (
            Vector3::new(WALL_POS_Z, WALL_POS_Y, WALL_POS_X),
            Vector3::new(WALL_SIZE_X, WALL_SIZE_Y, WALL_SIZE_Z),
        ),
        // South
        (
            Vector3::new(WALL_POS_X, WALL_POS_Y, WALL_POS_Z),
            Vector3::new(WALL_SIZE_Z, WALL_SIZE_Y, WALL_SIZE_X),
        ),
        // North
        (
            Vector3::new(WALL_POS_X, WALL_POS_Y, -WALL_POS_Z),
            Vector3::new(WALL_SIZE_Z, WALL_SIZE_Y, WALL_SIZE_X),
        ),
    ];

    for (pos, size) in walls {
        generate_terrain_piece(ecs_world, physics_world, pos, size, |body_handle| {
            let wall = Wall { body_handle };

            (wall, Nothing)
        });
    }
}

// Generate terrain peice
fn generate_terrain_piece<F, B>(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    position: Vector3,
    size: Vector3,
    mut generator_function: F,
) where
    F: FnMut(RigidBodyHandle) -> B,
    B: Bundle,
{
    // Create body
    let body = RigidBodyBuilder::fixed()
        .translation(vector![
            position.x / 2.0,
            position.y / 2.0,
            position.z / 2.0
        ])
        .build();

    // Insert body into physics world and get body handle
    let body_handle = physics_world.bodies.insert(body);

    // Create collider
    let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0).build();

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
