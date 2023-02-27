use crate::m6502::CPU;
use egui::{Align2, Context};

pub(crate) struct TopPanel {
    about_window_open: bool,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            about_window_open: false,
        }
    }

    pub fn draw(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Help", |ui| {
                    if ui.button("About...").clicked() {
                        self.about_window_open = true;
                        ui.close_menu();
                    }
                })
            });

            egui::Window::new("About")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .open(&mut self.about_window_open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("NES emulator made by Anton Gusev.");
                });
        });
    }
}

pub(crate) fn draw_app(ctx: &Context, top_panel: &mut TopPanel, cpu: &mut CPU) {
    top_panel.draw(ctx);

    egui::Window::new("Status").show(ctx, |ui| {
        cpu.status.render(ui);
    });
}
