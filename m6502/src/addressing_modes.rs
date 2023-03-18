use super::{bus::Bus, CPUError, CPU};

/// Addressing Mode
///
/// Each instruction has an addressing mode. Depending on it, we can (or can't) read/write value or
/// get an address.
#[derive(Copy, Clone, Debug)]
pub enum AddressingMode {
    /// Operation on a register
    ///
    /// Mnemonic examples:
    /// ```text
    /// CLC (clear the carry flag)
    /// ROL A (rotate A left)
    /// ROL (same)
    /// TXA (transfer X to A)
    /// PHA (push A to stack)
    /// RTS (return by pulling PC from stack)
    /// ```
    Implied,
    /// A literal operand is provided after the instruction.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDA #$07 (load 0x7 into A)
    /// ADC #$A0 (add 0xa0 to A with carry)
    /// CPX #$32 (compare X with 0x32)
    /// ```
    Immediate,
    /// A 16-bit address is provided after the instruction.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDA $3010 (load contents of address 0x3010 into A)
    /// ROL $08A0 (rotate contents of addr 0x08A0 left)
    /// JMP $4000 (jump to 0x4000)
    /// ```
    Absolute,
    /// An 8-bit zero-page address is provided after the instruction.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDA $80 (load the contents of address 0x0080 into A)
    /// BIT $A2 (perform bit-test with the contents of address 0x00A2)
    /// ```
    ZeroPage,
    /// A 16-bit address is provided after the instruction. X is added to it,
    /// and then it's used for addressing.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDA $3120,X (load the contents of address "$3120 + X" into A)
    /// INC $1400,X (increment the contents of address "$1400 + X")
    /// ```
    AbsoluteX,
    /// A 16-bit address is provided after the instruction. Y is added to it,
    /// and then it's used for addressing.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDX $8240,Y (load the contents of address "$8240 + Y" into X)
    /// ```
    AbsoluteY,
    /// An 8-bit zero-page address is provided after the instruction. X is added to it,
    /// and then it's used for addressing.
    ///
    /// ```text
    /// LDA $80,X (load the contents of address "$0080 + X" into A)
    /// LSR $82,X (shift the contents of address "$0082 + X" left)
    /// ```
    ZeroPageX,
    /// An 8-bit zero-page is provided after the instruction. X is added to it,
    /// and then it's used for addressing.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDX $60,Y (load the contents of address "$0060 + Y" into X)
    /// ```
    ZeroPageY,
    /// A 16-bit address is provided after the instruction. A 16-bit value is read from memory
    /// at this address then used for addressing.
    ///
    /// Mnemonic examples:
    /// ```text
    /// JMP ($FF82) (jump to address given in addresses "$FF82" and "$FF83")
    /// ```
    Indirect,
    /// An 8-bit zero-page address is provided after the instruction. X is added to it,
    /// then a 16-bit value is read from memory at the result. The value is then used for addressing.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDA ($70,X) (load the contents of the location given in addresses "$0070+X" and "$0070+1+X"
    ///     into A)
    /// STA ($A2,X) (store the contents of A in the location given in addresses "$00A2+X" and
    ///     "$00A3+X")
    /// EOR ($BA,X) (perform an exlusive OR of the contents of A and the contents of the location
    ///     given in addresses "$00BA+X" and "$00BB+X")
    /// ```
    IndirectX,
    /// An 8-bit zero-page address is provided after the instruction. A 16-bit value is read
    /// from memory at the result. Y is added to the value. Then the value is then used for addressing.
    ///
    /// Mnemonic examples:
    /// ```text
    /// LDA ($70,Y) (add the contents of the Y-register to the pointer provided in
    ///     "$0070" and "$0071" and load the contents of this address into A)
    /// STA ($A2,Y) (store the contents of A in the location given by the pointer in
    ///     "$00A2" and "$00A3" plus the contents of the Y-register)
    /// EOR ($BA,Y) (perform an exlusive OR of the contents of A and the address given
    ///     by the addition of Y to the pointer in "$00BA" and "$00BB")
    IndirectY,
    /// A signed 8-bit value is provided after the instruction.
    /// The resulting address is that value added to the next PC.
    ///
    /// Mnemonic examples:
    /// ```text
    /// BEQ $1005 (branch to location "$1005", if the zero flag is set.
    ///     if the current address is $1000, this will give an offset of $03.)
    /// BCS $08C4 (branch to location "$08C4", if the carry flag is set.
    ///     if the current address is $08D4, this will give an offset of $EE (−$12).)
    /// BCC $084A (branch to location "$084A", if the carry flag is clear.)
    /// ```
    Relative,
}

#[derive(Debug, Copy, Clone)]
pub enum OperandData {
    Implied,
    Literal(u8),
    Address(u16),
}

impl OperandData {
    pub fn value(self, cpu: &CPU<impl Bus>) -> Result<u8, CPUError> {
        match self {
            OperandData::Implied => Ok(cpu.ac),
            OperandData::Literal(lit) => Ok(lit),
            OperandData::Address(addr) => Ok(cpu.bus.read(addr as usize)),
        }
    }

    pub fn write(self, cpu: &mut CPU<impl Bus>, value: u8) -> Result<(), CPUError> {
        match self {
            OperandData::Implied => {
                cpu.ac = value;
                Ok(())
            }
            OperandData::Literal(_) => Err(CPUError::OperandNotWriteable(self)),
            OperandData::Address(addr) => {
                cpu.bus.write(addr as usize, value);
                Ok(())
            }
        }
    }

    pub fn address(self) -> Result<u16, CPUError> {
        match self {
            OperandData::Address(addr) => Ok(addr),
            _ => Err(CPUError::OperandNotAddress(self)),
        }
    }
}

impl<T: Bus> CPU<T> {
    pub fn fetch_op_data(&mut self, mode: AddressingMode) -> OperandData {
        match mode {
            AddressingMode::Implied => OperandData::Implied,
            AddressingMode::Immediate => OperandData::Literal(self.fetch_byte()),
            AddressingMode::Absolute => OperandData::Address(self.fetch_word()),
            AddressingMode::ZeroPage => OperandData::Address(self.fetch_byte() as u16),
            AddressingMode::AbsoluteX => OperandData::Address(self.fetch_word() + self.x as u16),
            AddressingMode::AbsoluteY => OperandData::Address(self.fetch_word() + self.y as u16),
            AddressingMode::ZeroPageX => {
                OperandData::Address(self.fetch_byte() as u16 + self.x as u16)
            }
            AddressingMode::ZeroPageY => {
                OperandData::Address(self.fetch_byte() as u16 + self.y as u16)
            }
            AddressingMode::Indirect => {
                let addr = self.fetch_word();
                let target = self.bus.read_word(addr as usize);
                OperandData::Address(target)
            }
            AddressingMode::IndirectX => {
                let addr = self.fetch_byte() as u16 + self.x as u16;
                let target = self.bus.read_word(addr as usize);
                OperandData::Address(target)
            }
            AddressingMode::IndirectY => {
                let addr = self.fetch_byte() as u16;
                let target = self.bus.read_word(addr as usize) + self.y as u16;
                OperandData::Address(target)
            }
            AddressingMode::Relative => {
                let obyte = self.fetch_byte();
                let offset = i8::from_be_bytes([obyte]);
                // log::info!("Relative addr byte: 0b{obyte:08b} offset: {offset}");
                if offset >= 0 {
                    OperandData::Address(self.pc + (offset as u16))
                } else {
                    OperandData::Address(self.pc - (-offset as u16))
                }
            }
        }
    }
}
