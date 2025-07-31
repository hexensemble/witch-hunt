use crate::components::*;
use crate::physics::*;
use crate::settings::*;
use crate::systems::ai::*;
use crate::systems::drawing::*;
use crate::systems::player::*;
use crate::systems::spawn::*;
use crate::systems::terrain::*;
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
    if let Some((_, player)) = game.ecs_world.query::<&mut Player>().iter().next() {
        // Handle player movement
        handle_player_movement(&mut game.physics_world, rl, player, game.mouse_look.yaw());

        // Update physics world
        game.physics_world.step();

        // Update camera
        update_camera(
            &mut game.camera,
            &game.physics_world,
            player,
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
        // Draw ground
        draw_ground(&mut d3d);

        // Draw walls
        draw_walls(&mut d3d);

        // Draw forest
        draw_forest(&mut d3d, &game.ecs_world, &game.physics_world);

        // Draw balls
        draw_balls(&mut d3d, &game.ecs_world, &game.physics_world);

        // Draw witches
        draw_witches(&mut d3d, &game.ecs_world, &game.physics_world);

        // Draw collision wireframes
        debug_colliders(&mut d3d, &game.physics_world, Color::RED);
    });

    // Draw HUD
    draw_hud(&mut d);
}
