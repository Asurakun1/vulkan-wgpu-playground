use wgpu::{util::DeviceExt, BufferUsages};

use crate::swapchain::State;

use super::CameraUniform;

pub trait UpdateCamera {
    fn update_camera(&mut self);
}

impl UpdateCamera for State {
    fn update_camera(&mut self) {
        self.camera_state
            .controller
            .update_camera(&mut self.camera_state.camera);

        self.camera_state
            .uniform
            .update_view_proj(&self.camera_state.camera);

        let staging_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("staging buffer for camera"),
                contents: bytemuck::cast_slice(&[self.camera_state.uniform]),
                usage: BufferUsages::COPY_SRC,
            });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.camera_state.buffer,
            0,
            std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
        );

        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
