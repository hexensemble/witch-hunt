use crate::settings::*;
use crate::world::grid::*;
use std::path::PathBuf;
use tiled::{LayerType, Loader, PropertyValue, TileLayer};

// Load Tiled map
pub fn load_tiled_map(filename: &str) -> Result<Grid, tiled::Error> {
    // Create loader
    let mut loader = Loader::new();

    // Get file path
    let file_path = PathBuf::from("assets").join("maps").join(filename);

    // Get map from file
    let map = loader.load_tmx_map(file_path)?;

    // Get map width and height
    let width = map.width as usize;
    let height = map.height as usize;

    // Create new list to add tiles
    let mut tiles: Vec<Vec<Tile>> = vec![vec![]; width];

    // Go through each map layer
    for layer in map.layers() {
        if let LayerType::Tiles(TileLayer::Finite(finite)) = layer.layer_type() {
            // Processing layer confirmation message
            if DEBUG_MODE {
                println!("Processing tile layer: {}", layer.name);
            }

            // Go through layer coordinates
            for x in 0..finite.width() {
                for y in 0..finite.height() {
                    // Flip Y since Tiled origin is top-left
                    let flipped_z = (map.height - 1 - y) as usize;

                    // Get tile property TileType and set our TileType
                    if let Some(tile) = finite.get_tile(x as i32, y as i32) {
                        let kind = if let Some(tile_def) = tile.get_tile() {
                            match tile_def.properties.get("TileType") {
                                Some(PropertyValue::StringValue(s)) => match s.as_str() {
                                    "Grass" => TileType::Grass,
                                    "Stone" => TileType::Stone,
                                    _ => TileType::Air,
                                },
                                _ => TileType::Air,
                            }
                        } else {
                            TileType::Air
                        };

                        // Get height based on layer ID
                        let height = layer.id() as usize - 1;

                        // Set grid coordinated for tile
                        let coord = GridCoord {
                            x: x as usize,
                            y: height,
                            z: flipped_z,
                        };

                        // Create new tile based on TileType and coordinates
                        let tile = Tile { kind, coord };

                        // Tile confirmation message
                        if DEBUG_MODE {
                            println!("🟩 {tile:?}");
                        }

                        // Add tile to tile list
                        tiles[x as usize].push(tile);
                    } else {
                        // If no tile insert a default Air tile
                        let coord = GridCoord {
                            x: x as usize,
                            y: height,
                            z: flipped_z,
                        };

                        let tile = Tile {
                            kind: TileType::Air,
                            coord,
                        };

                        // Tile confirmation message
                        if DEBUG_MODE {
                            println!("🟥 {tile:?}");
                        }

                        // Add tile to tile list
                        tiles[x as usize].push(tile);
                    }
                }
            }
        }
    }

    // Return newly created grid
    Ok(Grid::new(width, height, tiles))
}
