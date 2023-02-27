use crate::gui::Framework;
use crate::m6502::CPU;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use thiserror::Error;
use winit::dpi::LogicalSize;
use winit::error::OsError;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub mod gui;
pub mod m6502;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[derive(Error, Debug)]
pub enum NESEmulatorError {
    #[error("Couldn't create a window: {0}")]
    WindowError(#[source] OsError),
    #[error("Pixels lib failed: {0}")]
    PixelsError(#[from] pixels::Error),
}

fn main() -> Result<(), NESEmulatorError> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(640, 480);
        WindowBuilder::new()
            .with_title("Hello Pixels + egui!")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .map_err(NESEmulatorError::WindowError)?
    };

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    let mut top_panel = gui::egui_main::TopPanel::new();
    let mut cpu = CPU::new();

    event_loop.run(move |event, _wt, control_flow| {
        if input.update(&event) {
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
            }
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => framework.handle_event(&event),
            Event::RedrawRequested(_) => {
                framework.prepare(&window, |ctx| {
                    gui::egui_main::draw_app(ctx, &mut top_panel, &mut cpu);
                });

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    error!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}
