use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_inspector_egui::Inspectable;
use noise::utils::NoiseMap;

#[derive(Clone, Copy, Inspectable)]
pub struct MeshConfig {
    pub scale: f32,
    pub render_mode: RenderMode,
}
impl Default for MeshConfig {
    fn default() -> Self {
        Self {
            scale: 1.0,
            render_mode: Default::default(),
        }
    }
}

pub struct MeshImageData {
    pub mesh: Mesh,
    pub image: Image,
}

#[derive(Default, Clone, Copy, Inspectable)]
pub enum RenderMode {
    #[default]
    Heightmap,
    Color,
    Wireframe,
    Mesh,
}

pub fn get_mesh(map: NoiseMap, mesh_config: MeshConfig) -> MeshImageData {
    match mesh_config.render_mode {
        RenderMode::Heightmap => get_heightmap_texture_mesh(map, mesh_config.scale),
        RenderMode::Color => get_color_texture_mesh(map, mesh_config.scale),
        RenderMode::Wireframe => todo!(),
        RenderMode::Mesh => todo!(),
    }
}

fn get_heightmap_texture_mesh(map: NoiseMap, scale: f32) -> MeshImageData {
    let size = map.size().0 as u32;

    let data = to_heightmap_vec(map);
    let mesh = Mesh::from(shape::Plane {
        size: size as f32 * scale,
    });

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

fn get_color_texture_mesh(map: NoiseMap, scale: f32) -> MeshImageData {
    let size = map.size().0 as u32;

    let data = to_color_vec(map);
    let mesh = Mesh::from(shape::Plane {
        size: size as f32 * scale,
    });

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

fn to_heightmap_vec(map: NoiseMap) -> Vec<u8> {
    let size = map.size().0;
    let mut data: Vec<u8> = Vec::with_capacity(size * size);

    for i in &map {
        let i_normalized = (i * 0.5 + 0.5).clamp(0.0, 1.0);
        let i_u8 = (i_normalized * 255.0) as u8;
        data.push(i_u8); //r
        data.push(i_u8); //g
        data.push(i_u8); // b
        data.push(255); //a
    }

    data
}

fn to_color_vec(map: NoiseMap) -> Vec<u8> {
    let size = map.size().0;
    let mut data: Vec<u8> = Vec::with_capacity(size * size);

    for i in &map {
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
