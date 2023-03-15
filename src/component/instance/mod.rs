use self::instance_raw::InstanceRaw;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;

pub mod instance_raw;

pub struct Instance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let model =
            cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);

        InstanceRaw {
            model: model.into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}

pub struct InstanceState {
    pub instances: Vec<Instance>,
    pub buffer: wgpu::Buffer,
}

impl InstanceState {
    const NUM_INSTANCES_PER_ROW: u32 = 10;
    const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
        Self::NUM_INSTANCES_PER_ROW as f32 * 0.5,
        Self::NUM_INSTANCES_PER_ROW as f32 * 0.5,
        Self::NUM_INSTANCES_PER_ROW as f32 * 0.5,
    );
    const SPACE_BETWEEN: f32 = 25.0;

    pub fn new(device: &wgpu::Device) -> Self {
        let instances = (0..Self::NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..Self::NUM_INSTANCES_PER_ROW).flat_map(move |x| {
                    (0..Self::NUM_INSTANCES_PER_ROW).map(move |y| {
                        let x = Self::SPACE_BETWEEN
                            * (x as f32 - Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                        let y = Self::SPACE_BETWEEN
                            * (y as f32 - Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                        let z = Self::SPACE_BETWEEN
                            * (z as f32 - Self::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                        let position = cgmath::Vector3 { x, y, z };

                        let rotation = if position.is_zero() {
                            cgmath::Quaternion::from_axis_angle(
                                cgmath::Vector3::unit_z(),
                                cgmath::Deg(0.0),
                            )
                        } else {
                            let angle = (x + y + z) as f32 * 10.0;
                            cgmath::Quaternion::from_axis_angle(
                                position.normalize(),
                                cgmath::Deg(angle),
                            )
                        };

                        Instance { rotation, position }
                    })
                })
            })
            .collect::<Vec<_>>();
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("instance buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self { instances, buffer }
    }
}
