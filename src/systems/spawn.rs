use crate::components::*;
use crate::physics::*;
use crate::systems::ai::*;
use crate::world::grid::*;
use hecs::{Bundle, World};
use rand::seq::IndexedRandom;
use rand::{rngs::ThreadRng, Rng};
use rapier3d::prelude::*;
use raylib::prelude::*;

// Generate entities
fn generate_entities<F, B>(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    grid: &Grid,
    entity_count: u32,
    mut generator_fn: F,
) where
    F: FnMut(&mut GridCoord, &mut PhysicsWorld, &mut ThreadRng) -> (B, Option<ColliderHandle>),
    B: Bundle,
{
    //RNG
    let mut rng = rand::rng();

    // Num of entities generated
    let mut generated = 0;

    'outer: while generated < entity_count {
        // Generate random X and Z coords
        // At least 2.0 in from edge of World
        // Rounded up to match grid usize
        let x: f32 = rng.random_range(2.0..(grid.width as f32 - 2.0)).ceil();
        let y: f32 = 0.0;
        let z: f32 = rng.random_range(2.0..(grid.height as f32 - 2.0)).ceil();

        // Create position with generated coords
        let mut position = GridCoord {
            x: x as usize,
            y: y as usize,
            z: z as usize,
        };

        // Create entity component bundle and collider
        // Exposes 'postition', 'physics_world', and 'rng' variables out to the closure
        let (entity_bundle, maybe_collider) = generator_fn(&mut position, physics_world, &mut rng);

        // Check no bodies in the way at spawn position
        for (_, body_handle) in ecs_world.query::<&BodyHandle>().iter() {
            if let Some(body) = physics_world.bodies.get(body_handle.body_handle) {
                // Get body position from physics world
                let pos = body.translation();

                // Convert body positon to grid coordinates
                let converted_pos = GridCoord::from_rapier3d_vec(*pos);

                // Check and skip this spawn if blocked
                if position == converted_pos {
                    continue 'outer;
                }
            }
        }

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
    physics_world: &mut PhysicsWorld,
    grid: &Grid,
) -> GridCoord {
    // Create player start position initialized to zero
    let mut player_start_position = GridCoord::zero();

    generate_entities(ecs_world, physics_world, grid, 1, |position, p_world, _| {
        // Set player start position to generated start position
        player_start_position = *position;

        // Set player width and height
        let width = grid.tile_size;
        let height = grid.tile_size * 2.0;
        let depth = grid.tile_size;

        // Set spawn height
        position.y = 2;

        // Create body
        let body = RigidBodyBuilder::dynamic()
            .translation(position.to_rapier3d_vec(grid.tile_size))
            .lock_rotations()
            .linear_damping(4.0) // Slow down when keys are released
            .ccd_enabled(true)
            .build();

        // Insert body into physics world and create BodyHandle component
        let body_handle = BodyHandle {
            body_handle: p_world.bodies.insert(body),
        };

        // Create collider
        let collider =
            ColliderBuilder::round_cuboid(width / 2.0, height / 2.0, depth / 2.0, 0.1).build();

        // Insert collider into physics world, attach it to body, and get collider handle
        let collider_handle = p_world.colliders.insert_with_parent(
            collider,
            body_handle.body_handle,
            &mut p_world.bodies,
        );

        // Create Player component
        let player = Player {};

        // Return component bundle and collider handle
        ((player, body_handle), Some(collider_handle))
    });

    // Return player start position
    player_start_position
}

