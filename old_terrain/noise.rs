
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
    pub scale: f64,
}

pub fn get_noise_map(x_coord: usize, y_coord: usize, noise_config: &NoiseConfig) -> NoiseMap {
    let NoiseConfig {
        seed: _seed,
        size,
        frequency,
        lacunarity,
        octaves,
        persistence,
        scale
    } = *noise_config;

    let mut fbm = Fbm::new();
    fbm.frequency = frequency;
    fbm.lacunarity = lacunarity;
    fbm.persistence = persistence;
    fbm.octaves = octaves;

    let bounds_size = size as f64;
    let bounds_start_x = x_coord as f64 * bounds_size as f64 * scale;
    let bounds_end_x = (x_coord + 1) as f64 * bounds_size * scale;
    let bounds_start_y = y_coord as f64 * bounds_size as f64 * scale;
    let bounds_end_y = (y_coord + 1) as f64 * bounds_size * scale;

    let map = PlaneMapBuilder::new(&fbm)
        .set_size(size, size)
        .set_x_bounds(bounds_start_x, bounds_end_x)
        .set_y_bounds(bounds_start_y, bounds_end_y)
        .build();
    map
}


pub fn generate_terrain_heights(
    x: usize,
    y: usize,
    height_multiplier: f32,
    noise_config: &NoiseConfig,
) -> Vec<f32> {
    let mut terrain = Vec::new();
    let noise_map = get_noise_map(x, y, &noise_config);
    for y in 0..noise_config.size {
        for x in 0..noise_config.size {
            let mut value = noise_map.get_value(x, y) as f32;
            value *= 2.0;
            value -= 1.0;
            value *= height_multiplier;
            terrain.push(value);
        }
    }

    // TODO normalize heights!

    terrain
}


