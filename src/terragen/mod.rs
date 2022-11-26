mod debug;
pub mod mesh;
pub mod noise;
pub mod terrain_colors;

use ::noise::{Fbm, Perlin};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use self::{
    mesh::{MeshConfig, MeshImageData},
    noise::{NoiseConfig, NoiseMap},
};

#[derive(Component, Default, Inspectable)]
struct Terrain {
    mesh_config: MeshConfig,
    noise_config: NoiseConfig,
}

pub struct TerragenPlugin;
impl Plugin for TerragenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(remove_terrain.label("terragen_cleanup"))
            .add_system(spawn_chunks.after("terragen_cleanup"))
            .register_inspectable::<Terrain>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Name::new("TerraGen"),
        Terrain::default(),
    ));
}

fn remove_terrain(mut commands: Commands, query: Query<Entity, Changed<Terrain>>) {
    for terrain in &query {
        commands.entity(terrain).despawn_descendants();
    }
}

fn spawn_chunks(
    mut commands: Commands,
    query: Query<(Entity, &Terrain), Changed<Terrain>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((entity, terrain)) = query.get_single() else { return;};

    let NoiseConfig {
        seed,
        octaves,
        frequency,
        lacunarity,
        persistence,
        offset,
        falloff,
    } = terrain.noise_config;

    commands.entity(entity).with_children(|children| {
        for x in 0..=1 {
            for y in 0..=1 {
                let mut fbm: Fbm<Perlin> = Fbm::new(seed);
                fbm.frequency = frequency;
                fbm.lacunarity = lacunarity;
                fbm.persistence = persistence;
                fbm.octaves = octaves;

                let nm =
                    NoiseMap::new(&fbm, terrain.mesh_config.grid_size, (x, y), offset, falloff);

                let MeshImageData { mesh, image } = mesh::get_mesh(&nm, &terrain.mesh_config);

                let material = StandardMaterial {
                    base_color_texture: Some(images.add(image)),
                    unlit: false,
                    metallic: 0.0,
                    reflectance: 0.1,
                    perceptual_roughness: 1.0,
                    ..default()
                };

                let scale = terrain.mesh_config.scale;

                children
                    .spawn(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(material),
                        transform: Transform::from_xyz(x as f32 * scale, 0.0, y as f32 * -scale),
                        ..default()
                    })
                    .insert(Name::new(format!("Chunk_{x}_{y}")));

                // children.spawn(PbrBundle {
                //     mesh: meshes.add(Mesh::from(shape::Plane {
                //         size: (1.0 * scale) / 2.0,
                //     })),
                //     transform: Transform::from_xyz(x as f32 * scale, 0.5, y as f32 * -scale),
                //     ..default()
                // });
            }
        }
    });
}