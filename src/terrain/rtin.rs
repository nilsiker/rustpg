use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Mesh, default}};
use mesh_gen::generate_mesh;
use rtin::Rtin;

use self::noise_gen::{get_noise_map, NoiseConfig};
pub struct MeshData {
    vertices: Vec<[f32; 3]>,
    indices: Vec<u16>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
}

#[derive(Hash, PartialEq, std::cmp::Eq, Clone, Copy)]
pub struct ChunkCoord {
    x: usize,
    y: usize,
}

pub struct TerrainGenerator {
    rtin: Rtin,
    chunk_size: usize,
    flat_shading: bool,
    noise: NoiseConfig,
    terrains: HashMap<ChunkCoord, Vec<f32>>,
}

pub struct Chunk {
    pub coordinate: ChunkCoord,
    pub position: Vec3,
    pub mesh: Mesh,
}

impl TerrainGenerator {
    pub fn new(chunk_size: usize, flat_shading: bool) -> TerrainGenerator {
        if chunk_size > 0 && chunk_size & (chunk_size - 1) != 0 {
            panic!("Expected chunk size to be 2^n, got {chunk_size}");
        }
        let grid_size = chunk_size + 1;
        let rtin = Rtin::new(grid_size);
        let noise = NoiseConfig {
            seed: 0,
            size: grid_size,
            frequency: 0.02,
            octaves: 2,
            ..default()
        };

        TerrainGenerator {
            rtin,
            chunk_size,
            flat_shading,
            noise,
            terrains: HashMap::new(),
        }
    }
    pub fn generate_terrain_chunk(&mut self, x: usize, y: usize, precision: f32) -> Chunk {
        let coord = ChunkCoord { x, y };
        let terrain = match self.terrains.get(&coord) {
            Some(terrain) => terrain,
            None => {
                let terrain = generate_terrain_heights(x, y, 15.0, &self.noise);
                self.terrains.insert(coord, terrain);
                &self
                    .terrains
                    .get(&coord)
                    .expect("adding of terrain failed unexpectedly")
            }
        };

        let tile = self.rtin.create_tile(&terrain);
        let mesh_data = tile.generate_mesh_data(15.0 * (1.0-precision)); // TODO parameterize max_error
        let mesh = generate_mesh(mesh_data, self.flat_shading);

        Chunk {
            coordinate: coord,
            position: Vec3::new(
                x as f32 * self.chunk_size as f32,
                0.0,
                y as f32 *  self.chunk_size as f32,
            ),
            mesh,
        }
    }
}
pub mod rtin {
    use super::MeshData;

    #[derive(Default)]
    struct MeshHelpers {
        indices: Vec<u16>,
        vertices: Vec<[f32; 3]>,
        triangles: Vec<u16>,
        num_vertices: usize,
        num_triangles: usize,
        tri_index: usize,
        max_error: f32,
    }

    pub struct Rtin {
        pub grid_size: usize,
        // thread_count: usize,
        num_triangles: usize,
        num_parent_triangles: usize,
        coords: Vec<u16>,
    }

    impl Rtin {
        pub fn new(grid_size: usize) -> Rtin {
            let tile_size = grid_size - 1;
            if tile_size & (tile_size - 1) != 0 {
                panic!("Expected grid size to be 2^n+1, instead got {}", grid_size);
            }

            let num_triangles = tile_size * tile_size * 2 - 2;
            let num_parent_triangles = num_triangles - tile_size * tile_size;
            let tile_size = tile_size as u16;
            let mut coords = vec![0u16; num_triangles * 4];
            for i in 0..num_triangles {
                let mut id = i + 2;
                let [mut ax, mut ay, mut bx, mut by, mut cx, mut cy] = [0u16; 6];
                if id & 1 != 0 {
                    bx = tile_size;
                    by = tile_size;
                    cx = tile_size;
                } else {
                    ax = tile_size;
                    ay = tile_size;
                    cy = tile_size;
                }

                while id / 2 > 1 {
                    id /= 2;
                    let mx = (ax + bx) / 2;
                    let my = (ay + by) / 2;

                    if id & 1 != 0 {
                        bx = ax;
                        by = ay;
                        ax = cx;
                        ay = cy;
                    } else {
                        ax = bx;
                        ay = by;
                        bx = cx;
                        by = cy;
                    }
                    cx = mx;
                    cy = my;
                }
                let k = i * 4;
                coords[k + 0] = ax;
                coords[k + 1] = ay;
                coords[k + 2] = bx;
                coords[k + 3] = by;
            }

            Rtin {
                grid_size,
                num_triangles,
                num_parent_triangles,
                coords,
            }
        }

