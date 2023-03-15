use cgmath::Rotation3;
use wgpu::SurfaceError;
use winit::{event::WindowEvent, window::Window};

use self::render_pipeline::create_render_pipeline;

use super::{
    camera::camera_state,
    instance::{instance_raw::InstanceRaw, InstanceState},
    light::LightState,
    model::{
        self, draw_light::DrawLight, draw_model::DrawModel, model::Model, ModelVertex, Vertex,
    },
    texture,
    triangle::{self},
};
mod bind_group_layouts;
mod render_pipeline;
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    render_pipeline: wgpu::RenderPipeline,
    light_render_pipeline: wgpu::RenderPipeline,
    triangle: triangle::Triangle,
    depth_texture: texture::Texture,
    camera_state: camera_state::CameraState,
    instances: InstanceState,
    model: Model,
    light_state: LightState,
    box_model: Model,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });

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
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let bind_group_layouts = bind_group_layouts::BindGroupLayouts::new(&device);

        /*
        Components
         */
        let camera_state =
            camera_state::CameraState::new(&device, &config, &bind_group_layouts.camera);
        let light_state = LightState::new(&device, &bind_group_layouts.light);
        let triangle = triangle::Triangle::new(&device, &queue, &bind_group_layouts.texture);
        let model = Model::load_model("Gear1.obj", &device, &queue, &bind_group_layouts.texture)
            .await
            .unwrap();
        let depth_texture = texture::Texture::create_depth_texture(&device, &config);
        let instances = InstanceState::new(&device);

        let box_model =
            Model::load_model("harold.obj", &device, &queue, &bind_group_layouts.texture)
                .await
                .unwrap();
        /*
        end of components
         */

        let render_pipeline = {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("render pipeline layout"),
                    bind_group_layouts: &[
                        &bind_group_layouts.texture,
                        &bind_group_layouts.camera,
                        &bind_group_layouts.light,
                    ],
                    push_constant_ranges: &[],
                });

            create_render_pipeline(
                &device,
                &render_pipeline_layout,
                &shader,
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[ModelVertex::desc(), InstanceRaw::desc()],
            )
        };

        let light_render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("light pipeline layout"),
                bind_group_layouts: &[&bind_group_layouts.camera, &bind_group_layouts.light],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("light shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
            });

            create_render_pipeline(
                &device,
                &layout,
                &shader,
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[ModelVertex::desc()],
            )
        };

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            light_render_pipeline,
            triangle,
            depth_texture,
            camera_state,
            instances,
            model,
            light_state,
            box_model,
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

        let old_position: cgmath::Vector3<_> = self.light_state.light_uniform.position.into();
        self.light_state.light_uniform.position =
            (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0))
                * old_position)
                .into();
        self.queue.write_buffer(
            &self.light_state.light_buffer,
            0,
            bytemuck::cast_slice(&[self.light_state.light_uniform]),
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
            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config);
        }
    }
    pub fn render(&mut self) -> Result<(), SurfaceError> {
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        render_pass.set_pipeline(&self.light_render_pipeline);
        render_pass.set_vertex_buffer(1, self.instances.buffer.slice(..));

        render_pass.draw_light_model(
            &self.box_model,
            &self.camera_state.bind_group,
            &self.light_state.bind_group,
        );

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw_model_instanced(
            &self.model,
            &self.camera_state.bind_group,
            &self.light_state.bind_group,
            0..self.instances.instances.len() as u32,
        );
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
