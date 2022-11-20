use std::collections::HashMap;

use bevy::{
    math::Vec3,
    prelude::{default, Mesh},
};

use self::noise_gen::{get_noise_map, NoiseConfig};

/// Here we put all terragen stuff, inspired by Sebastian Lague!
///

pub mod noise_gen {
    use noise::{
        utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder},
        Fbm,
    };
    #[derive(Default)]
    pub struct NoiseConfig {
        pub seed: u32,
        pub size: usize,
        pub frequency: f64,
        pub lacunarity: f64,
        pub persistence: f64,
        pub octaves: usize,
    }

    pub fn get_noise_map(x_coord: usize, y_coord: usize, noise_config: &NoiseConfig) -> NoiseMap {
        let NoiseConfig {
            seed: _seed,
            size,
            frequency,
            lacunarity,
            octaves,
            persistence,
        } = noise_config;

        let mut fbm = Fbm::new();
        fbm.frequency = *frequency;
        fbm.lacunarity = *lacunarity;
        fbm.persistence = *persistence;
        fbm.octaves = *octaves;

        let bounds_size = (*size) as f64;
        let bounds_start_x = x_coord as f64 * bounds_size as f64;
        let bounds_end_x = (x_coord + 1) as f64 * bounds_size;
        let bounds_start_y = y_coord as f64 * bounds_size as f64;
        let bounds_end_y = (y_coord + 1) as f64 * bounds_size;

        let map = PlaneMapBuilder::new(&fbm)
            .set_size(*size, *size)
            .set_x_bounds(bounds_start_x, bounds_end_x + 1.0)
            .set_y_bounds(bounds_start_y, bounds_end_y + 1.0)
            .build();
        map
    }
}
