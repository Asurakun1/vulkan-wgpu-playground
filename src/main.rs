mod Texture;
mod triangle;
mod component;
use pollster::block_on;
use component::window::run;
fn main() {
    block_on(run())
}
