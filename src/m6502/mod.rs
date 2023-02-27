use status::Status;

/// Addressing modes and how the CPU uses them
pub mod addressing_modes;
/// A list of instructions
pub mod instructions;
/// Opcode table generation and parsing
pub mod opcode_table;
/// Status register
pub mod status;

/// A 6502 CPU
pub struct CPU {
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
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            status: Status::default(),
            stack_pointer: 0,
        }
    }
}
