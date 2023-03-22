use std::time::Instant;

use wgpu::util::DeviceExt;

use crate::swapchain::State;

use super::MovementUniform;

pub trait UpdateMovement {
    fn update_movement(&mut self);
    fn rotate(&mut self);
}

impl UpdateMovement for State {
    fn update_movement(&mut self) {
        self.rotate();
        self.movement
            .uniform
            .new_transform(&self.movement.transform);

        let staging_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("staging buffer"),
                contents: bytemuck::cast_slice(&[self.movement.uniform]),
                usage: wgpu::BufferUsages::COPY_SRC,
            });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.movement.buffer,
            0,
            std::mem::size_of::<MovementUniform>() as wgpu::BufferAddress,
        );

        self.queue.submit(std::iter::once(encoder.finish()));
    }
    fn rotate(&mut self) {
        let now = Instant::now();
        let delta_time = (now - self.start_time).as_secs_f32();
        self.start_time = now;
        self.movement.update(delta_time);
    }
}
