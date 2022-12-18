pub mod mesh;
pub mod noise;
pub mod terrain_colors;

use ::noise::{Fbm, Perlin};
use bevy::{prelude::*, utils::HashSet};
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
            .add_event::<PlayerPositionChangedEvent>()
            .insert_resource(PlayerChunk((0, 0)))
            .insert_resource(ChunkPool(HashSet::new()))
            .insert_resource(SpawnedChunks(HashSet::new()))
            .add_system(remove_terrain.label("terragen_cleanup"))
            .add_system(spawn_chunks.after("terragen_cleanup"))
            .add_system(register_player_chunk)
            .add_system(log_current_chunk)
            .add_system(update_chunk_pool)
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
    query: Query<(Entity, &Terrain)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pool: Res<ChunkPool>,
    mut spawned: ResMut<SpawnedChunks>,
) {
    if !pool.is_changed() {
        return;
    }
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
        for (x, y) in &pool.0 {
            if spawned.0.contains(&(*x, *y)) {
                continue;
            }

            let mut fbm: Fbm<Perlin> = Fbm::new(seed);
            fbm.frequency = frequency;
            fbm.lacunarity = lacunarity;
            fbm.persistence = persistence;
            fbm.octaves = octaves;

            let nm = NoiseMap::new(
                &fbm,
                terrain.mesh_config.grid_size,
                (*x, *y),
                offset,
                falloff,
            );

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
                    transform: Transform::from_xyz(*x as f32 * scale, 0.0, *y as f32 * -scale),
                    ..default()
                })
                .insert(Name::new(format!("({x},{y})")))
                .insert(DistanceOcclusion);

            spawned.0.insert((*x, *y)); // TODO make a system that removes very distant chunks.
        }
    });
}

pub struct PlayerPositionChangedEvent(pub Vec3);

#[derive(Component)]
struct DistanceOcclusion;

#[derive(Resource, Eq, PartialEq)]
struct PlayerChunk((i32, i32));

fn register_player_chunk(
    mut player_chunk: ResMut<PlayerChunk>,
    terrain: Query<&Terrain>,
    mut events: EventReader<PlayerPositionChangedEvent>,
) {
    let Ok(terrain) = terrain.get_single() else {return;};
    let chunk_size = terrain.mesh_config.scale;
    for event in events.iter() {
        let mut pos = event.0;
        pos.x += chunk_size / 2.0;
        pos.z += chunk_size / 2.0;
        pos.x /= chunk_size;
        pos.z /= chunk_size;
        let coord = Vec2::new(pos.x.floor(), pos.z.floor());
        let new_chunk_candidate = PlayerChunk((coord.x as i32, -coord.y as i32));
        if *player_chunk != new_chunk_candidate {
            *player_chunk = new_chunk_candidate;
        }
    }
}
fn log_current_chunk(current_chunk: Res<PlayerChunk>) {
    if current_chunk.is_changed() {
        bevy::log::info!("Player is in chunk: {:?}", current_chunk.0);
    }
}

fn update_chunk_pool(player_chunk: Res<PlayerChunk>, mut pool: ResMut<ChunkPool>) {
    if player_chunk.is_changed() {
        pool.0.clear();

        let (x, y) = player_chunk.0;
        for nx in x - 2..=x + 2 {
            for ny in y - 2..=y + 2 {
                if !pool.0.contains(&(nx, ny)) {
                    pool.0.insert((nx, ny));
                }
            }
        }
    }
}

#[derive(Resource)]
struct ChunkPool(HashSet<(i32, i32)>);

#[derive(Resource)]
struct SpawnedChunks(HashSet<(i32, i32)>);
