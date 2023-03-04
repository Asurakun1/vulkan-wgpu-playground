mod swapchain;
mod window;
mod vertex_buffer;
mod model;

use pollster::block_on;
use window::run;

fn main() {
    block_on(run())
}
