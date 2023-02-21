mod window;
mod surface;
use pollster;
use window::run;
fn main(){

    pollster::block_on(run());

}