        pub fn create_tile<'a>(&self, terrain: &'a Vec<f32>) -> Tile<'a> {
            let size = self.grid_size;
            let terrain_size = size * size;
            if terrain.len() != terrain_size {
                panic!(
                    "Expected terrain data of length {}, instead got {}",
                    size * size,
                    terrain.len()
                );
            }

            let coords = &self.coords;
            let mut errors = vec![0.0; terrain_size];

            for i in (0..self.num_triangles).rev() {
                let k = i * 4;
                let ax = coords[k + 0] as usize;
                let ay = coords[k + 1] as usize;
                let bx = coords[k + 2] as usize;
                let by = coords[k + 3] as usize;
                let mx = (ax + bx) / 2 as usize;
                let my = (ay + by) / 2 as usize;
                let cx = mx + my - ay as usize;
                let cy = my + ax - mx as usize;

                // calculate error in middle of long edge of triangle
                let interpolated_height = (terrain[ay * size + ax] + terrain[by * size + bx]) / 2.0;
                let middle_index = my * size + mx;
                let middle_error = (interpolated_height - terrain[middle_index]).abs();

                if middle_error > errors[middle_index] {
                    errors[middle_index] = middle_error;
                }

                if i < self.num_parent_triangles {
                    let left_child_index = ((ay + cy) / 2) * size + ((ax + cx) / 2);
                    let right_child_index = ((by + cy) / 2) * size + ((bx + cx) / 2);
                    errors[middle_index] = errors[middle_index]
                        .max(errors[left_child_index].max(errors[right_child_index]));
                }
            }

