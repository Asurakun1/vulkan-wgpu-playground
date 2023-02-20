mod window;
mod surface;
use pollster;
fn main(){

    pollster::block_on(window::run());

}