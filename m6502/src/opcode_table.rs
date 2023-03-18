use crate::addressing_modes::AddressingMode;
use crate::instructions::Instruction;

/// A rule by which the final cycle count of an instruction is determined
#[derive(Copy, Clone, Debug)]
pub enum CycleRule {
    /// The operation always takes the specified number of cycles.
    None,
    /// Add 1 to cycles if page boundary is crossed.
    AddOnCross,
    /// Add 1 to cycles if branch occurs on same page.
    /// Add 2 to cycles if branch occurs to different page.
    AddOneTwo,
}

#[derive(Copy, Clone, Debug)]
pub struct OpcodeEntry {
    /// The byte corresponding to the instruction
    pub code: u8,
    /// The instruction invoked
    pub instruction: Instruction,
    /// The addressing mode of the invocation
    pub addressing_mode: AddressingMode,
    /// Instruction length (with operand)
    pub bytes: u8,
    /// Baseline cycle count
    pub cycles: u8,
    /// A rule by which the final cycle count is determinef
    pub cycle_rule: CycleRule,
}

impl OpcodeEntry {
    pub const fn new(
        code: u8,
        instruction: Instruction,
        addressing_mode: AddressingMode,
        bytes: u8,
        cycles: u8,
        cycle_rule: CycleRule,
    ) -> Self {
        Self {
            code,
            instruction,
            addressing_mode,
            bytes,
            cycles,
            cycle_rule,
        }
    }
}

lazy_static::lazy_static! {
    pub static ref OPCODE_TABLE: [Option<OpcodeEntry>; 256] = {
        let mut x = [None; 256];
        for i in OPCODE_LIST {
            x[i.code as usize] = Some(i.clone());
        }
        x
    };
}

/// The table used during instruction parsing. Auto-generated.
// pub const OPCODE_TABLE: [Option<OpcodeEntry>; 256] = generate_table();

