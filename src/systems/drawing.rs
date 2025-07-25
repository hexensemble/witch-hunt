use crate::components::*;
use crate::physics::*;
use crate::settings::*;
use hecs::World;
use rapier3d::math::Isometry;
use raylib::prelude::*;

// Draw ground
pub fn draw_ground(d3d: &mut RaylibMode3D<RaylibDrawHandle>) {
    d3d.draw_plane(
        Vector3::new(GROUND_POS_X, GROUND_POS_Y, GROUND_POS_Z),
        Vector2::new(GROUND_SIZE_X, GROUND_SIZE_Z),
        Color::LIMEGREEN,
    );
}

// Draw walls
pub fn draw_walls(d3d: &mut RaylibMode3D<RaylibDrawHandle>) {
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
    for (_, ball) in ecs_world.query::<&Ball>().iter() {
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
        } else {
            // Add support for other shapes if needed
            d3d.draw_text("Unsupported shape", 10, 10, 20, color);
        }
    }
}
