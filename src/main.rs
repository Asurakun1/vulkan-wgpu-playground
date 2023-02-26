mod surface;
mod window;
mod vertex_buffer;
mod texture;
mod camera;
mod camera_controller;

use pollster::block_on;
use window::run;
fn main() {
    block_on(run());
}
