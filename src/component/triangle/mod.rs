use wgpu::{util::DeviceExt, BufferUsages};

use super::texture;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]

pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 1,
                },
            ],
        }
    }
}

pub struct Triangle {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub num_size: u32,
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

        let num_size = Self::INDICES.len() as u32;

        let texture = texture::Texture::from_bytes("Asura.png", device, queue);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("triangle texture bind group"),
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
            num_size,
            bind_group,
        }
    }

    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [0.0, 0.5, 0.0],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            tex_coords: [0.0, 0.0],
        },
    ];

    const INDICES: &[u32] = &[0, 1, 2];
}
