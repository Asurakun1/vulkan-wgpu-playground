#![windows_subsystem = "windows"]
mod surface;
mod vertex_buffer;
mod window;

use pollster::block_on;
use window::run;
fn main() {
    block_on(run());
}