// Generate trees
pub fn generate_trees(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    grid: &Grid,
    num_of_trees: u32,
) {
    generate_entities(
        ecs_world,
        physics_world,
        grid,
        num_of_trees,
        |position, p_world, rng| {
            // Generate random tree size and color
            let even_widths = [1.0, 3.0];
            let leaf_width: f32 = *even_widths.choose(rng).unwrap();
            let leaf_height: f32 = rng.random_range(1.0..=8.0);
            let trunk_height: f32 = rng.random_range(1.0..=2.0);
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

            // Set spawn height
            position.y = 0;

            // Create body
            // Center the collider vertically
            let body = RigidBodyBuilder::fixed()
                .translation(position.to_rapier3d_vec_new(
                    position.x,
                    (total_height / 2.0) as usize,
                    position.z,
                    grid.tile_size,
                ))
                .build();

            // Insert body into physics world and create BodyHandle component
            let body_handle = BodyHandle {
                body_handle: p_world.bodies.insert(body),
            };

            // Create collider
            let collider =
                ColliderBuilder::cuboid(half_width, total_height / 2.0, half_width).build();

            // Insert collider into physics world, attach it to body, and get collider handle
            let collider_handle = p_world.colliders.insert_with_parent(
                collider,
                body_handle.body_handle,
                &mut p_world.bodies,
            );

            // Create Tree component
            let tree = Tree {
                leaf_width,
                leaf_height,
                trunk_height,
                leaf_color,
                trunk_color,
            };

            // Return component bundle and collider handle
            ((tree, body_handle), Some(collider_handle))
        },
    );
}

// Generate balls
pub fn generate_balls(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    grid: &Grid,
    num_of_balls: u32,
) {
    generate_entities(
        ecs_world,
        physics_world,
        grid,
        num_of_balls,
        |position, p_world, _| {
            // Ball size
            let ball_size = 0.5;

            // Set spawn height
            position.y = 10;

            // Create body
            let body = RigidBodyBuilder::dynamic()
                .translation(position.to_rapier3d_vec(grid.tile_size))
                .ccd_enabled(true)
                .build();

            // Insert body into physics world and create BodyHandle component
            let body_handle = BodyHandle {
                body_handle: p_world.bodies.insert(body),
            };

            // Create collider
            let collider = ColliderBuilder::ball(ball_size)
                .density(1.0)
                .restitution(0.7)
                .build();

            // Insert collider into physics world, attach it to body, and get collider handle
            let collider_handle = p_world.colliders.insert_with_parent(
                collider,
                body_handle.body_handle,
                &mut p_world.bodies,
            );

            // Create Ball component
            let ball = crate::components::Ball {
                size: ball_size,
                color: Color::BLUE,
            };

            // Return component bundle and collider handle
            ((ball, body_handle), Some(collider_handle))
        },
    );
}

// Generate witches
pub fn generate_witches(
    ecs_world: &mut World,
    physics_world: &mut PhysicsWorld,
    grid: &Grid,
    num_of_witches: u32,
) {
    generate_entities(
        ecs_world,
        physics_world,
        grid,
        num_of_witches,
        |position, p_world, _| {
            // Set witch width and height
            let width = grid.tile_size;
            let height = grid.tile_size * 2.0;
            let depth = grid.tile_size;

            // Set spawn height
            position.y = 2;

            // Create body
            let body = RigidBodyBuilder::dynamic()
                .translation(position.to_rapier3d_vec(grid.tile_size))
                .ccd_enabled(true)
                .build();

            // Insert body into physics world and create BodyHandle component
            let body_handle = BodyHandle {
                body_handle: p_world.bodies.insert(body),
            };

            // Create collider
            let collider =
                ColliderBuilder::round_cuboid(width / 2.0, height / 2.0, depth / 2.0, 0.1).build();

            // Insert collider into physics world, attach it to body, and get collider handle
            let collider_handle = p_world.colliders.insert_with_parent(
                collider,
                body_handle.body_handle,
                &mut p_world.bodies,
            );

            // Create witch
            let witch = Witch {
                width,
                height,
                collider_handle,
                color: Color::PURPLE,
                state: WitchState::Patrolling,
                target: generate_patrol_point(),
            };

            // Return component bundle and collider handle
            ((witch, body_handle), Some(collider_handle))
        },
    );
}
