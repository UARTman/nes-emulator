#![feature(bigint_helper_methods)]

use egui::{text::LayoutJob, TextFormat, Ui};
use status::Status;
use thiserror::Error;

use self::{addressing_modes::OperandData, bus::Bus, instructions::Instruction};

/// Addressing modes and how the CPU uses them
pub mod addressing_modes;
/// Memory bus
pub mod bus;
/// Main instruction logic
pub mod execution;
/// A list of instructions
pub mod instructions;
/// Opcode table generation and parsing
pub mod opcode_table;
/// Functions forking with stack.
pub mod stack;
/// Status register
pub mod status;

/// A 6502 CPU
#[derive(Clone)]
pub struct CPU<T: Bus> {
    /// Program counter
    pub pc: u16,
    /// Accumulator
    pub ac: u8,
    /// Register X
    pub x: u8,
    /// Register Y
    pub y: u8,
    /// Status register
    pub status: Status,
    /// Stack pointer
    ///
    /// The stack is LIFO, top-down, 8 bit range, 0x0100 - 0x01FF.
    pub stack_pointer: u8,
    pub bus: T,
    pub cycles_left: u8,
}

impl<T> CPU<T>
where
    T: Bus,
{
    pub fn new(bus: T) -> Self {
        Self {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            status: Status::default(),
            stack_pointer: 0,
            bus,
            cycles_left: 0,
        }
    }
}
impl<T: Bus> CPU<T> {
    pub fn fetch_byte(&mut self) -> u8 {
        let r = self.bus.read(self.pc as usize);
        self.pc += 1;
        r
    }

    pub fn fetch_word(&mut self) -> u16 {
        let r = self.bus.read_word(self.pc as usize);
        self.pc += 2;
        r
    }

    pub fn tick(&mut self) -> Result<(), CPUError> {
        if self.cycles_left == 0 {
            self.execute()?;
        }
        self.cycles_left -= 1;
        Ok(())
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.label(&format!(
            "A: 0x{:02x} X: 0x{:02x} Y: 0x{:02} PC: 0x{:04x} SP: 0x{:02x}",
            self.ac, self.x, self.y, self.pc, self.stack_pointer
        ));
        ui.label({
            fn style_flag(flag: bool) -> TextFormat {
                let mut tf = TextFormat::default();
                tf.color = if flag {
                    egui::Color32::RED
                } else {
                    egui::Color32::DARK_GRAY
                };
                tf
            }
            let mut layout = LayoutJob::default();
            layout.append(
                &format!("Status register: 0x{:02x} (", self.status.byte),
                0.0,
                TextFormat::default(),
            );
            layout.append("N", 0.0, style_flag(self.status.negative()));
            layout.append("V", 0.0, style_flag(self.status.overflow()));
            layout.append("_", 0.0, style_flag(self.status._ignored_flag()));
            layout.append("B", 0.0, style_flag(self.status.break_flag()));
            layout.append("D", 0.0, style_flag(self.status.decimal()));
            layout.append("I", 0.0, style_flag(self.status.interrupt_disabled()));
            layout.append("Z", 0.0, style_flag(self.status.zero()));
            layout.append("C", 0.0, style_flag(self.status.carry()));
            layout.append(")", 0.0, TextFormat::default());
            layout
        });
        ui.label("Stack:");
        for i in 0..(self.stack_pointer / 2) as usize {
            ui.label(&format!(
                "0x{:04x}",
                self.bus
                    .read_word(0x0100 + self.stack_pointer as usize - 2 - i * 2)
            ));
        }
    }
}

#[derive(Error, Debug)]
pub enum CPUError {
    #[error("No instruction implemented for opcode {0:#04X}.")]
    NoInstruction(u8),
    #[error("Instruction {0:?} unimplemented.")]
    UnimplementedInstruction(Instruction),
    #[error("Operand {0:?} has no data.")]
    OperandNoData(OperandData),
    #[error("Operand {0:?} is not an address.")]
    OperandNotAddress(OperandData),
    #[error("Operand {0:?} can't be written to.")]
    OperandNotWriteable(OperandData),
}
