use crate::components::*;
use crate::physics::*;
use crate::settings::*;
use hecs::World;
use rapier3d::prelude::*;
use raylib::prelude::*;

// Generate map
pub fn generate_map(ecs_world: &mut World, physics_world: &mut PhysicsWorld) {
    // Generate ground
    let ground_position = Vector3::new(GROUND_POS_X, -GROUND_POS_Y, GROUND_POS_Z);
    let ground_size = Vector3::new(GROUND_SIZE_X, GROUND_SIZE_Y, GROUND_SIZE_Z);
    generate_ground(ecs_world, physics_world, ground_position, ground_size);

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
        generate_wall(ecs_world, physics_world, pos, size);
    }
}

// Generate ground
fn generate_ground(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    position: Vector3,
    size: Vector3,
) {
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

    // Create ground
    let ground = Ground { body_handle };

    // Spawn entity into the world
    let entity = ecs_world.spawn((ground, Nothing));

    // Set collider's user_data field to ECS entity ID
    if let Some(collider) = physics_world.colliders.get_mut(collider_handle) {
        collider.user_data = entity.to_bits().get() as u128;
    }
}

// Generate walls
fn generate_wall(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    position: Vector3,
    size: Vector3,
) {
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

    // Create wall
    let wall = Wall { body_handle };

    // Spawn entity into the world
    let entity = ecs_world.spawn((wall, Nothing));

    // Set collider's user_data field to ECS entity ID
    if let Some(collider) = physics_world.colliders.get_mut(collider_handle) {
        collider.user_data = entity.to_bits().get() as u128;
    }
}
