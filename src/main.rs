mod model;
mod swapchain;
mod vertex_buffer;
mod window;
mod texture;
use pollster::block_on;
use window::run;
fn main() {
    block_on(run());
}
