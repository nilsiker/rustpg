mod rtin;
mod simple;

use bevy::prelude::*;

use self::rtin::TerrainGenerator;
#[derive(Resource)]
struct TerrainGridSize(usize);
#[derive(Resource)]
struct TerrainFlatShading(bool);
pub struct TerrainPlugin {
    grid_size: usize,
    flat_shaded: bool,
}
impl Default for TerrainPlugin {
    fn default() -> Self {
        Self {
            grid_size: 257,
            flat_shaded: true,
        }
    }
}
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainGridSize(self.grid_size))
            .insert_resource(TerrainFlatShading(self.flat_shaded))
            .add_startup_system(debug_terrain);
    }
}

fn debug_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_size: Res<TerrainGridSize>,
) {
    let grid_size = grid_size.0;
    let chunk_size = grid_size - 1;

    // meshes
    for x in 0..5 {
        for y in 0..5 {
            let mut terragen = TerrainGenerator::new(chunk_size, false);
            let chunk = terragen.generate_terrain_chunk(x, y, 0.5);
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
