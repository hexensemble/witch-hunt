use crate::components::*;
use crate::physics::PhysicsWorld;
use hecs::World;
use rand::Rng;
use rapier3d::prelude as rap3d;
use rapier3d::prelude::*;
use raylib::prelude as rl;
use raylib::prelude::*;

// Update witch AI
pub fn update_witch_ai(ecs_world: &mut World, physics_world: &mut PhysicsWorld) -> bool {
    // Set 'game over' flag to false
    let mut game_over = false;

    // Get player position
    let player_position = {
        if let Some((_, player)) = ecs_world.query::<&Player>().iter().next() {
            if let Some(player_body) = physics_world.bodies.get(player.body_handle) {
                *player_body.translation()
            } else {
                eprintln!("Couldn't find Player while attempting call: update_witch_ai. Did player spawn?");
                panic!();
            }
        } else {
            eprintln!(
                "Couldn't find Player while attempting call: update_witch_ai. Did player spawn?"
            );
            panic!();
        }
    };

    for (_, witch) in ecs_world.query::<&mut Witch>().iter() {
        // Get witch position
        let witch_position = match physics_world.bodies.get(witch.body_handle) {
            Some(body) => *body.translation(),
            None => continue,
        };

        // Set 'should chase' flag to false
        let mut should_chase = false;

        // Get direction from witch to player
        let direction_to_player = player_position - witch_position;

        // Get distance from witch to player
        // .norm() returns the length (magnitude) of the direction vector
        let distance_to_player = direction_to_player.norm();

        // Get direction without a length
        let direction_to_player_normalized = direction_to_player / distance_to_player;

        // Create a ray starting at witch postion and in normalized direction of player
        let ray = rapier3d::parry::query::Ray::new(
            point![witch_position.x, witch_position.y, witch_position.z],
            direction_to_player_normalized,
        );

        // Filter out things we don't need to check
        let filter = QueryFilter::default().exclude_collider(witch.collider_handle);

        // Run query to see if ray collides with anything and return a hit_handle of that object
        if let Some((hit_handle, _)) = physics_world.query_pipeline.cast_ray(
            &physics_world.bodies,
            &physics_world.colliders,
            &ray,
            distance_to_player,
            true,
            filter,
        )
        // If there's a collision was it the player?
        {
            if let Some(collider) = physics_world.colliders.get(hit_handle) {
                let hit_entity = hecs::Entity::from_bits(collider.user_data as u64);

                // If the hit entity was the player then witch should chase
                if ecs_world.get::<&Player>(hit_entity.expect("")).is_ok() {
                    should_chase = true;
                }
            }
        }
        // If no collision then witch should chase player
        else {
            should_chase = true;
        }

        // Switch state check
        match (should_chase, &witch.state) {
            // If 'should chase' flag is true and witch is currently patrolling
            (true, WitchState::Patrolling) => {
                println!("👁️ Witch spotted the player. Switching to chase.");
                witch.state = WitchState::Chasing;
            }
            // If 'should chase' flag is false and witch is currently chasing
            (false, WitchState::Chasing) => {
                println!("🤫 Witch resumes patrol.");
                witch.state = WitchState::Patrolling;
                witch.target = generate_patrol_point();
            }
            _ => {}
        }

        // Movement logic
        // Get witch body handle
        if let Some(witch_body) = physics_world.bodies.get_mut(witch.body_handle) {
            // Get target based on witch state
            let target = match witch.state {
                WitchState::Chasing => player_position,
                WitchState::Patrolling => raylib_vec_to_rapier_vec(witch.target),
            };

            // Get direction from witch to target
            let direction_to_target = target - witch_position;

            // Get distance from witch to target
            // .norm() returns the length (magnitude) of the direction vector
            let distance_to_target = direction_to_target.norm();

            // If target is more than 1.0 away then head towards it
            if distance_to_target > 1.0 {
                // Get direction without a length
                let direction_to_target_normalized = direction_to_target / distance_to_target;

                // Set speed based on witch state
                let speed = match witch.state {
                    WitchState::Chasing => 3.0,
                    WitchState::Patrolling => 5.0,
                };

                // Set witch in motion
                witch_body.set_linvel(direction_to_target_normalized * speed, true);
            }
            // Otherwise (and if witch is in Patrolling state) pick a new point
            else if matches!(witch.state, WitchState::Patrolling) {
                witch.target = generate_patrol_point();
                println!("🚶 Witch picked new patrol point: {:?}", witch.target);
            }

            // If witch is in chasing state and it's reached the target, game over
            if distance_to_target < 1.0 && matches!(witch.state, WitchState::Chasing) {
                game_over = true;
            }
        }
    }

    game_over
}

// Generate a random point for witch to patrol to
pub fn generate_patrol_point() -> Vector3 {
    let mut rng = rand::rng();
    Vector3::new(
        rng.random_range(-10.0..=10.0),
        0.0,
        rng.random_range(-10.0..=10.0),
    )
}

// Convert a Raylib Vector3 to a Rapier Vector<f32>
pub fn raylib_vec_to_rapier_vec(v: rl::Vector3) -> rap3d::Vector<f32> {
    rap3d::vector![v.x, v.y, v.z]
}
