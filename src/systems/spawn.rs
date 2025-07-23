use crate::components::*;
use crate::physics::*;
use hecs::{Bundle, World};
use rand::{rngs::ThreadRng, Rng};
use rapier3d::prelude::*;
use raylib::prelude::*;

// Generate entities
fn generate_entities<F, B>(
    ecs_world: &mut World,
    positions: &mut Vec<Vector3>,
    physics_world: &mut PhysicsWorld,
    entity_count: u32,
    mut generator_fn: F,
) where
    F: FnMut(Vector3, &mut PhysicsWorld, &mut ThreadRng) -> (B, Option<ColliderHandle>),
    B: Bundle,
{
    //RNG
    let mut rng = rand::rng();

    // Num of entities generated
    let mut generated = 0;

    while generated < entity_count {
        // Generate random X and Z coords
        let x = rng.random_range(-25.0..=25.0);
        let z = rng.random_range(-25.0..=25.0);

        // Create position with generated coords
        let position = Vector3::new(x, 0.0, z);

        // Check if positions list already has an entity at these coords, if so skip this iteration
        // and move on to the next, otherwise carry on
        if positions
            .iter()
            .any(|pos| pos.x == position.x && pos.z == position.z)
        {
            continue;
        }

        // Add coords to positions list
        positions.push(position);

        // Create entity component bundle and collider
        // Exposes 'postition', 'physics_world', and 'rng' variables out to the closure
        let (entity_bundle, maybe_collider) = generator_fn(position, physics_world, &mut rng);

        // Spawn entity into the world
        let entity = ecs_world.spawn(entity_bundle);

        // If entity has a collider, set collider's user_data field to ECS entity ID
        if let Some(collider_handle) = maybe_collider {
            if let Some(collider) = physics_world.colliders.get_mut(collider_handle) {
                collider.user_data = entity.to_bits().get() as u128;
            }
        }

        // Increment num of entities generated
        generated += 1;
    }
}

// Generate player
pub fn generate_player(
    ecs_world: &mut World,
    positions: &mut Vec<Vector3>,
    physics_world: &mut PhysicsWorld,
) -> Vector3 {
    // Create player start position initialized to zero
    let mut player_start_position = Vector3::zero();

    generate_entities(
        ecs_world,
        positions,
        physics_world,
        1,
        |position, p_world, _| {
            // Set player start position to generated start position
            player_start_position = position;

            // Set player width and height
            let width = 1.0;
            let height = 2.0;

            // Create body
            let body = RigidBodyBuilder::dynamic()
                .translation(vector![position.x, height / 2.0, position.z])
                .lock_rotations()
                .linear_damping(4.0) // Slow down when keys are released
                .ccd_enabled(true)
                .build();

            // Insert body into physics world and get body handle
            let body_handle = p_world.bodies.insert(body);

            // Create collider
            let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0, width / 2.0).build();

            // Insert collider into physics world, attach it to body, and get collider handle
            let collider_handle =
                p_world
                    .colliders
                    .insert_with_parent(collider, body_handle, &mut p_world.bodies);

            // Create player
            let player = Player { body_handle };

            // Return component bundle and collider handle
            ((player, Nothing), Some(collider_handle))
        },
    );

    // Return player start position
    player_start_position
}

// Generate trees
pub fn generate_trees(
    ecs_world: &mut World,
    positions: &mut Vec<Vector3>,
    physics_world: &mut PhysicsWorld,
    num_of_trees: u32,
) {
    generate_entities(
        ecs_world,
        positions,
        physics_world,
        num_of_trees,
        |position, p_world, rng| {
            // Generate random tree size and color
            let leaf_width: f32 = rng.random_range(1.0..=3.0);
            let leaf_height = rng.random_range(2.0..=4.0);
            let trunk_height = rng.random_range(0.8..=1.5);
            let color_picker = rng.random_range(0..=1);

            // Apply tree color
            let (leaf_color, trunk_color) = match color_picker {
                0 => (Color::GREEN, Color::BROWN),
                1 => (Color::DARKGREEN, Color::DARKBROWN),
                _ => (Color::GRAY, Color::GRAY),
            };

            // Get tree size for collider
            let half_width = leaf_width.max(0.25) / 2.0;
            let total_height = trunk_height + leaf_height;

            // Create body
            let body = RigidBodyBuilder::fixed()
                .translation(vector![position.x, total_height / 2.0, position.z])
                .build();

            // Insert body into physics world and get body handle
            let body_handle = p_world.bodies.insert(body);

            // Create collider
            let collider =
                ColliderBuilder::cuboid(half_width, total_height / 2.0, half_width).build();

            // Insert collider into physics world, attach it to body, and get collider handle
            let collider_handle =
                p_world
                    .colliders
                    .insert_with_parent(collider, body_handle, &mut p_world.bodies);

            // Create tree
            let tree = Tree {
                leaf_width,
                leaf_height,
                trunk_height,
                body_handle,
                leaf_color,
                trunk_color,
            };

            // Return component bundle and collider handle
            ((tree, Nothing), Some(collider_handle))
        },
    );
}

// Generate balls
pub fn generate_balls(
    ecs_world: &mut World,
    positions: &mut Vec<Vector3>,
    physics_world: &mut PhysicsWorld,
    num_of_balls: u32,
) {
    generate_entities(
        ecs_world,
        positions,
        physics_world,
        num_of_balls,
        |position, p_world, _| {
            // Ball size
            let ball_size = 0.5;

            // Create body
            let body = RigidBodyBuilder::dynamic()
                .translation(vector![position.x, 10.0, position.z])
                .ccd_enabled(true)
                .build();

            // Insert body into physics world and get body handle
            let body_handle = p_world.bodies.insert(body);

            // Create collider
            let collider = ColliderBuilder::ball(ball_size)
                .density(1.0)
                .restitution(0.7)
                .build();

            // Insert collider into physics world, attach it to body, and get collider handle
            let collider_handle =
                p_world
                    .colliders
                    .insert_with_parent(collider, body_handle, &mut p_world.bodies);

            // Create ball
            let ball = crate::components::Ball {
                size: ball_size,
                body_handle,
                color: Color::BLUE,
            };

            // Return component bundle and collider handle
            ((ball, Nothing), Some(collider_handle))
        },
    );
}
