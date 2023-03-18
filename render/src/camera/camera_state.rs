use wgpu::{util::DeviceExt, BufferUsages};

use super::{controller, Camera, CameraUniform};

pub struct CameraState {
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub controller: controller::Controller,
}

impl CameraState {
    pub fn new(
        config: &wgpu::SurfaceConfiguration,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera bind group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let controller = controller::Controller::new(0.1);

        Self {
            camera,
            camera_buffer,
            camera_uniform,
            bind_group,
            controller,
        }
    }
}
