use wgpu::{util::DeviceExt, BufferUsages};

use self::light_uniform::LightUniform;

mod light_uniform;

pub struct LightState {
    pub light_buffer: wgpu::Buffer,
    pub light_uniform: LightUniform,
    pub bind_group: wgpu::BindGroup,
}

impl LightState {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let light_uniform = LightUniform {
            position: [25.0, 4.0, 0.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        };

        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("light buffer"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light bind group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
        });

        Self {
            light_buffer,
            light_uniform,
            bind_group,
        }
    }
}
