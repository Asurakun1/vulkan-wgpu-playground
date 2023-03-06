mod camera;
mod camera_controller;
mod instance;
mod model;
mod swapchain;
mod texture;
mod triangle_model;
mod vertex_buffer;
mod window;

use pollster::block_on;
use window::run;
fn main() {
    block_on(run())
}
