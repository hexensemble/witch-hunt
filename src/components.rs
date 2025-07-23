use rapier3d::prelude::*;
use raylib::prelude::*;

// Player component
pub struct Player {
    pub body_handle: RigidBodyHandle,
}

// Tree component
pub struct Tree {
    pub leaf_width: f32,
    pub leaf_height: f32,
    pub trunk_height: f32,
    pub body_handle: RigidBodyHandle,
    pub leaf_color: Color,
    pub trunk_color: Color,
}

// Ball component
pub struct Ball {
    pub size: f32,
    pub body_handle: RigidBodyHandle,
    pub color: Color,
}

// Nothing component
pub struct Nothing;
