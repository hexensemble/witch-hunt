use crate::components::*;
use crate::physics::*;
use rapier3d::na::Vector3 as RapierVector3;
use rapier3d::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

// Handle player movement
pub fn handle_player_movement(
    physics_world: &mut PhysicsWorld,
    rl: &RaylibHandle,
    body_handle: &BodyHandle,
    yaw: f32,
) {
    // Get body for player from physics world
    if let Some(body) = physics_world.bodies.get_mut(body_handle.body_handle) {
        // Movement vector
        let mut movement: RapierVector3<f32> = RapierVector3::zeros();

        let forward = build_camera_forward(yaw, 0.0);
        let right = vector![-forward.z, 0.0, forward.x];

        // Keys
        if rl.is_key_down(KEY_W) {
            movement += forward;
        }
        if rl.is_key_down(KEY_S) {
            movement -= forward;
        }
        if rl.is_key_down(KEY_A) {
            movement -= right;
        }
        if rl.is_key_down(KEY_D) {
            movement += right;
        }

        // If player is moving (AKA not zero)
        if movement != RapierVector3::zeros() {
            movement = movement.normalize() * 4.0; // Preserves direction, constant movement speed of 4
            let current_vel = body.linvel(); // Get current velocity
            body.set_linvel(vector![movement.x, current_vel.y, movement.z], true);
            // Apply new velocity only on X/Z axes
        }
    }
}

// Update camera
pub fn update_camera(
    camera: &mut Camera3D,
    physics_world: &PhysicsWorld,
    body_handle: &BodyHandle,
    yaw: f32,
    pitch: f32,
) {
    if let Some(body) = physics_world.bodies.get(body_handle.body_handle) {
        let position = body.translation();
        camera.position = Vector3::new(position.x, position.y + 1.0, position.z);

        let forward = build_camera_forward(yaw, pitch);
        camera.target = camera.position + Vector3::new(forward.x, forward.y, forward.z);
    }
}

// Compute camera forward facing direction
fn build_camera_forward(yaw: f32, pitch: f32) -> RapierVector3<f32> {
    let x = yaw.cos() * pitch.cos();
    let y = pitch.sin();
    let z = yaw.sin() * pitch.cos();

    vector![x, y, z].normalize()
}

// Mouse look
pub struct MouseLook {
    pub yaw: f32,
    pub pitch: f32,
    pub mouse_sensitivity: f32,
}

// Mouse look functions
impl MouseLook {
    pub fn new(mouse_sensitivity: f32) -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            mouse_sensitivity,
        }
    }

    // Get mouse info
    pub fn update_from_mouse(&mut self, rl: &RaylibHandle) {
        let mouse_delta = rl.get_mouse_delta();
        self.yaw += mouse_delta.x * self.mouse_sensitivity;
        self.pitch -= mouse_delta.y * self.mouse_sensitivity;
        self.pitch = self.pitch.clamp(
            -std::f32::consts::FRAC_PI_2 + 0.01,
            std::f32::consts::FRAC_PI_2 - 0.01,
        );
    }

    // Get yaw
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    // Get pitch
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
}
