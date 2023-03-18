use wgpu::TextureFormat;

use crate::{swapchain::bind_grp_layouts::BindGroupLayouts, triangle::vertex::Vertex};

use super::create_render_pipeline;

pub struct PipelineState {
    pub bind_group_layouts: BindGroupLayouts,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl PipelineState {
    pub fn new(device: &wgpu::Device, format: TextureFormat) -> Self {
        let bind_group_layouts = BindGroupLayouts::new(device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[&bind_group_layouts.texture, &bind_group_layouts.camera],
                push_constant_ranges: &[],
            });

        let render_pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("triangle shader"),
                source: wgpu::ShaderSource::Wgsl(
                    std::fs::read_to_string("shaders/shader.wgsl")
                        .unwrap()
                        .into(),
                ),
            });

            create_render_pipeline(
                device,
                "triangle",
                format,
                Some(&render_pipeline_layout),
                &shader,
                &[Vertex::desc()],
            )
        };
        Self {
            render_pipeline,
            bind_group_layouts,
        }
    }
}
