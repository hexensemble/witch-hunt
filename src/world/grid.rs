use rapier3d::prelude::*;
use raylib::prelude::*;

// Grid
#[derive(Debug)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub tile_size: f32,
}

// Grid functions
impl Grid {
    pub fn new(width: usize, height: usize, tiles: Vec<Vec<Tile>>) -> Self {
        Self {
            width,
            height,
            tiles,
            tile_size: 1.0,
        }
    }
}

// Grid coordinates
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

// Grid coordinates functions
impl GridCoord {
    // Zero'd out grid coordinates
    pub fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    // Grid coordinates to Raylib Vector3
    pub fn to_raylib_vec3(&self, tile_size: f32) -> Vector3 {
        Vector3::new(
            self.x as f32 * tile_size,
            self.y as f32 * tile_size,
            self.z as f32 * tile_size,
        )
    }

    // Grid coordinates to Rapier3D Vector
    pub fn to_rapier3d_vec(&self, tile_size: f32) -> Vector<f32> {
        vector![
            self.x as f32 * tile_size,
            self.y as f32 * tile_size,
            self.z as f32 * tile_size,
        ]
    }

    // Grid coordinates to Rapier3D Vector but takes new coordinates
    pub fn to_rapier3d_vec_new(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        tile_size: f32,
    ) -> Vector<f32> {
        self.x = x;
        self.y = y;
        self.z = z;

        vector![
            self.x as f32 * tile_size,
            self.y as f32 * tile_size,
            self.z as f32 * tile_size,
        ]
    }

    // Rapier3D Vector to grid coordinates
    pub fn from_rapier3d_vec(vector: Vector<f32>) -> Self {
        Self {
            x: vector.x as usize,
            y: vector.y as usize,
            z: vector.z as usize,
        }
    }
}

// Tile
#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub kind: TileType,
    pub coord: GridCoord,
}

// Tile Types
#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Air,
    Grass,
    Stone,
}
