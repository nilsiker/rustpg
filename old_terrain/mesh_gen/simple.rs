use bevy::{prelude::*, utils::HashMap};

use super::{
    mesh_gen,
    noise::{generate_terrain_heights, get_noise_map, NoiseConfig},
};

#[derive(Default, Eq, PartialEq, Hash, Clone)]
pub struct ChunkCoord {
    pub x: usize,
    pub y: usize,
}

pub struct Chunk {
    pub coordinate: ChunkCoord,
    pub position: Vec3,
    pub mesh: Mesh,
}

pub struct TerrainGenerator {
    pub grid_size: usize,
    pub noise: NoiseConfig,
    pub terrains: HashMap<ChunkCoord, Vec<f32>>,
}

impl TerrainGenerator {
    pub fn new(grid_size: usize, noise: NoiseConfig) -> Self {
        if grid_size > 0 && grid_size & (grid_size - 1) != 0 {
            panic!("Expected grid size to be 2^n, got {grid_size}");
        }

        Self {
            grid_size,
            noise,
            terrains: HashMap::new(),
        }
    }
    pub fn get_chunk(&mut self, x: usize, y: usize) -> Chunk {
        let map = get_noise_map(x, y, &self.noise);
        let coord = ChunkCoord { x, y };
        let terrain = match self.terrains.get(&coord) {
            Some(terrain) => terrain,
            None => {
                let terrain = generate_terrain_heights(x, y, 15.0, &self.noise);
                self.terrains.insert(coord.clone(), terrain);
                &self
                    .terrains
                    .get(&coord)
                    .expect("adding of terrain failed unexpectedly")
            }
        };

        map.write_to_file(&format!("simple_chunk_{}.{}.png", x, y));

        let mesh = mesh_gen::generate_mesh(&terrain, 1, self.grid_size - 1, true);

        Chunk {
            coordinate: coord,
            position: Vec3::new(
                x as f32 * self.grid_size as f32,
                0.0,
                y as f32 * self.grid_size as f32,
            ),
            mesh,
        }
    }
}
