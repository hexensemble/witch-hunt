use crate::components::*;
use crate::physics::*;
use hecs::World;
use rapier3d::prelude::*;
use raylib::prelude::*;

// Draw blocks
pub fn draw_blocks(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    ecs_world: &World,
    physics_world: &PhysicsWorld,
) {
    for (_, block) in ecs_world.query::<&Block>().iter() {
        if let Some(body) = physics_world.bodies.get(block.body_handle) {
            // Get position from physics world
            let position = body.translation();

            // Draw block
            d3d.draw_cube(
                Vector3::new(position.x, position.y, position.z),
                block.width,
                block.height,
                block.width,
                block.color,
            );
        }
    }
}

// Draw forest
pub fn draw_forest(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    ecs_world: &World,
    physics_world: &PhysicsWorld,
) {
    for (_, tree) in ecs_world.query::<&Tree>().iter() {
        // Get position from physics world
        if let Some(body) = physics_world.bodies.get(tree.body_handle) {
            let position = body.translation();

            // Draw leaves
            d3d.draw_cube(
                Vector3::new(
                    position.x,
                    tree.trunk_height + tree.leaf_height / 2.0,
                    position.z,
                ),
                tree.leaf_width,
                tree.leaf_height,
                tree.leaf_width,
                tree.leaf_color,
            );

            // Draw trunk
            d3d.draw_cube(
                Vector3::new(position.x, tree.trunk_height / 2.0, position.z),
                0.25,
                tree.trunk_height,
                0.25,
                tree.trunk_color,
            );
        }
    }
}

// Draw balls
pub fn draw_balls(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    ecs_world: &World,
    physics_world: &PhysicsWorld,
) {
    for (_, ball) in ecs_world.query::<&crate::components::Ball>().iter() {
        // Get position from physics world
        if let Some(body) = physics_world.bodies.get(ball.body_handle) {
            let position = body.translation();

            // Draw ball
            d3d.draw_sphere(
                Vector3::new(position.x, position.y, position.z),
                ball.size,
                ball.color,
            );
        }
    }
}

// Draw witches
pub fn draw_witches(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    ecs_world: &World,
    physics_world: &PhysicsWorld,
) {
    for (_, witch) in ecs_world.query::<&Witch>().iter() {
        // Get position from physics world
        if let Some(body) = physics_world.bodies.get(witch.body_handle) {
            let position = body.translation();

            // Draw witch
            d3d.draw_cube(
                Vector3::new(position.x, position.y, position.z),
                witch.width,
                witch.height,
                witch.width,
                witch.color,
            );
        }
    }
}

//Draw HUD
pub fn draw_hud(d: &mut RaylibDrawHandle) {
    d.draw_rectangle(10, 10, 220, 70, Color::GRAY);
    d.draw_rectangle_lines(10, 10, 220, 70, Color::BLUE);
    d.draw_text(
        "First person camera default controls:",
        20,
        20,
        10,
        Color::BLACK,
    );
    d.draw_text("- Move with keys: W, A, S, D", 40, 40, 10, Color::DARKGRAY);
    d.draw_text("- Mouse move to look around", 40, 60, 10, Color::DARKGRAY);
}

// Draw collider wireframes
pub fn debug_colliders(
    d3d: &mut RaylibMode3D<RaylibDrawHandle>,
    physics_world: &PhysicsWorld,
    color: Color,
) {
    for (_, collider) in physics_world.colliders.iter() {
        let shape = collider.shape();
        let iso: &Isometry<f32> = collider.position();

        let translation = iso.translation.vector;
        let pos = Vector3::new(translation.x, translation.y, translation.z);

        if let Some(cuboid) = shape.as_cuboid() {
            let half_extents = cuboid.half_extents;
            d3d.draw_cube_wires(
                pos,
                half_extents.x * 2.0,
                half_extents.y * 2.0,
                half_extents.z * 2.0,
                color,
            );
        } else if let Some(ball) = shape.as_ball() {
            d3d.draw_sphere_wires(pos, ball.radius, 8, 8, color);
        } else if let Some(round_cuboid) = shape.as_round_cuboid() {
            let half_extents = round_cuboid.inner_shape.half_extents;
            d3d.draw_cube_wires(
                pos,
                half_extents.x * 2.0,
                half_extents.y * 2.0,
                half_extents.z * 2.0,
                color,
            );
        } else {
            // Add support for other shapes if needed
            d3d.draw_text("Unsupported shape", 10, 10, 20, color);
        }
    }
}
