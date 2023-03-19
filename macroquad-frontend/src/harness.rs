use egui::Ui;
use m6502::{bus::Bus, CPUError, CPU};

#[derive(Debug)]
pub enum HarnessState {
    Paused,
    Running,
    Error(CPUError),
}

impl HarnessState {
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

pub struct Harness<T: Bus + Clone> {
    pub old_cpu: CPU<T>,
    pub cpu: CPU<T>,
    pub frequency: u32,
    pub state: HarnessState,
}

impl<T: Bus + Clone> Harness<T> {
    pub fn new(cpu: CPU<T>) -> Self {
        Self {
            old_cpu: cpu.clone(),
            cpu,
            frequency: 60,
            state: HarnessState::Paused,
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
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
                    self.single_step()
                }
            }
            HarnessState::Running => {
                if ui.button("Pause").clicked() {
                    self.state = HarnessState::Paused;
                }
            }
            HarnessState::Error(_) => {
                if ui.button("Reset").clicked() {
                    self.cpu = self.old_cpu.clone();
                    self.state = HarnessState::Paused;
                }
            }
        }
        ui.horizontal(|ui| {
            ui.label("Frequency:");
            ui.add(egui::DragValue::new(&mut self.frequency).clamp_range(60..=1000000));
        });
        self.cpu.render(ui);
    }

    pub fn frame(&mut self, cpf: u32) {
        if self.state.is_running() {
            for _ in 0..cpf {
                if let Err(e) = self.cpu.tick() {
                    self.state = HarnessState::Error(e);
                    break;
                }
            }
        }
    }

    pub fn single_step(&mut self) {
        if let Err(e) = self.cpu.execute() {
            self.state = HarnessState::Error(e);
        }
    }
}
