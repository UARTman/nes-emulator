use crate::snake::MCSnakeCanvas;
use egui::{Color32, Frame, Style};
use harness::Harness;
use macroquad::prelude::*;
use snake_game::snake_cpu;

pub mod harness;
pub mod snake;

trait EguiWindowTransparentExt {
    fn transparent(self) -> Self;
}

impl<'a> EguiWindowTransparentExt for egui::Window<'a> {
    fn transparent(self) -> Self {
        let fr = Frame::window(&Style::default());
        let col = fr.fill;
        let fr1 = fr.fill(Color32::from_rgba_unmultiplied(
            col.r(),
            col.g(),
            col.b(),
            245,
        ));
        self.frame(fr1)
    }
}

#[macroquad::main("6502 Emulator")]
async fn main() {
    let mut harness = Harness::new(snake_cpu::<MCSnakeCanvas>());
    harness.cpu.pc = 0x600;
    harness.frequency = 10000;
    let texture = Texture2D::from_image(&harness.cpu.bus.canvas.image);
    texture.set_filter(FilterMode::Nearest);
    let mut draw_params = DrawTextureParams::default();
    let margin = 30.0;
    let mut width = screen_width();
    let mut height = screen_height();
    let mut screen_m = if screen_width() > screen_height() {
        screen_height()
    } else {
        screen_width()
    };
    let mut side = screen_m - 2.0 * margin;
    draw_params.dest_size = Some(Vec2 { x: side, y: side });
    let mut cpu_window_open = true;

    loop {
        let fps = get_fps();
        let cpf = harness.frequency / (fps as u32) + 1;

        if width != screen_width() || height != screen_height() {
            width = screen_width();
            height = screen_height();
            screen_m = if screen_width() > screen_height() {
                screen_height()
            } else {
                screen_width()
            };
            side = screen_m - 2.0 * margin;
            draw_params.dest_size = Some(Vec2 { x: side, y: side });
        }

        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            harness.cpu.bus.memory[0xFF] = 0x77;
        } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            harness.cpu.bus.memory[0xFF] = 0x61;
        } else if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            harness.cpu.bus.memory[0xFF] = 0x64;
        } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            harness.cpu.bus.memory[0xFF] = 0x73;
        }

        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::TopBottomPanel::top("global-top").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("View", |ui| {
                            ui.checkbox(&mut cpu_window_open, "CPU");
                        })
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("FPS: {fps}. CPF: {cpf}."));
                    });
                });
            });

            egui::Window::new("CPU")
                .transparent()
                .open(&mut cpu_window_open)
                .show(egui_ctx, |ui| {
                    harness.render(ui);
                });
        });

        texture.update(&harness.cpu.bus.canvas.image);
        draw_texture_ex(
            texture,
            (screen_width() - screen_m) / 2.0 + margin,
            (screen_height() - screen_m) / 2.0 + margin,
            WHITE,
            draw_params.clone(),
        );

        // Draw things before egui
        egui_macroquad::draw();
        harness.frame(cpf);

        next_frame().await
    }
}
