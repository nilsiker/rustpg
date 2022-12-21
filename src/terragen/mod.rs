pub mod mesh;
pub mod noise;
pub mod terrain_colors;

use ::noise::{Fbm, Perlin};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::HashSet,
};
use bevy_inspector_egui::{Inspectable, InspectorPlugin, RegisterInspectable};
use futures_lite::future;

use self::{
    mesh::{MeshConfig, MeshImageData},
    noise::{NoiseConfig, NoiseMap},
};

#[derive(Component, Default, Inspectable)]
struct Terrain;

#[derive(Default)]
pub struct TerragenPlugin {
    pub mesh_config: MeshConfig,
    pub noise_config: NoiseConfig,
    pub inspectors: bool,
}

impl Plugin for TerragenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_event::<PlayerPositionChangedEvent>()
            .insert_resource(self.mesh_config.clone())
            .insert_resource(self.noise_config.clone())
            .insert_resource(PlayerChunk((0, 0)))
            .insert_resource(ChunkPool(HashSet::new()))
            .insert_resource(SpawnedChunks(HashSet::new()))
            .add_system(spawn_tasks)
            .add_system(remove_terrain.label("terragen_cleanup"))
            .add_system(spawn_chunks.after("terragen_cleanup"))
            .add_system(register_player_chunk)
            .add_system(update_chunk_pool)
            .register_inspectable::<Terrain>();

        if self.inspectors {
            app.add_plugin(InspectorPlugin::<MeshConfig>::new_insert_manually());
            app.add_plugin(InspectorPlugin::<NoiseConfig>::new_insert_manually());
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Name::new("TerraGen"),
        Terrain::default(),
    ));
}

fn remove_terrain(
    mut commands: Commands,
    query: Query<Entity, With<Terrain>>,
    mesh_config: Res<MeshConfig>,
    noise_config: Res<NoiseConfig>,
    mut spawned: ResMut<SpawnedChunks>,
) {
    if mesh_config.is_changed() || noise_config.is_changed() {
        for terrain in &query {
            commands.entity(terrain).despawn_descendants();
            spawned.0.clear();
        }
    }
}

fn spawn_tasks(
    mut commands: Commands,
    query: Query<Entity, With<Terrain>>,
    mesh_config: Res<MeshConfig>,
    noise_config: Res<NoiseConfig>,
    pool: Res<ChunkPool>,
    mut spawned: ResMut<SpawnedChunks>,
) {
    if !pool.is_changed() {
        return;
    }
    let Ok(entity, ) = query.get_single() else { return;};
    let thread_pool = AsyncComputeTaskPool::get();

    let to_spawn = pool
        .0
        .iter()
        .filter(|coord| !spawned.0.contains(*coord))
        .cloned()
        .collect::<Vec<(i32, i32)>>();

    let NoiseConfig {
        seed,
        octaves,
        frequency,
        lacunarity,
        persistence,
        offset,
        falloff,
    } = *noise_config;
    for (x, y) in to_spawn {
        let mesh_config = mesh_config.clone();
        let task = thread_pool.spawn(async move {
            let mut fbm: Fbm<Perlin> = Fbm::new(seed);
            fbm.frequency = frequency;
            fbm.lacunarity = lacunarity;
            fbm.persistence = persistence;
            fbm.octaves = octaves;

            let nm = NoiseMap::new(&fbm, mesh_config.grid_size, (x, y), offset, falloff);
            ((x, y), mesh::get_mesh(&nm, &mesh_config))
        });
        spawned.0.insert((x, y)); // TODO make a system that removes very distant chunks.
        commands.entity(entity).with_children(|children| {
            children.spawn((ComputeMeshImageData(task),));
        });
    }
}

#[derive(Component)]
struct ComputeMeshImageData(Task<((i32, i32), MeshImageData)>);

fn spawn_chunks(
    mut commands: Commands,
    query: Query<Entity, With<Terrain>>,
    mesh_config: Res<MeshConfig>,
    mut tasks: Query<(Entity, &mut ComputeMeshImageData)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(entity) = query.get_single() else { return;};
    for (task_entity, mut task) in &mut tasks {
        if let Some(((x, y), MeshImageData { mesh, image })) =
            futures_lite::future::block_on(future::poll_once(&mut task.0))
        {
            commands.entity(entity).with_children(|children| {
                let material = StandardMaterial {
                    base_color_texture: Some(images.add(image)),
                    unlit: false,
                    metallic: 0.0,
                    reflectance: 0.1,
                    perceptual_roughness: 1.0,
                    ..default()
                };

                let scale = mesh_config.scale;

                children
                    .spawn(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(material),
                        transform: Transform::from_xyz(x as f32 * scale, 0.0, y as f32 * -scale),
                        ..default()
                    })
                    .insert(Name::new(format!("({x},{y})")))
                    .insert(DistanceOcclusion);
            });

            commands.entity(task_entity).despawn_recursive();
        }
    }
}

pub struct PlayerPositionChangedEvent(pub Vec3);

#[derive(Component)]
struct DistanceOcclusion;

#[derive(Resource, Eq, PartialEq)]
struct PlayerChunk((i32, i32));

fn register_player_chunk(
    mut player_chunk: ResMut<PlayerChunk>,
    terrain: Query<&Terrain>,
    mesh_config: Res<MeshConfig>,
    mut events: EventReader<PlayerPositionChangedEvent>,
) {
    let Ok(_) = terrain.get_single() else {return;};
    let chunk_size = mesh_config.scale;
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
