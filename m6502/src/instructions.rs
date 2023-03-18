#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    /// Add with carry
    ADC,
    /// And (with accumulator)
    AND,
    /// Arithmetic shift left
    ASL,
    /// Branch on carry clear
    BCC,
    /// Branch on carry set
    BCS,
    /// Branch on equal (zero set)
    BEQ,
    /// Bit test
    BIT,
    /// Branch on minus (negative set)
    BMI,
    /// Branch on not equal (zero clear)
    BNE,
    /// Branch on plus (negative clear)
    BPL,
    /// Break / interrupt
    BRK,
    /// Branch on overflow clear
    BVC,
    /// Branch on overflow set
    BVS,
    /// Clear carry
    CLC,
    /// Clear decimal
    CLD,
    /// Clear interrupt disable
    CLI,
    /// Clear overflow
    CLV,
    /// Compare (with accumulator)
    CMP,
    /// Compare with x
    CPX,
    /// Compare with y
    CPY,
    /// Decrement
    DEC,
    /// Decrement x
    DEX,
    /// Decrement y
    DEY,
    /// Exclusive or (with accumulator)
    EOR,
    /// Increment
    INC,
    /// Increment x
    INX,
    /// Increment y
    INY,
    /// Jump
    JMP,
    /// Jump subroutine
    JSR,
    /// Load accumulator
    LDA,
    /// Load x
    LDX,
    /// Load y
    LDY,
    /// Logical shift right
    LSR,
    /// No operation
    NOP,
    /// Or with accumulator
    ORA,
    /// Push accumulator
    PHA,
    /// Push processor status (sr)
    PHP,
    /// Pull accumulator
    PLA,
    /// Pull processor status (sr)
    PLP,
    /// Rotate left
    ROL,
    /// Rotate right
    ROR,
    /// Return from interrupt
    RTI,
    /// Return from subroutine
    RTS,
    /// Subtract with carry
    SBC,
    /// Set carry
    SEC,
    /// Set decimal
    SED,
    /// Set interrupt disable
    SEI,
    /// Store accumulator
    STA,
    /// Store x
    STX,
    /// Store y
    STY,
    /// Transfer accumulator to x
    TAX,
    /// Transfer accumulator to y
    TAY,
    /// Transfer stack pointer to x
    TSX,
    /// Transfer x to accumulator
    TXA,
    /// Transfer x to stack pointer
    TXS,
    /// Transfer y to accumulator
    TYA,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        f.write_str(match *self {
            ADC => "Add with carry",
            AND => "And (with accumulator)",
            ASL => "Arithmetic shift left",
            BCC => "Branch on carry clear",
            BCS => "Branch on carry set",
            BEQ => "Branch on equal (zero set)",
            BIT => "Bit test",
            BMI => "Branch on minus (negative set)",
            BNE => "Branch on not equal (zero clear)",
            BPL => "Branch on plus (negative clear)",
            BRK => "Break / interrupt",
            BVC => "Branch on overflow clear",
            BVS => "Branch on overflow set",
            CLC => "Clear carry",
            CLD => "Clear decimal",
            CLI => "Clear interrupt disable",
            CLV => "Clear overflow",
            CMP => "Compare (with accumulator)",
            CPX => "Compare with x",
            CPY => "Compare with y",
            DEC => "Decrement",
            DEX => "Decrement x",
            DEY => "Decrement y",
            EOR => "Exclusive or (with accumulator)",
            INC => "Increment",
            INX => "Increment x",
            INY => "Increment y",
            JMP => "Jump",
            JSR => "Jump subroutine",
            LDA => "Load accumulator",
            LDX => "Load x",
            LDY => "Load y",
            LSR => "Logical shift right",
            NOP => "No operation",
            ORA => "Or with accumulator",
            PHA => "Push accumulator",
            PHP => "Push processor status (sr)",
            PLA => "Pull accumulator",
            PLP => "Pull processor status (sr)",
            ROL => "Rotate left",
            ROR => "Rotate right",
            RTI => "Return from interrupt",
            RTS => "Return from subroutine",
            SBC => "Subtract with carry",
            SEC => "Set carry",
            SED => "Set decimal",
            SEI => "Set interrupt disable",
            STA => "Store accumulator",
            STX => "Store x",
            STY => "Store y",
            TAX => "Transfer accumulator to x",
            TAY => "Transfer accumulator to y",
            TSX => "Transfer stack pointer to x",
            TXA => "Transfer x to accumulator",
            TXS => "Transfer x to stack pointer",
            TYA => "Transfer y to accumulator",
        })
    }
}
