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
    pub grid: Grid,
    pub camera: Camera3D,
    pub mouse_look: MouseLook,
}

// Functions for Game
impl Game {
    // Start a new game
    pub fn new() -> Result<Self, tiled::Error> {
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
        let mut grid = load_tiled_map("map_01.tmx")?;

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
                let radius = (tree.leaf_width / 2.0).floor() as isize;

                grid.fill_area(converted_pos, radius, TileType::Tree);
            }
        }

        Ok(Self {
            ecs_world,
            physics_world,
            grid,
            camera,
            mouse_look,
        })
    }

    // Update
    pub fn update(&mut self, rl: &mut RaylibHandle, next_state: &mut Option<State>) {
        // If cursor is showing then disable it
        if !rl.is_cursor_hidden() {
            rl.disable_cursor();
        }
        // Get mouse info
        self.mouse_look.update_from_mouse(rl);

        // Get player from ECS
        if let Some((_, (_, body_handle))) = self
            .ecs_world
            .query::<(&mut Player, &BodyHandle)>()
            .iter()
            .next()
        {
            // Handle player movement
            handle_player_movement(
                &mut self.physics_world,
                rl,
                body_handle,
                self.mouse_look.yaw(),
            );

            // Update physics world
            self.physics_world.step();

            // Update camera
            update_camera(
                &mut self.camera,
                &self.physics_world,
                body_handle,
                self.mouse_look.yaw(),
                self.mouse_look.pitch(),
            );
        } else {
            // Update physics world
            self.physics_world.step();
        }

        // Update witch AI
        if update_witch_ai(&mut self.ecs_world, &mut self.physics_world) {
            // If witch AI returns game over
            println!("GAME OVER!");

            // Set next state to Title Screen
            *next_state = Some(State::TitleScreen);
        }
    }

    // Render
    pub fn render(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Begin drawing frame
        let mut d = rl.begin_drawing(thread);

        // Clear frame
        d.clear_background(Color::SKYBLUE);

        // Draw 3D objects
        d.draw_mode3D(self.camera, |mut d3d, _camera| {
            // Draw blocks
            draw_blocks(&mut d3d, &self.ecs_world, &self.physics_world);

            // Draw forest
            draw_forest(&mut d3d, &self.ecs_world, &self.physics_world);

            // Draw balls
            draw_balls(&mut d3d, &self.ecs_world, &self.physics_world);

            // Draw witches
            draw_witches(&mut d3d, &self.ecs_world, &self.physics_world);

            // Draw collision wireframes
            if DEBUG_MODE {
                debug_colliders(&mut d3d, &self.physics_world, Color::RED);
            }
        });

        // Draw HUD
        draw_hud(&mut d);
    }
}
