use crate::components::*;
use crate::physics::*;
use crate::settings::*;
use crate::systems::ai::*;
use crate::systems::drawing::*;
use crate::systems::player::*;
use crate::systems::spawn::*;
use crate::systems::terrain::*;
use crate::world::grid::*;
use crate::world::loader::*;
use crate::State;
use hecs::World;
use raylib::prelude::*;

// Game
pub struct Game {
    pub ecs_world: World,
    pub physics_world: PhysicsWorld,
    pub camera: Camera3D,
    pub mouse_look: MouseLook,
}

// Start a new game
pub fn new_game() -> Game {
    // Create camera
    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 2.0, 4.0),
        Vector3::new(0.0, 2.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );

    // Create mouse look
    let mouse_look = MouseLook::new(MOUSE_SENSITIVITY);

    // Create ECS world
    let mut ecs_world = World::new();

    // Create physics world
    let mut physics_world = PhysicsWorld::new();

    // Load tile map and get grid
    match load_tiled_map("map_01.tmx") {
        Ok(mut grid) => {
            if DEBUG_MODE {
                println!("Map loaded successfully!");
            }

            // Generate blocks
            generate_blocks(&mut ecs_world, &mut physics_world, &grid);

            // Generate player
            let player_start_position = generate_player(&mut ecs_world, &mut physics_world, &grid);

            // Set camera position to player start position
            camera.position = player_start_position.to_raylib_vec3(grid.tile_size);

            // Generate trees
            generate_trees(&mut ecs_world, &mut physics_world, &grid, NUM_OF_TREES);

            // Generate balls
            generate_balls(&mut ecs_world, &mut physics_world, &grid, NUM_OF_BALLS);

            // Generate witches
            generate_witches(&mut ecs_world, &mut physics_world, &grid, NUM_OF_WITCHES);

            // Add trees to grid
            for (_, (tree, body_handle)) in ecs_world.query::<(&Tree, &BodyHandle)>().iter() {
                if let Some(body) = physics_world.bodies.get(body_handle.body_handle) {
                    let pos = body.translation();
                    let converted_pos = GridCoord::from_rapier3d_vec(*pos);
                    let radius = (tree.leaf_width / 2.0).ceil() as isize;

                    grid.fill_area(converted_pos, radius, TileType::Tree);
                }
            }
        }
        Err(e) => eprintln!("Error loading map: {e}"),
    }

    Game {
        ecs_world,
        physics_world,
        camera,
        mouse_look,
    }
}

// Update
pub fn update(rl: &mut RaylibHandle, next_state: &mut Option<State>, game: &mut Game) {
    // If cursor is showing then disable it
    if !rl.is_cursor_hidden() {
        rl.disable_cursor();
    }
    // Get mouse info
    game.mouse_look.update_from_mouse(rl);

    // Get player from ECS
    if let Some((_, (_, body_handle))) = game
        .ecs_world
        .query::<(&mut Player, &BodyHandle)>()
        .iter()
        .next()
    {
        // Handle player movement
        handle_player_movement(
            &mut game.physics_world,
            rl,
            body_handle,
            game.mouse_look.yaw(),
        );

        // Update physics world
        game.physics_world.step();

        // Update camera
        update_camera(
            &mut game.camera,
            &game.physics_world,
            body_handle,
            game.mouse_look.yaw(),
            game.mouse_look.pitch(),
        );
    } else {
        // Update physics world
        game.physics_world.step();
    }

    // Update witch AI
    if update_witch_ai(&mut game.ecs_world, &mut game.physics_world) {
        // If witch AI returns game over
        println!("GAME OVER!");

        // Set next state to Title Screen
        *next_state = Some(State::TitleScreen);
    }
}

// Render
pub fn render(rl: &mut RaylibHandle, thread: &RaylibThread, game: &mut Game) {
    // Begin drawing frame
    let mut d = rl.begin_drawing(thread);

    // Clear frame
    d.clear_background(Color::SKYBLUE);

    // Draw 3D objects
    d.draw_mode3D(game.camera, |mut d3d, _camera| {
        // Draw blocks
        draw_blocks(&mut d3d, &game.ecs_world, &game.physics_world);

        // Draw forest
        draw_forest(&mut d3d, &game.ecs_world, &game.physics_world);

        // Draw balls
        draw_balls(&mut d3d, &game.ecs_world, &game.physics_world);

        // Draw witches
        draw_witches(&mut d3d, &game.ecs_world, &game.physics_world);

        // Draw collision wireframes
        if DEBUG_MODE {
            debug_colliders(&mut d3d, &game.physics_world, Color::RED);
        }
    });

    // Draw HUD
    draw_hud(&mut d);
}
