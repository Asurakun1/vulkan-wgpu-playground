use super::Instance;
use cgmath::prelude::*;
use wgpu::{util::DeviceExt, BufferUsages};

pub struct State {
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,
}

impl State {
    const NUM_INSTANCES_PER_ROW: u32 = 10;
    const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
        Self::NUM_INSTANCES_PER_ROW as f32 * 0.5,
        Self::NUM_INSTANCES_PER_ROW as f32 * 0.5,
        Self::NUM_INSTANCES_PER_ROW as f32 * 0.5,
    );
    const SPACE_BETWEEN: f32 = 0.4;

    pub fn new(device: &wgpu::Device) -> Self {
        let instances = (0..Self::NUM_INSTANCES_PER_ROW)
            .flat_map(|x| {
                (0..Self::NUM_INSTANCES_PER_ROW).flat_map(move |y| {
                    (0..Self::NUM_INSTANCES_PER_ROW).map(move |z| {
                        let x = Self::SPACE_BETWEEN
                            * (x as f32 * Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                        let y = Self::SPACE_BETWEEN
                            * (y as f32 * Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                        let z = Self::SPACE_BETWEEN
                            * (z as f32 * Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);

                        let position = cgmath::Vector3 { x, y, z } - Self::INSTANCE_DISPLACEMENT;

                        let rotation = if position.is_zero() {
                            cgmath::Quaternion::from_axis_angle(
                                cgmath::Vector3::unit_z(),
                                cgmath::Deg(0.0),
                            )
                        } else {
                            cgmath::Quaternion::from_axis_angle(
                                position.normalize(),
                                cgmath::Deg(45.0),
                            )
                        };

                        Instance { position, rotation }
                    })
                })
            })
            .collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("instance buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: BufferUsages::VERTEX,
        });

        Self {
            instances,
            instance_buffer,
        }
    }
}
