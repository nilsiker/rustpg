use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

use super::mesh::MeshConfig;

#[derive(Component, Inspectable, Clone)]
pub struct NoiseConfig {
    pub seed: u32,
    #[inspectable(min = 0, max = 6)]
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset: Vec2,
}
impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            octaves: 4,
            frequency: 0.01,
            lacunarity: 1.1,
            persistence: 1.0,
            offset: Vec2::default(),
        }
    }
}

pub struct Noise {
    fbm: Fbm<Perlin>,
    config: NoiseConfig,
}

impl Default for Noise {
    fn default() -> Self {
        Self::new(NoiseConfig::default())
    }
}
impl Noise {
    pub fn new(config: NoiseConfig) -> Self {
        let NoiseConfig {
            seed,
            octaves,
            frequency,
            lacunarity,
            persistence,
            ..
        } = config;

        let mut fbm = Fbm::new(seed);
        fbm.frequency = frequency;
        fbm.lacunarity = lacunarity;
        fbm.octaves = octaves;
        fbm.persistence = persistence;

        Self { fbm, config }
    }

    pub fn generate_noise_map(&self, x: usize, y: usize, mesh_config: &MeshConfig) -> NoiseMap {
        todo!(); // TRY worldgen crate instead of this!
        let NoiseConfig { offset, .. } = self.config;
        let &MeshConfig {
            grid_size, scale, ..
        } = mesh_config;
        let scale = scale as f64;

        let lower_x_bound = x as f64 * scale + offset.x as f64;
        let upper_x_bound = (x + 1) as f64 * scale + offset.x as f64;

        let lower_y_bound = y as f64 * scale + offset.y as f64;
        let upper_y_bound = (y + 1) as f64 * scale + offset.y as f64;

        let nm = PlaneMapBuilder::<_, 2>::new(&self.fbm)
            .set_size(grid_size, grid_size)
            .set_x_bounds(lower_x_bound, upper_x_bound)
            .set_y_bounds(lower_y_bound, upper_y_bound);

        let x_b = nm.x_bounds();
        let y_b = nm.y_bounds();

        bevy::log::info!("chunk ({x},{y}):");
        bevy::log::info!("x bounds: {:?}", x_b);
        bevy::log::info!("y bounds: {:?}", y_b);
        nm.build()
    }
}
