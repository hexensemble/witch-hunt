use rapier3d::prelude::*;
use raylib::prelude::*;

// Player component
pub struct Player {}

// Tree component
pub struct Tree {
    pub leaf_width: f32,
    pub leaf_height: f32,
    pub trunk_height: f32,
    pub leaf_color: Color,
    pub trunk_color: Color,
}

// Ball component
pub struct Ball {
    pub size: f32,
    pub color: Color,
}

// Witch component
pub struct Witch {
    pub width: f32,
    pub height: f32,
    pub collider_handle: ColliderHandle,
    pub color: Color,
    pub state: WitchState,
    pub target: Vector3,
}

// Block component
pub struct Block {
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

// Body Handle component
pub struct BodyHandle {
    pub body_handle: RigidBodyHandle,
}

// Witch behavior state
pub enum WitchState {
    Patrolling,
    Chasing,
}
