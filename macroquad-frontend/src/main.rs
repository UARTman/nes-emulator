use std::cmp::min;

use harness::Harness;
use m6502::{
    bus::{Snake, SnakeCanvas},
    CPU,
};
use macroquad::prelude::*;

pub mod harness;

pub struct MCSnakeCanvas {
    pub image: Image,
}

impl Default for MCSnakeCanvas {
    fn default() -> Self {
        let mut image = Image {
            bytes: vec![],
            width: 32,
            height: 32,
        };
        for _ in 0..32 * 32 {
            image.bytes.push(60);
            image.bytes.push(60);
            image.bytes.push(60);
            image.bytes.push(255);
        }
        image.width = 32;
        image.height = 32;
        Self { image }
    }
}

impl SnakeCanvas for MCSnakeCanvas {
    fn write_pixel(&mut self, at: usize, colors: (u8, u8, u8, u8)) {
        self.image.bytes[at * 4] = colors.0;
        self.image.bytes[at * 4 + 1] = colors.1;
        self.image.bytes[at * 4 + 2] = colors.2;
        self.image.bytes[at * 4 + 3] = colors.3;
    }
}

#[macroquad::main("6502 Emulator")]
async fn main() {
    let mut harness = Harness::new(CPU::new(Snake::<MCSnakeCanvas>::new()));
    harness.cpu.pc = 0x600;
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
    let mut fps_monitor_open = true;
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
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("View", |ui| {
                        ui.checkbox(&mut fps_monitor_open, "FPS monitor");
                        ui.checkbox(&mut cpu_window_open, "CPU");
                    })
                })
            });

            egui::Window::new("FPS monitor").open(&mut fps_monitor_open).show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("FPS: {}", fps));
                    ui.label(format!("Cycles per frame: {}", cpf));
                });
            });
            egui::Window::new("CPU").show(egui_ctx, |ui| {
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
