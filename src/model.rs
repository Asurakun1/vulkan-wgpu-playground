use wgpu::util::DeviceExt;

use crate::{vertex_buffer::Vertex, texture::Texture};

pub struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub texture: Texture
}

impl Model {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(Model::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index buffer"),
            contents: bytemuck::cast_slice(Model::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = Model::INDICES.len() as u32;

        let texture = Texture::new(device, queue);

        Self {
            vertex_buffer,
            num_indices,
            index_buffer,
            texture
        }
    }

    #[rustfmt::skip]
    const VERTICES: &[Vertex] = &[
        Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] }, // bottom left
        Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] }, // bottom right
        Vertex { position: [-0.5, 0.5, 0.0], color: [0.0, 0.0, 1.0] }, // top left
        Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 1.0, 0.0] } // top right
    ];

    #[rustfmt::skip]
    const INDICES: &[u16] = &[
        0, 1, 2, 2, 1, 3

    ];
}
