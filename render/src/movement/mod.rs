use cgmath::{Matrix4, SquareMatrix};
use wgpu::{util::DeviceExt, BufferUsages};

pub struct Transform;

impl Transform {
    pub fn transform(&self) -> cgmath::Matrix4<f32> {
        let mut model: Matrix4<f32> = cgmath::Matrix4::identity();
        let translation = cgmath::Vector3::new(0.0, 0.0, 0.0);
        model = model * Matrix4::from_translation(translation);

        let rotation = cgmath::Deg(45.0);
        model = model * Matrix4::from_angle_x(rotation);
        model = model * Matrix4::from_angle_y(rotation);
        model = model * Matrix4::from_angle_z(rotation);

        let scale = cgmath::Vector3::new(1.0, 1.0, 1.0);
        model = model * Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z);

        model
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MovementUniform {
    model: [[f32; 4]; 4],
}

impl MovementUniform {
    pub fn new() -> Self {
        let model = Matrix4::<f32>::identity();

        Self {
            model: model.into(),
        }
    }

    pub fn new_transform(&mut self, trans: &Transform) {
        self.model = trans.transform().into();
    }
}

pub struct Movement {
    pub uniform: MovementUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub transform: Transform,
}

impl Movement {
    pub fn new(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> Self {
        let transform = Transform;
        let mut uniform = MovementUniform::new();
        uniform.new_transform(&transform);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("movement buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("movement bind group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            uniform,
            buffer,
            bind_group,
            transform,
        }
    }
}
