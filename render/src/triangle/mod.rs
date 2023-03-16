use wgpu::{util::DeviceExt, BufferUsages};

use crate::render_pipeline;

use self::vertex::Vertex;

mod vertex;
pub mod draw_triangle;
pub struct Triangle {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Triangle {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
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

        let render_pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("triangle shader"),
                source: wgpu::ShaderSource::Wgsl(
                    std::fs::read_to_string("shaders/shader.wgsl")
                        .unwrap()
                        .into(),
                ),
            });

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("render pipeline layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

            render_pipeline::create_render_pipeline(
                device,
                "triangle",
                format,
                Some(&render_pipeline_layout),
                &shader,
                &[Vertex::desc()],
            )
        };

        Self {
            vertex_buffer,
            index_buffer,
            num_elements,
            render_pipeline,
        }
    }

    const VERTICES: &[vertex::Vertex] = &[
        Vertex {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];

    const INDICES: &[u32] = &[0, 1, 2];
}
