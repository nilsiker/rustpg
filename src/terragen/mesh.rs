use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::{Extent3d, PrimitiveTopology, TextureDimension, TextureFormat};
use bevy_inspector_egui::Inspectable;
use noise::utils::NoiseMap;

#[derive(Clone, Copy, Inspectable)]
pub enum GenerationMode {
    Simple,
    Rtin,
}

struct MeshData {
    vertices: Vec<[f32; 3]>,
    indices: Vec<u32>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
}

#[derive(Inspectable)]
pub struct MeshConfig {
    pub grid_size: usize,
    pub scale: f32,
    pub height_multiplier: f32,
    pub render_mode: RenderMode,
    pub generation_mode: GenerationMode,
    pub flat_shading: bool,
}
impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            grid_size: 257,
            scale: 256.0,
            height_multiplier: 1.0,
            render_mode: Default::default(),
            generation_mode: GenerationMode::Simple,
            flat_shading: true,
        }
    }
}

pub struct MeshImageData {
    pub mesh: Mesh,
    pub image: Image,
}

#[derive(Default, Clone, Copy, Inspectable)]
pub enum RenderMode {
    Heightmap,
    Color,
    Wireframe,
    #[default]
    Mesh,
}

pub fn get_mesh(map: &NoiseMap, mesh_config: &MeshConfig) -> MeshImageData {
    match mesh_config.render_mode {
        RenderMode::Heightmap => get_heightmap_texture_mesh(map, mesh_config.scale),
        RenderMode::Color => get_color_texture_mesh(map, mesh_config.scale),
        RenderMode::Wireframe => todo!(),
        RenderMode::Mesh => match mesh_config.generation_mode {
            GenerationMode::Simple => generate_simple_mesh(map, mesh_config),
            GenerationMode::Rtin => generate_rtin_mesh(map, mesh_config),
        },
    }
}
fn get_heightmap_texture_mesh(map: &NoiseMap, scale: f32) -> MeshImageData {
    let size = map.size().0 as u32;

    let data = to_heightmap_vec(map);
    let mesh = Mesh::from(shape::Plane { size: scale });

    let image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data.as_slice(),
        TextureFormat::Rgba8UnormSrgb,
    );

    MeshImageData { mesh, image }
}

fn get_color_texture_mesh(map: &NoiseMap, scale: f32) -> MeshImageData {
    let size = map.size().0 as u32;

    let data = to_color_vec(map);
    let mesh = Mesh::from(shape::Plane { size: scale });

    let image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data.as_slice(),
        TextureFormat::Rgba8UnormSrgb,
    );

    MeshImageData { mesh, image }
}

fn generate_simple_mesh(map: &NoiseMap, mesh_config: &MeshConfig) -> MeshImageData {
    let size = map.size().0 as u32;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mesh_data = generate_simple_mesh_data(map, mesh_config);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_data.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_data.uvs);
    mesh.set_indices(Some(Indices::U32(mesh_data.indices)));

    if mesh_config.flat_shading {
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();
    } else {
        todo!() // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_data.normals);
    }

    let image = Image::new_fill(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        to_heightmap_vec(map).as_slice(), // TODO replace?
        TextureFormat::Rgba8UnormSrgb,
    );

    MeshImageData { mesh, image }
}

fn generate_simple_mesh_data(map: &NoiseMap, mesh_config: &MeshConfig) -> MeshData {
    let (width, height) = map.size();


    let scale = mesh_config.scale;

    let top_left_x = (width-1) as f32 / -2.0;
    let top_left_z = (height-1) as f32 / 2.0;

    let mut vertices = vec![[0.0; 3]; height * width];

    let mut indices = vec![0; (height - 1) * (width - 1) * 6];

    let mut uvs = vec![[0.0; 2]; height * width];

    let mut vertex_index = 0;
    let mut triangle_index = 0;

    let mut add_triangle = |a: usize, b: usize, c: usize| {
        indices[triangle_index] = a as u32;
        indices[triangle_index + 1] = b as u32;
        indices[triangle_index + 2] = c as u32;
        triangle_index += 3;
    };

    for y in 0..height {
        for x in 0..width {
            let xf = x as f32;
            let zf = y as f32;
            let height_value = map.get_value(x, y) as f32 * mesh_config.height_multiplier;

            vertices[vertex_index] = [
                (top_left_x + xf) / (width - 1) as f32 * scale ,
                height_value,
                (top_left_z - zf) / (height - 1) as f32 * scale,
            ];
            uvs[vertex_index] = [x as f32 / width as f32, y as f32 / height as f32];

            if x < width - 1 && y < height - 1 {
                add_triangle(vertex_index, vertex_index + width + 1, vertex_index + width);
                add_triangle(vertex_index + width + 1, vertex_index, vertex_index + 1);
            }

            vertex_index += 1;
        }
    }

    MeshData {
        vertices,
        indices,
        uvs,
        normals: vec![],
    }
}

fn generate_rtin_mesh(map: &NoiseMap, mesh_config: &MeshConfig) -> MeshImageData {
    todo!()
}

fn to_heightmap_vec(map: &NoiseMap) -> Vec<u8> {
    let size = map.size().0;
    let mut data: Vec<u8> = Vec::with_capacity(size * size);

    for i in map {
        let i_normalized = (i * 0.5 + 0.5).clamp(0.0, 1.0);
        let i_u8 = (i_normalized * 255.0) as u8;
        data.push(i_u8); //r
        data.push(i_u8); //g
        data.push(i_u8); // b
        data.push(255); //a
    }

    data
}

fn to_color_vec(map: &NoiseMap) -> Vec<u8> {
    let size = map.size().0;
    let mut data: Vec<u8> = Vec::with_capacity(size * size);

    for i in map {
        let i = *i;
        let rgb = if i < 0.0 {
            (50, 50, 255)
        } else if i <= 1.0 {
            (50, 255, 50)
        } else if i > 1.0 && i < 1.5 {
            (127, 127, 127)
        } else {
            (255, 255, 255)
        };
        data.push(rgb.0); //r
        data.push(rgb.1); //g
        data.push(rgb.2); // b
        data.push(255); //a
    }

    data
}
