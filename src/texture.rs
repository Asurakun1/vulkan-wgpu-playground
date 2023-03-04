use std::num::NonZeroU32;

use image::{EncodableLayout, GenericImageView};
use wgpu::Extent3d;

pub struct Texture {}

impl Texture {
    pub fn new(file: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        let path = std::fs::read(file).unwrap();
        let bytes = path.as_bytes();
        let image = image::load_from_memory(bytes).unwrap();
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = Self::texture(device, texture_size);

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 1,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * dimensions.0),
                rows_per_image: NonZeroU32::new(dimensions.1),
            },
            texture_size,
        );
    }

    fn texture(device: &wgpu::Device, size: Extent3d) -> wgpu::Texture {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        texture
    }
}
