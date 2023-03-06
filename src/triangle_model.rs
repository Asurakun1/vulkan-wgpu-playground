use wgpu::util::DeviceExt;

use crate::texture::Texture;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

pub struct Triangle {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_size: u32,
    pub texture: Texture,
}

impl Triangle {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangle vertex buffer"),
            contents: bytemuck::cast_slice(Self::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_size = Self::INDICES.len() as u32;
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangle index buffer"),
            contents: bytemuck::cast_slice(Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let texture = Texture::new(device, queue, "Asura.png");

        Self {
            vertex_buffer,
            num_size,
            index_buffer,
            texture,
        }
    }


    #[rustfmt::skip]
    const VERTICES: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 1.0 - 0.99240386], }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 1.0 - 0.56958647], }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 1.0 - 0.05060294], }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 1.0 - 0.1526709], }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 1.0 - 0.7347359], }, // E
    ];

    #[rustfmt::skip]
    const INDICES: &[u32] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];
}
