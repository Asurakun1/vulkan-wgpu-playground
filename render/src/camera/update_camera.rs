use crate::swapchain::State;

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

        self.queue.write_buffer(
            &self.camera_state.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_state.uniform]),
        );
    }
}
