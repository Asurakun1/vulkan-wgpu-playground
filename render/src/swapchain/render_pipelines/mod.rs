pub mod pipeline_state;

pub fn create_render_pipeline(
    device: &wgpu::Device,
    name: &str,
    format: wgpu::TextureFormat,
    layout: Option<&wgpu::PipelineLayout>,
    shader: &wgpu::ShaderModule,
    vertex_buffer_layouts: &[wgpu::VertexBufferLayout],
    depth_format: Option<wgpu::TextureFormat>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(name),
        layout,
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: vertex_buffer_layouts,
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None, //Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: depth_format.map(|f| wgpu::DepthStencilState {
            format: f,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    })
}
