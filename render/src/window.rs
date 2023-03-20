use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::swapchain::State;

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Asura's engine")
        .with_inner_size(LogicalSize {
            width: 1280,
            height: 720,
        })
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id,
            ref event,
        } if window_id == state.window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(phyiscal_size) => {
                        state.resize(*phyiscal_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }

                    _ => {}
                }
            }
        }

        Event::RedrawRequested(window_id) if window_id == state.window.id() => {
            state.update();

            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(wgpu::SurfaceError::Outdated) => state.resize(state.window.inner_size()),
                Err(e) => eprint!("{:?}", e),
            }
        }

        Event::MainEventsCleared => {
            state.window.request_redraw();
        }

        _ => {}
    });
}
