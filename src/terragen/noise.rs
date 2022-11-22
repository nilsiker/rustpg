use array2d::Array2D;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use noise::{
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin, NoiseFn,
};

#[derive(Component, Inspectable, Clone)]
pub struct NoiseConfig {
    pub seed: u32,
    pub size: i32,
    #[inspectable(min = 0, max = 6)]
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}
impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            size: 32,
            octaves: 4,
            frequency: 0.01,
            lacunarity: 1.1,
            persistence: 1.0,
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
        let NoiseConfig { seed, .. } = config;

        let mut fbm = Fbm::new(seed);
        fbm.frequency = config.frequency;
        fbm.lacunarity = config.lacunarity;
        fbm.octaves = config.octaves;
        fbm.persistence = config.persistence;

        Self { fbm, config }
    }

    pub fn generate_noise_map(&self, x: i32, y: i32) -> NoiseMap {
        let NoiseConfig { size, .. } = self.config;

        let lower_x_bound = (x * size) as f64;
        let upper_x_bound = ((x + 1) * size) as f64;

        let lower_y_bound = (y * size) as f64;
        let upper_y_bound = ((y + 1) * size) as f64;

        PlaneMapBuilder::<_, 2>::new(&self.fbm)
            .set_size(size as usize, size as usize)
            .set_x_bounds(lower_x_bound, upper_x_bound)
            .set_y_bounds(lower_y_bound, upper_y_bound)
            .build()
    }
}
