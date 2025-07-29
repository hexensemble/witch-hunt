use components::*;
use hecs::World;
use physics::*;
use raylib::prelude::*;
use settings::*;
use systems::ai::*;
use systems::drawing::*;
use systems::player::*;
use systems::spawn::*;
use systems::terrain::*;

mod components;
mod physics;
mod settings;
mod systems;

fn main() {
    // Create Raylib handle and thread
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Witch Hunt")
        .build();

    // Create camera
    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 2.0, 4.0),
        Vector3::new(0.0, 2.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );

    // Create mouse look
    let mut mouse_look = MouseLook::new(MOUSE_SENSITIVITY);

    // Set FPS
    rl.set_target_fps(60);

    // Create ECS world
    let mut ecs_world = World::new();

    // Create physics world
    let mut physics_world = PhysicsWorld::new();

    // Create terrain
    generate_terrain(&mut ecs_world, &mut physics_world);

    // List of entity positions for checking spawn locations don't duplicate
    let mut positions: Vec<Vector3> = Vec::new();

    // Generate player
    let player_start_position = generate_player(&mut ecs_world, &mut positions, &mut physics_world);

    // Set camera position to player start position
    camera.position = Vector3::new(
        player_start_position.x,
        player_start_position.y,
        player_start_position.z,
    );

    // Generate trees
    generate_trees(
        &mut ecs_world,
        &mut positions,
        &mut physics_world,
        NUM_OF_TREES,
    );

    // Generate balls
    generate_balls(
        &mut ecs_world,
        &mut positions,
        &mut physics_world,
        NUM_OF_BALLS,
    );

    // Generate witches
    generate_witches(
        &mut ecs_world,
        &mut positions,
        &mut physics_world,
        NUM_OF_WITCHES,
    );

    // Set game state to title screen
    let mut current_screen = GameState::Title;

    // Main game loop
    while !rl.window_should_close() {
        // Match current game state
        match current_screen {
            // Title screen
            GameState::Title => {
                // If cursor is hidden then enable it
                if rl.is_cursor_hidden() {
                    rl.enable_cursor();
                }

                // Set text style for GUI
                rl.gui_set_style(GuiControl::DEFAULT, GuiDefaultProperty::TEXT_SIZE, 30);

                // Press Enter or Space to continue
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
                    || rl.is_key_pressed(KeyboardKey::KEY_SPACE)
                {
                    // Set game state to Game
                    current_screen = GameState::Game;
                }

                // Begin drawing frame
                let mut d = rl.begin_drawing(&thread);

                // Clear frame
                d.clear_background(Color::WHITE);

                // Start button
                if d.gui_button(Rectangle::new(300.0, 150.0, 200.0, 50.0), "START") {
                    current_screen = GameState::Game;
                }

                // Quit button
                if d.gui_button(Rectangle::new(300.0, 250.0, 200.0, 50.0), "QUIT") {
                    break;
                }
            }
            // Game
            GameState::Game => {
                // If cursor is showing then disable it
                if !rl.is_cursor_hidden() {
                    rl.disable_cursor();
                }
                // Get mouse info
                mouse_look.update_from_mouse(&rl);

                // Get player from ECS
                if let Some((_, player)) = ecs_world.query::<&mut Player>().iter().next() {
                    // Handle player movement
                    handle_player_movement(&mut physics_world, &rl, player, mouse_look.yaw());

                    // Update physics world
                    physics_world.step();

                    // Update camera
                    update_camera(
                        &mut camera,
                        &physics_world,
                        player,
                        mouse_look.yaw(),
                        mouse_look.pitch(),
                    );
                } else {
                    // Update physics world
                    physics_world.step();
                }

                // Update witch AI
                if update_witch_ai(&mut ecs_world, &mut physics_world) {
                    println!("GAME OVER!");
                    break;
                }

                // Begin drawing frame
                let mut d = rl.begin_drawing(&thread);

                // Clear frame
                d.clear_background(Color::SKYBLUE);

                // Draw 3D objects
                d.draw_mode3D(camera, |mut d3d, _camera| {
                    // Draw ground
                    draw_ground(&mut d3d);

                    // Draw walls
                    draw_walls(&mut d3d);

                    // Draw forest
                    draw_forest(&mut d3d, &ecs_world, &physics_world);

                    // Draw balls
                    draw_balls(&mut d3d, &ecs_world, &physics_world);

                    // Draw witches
                    draw_witches(&mut d3d, &ecs_world, &physics_world);

                    // Draw collision wireframes
                    debug_colliders(&mut d3d, &physics_world, Color::RED);
                });

                // Draw HUD
                draw_hud(&mut d);
            }
        }
    }
}

// Game states
enum GameState {
    Title,
    Game,
}
