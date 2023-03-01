mod window;
mod swapchain;
mod vertex_buffer;
mod model;
mod texture;
use window::run;
use pollster::block_on;
fn main() {
    block_on(run())
}
