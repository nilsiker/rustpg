

use rustpg::terragen::noise::{Noise, NoiseConfig};

fn main() {
    let noise = Noise::new(NoiseConfig {
        size: 100,
        octaves: 4,
        ..Default::default()
    });

    println!("start");
    for i in 0..10 {
        noise.generate_noise_map(i, 0);
        println!("{i} done");
    }
    println!("finish");
}
