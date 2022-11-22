mod mesh_gen;
mod noise;

use bevy::prelude::*;

use self::{mesh_gen::{GenerationMode, TerrainGenerator}};

#[derive(Resource)]
struct TerrainChunkSize(usize);
#[derive(Resource)]
struct TerrainFlatShading(bool);
pub struct TerrainPlugin {
    chunk_size: usize,
    flat_shaded: bool,
}
impl Default for TerrainPlugin {
    fn default() -> Self {
        Self {
            chunk_size: 256,
            flat_shaded: true,
        }
    }
}
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainChunkSize(self.chunk_size))
            .insert_resource(TerrainFlatShading(self.flat_shaded))
            .add_startup_system(debug_terrain);
    }
}

fn debug_terrain(
    chunk_size: Res<TerrainChunkSize>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk_size = chunk_size.0;

    let mut terragen = TerrainGenerator::new(GenerationMode::Rtin, chunk_size, true);
    // meshes
    for x in 0..=5 {
        for y in 0..=5 {
            let chunk = terragen.generate_terrain_chunk(x, y, 10.0);

            let mut material: StandardMaterial = Color::rgb(0.4, 0.8, 0.4).into();
            material.reflectance = 0.2;
            material.metallic = 0.0;
            commands.spawn(PbrBundle {
                mesh: meshes.add(chunk.mesh),
                material: materials.add(material),
                transform: Transform::from_translation(chunk.position),
                ..Default::default()
            });
        }
    }
}
