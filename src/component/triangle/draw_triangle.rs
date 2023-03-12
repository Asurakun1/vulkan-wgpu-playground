use std::ops::Range;

use crate::component::instance::InstanceState;

use super::Triangle;

pub trait DrawTriangle<'a> {
    fn draw_triangle(&mut self, triangle: &'a Triangle, instances: &'a InstanceState);
    fn draw_instanced_triangle(
        &mut self,
        triangle: &'a Triangle,
        range: Range<u32>,
        instances: &'a InstanceState,
    );
}

impl<'a, 'b> DrawTriangle<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_triangle(&mut self, triangle: &'b Triangle, instances: &'b InstanceState) {
        self.draw_instanced_triangle(triangle, 0..1, instances);
    }

    fn draw_instanced_triangle(
        &mut self,
        triangle: &'b Triangle,
        range: Range<u32>,
        instances: &'b InstanceState,
    ) {
        self.set_bind_group(0, &triangle.bind_group, &[]);
        self.set_vertex_buffer(0, triangle.vertex_buffer.slice(..));
        self.set_vertex_buffer(1, instances.buffer.slice(..));
        self.set_index_buffer(triangle.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..triangle.num_elements, 0, range);
    }
}
