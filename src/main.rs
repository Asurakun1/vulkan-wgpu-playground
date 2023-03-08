mod component;

use component::window::run;
use pollster::block_on;
fn main() {
    block_on(run())
}
