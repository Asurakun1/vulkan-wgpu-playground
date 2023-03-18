use std::ops::Range;

use wgpu::RenderPass;

use super::Triangle;

pub trait DrawTriangle<'a> {
    fn draw_triangle(&mut self, triangle: &'a Triangle);
    fn draw_triangle_indexed(&mut self, triangle: &'a Triangle, instances: Range<u32>);
}

impl<'a, 'b> DrawTriangle<'b> for RenderPass<'a>
where
    'b: 'a,
{
    fn draw_triangle(&mut self, triangle: &'a Triangle) {
        self.draw_triangle_indexed(triangle, 0..1);
    }
    fn draw_triangle_indexed(&mut self, triangle: &'a Triangle, instances: Range<u32>) {
        self.set_bind_group(0, &triangle.bind_group, &[]);
        self.set_vertex_buffer(0, triangle.vertex_buffer.slice(..));
        self.set_index_buffer(triangle.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..triangle.num_elements, 0, instances);
    }
}
