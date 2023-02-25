mod window;
mod surface;
mod vertex_buffer;

use window::run;
use pollster::block_on;
fn main(){
    block_on(run());
}