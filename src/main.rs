use components::*;
use hecs::World;
use map::*;
use physics::*;
use raylib::prelude::*;
use settings::*;
use systems::drawing::*;
use systems::player::*;
use systems::spawn::*;

mod components;
mod map;
mod physics;
mod settings;
mod systems;

fn main() {
    // Create Raylib handle and thread
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Raylib Test")
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

    // Disable cursor
    rl.disable_cursor();

    // Set FPS
    rl.set_target_fps(60);

    // Create ECS world
    let mut ecs_world = World::new();

    // Create physics world
    let mut physics_world = PhysicsWorld::new();

    // Create map
    generate_map(&mut ecs_world, &mut physics_world);

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

    // Set game state to title screen
    let mut current_screen = GameState::Title;

    // Main game loop
    while !rl.window_should_close() {
        // Match current game state
        match current_screen {
            // Title screen
            GameState::Title => {
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

                // Draw welcome message
                d.draw_text(
                    "Welcome! Press ENTER or SPACE to continue...",
                    40,
                    40,
                    10,
                    Color::BLACK,
                );
            }
            // Game
            GameState::Game => {
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

                // Begin drawing frame
                let mut d = rl.begin_drawing(&thread);

                // Clear frame
                d.clear_background(Color::SKYBLUE);

                // Draw 3D objects
                d.draw_mode3D(camera, |mut d3d, _camera| {
                    // Draw ground
                    // TODO Move to drawing.rs
                    d3d.draw_plane(
                        Vector3::new(GROUND_POS_X, GROUND_POS_Y, GROUND_POS_Z),
                        Vector2::new(GROUND_SIZE_X, GROUND_SIZE_Z),
                        Color::LIMEGREEN,
                    );

                    // Draw walls
                    // TODO Move to drawing.rs
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
                        d3d.draw_cube(
                            Vector3::new(pos.x / 2.0, pos.y / 2.0, pos.z / 2.0),
                            size.x,
                            size.y,
                            size.z,
                            Color::DARKGRAY,
                        );
                    }

                    // Draw forest
                    draw_forest(&mut d3d, &ecs_world, &physics_world);

                    // Draw balls
                    draw_balls(&mut d3d, &ecs_world, &physics_world);

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
