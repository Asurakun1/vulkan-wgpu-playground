mod surface;
mod window;
mod vertex_buffer;


use pollster::block_on;
use window::run;

fn main() {
    block_on(run());
}
