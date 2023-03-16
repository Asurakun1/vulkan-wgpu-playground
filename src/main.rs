use pollster::block_on;
use render::window::run;
fn main() {
    block_on(run());
}
