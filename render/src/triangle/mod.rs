use wgpu::{util::DeviceExt, BufferUsages};

use crate::texture::Texture;

use self::vertex::Vertex;

pub mod draw_triangle;
pub mod vertex;
pub struct Triangle {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub bind_group: wgpu::BindGroup,
}

impl Triangle {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(Self::VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(Self::INDICES),
            usage: BufferUsages::INDEX,
        });

        let num_elements = Self::INDICES.len() as u32;

        let texture = Texture::from_bytes(device, queue, "harold-01.jpg");
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture bind group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_elements,
            bind_group,
        }
    }

    const VERTICES: &[vertex::Vertex] = &[
        Vertex {
            position: [0.0, 0.5, 0.0],
            tex_coords: [1.0, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coords: [0.0, 1.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            tex_coords: [0.0, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];

    const INDICES: &[u32] = &[0, 1, 2];
}
