use std::path::PathBuf;
use winit::{event::WindowEvent, window::Window};

use crate::{
    camera::{self, camera_state},
    triangle::{self, draw_triangle::DrawTriangle},
};

use self::render_pipelines::pipeline_state::PipelineState;

mod bind_grp_layouts;
mod render_pipelines;
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    triangle: triangle::Triangle,
    pipelines: PipelineState,
    camera_state: camera_state::CameraState,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = {
            let dxc_path = PathBuf::from("dx12c/dxcompiler.dll");
            let dxil_path = PathBuf::from("dx12c/dxil.dll");

            wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN,
                dx12_shader_compiler: wgpu::Dx12Compiler::Dxc {
                    dxil_path: Some(dxil_path),
                    dxc_path: Some(dxc_path),
                },
            })
        };

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps
            .formats
            .into_iter()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let pipelines = PipelineState::new(&device, config.format);

        let triangle =
            triangle::Triangle::new(&device, &queue, &pipelines.bind_group_layouts.texture);

        let camera_state =
            camera_state::CameraState::new(&config, &device, &pipelines.bind_group_layouts.camera);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            triangle,
            pipelines,
            camera_state,
        }
    }

    pub fn update(&mut self) {
        self.camera_state
            .controller
            .update_camera(&mut self.camera_state.camera);
        self.camera_state
            .camera_uniform
            .update_view_proj(&self.camera_state.camera);
        self.queue.write_buffer(
            &self.camera_state.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_state.camera_uniform]),
        );
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_state.controller.process_events(event)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.height > 0 && new_size.width > 0 {
            self.size = new_size;
            self.config.height = new_size.height;
            self.config.width = new_size.width;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("render view"),
            ..Default::default()
        });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        /*

        entire render sequenece for scenes etc.

         */
        render_pass.set_pipeline(&self.pipelines.render_pipeline);
        render_pass.set_bind_group(1, &self.camera_state.bind_group, &[]);
        render_pass.draw_triangle_indexed(&self.triangle, 0..1);

        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}