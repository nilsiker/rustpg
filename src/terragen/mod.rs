pub mod mesh;
pub mod noise;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use self::{
    mesh::{MeshConfig, MeshImageData, RenderMode},
    noise::{Noise, NoiseConfig},
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
            .add_system(generate_terrain.after("terragen_cleanup"))
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

fn generate_terrain(
    mut commands: Commands,
    query: Query<(Entity, &Terrain), Changed<Terrain>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((entity, terrain)) = query.get_single() else { return;};

    commands.entity(entity).with_children(|children| {
        let noise = Noise::new(terrain.noise_config.clone());
        for x in 0..=10 {
            for y in 0..=10 {
                let nm = noise.generate_noise_map(x, y);
                let MeshImageData { mesh, image } = mesh::get_mesh(nm, terrain.mesh_config);
                let material = StandardMaterial {
                    base_color_texture: Some(images.add(image)),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,

                    ..default()
                };

                let size = terrain.noise_config.size as f32;
                let scale = terrain.mesh_config.scale;

                children
                    .spawn(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(material),
                        transform: Transform::from_xyz(
                            x as f32 * size * scale,
                            0.0,
                            y as f32 * -size * scale,
                        ),
                        ..default()
                    })
                    .insert(Name::new(format!("Chunk_{x}_{y}")));
            }
        }
    });
}
