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