            Tile {
                errors,
                size,
                terrain,
            }
        }
    }

    pub struct Tile<'a> {
        pub errors: Vec<f32>,
        terrain: &'a Vec<f32>,
        size: usize,
    }

    impl Tile<'_> {
        fn count_elements(
            &self,
            mesh_helpers: &mut MeshHelpers,
            ax: u16,
            ay: u16,
            bx: u16,
            by: u16,
            cx: u16,
            cy: u16,
        ) {
            let size = self.size;
            let max_error = mesh_helpers.max_error;
            let mx = (ax + bx) >> 1;
            let my = (ay + by) >> 1;

            if (ax as i16 - cx as i16).abs() + (ay as i16 - cy as i16).abs() > 1
                && self.errors[my as usize * size + mx as usize] > max_error
            {
                self.count_elements(mesh_helpers, cx, cy, ax, ay, mx, my);
                self.count_elements(mesh_helpers, bx, by, cx, cy, mx, my);
            } else {
                let indices = &mut mesh_helpers.indices;
                if indices[ay as usize * size + ax as usize] == 0 {
                    mesh_helpers.num_vertices += 1;
                    indices[ay as usize * size + ax as usize] = mesh_helpers.num_vertices as u16
                }
                if indices[by as usize * size + bx as usize] == 0 {
                    mesh_helpers.num_vertices += 1;
                    indices[by as usize * size + bx as usize] = mesh_helpers.num_vertices as u16
                }
                if indices[cy as usize * size + cx as usize] == 0 {
                    mesh_helpers.num_vertices += 1;
                    indices[cy as usize * size + cx as usize] = mesh_helpers.num_vertices as u16
                }
                mesh_helpers.num_triangles += 1;
            }
        }

        fn process_triangle(
            &self,
            mesh_helpers: &mut MeshHelpers,
            ax: u16,
            ay: u16,
            bx: u16,
            by: u16,
            cx: u16,
            cy: u16,
        ) {
            let size = self.size;
            let max_error = mesh_helpers.max_error;

            let mx = (ax + bx) >> 1;
            let my = (ay + by) >> 1;

            if (ax as i16 - cx as i16).abs() + (ay as i16 - cy as i16).abs() > 1
                && self.errors[my as usize * size + mx as usize] > max_error
            {
                self.process_triangle(mesh_helpers, cx, cy, ax, ay, mx, my);
                self.process_triangle(mesh_helpers, bx, by, cx, cy, mx, my);
            } else {
                let indices = &mut mesh_helpers.indices;
                let vertices = &mut mesh_helpers.vertices;
                let triangles = &mut mesh_helpers.triangles;

                let a = indices[ay as usize * size + ax as usize] - 1;
                let b = indices[by as usize * size + bx as usize] - 1;
                let c = indices[cy as usize * size + cx as usize] - 1;

                let a_terrain_y = self.terrain[ay as usize * self.size + ax as usize];
                let b_terrain_y = self.terrain[by as usize * self.size + bx as usize];
                let c_terrain_y = self.terrain[cy as usize * self.size + cx as usize];

                let av = [ax as f32, a_terrain_y, ay as f32];
                let bv = [bx as f32, b_terrain_y, by as f32];
                let cv = [cx as f32, c_terrain_y, cy as f32];

                vertices[a as usize] = av;
                vertices[b as usize] = bv;
                vertices[c as usize] = cv;

                triangles[mesh_helpers.tri_index + 0] = a as u16;
                triangles[mesh_helpers.tri_index + 1] = b as u16;
                triangles[mesh_helpers.tri_index + 2] = c as u16;
                mesh_helpers.tri_index += 3;
            }
        }

        pub fn generate_mesh_data(&self, max_error: f32) -> MeshData {
            let max = (self.size - 1) as u16;
            let mut mesh_helpers = MeshHelpers {
                max_error,
                indices: vec![0u16; self.terrain.len()],
                ..Default::default()
            };

            self.count_elements(&mut mesh_helpers, 0, 0, max, max, max, 0);
            self.count_elements(&mut mesh_helpers, max, max, 0, 0, 0, max);

            mesh_helpers.vertices = vec![[0f32; 3]; mesh_helpers.num_vertices]; // todo directly assign [f32; 3], to minimize memory use
            mesh_helpers.triangles = vec![0u16; mesh_helpers.num_triangles * 3];

            self.process_triangle(&mut mesh_helpers, 0, 0, max, max, max, 0);
            self.process_triangle(&mut mesh_helpers, max, max, 0, 0, 0, max);

            let uvs = vec![[1.0, 1.0]; mesh_helpers.num_vertices]; // todo get actual uvs
            let normals = vec![[0.0, 1.0, 0.0]; mesh_helpers.num_vertices]; // todo get actual normals

            MeshData {
                vertices: mesh_helpers.vertices,
                indices: mesh_helpers.triangles,
                uvs,
                normals,
            }
        }
    }
}

pub mod mesh_gen {
    use super::MeshData;
    use bevy::{
        prelude::Mesh,
        render::{
            mesh::{Indices, VertexAttributeValues},
            render_resource::PrimitiveTopology,
        },
    };

    pub fn generate_mesh(mesh_data: MeshData, flat_shading: bool) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(mesh_data.vertices),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(mesh_data.uvs),
        );
        mesh.set_indices(Some(Indices::U16(mesh_data.indices)));

        if flat_shading {
            mesh.duplicate_vertices();
            mesh.compute_flat_normals();
        } else {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                VertexAttributeValues::Float32x3(mesh_data.normals),
            );
        }

        mesh
    }
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

pub mod noise_gen {
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
    }

    pub fn get_noise_map(x_coord: usize, y_coord: usize, noise_config: &NoiseConfig) -> NoiseMap {
        let NoiseConfig {
            seed:_seed,
            size,
            frequency,
            lacunarity,
            octaves,
            persistence,
        } = noise_config;

        let mut fbm = Fbm::new();
        fbm.frequency = *frequency;
        fbm.lacunarity = *lacunarity;
        fbm.persistence = *persistence;
        fbm.octaves = *octaves;

        let bounds_size = (*size) as f64;
        let bounds_start_x = x_coord as f64 * bounds_size as f64;
        let bounds_end_x = (x_coord + 1) as f64 * bounds_size;
        let bounds_start_y = y_coord as f64 * bounds_size as f64;
        let bounds_end_y = (y_coord + 1) as f64 * bounds_size;

        let map = PlaneMapBuilder::new(&fbm)
            .set_size(*size, *size)
            .set_x_bounds(bounds_start_x, bounds_end_x + 1.0)
            .set_y_bounds(bounds_start_y, bounds_end_y + 1.0)
            .build();
        map
    }
}
