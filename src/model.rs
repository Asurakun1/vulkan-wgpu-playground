use crate::texture::Texture;
use wgpu::util::DeviceExt;

use crate::vertex_buffer::Vertex;

pub struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub num_indices: u32,
}

impl Model {
    #[rustfmt::skip]
    pub const VERTICES: &[Vertex] = &[
        Vertex { position: [0.0, 0.5, 0.0], tex_coords: [1.0, 0.5] }, //A
        Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0] }, //B
        Vertex { position: [0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] }, //C
    ];

    #[rustfmt::skip]
    pub const INDICES: &[u16] = &[
        0, 1, 2
        ];

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

        let (diffuse_bind_group, texture_bind_group_layout) =
            Self::diffuse_bind_group(device, queue);

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            texture_bind_group_layout,
        }
    }

    fn diffuse_bind_group(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
        let diffuse_bytes = include_bytes!("../res/Asura.png");
        let diffuse_texture = Texture::from_bytes(device, queue, diffuse_bytes, "Asura").unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("diffuse texture bind group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
        });

        (diffuse_bind_group, texture_bind_group_layout)
    }
}
