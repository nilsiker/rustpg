use rustpg::terragen::noise::{ NoiseConfig, NoiseMap};

fn main() {
    let nm = NoiseMap::new(NoiseConfig::default());

    println!("{:?}", nm);
}
