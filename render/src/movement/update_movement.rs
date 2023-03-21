use std::time::Instant;

use crate::swapchain::State;

pub trait UpdateMovement {
    fn update_movement(&mut self);
    fn rotate(&mut self);
}

impl UpdateMovement for State {
    fn update_movement(&mut self) {
        self.movement
            .uniform
            .new_transform(&self.movement.transform);

        self.queue.write_buffer(
            &self.movement.buffer,
            0,
            bytemuck::cast_slice(&[self.movement.uniform]),
        );

        self.rotate();
    }
    fn rotate(&mut self) {
        let now = Instant::now();
        let delta_time = (now - self.start_time).as_secs_f32();
        self.start_time = now;
        self.movement.update(delta_time);
    }
}
