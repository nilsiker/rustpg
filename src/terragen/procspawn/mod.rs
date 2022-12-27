use std::marker::PhantomData;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, utils::HashSet};
use bevy_rapier3d::prelude::{Collider, RapierContext, RigidBody};

use crate::terragen::Chunk;

use super::{ObjectDistance, PlayerChunk};

pub struct SpawningData {
    disc_radius: f32,
}

pub struct ProcSpawnConfig {
    to_spawn: Vec<SpawningData>,
}

#[derive(Component)]
struct SpawnedObjects;

#[derive(Default)]
pub struct ProcSpawnPlugin;

impl Plugin for ProcSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_spawn_points);
    }
}

fn add_spawn_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    player_chunk: Res<PlayerChunk>,
    object_distance: Res<ObjectDistance>,
    query: Query<(Entity, &Handle<Mesh>, &Chunk), (With<Chunk>, Without<SpawnedObjects>)>,
) {
    if !player_chunk.is_changed() {
        return;
    }

    let chunks: Vec<(Entity, &Handle<Mesh>, &Chunk)> = query
        .into_iter()
        .filter(|(_, _, chunk)| {
            let (x, y) = player_chunk.0;

            (x - object_distance.0..=x + object_distance.0).contains(&chunk.x)
                && (y - object_distance.0..=y + object_distance.0).contains(&chunk.y)
        })
        .collect();

    for (entity, mesh_handle, _) in chunks {
        let positions = match meshes.get(mesh_handle) {
            Some(mesh) => match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                Some(VertexAttributeValues::Float32x3(positions)) => Some(positions.clone()),
                _ => None,
            },
            None => None,
        };

        if let Some(positions) = positions {
            let mut vec = vec![];
            for pos in positions {
                let v: Vec3 = pos.into();
                if !vec.contains(&v) {
                    vec.push(v);
                    commands
                        .entity(entity)
                        .with_children(|children| {
                            children.spawn(PbrBundle {
                                mesh: meshes.add(
                                    shape::Icosphere {
                                        radius: 1.0,
                                        subdivisions: 0,
                                    }
                                    .into(),
                                ),
                                transform: Transform::from_translation(v),
                                ..default()
                            });
                        })
                        .insert(SpawnedObjects);
                }
            }
        }
    }
}
