#![feature(bigint_helper_methods)]

use crate::gui::Framework;
use crate::m6502::CPU;
use egui::Context;
use log::{error, info};
use m6502::bus::{Bus, Ram, Snake};
use m6502::CPUError;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use thiserror::Error;
use winit::dpi::LogicalSize;
use winit::error::OsError;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub mod gui;
pub mod m6502;

#[derive(Error, Debug)]
pub enum NESEmulatorError {
    #[error("Couldn't create a window: {0}")]
    WindowError(#[source] OsError),
    #[error("Pixels lib failed: {0}")]
    PixelsError(#[from] pixels::Error),
    #[error("CPU has encountered an error.")]
    CPUError(#[from] CPUError),
}

#[derive(Debug)]
enum HarnessState {
    Paused,
    Running,
    Error(CPUError),
}

impl HarnessState {
    pub fn is_paused(&self) -> bool {
        match self {
            HarnessState::Paused => true,
            _ => false,
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self, HarnessState::Running)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, HarnessState::Error(_))
    }

    pub fn error(&self) -> Option<&CPUError> {
        match self {
            HarnessState::Error(e) => Some(e),
            _ => None,
        }
    }
}

struct CPUHarness<T: Bus> {
    cpu: CPU<T>,
    frequency: usize,
    state: HarnessState,
}

impl<T: Bus> CPUHarness<T> {
    pub fn new(bus: T) -> Self {
        Self {
            cpu: CPU::new(bus),
            frequency: 1,
            state: HarnessState::Paused,
        }
    }

    pub fn tick(&mut self) {
        if !self.state.is_running() {
            return;
        }
        // TODO: implement frequency
        if let Err(e) = self.cpu.tick() {
            self.state = HarnessState::Error(e);
        }
    }

    pub fn render(&mut self, ctx: &Context) {
        egui::Window::new("CPU Harness").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if !self.state.is_error() {
                    ui.label(&format!("CPU is {:#?}", self.state));
                } else {
                    ui.label(&format!(
                        "CPU has encountered an error: {}",
                        self.state.error().unwrap()
                    ));
                }

                match self.state {
                    HarnessState::Paused => {
                        if ui.button("Resume").clicked() {
                            self.state = HarnessState::Running;
                        }
                        if ui.button("Single-step").clicked() {
                            self.state = HarnessState::Running;
                            self.cpu.cycles_left = 0;
                            self.tick();
                            self.state = HarnessState::Paused;
                        }
                    }
                    HarnessState::Running => {
                        if ui.button("Pause").clicked() {
                            self.state = HarnessState::Paused;
                        }
                    }
                    HarnessState::Error(_) => {
                        ui.label("TODO: Implement reset on errors.");
                    }
                }
            });
            ui.set_enabled(!self.state.is_running());
            ui.add(egui::DragValue::new(&mut self.frequency));
            ui.set_enabled(true);
            self.cpu.render(ui);
        });
    }
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
        info!("scale factor: {scale_factor}");
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(32, 32, surface_texture)?;
        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            // scale_factor,
            1.0,
            &pixels,
        );

        (pixels, framework)
    };

    let mut top_panel = gui::egui_main::TopPanel::new();

    let mut cpu = CPUHarness::new(Snake::new());
    cpu.cpu.pc = 0x0600;

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
            if input.key_pressed(VirtualKeyCode::W) || input.key_pressed(VirtualKeyCode::Up) {
                cpu.cpu.bus.memory[0xFF] = 0x77;
            } else if input.key_pressed(VirtualKeyCode::A)
                || input.key_pressed(VirtualKeyCode::Left)
            {
                cpu.cpu.bus.memory[0xFF] = 0x61;
            } else if input.key_pressed(VirtualKeyCode::D)
                || input.key_pressed(VirtualKeyCode::Right)
            {
                cpu.cpu.bus.memory[0xFF] = 0x64;
            } else if input.key_pressed(VirtualKeyCode::S)
                || input.key_pressed(VirtualKeyCode::Down)
            {
                cpu.cpu.bus.memory[0xFF] = 0x73;
            }
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => framework.handle_event(&event),
            Event::RedrawRequested(_) => {
                framework.prepare(&window, |ctx| {
                    gui::egui_main::draw_app(ctx, &mut top_panel, &mut cpu);
                    egui::Window::new("Snake game debug").show(ctx, |ui| {
                        ui.label(format!("Apple location: 0x{:04x}", cpu.cpu.bus.read_word(0)));
                        ui.label(format!("Snake location: 0x{:04x}", cpu.cpu.bus.read_word(0x10)));
                        ui.label(format!("Snake length: {}", cpu.cpu.bus.memory[3]));
                        ui.label(format!("Snake direction: {}", cpu.cpu.bus.memory[2]));
                        ui.label(format!("Input byte: 0x{:02x}", cpu.cpu.bus.memory[0xFF]));
                    });
                });

                pixels.get_frame_mut().copy_from_slice(&cpu.cpu.bus.pixbuf);

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
        for _ in 0..(cpu.frequency) {   
            cpu.tick();
        }
    });
}