/// An opcode list
#[rustfmt::skip]
const OPCODE_LIST: &[OpcodeEntry] = &[
    // ADC - Add Memory to Accumulator with Carry
    OpcodeEntry::new(0x69, Instruction::ADC, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0x65, Instruction::ADC, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x75, Instruction::ADC, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0x6D, Instruction::ADC, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0x7D, Instruction::ADC, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x79, Instruction::ADC, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x61, Instruction::ADC, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x71, Instruction::ADC, AddressingMode::IndirectY, 2, 5, CycleRule::AddOnCross),
    // AND - Add memory with Accumulator
    OpcodeEntry::new(0x29, Instruction::AND, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0x25, Instruction::AND, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x35, Instruction::AND, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0x2D, Instruction::AND, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0x3D, Instruction::AND, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x39, Instruction::AND, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x21, Instruction::AND, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x31, Instruction::AND, AddressingMode::IndirectY, 2, 5, CycleRule::AddOnCross),
    // ASL - Shift left
    OpcodeEntry::new(0x0A, Instruction::ASL, AddressingMode::Implied, 1, 2, CycleRule::None),
    OpcodeEntry::new(0x06, Instruction::ASL, AddressingMode::ZeroPage, 2, 5, CycleRule::None),
    OpcodeEntry::new(0x16, Instruction::ASL, AddressingMode::ZeroPageX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x0E, Instruction::ASL, AddressingMode::Absolute, 3, 6, CycleRule::None),
    OpcodeEntry::new(0x1E, Instruction::ASL, AddressingMode::AbsoluteX, 3, 7, CycleRule::None),
    // BCC - Branch on Carry Clear
    OpcodeEntry::new(0x90, Instruction::BCC, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // BCS - Branch on Carry Set
    OpcodeEntry::new(0xB0, Instruction::BCS, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // BEQ - Branch on Result Zero
    OpcodeEntry::new(0xF0, Instruction::BEQ, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // BIT - Test bits in memory with accumulator
    OpcodeEntry::new(0x24, Instruction::BIT, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x2C, Instruction::BIT, AddressingMode::Absolute, 3, 4, CycleRule::None),
    // BMI - Branch on Result Minus
    OpcodeEntry::new(0x30, Instruction::BMI, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // BNE - Branch on Result not Zero
    OpcodeEntry::new(0xD0, Instruction::BNE, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // BPL - Branch on Result Plus
    OpcodeEntry::new(0x10, Instruction::BPL, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // BRK - Force Break
    OpcodeEntry::new(0x00, Instruction::BRK, AddressingMode::Implied, 1, 7, CycleRule::None),
    // BVC - Branch on Overflow Clear
    OpcodeEntry::new(0x50, Instruction::BVC, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // BVS - Branch on Overflow Set
    OpcodeEntry::new(0x70, Instruction::BVS, AddressingMode::Relative, 2, 2, CycleRule::AddOneTwo),
    // CLC - Clear Carry Flag
    OpcodeEntry::new(0x18, Instruction::CLC, AddressingMode::Implied, 1, 2, CycleRule::None),
    // CLD - Clear Decimal Mode
    OpcodeEntry::new(0xD8, Instruction::CLD, AddressingMode::Implied, 1, 2, CycleRule::None),
    // CLI - Clear Interrupt Disable
    OpcodeEntry::new(0x58, Instruction::CLI, AddressingMode::Implied, 1, 2, CycleRule::None),
    // CMP - Compare Memory with Accumulator
    OpcodeEntry::new(0xC9, Instruction::CMP, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0xC5, Instruction::CMP, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0xD5, Instruction::CMP, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0xCD, Instruction::CMP, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0xDD, Instruction::CMP, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0xD9, Instruction::CMP, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0xC1, Instruction::CMP, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0xD1, Instruction::CMP, AddressingMode::IndirectY, 2, 5, CycleRule::AddOnCross),
    // CPX - Compare Memory with X
    OpcodeEntry::new(0xE0, Instruction::CPX, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0xE4, Instruction::CPX, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0xEC, Instruction::CPX, AddressingMode::Absolute, 3, 4, CycleRule::None),
    // CPY - Compare Memory with Y
    OpcodeEntry::new(0xC0, Instruction::CPY, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0xC4, Instruction::CPY, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0xCC, Instruction::CPY, AddressingMode::Absolute, 3, 4, CycleRule::None),
    // DEC - Decrement Memory by One
    OpcodeEntry::new(0xC6, Instruction::DEC, AddressingMode::ZeroPage, 2, 5, CycleRule::None),
    OpcodeEntry::new(0xD6, Instruction::DEC, AddressingMode::ZeroPageX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0xCE, Instruction::DEC, AddressingMode::Absolute, 3, 6, CycleRule::None),
    OpcodeEntry::new(0xDE, Instruction::DEC, AddressingMode::AbsoluteX, 3, 7, CycleRule::None),
    // DEX - Decrement X by One
    OpcodeEntry::new(0xCA, Instruction::DEX, AddressingMode::Implied, 1, 2, CycleRule::None),
    // DEY - Decrement Y by One
    OpcodeEntry::new(0x88, Instruction::DEY, AddressingMode::Implied, 1, 2, CycleRule::None),
    // EOR - Exclusive Or Memory with Accumulator
    OpcodeEntry::new(0x49, Instruction::EOR, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0x45, Instruction::EOR, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x55, Instruction::EOR, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0x4D, Instruction::EOR, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0x5D, Instruction::EOR, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x59, Instruction::EOR, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x41, Instruction::EOR, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x51, Instruction::EOR, AddressingMode::IndirectY, 2, 5, CycleRule::AddOnCross),
    // INC - Increment Memory by One
    OpcodeEntry::new(0xE6, Instruction::INC, AddressingMode::ZeroPage, 2, 5, CycleRule::None),
    OpcodeEntry::new(0xF6, Instruction::INC, AddressingMode::ZeroPageX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0xEE, Instruction::INC, AddressingMode::Absolute, 3, 6, CycleRule::None),
    OpcodeEntry::new(0xFE, Instruction::INC, AddressingMode::AbsoluteX, 3, 7, CycleRule::None),
    // INX - Increment X by One
    OpcodeEntry::new(0xE8, Instruction::INX, AddressingMode::Implied, 1, 2, CycleRule::None),
    // INY - Increment Y by One
    OpcodeEntry::new(0xC8, Instruction::INY, AddressingMode::Implied, 1, 2, CycleRule::None),
    // JMP - Jump to new Location
    OpcodeEntry::new(0x4C, Instruction::JMP, AddressingMode::Absolute, 3, 3, CycleRule::None),
    OpcodeEntry::new(0x6C, Instruction::JMP, AddressingMode::Indirect, 3, 5, CycleRule::None),
    // JSR - Jump to new Location saving return address
    OpcodeEntry::new(0x20, Instruction::JSR, AddressingMode::Absolute, 3, 6, CycleRule::None),
    // LDA - Load Accumulator with Memory
    OpcodeEntry::new(0xA9, Instruction::LDA, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0xA5, Instruction::LDA, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0xB5, Instruction::LDA, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0xAD, Instruction::LDA, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0xBD, Instruction::LDA, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0xB9, Instruction::LDA, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0xA1, Instruction::LDA, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0xB1, Instruction::LDA, AddressingMode::IndirectY, 2, 5, CycleRule::AddOnCross),
    // LDX - Load X with Memory
    OpcodeEntry::new(0xA2, Instruction::LDX, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0xA6, Instruction::LDX, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0xB6, Instruction::LDX, AddressingMode::ZeroPageY, 2, 4, CycleRule::None),
    OpcodeEntry::new(0xAE, Instruction::LDX, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0xBE, Instruction::LDX, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    // LDY - Load Y with Memory
    OpcodeEntry::new(0xA0, Instruction::LDY, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0xA4, Instruction::LDY, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0xB4, Instruction::LDY, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0xAC, Instruction::LDY, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0xBC, Instruction::LDY, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    // LSR - Shift One Bit Right
    OpcodeEntry::new(0x4A, Instruction::LSR, AddressingMode::Implied, 1, 2, CycleRule::None),
    OpcodeEntry::new(0x46, Instruction::LSR, AddressingMode::ZeroPage, 2, 5, CycleRule::None),
    OpcodeEntry::new(0x56, Instruction::LSR, AddressingMode::ZeroPageX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x4E, Instruction::LSR, AddressingMode::Absolute, 3, 6, CycleRule::None),
    OpcodeEntry::new(0x5E, Instruction::LSR, AddressingMode::AbsoluteX, 3, 7, CycleRule::None),
    // NOP - No Operation
    OpcodeEntry::new(0xEA, Instruction::NOP, AddressingMode::Implied, 1, 2, CycleRule::None),
    // ORA - Or Memory With Accumulator
    OpcodeEntry::new(0x09, Instruction::ORA, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0x05, Instruction::ORA, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x15, Instruction::ORA, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0x0D, Instruction::ORA, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0x1D, Instruction::ORA, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x19, Instruction::ORA, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0x01, Instruction::ORA, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x11, Instruction::ORA, AddressingMode::IndirectY, 2, 5, CycleRule::AddOnCross),
    // PHA - Push Accumulator on Stack
    OpcodeEntry::new(0x48, Instruction::PHA, AddressingMode::Implied, 1, 3, CycleRule::None),
    // PHP - Push Processor Status on Stack
    OpcodeEntry::new(0x08, Instruction::PHP, AddressingMode::Implied, 1, 3, CycleRule::None),
    // PLA - Pull Accumulator from Stack
    OpcodeEntry::new(0x68, Instruction::PLA, AddressingMode::Implied, 1, 4, CycleRule::None),
    // PLP - Pull Processor Status from Stack
    OpcodeEntry::new(0x28, Instruction::PLP, AddressingMode::Implied, 1, 4, CycleRule::None),
    // ROL - Rotate One Bit Left
    OpcodeEntry::new(0x2A, Instruction::ROL, AddressingMode::Implied, 1, 2, CycleRule::None),
    OpcodeEntry::new(0x26, Instruction::ROL, AddressingMode::ZeroPage, 2, 5, CycleRule::None),
    OpcodeEntry::new(0x36, Instruction::ROL, AddressingMode::ZeroPageX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x2E, Instruction::ROL, AddressingMode::Absolute, 3, 6, CycleRule::None),
    OpcodeEntry::new(0x3E, Instruction::ROL, AddressingMode::AbsoluteX, 3, 7, CycleRule::None),
    // ROR - Rotate One Bit Right
    OpcodeEntry::new(0x6A, Instruction::ROR, AddressingMode::Implied, 1, 2, CycleRule::None),
    OpcodeEntry::new(0x66, Instruction::ROR, AddressingMode::ZeroPage, 2, 5, CycleRule::None),
    OpcodeEntry::new(0x76, Instruction::ROR, AddressingMode::ZeroPageX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x6E, Instruction::ROR, AddressingMode::Absolute, 3, 6, CycleRule::None),
    OpcodeEntry::new(0x7E, Instruction::ROR, AddressingMode::AbsoluteX, 3, 7, CycleRule::None),
    // RTI - Return from Interrupt
    OpcodeEntry::new(0x40, Instruction::RTI, AddressingMode::Implied, 1, 6, CycleRule::None),
    // RTS - Return from Subroutine
    OpcodeEntry::new(0x60, Instruction::RTS, AddressingMode::Implied, 1, 6, CycleRule::None),
    // SBC - Subtract Memory from Accumulator with Borrow
    OpcodeEntry::new(0xE9, Instruction::SBC, AddressingMode::Immediate, 2, 2, CycleRule::None),
    OpcodeEntry::new(0xE5, Instruction::SBC, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0xF5, Instruction::SBC, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0xED, Instruction::SBC, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0xFD, Instruction::SBC, AddressingMode::AbsoluteX, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0xF9, Instruction::SBC, AddressingMode::AbsoluteY, 3, 4, CycleRule::AddOnCross),
    OpcodeEntry::new(0xE1, Instruction::SBC, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0xF1, Instruction::SBC, AddressingMode::IndirectY, 2, 5, CycleRule::AddOnCross),
    // SEC - Set Carry Flag
    OpcodeEntry::new(0x38, Instruction::SEC, AddressingMode::Implied, 1, 2, CycleRule::None),
    // SED - Set Decimal Flag
    OpcodeEntry::new(0xF8, Instruction::SED, AddressingMode::Implied, 1, 2, CycleRule::None),
    // SEI - Set Interrupt Disable
    OpcodeEntry::new(0x78, Instruction::SEI, AddressingMode::Implied, 1, 2, CycleRule::None),
    // STA - Store Accumulator in Memory
    OpcodeEntry::new(0x85, Instruction::STA, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x95, Instruction::STA, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0x8D, Instruction::STA, AddressingMode::Absolute, 3, 4, CycleRule::None),
    OpcodeEntry::new(0x9D, Instruction::STA, AddressingMode::AbsoluteX, 3, 5, CycleRule::None),
    OpcodeEntry::new(0x99, Instruction::STA, AddressingMode::AbsoluteY, 3, 5, CycleRule::None),
    OpcodeEntry::new(0x81, Instruction::STA, AddressingMode::IndirectX, 2, 6, CycleRule::None),
    OpcodeEntry::new(0x91, Instruction::STA, AddressingMode::IndirectY, 2, 6, CycleRule::None),
    // STX - Store X in Memory
    OpcodeEntry::new(0x86, Instruction::STX, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x96, Instruction::STX, AddressingMode::ZeroPageY, 2, 4, CycleRule::None),
    OpcodeEntry::new(0x8E, Instruction::STX, AddressingMode::Absolute, 3, 4, CycleRule::None),
    // STY - Store Y in Memory
    OpcodeEntry::new(0x84, Instruction::STY, AddressingMode::ZeroPage, 2, 3, CycleRule::None),
    OpcodeEntry::new(0x94, Instruction::STY, AddressingMode::ZeroPageX, 2, 4, CycleRule::None),
    OpcodeEntry::new(0x8C, Instruction::STY, AddressingMode::Absolute, 3, 4, CycleRule::None),
    // TAX - Transfer Accumulator to X
    OpcodeEntry::new(0xAA, Instruction::TAX, AddressingMode::Implied, 1, 2, CycleRule::None),
    // TAY - Transfer Accumulator to Y
    OpcodeEntry::new(0xA8, Instruction::TAY, AddressingMode::Implied, 1, 2, CycleRule::None),
    // TSX - Transfer Stack Pointer to X
    OpcodeEntry::new(0xBA, Instruction::TSX, AddressingMode::Implied, 1, 2, CycleRule::None),
    // TXA - Transfer X to Accumulator
    OpcodeEntry::new(0x8A, Instruction::TXA, AddressingMode::Implied, 1, 2, CycleRule::None),
    // TXS - Transfer X to Stack Pointer
    OpcodeEntry::new(0x9A, Instruction::TXS, AddressingMode::Implied, 1, 2, CycleRule::None),
    // TXA - Transfer Y to Accumulator
    OpcodeEntry::new(0x98, Instruction::TYA, AddressingMode::Implied, 1, 2, CycleRule::None),
];
