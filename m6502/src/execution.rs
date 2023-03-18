use super::{bus::Bus, instructions::Instruction, opcode_table::OPCODE_TABLE, CPUError, CPU};

impl<T: Bus> CPU<T> {
    pub fn execute(&mut self) -> Result<(), CPUError> {
        // log::trace!("{:02x} {:02x} {:02x}", self.bus.read(self.pc as usize - 2), self.bus.read(self.pc as usize - 1), self.bus.read(0x0600));
        let opcode = self.fetch_byte();
        let opcode_data = OPCODE_TABLE[opcode as usize].ok_or(CPUError::NoInstruction(opcode))?;
        let operand = self.fetch_op_data(opcode_data.addressing_mode);
        let opfmt = {
            match operand {
                crate::addressing_modes::OperandData::Implied => "".into(),
                crate::addressing_modes::OperandData::Literal(x) => format!("0x{x:02x}"),
                crate::addressing_modes::OperandData::Address(x) => format!("(0x{x:04x})"),
            }
        };
        log::debug!(
            "PC:0x{:04x} {:?} ({:?}, {})",
            self.pc - opcode_data.bytes as u16,
            opcode_data.instruction,
            opcode_data.addressing_mode,
            opfmt
        );
        match opcode_data.instruction {
            Instruction::ADC => {
                let rhs = operand.value(self)?;
                let (new, carry) = self.ac.carrying_add(rhs, self.status.carry());
                self.status
                    .set_overflow(((self.ac ^ new) & (rhs ^ new) & 0x80) != 0);
                self.ac = new;
                self.status.set_carry(carry);
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
                // TODO: bcd
            }
            Instruction::AND => {
                let rhs = operand.value(self)?;
                self.ac &= rhs;
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
            }
            Instruction::ASL => {
                let lhs = operand.value(self).unwrap_or(self.ac);
                let (new, _) = lhs.overflowing_shl(1);
                operand.write(self, new)?;
                self.status.set_carry(lhs & 0b10000000 != 0);
                self.status.set_zero(new == 0);
                self.status.set_negative((new & 0b10000000) != 0);
            }
            Instruction::BCC => {
                let target = operand.address()?; // TODO: Change get_address handling.
                if !self.status.carry() {
                    self.pc = target;
                }
            }
            Instruction::BCS => {
                let target = operand.address()?;
                if self.status.carry() {
                    self.pc = target;
                }
            }
            Instruction::BEQ => {
                let target = operand.address()?;
                if self.status.zero() {
                    log::info!("BEQ jumping");
                    self.pc = target;
                }
            }
            Instruction::BIT => {
                let o = operand.value(self)?;
                self.status.set_zero((o & self.ac) == 0);
                self.status.byte |= o & 0b11000000;
            }
            Instruction::BMI => {
                let target = operand.address()?;
                if self.status.negative() {
                    self.pc = target;
                }
            }
            Instruction::BNE => {
                let target = operand.address()?;
                if !self.status.zero() {
                    self.pc = target;
                }
            }
            Instruction::BPL => {
                let target = operand.address()?;
                if !self.status.negative() {
                    self.pc = target;
                }
            }
            // Instruction::BRK => todo!(),
            Instruction::BVC => {
                let target = operand.address()?;
                if !self.status.overflow() {
                    self.pc = target;
                }
            }
            Instruction::BVS => {
                let target = operand.address()?;
                if self.status.overflow() {
                    self.pc = target;
                }
            }
            Instruction::CLC => self.status.set_carry(false),
            Instruction::CLD => self.status.set_decimal(false),
            Instruction::CLI => self.status.set_interrupt_disabled(false),
            Instruction::CLV => self.status.set_overflow(false),
            Instruction::CMP => {
                let rhs = operand.value(self)?;
                let (res, borrow) = self.ac.borrowing_sub(rhs, false);
                self.status.set_carry(!borrow);
                self.status.set_zero(res == 0);
                self.status.set_negative((res & 0b10000000) != 0);
                log::info!(
                    "CMP: A(0x{:02x}) with 0x{rhs:02x} - C={} Z={} N={}",
                    self.ac,
                    self.status.carry(),
                    self.status.zero(),
                    self.status.negative()
                );
            }
            Instruction::CPX => {
                let rhs = operand.value(self)?;
                let (res, borrow) = self.x.borrowing_sub(rhs, false);
                self.status.set_carry(borrow);
                self.status.set_zero(res == 0);
                self.status.set_negative((res & 0b10000000) != 0);
            }
            Instruction::CPY => {
                let rhs = operand.value(self)?;
                let (res, borrow) = self.y.borrowing_sub(rhs, false);
                self.status.set_carry(borrow);
                self.status.set_zero(res == 0);
                self.status.set_negative((res & 0b10000000) != 0);
            }
            Instruction::DEC => {
                let m = operand.value(self)?;
                let res = m.wrapping_sub(1);
                operand.write(self, res)?;
                self.status.set_zero(res == 0);
                self.status.set_negative((res & 0b10000000) != 0);
            }
            Instruction::DEX => {
                self.x = self.x.wrapping_sub(1);
                self.status.set_zero(self.x == 0);
                self.status.set_negative((self.x & 0b10000000) != 0);
            }
            Instruction::DEY => {
                self.y = self.y.wrapping_sub(1);
                self.status.set_zero(self.y == 0);
                self.status.set_negative((self.y & 0b10000000) != 0);
            }
            Instruction::EOR => {
                let rhs = operand.value(self)?;
                self.ac ^= rhs;
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
            }
            Instruction::INC => {
                let m = operand.value(self)?;
                let res = m.wrapping_add(1);
                operand.write(self, res)?;
                self.status.set_zero(res == 0);
                self.status.set_negative((res & 0b10000000) != 0);
            }
            Instruction::INX => {
                self.x = self.x.wrapping_add(1);
                self.status.set_zero(self.x == 0);
                self.status.set_negative((self.x & 0b10000000) != 0);
            }
            Instruction::INY => {
                self.y = self.y.wrapping_add(1);
                self.status.set_zero(self.y == 0);
                self.status.set_negative((self.y & 0b10000000) != 0);
            }
            Instruction::JMP => {
                self.pc = operand.address()?;
                log::trace!("0x{:04x}", self.pc)
            }
            Instruction::JSR => {
                let target = operand.address()?;
                self.push_word(self.pc);
                self.pc = target;
            }
            Instruction::LDA => {
                self.ac = operand.value(self)?;
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
            }
            Instruction::LDX => {
                self.x = operand.value(self)?;
                self.status.set_zero(self.x == 0);
                self.status.set_negative((self.x & 0b10000000) != 0);
            }
            Instruction::LDY => {
                self.y = operand.value(self)?;
                self.status.set_zero(self.y == 0);
                self.status.set_negative((self.y & 0b10000000) != 0);
            }
            Instruction::LSR => {
                let x = operand.value(self)?;
                let new = x & 1 != 0;
                let (res, _) = x.overflowing_shr(1);
                operand.write(self, res)?;
                self.status.set_overflow(new);
                self.status.set_zero(res == 0);
                self.status.set_carry(new);
            }
            Instruction::NOP => {}
            Instruction::ORA => {
                self.ac |= operand.value(self)?;
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
            }
            Instruction::PHA => {
                self.push_byte(self.ac);
            }
            Instruction::PHP => self.push_byte(self.status.byte),
            Instruction::PLA => {
                self.ac = self.pull_byte();
            }
            Instruction::PLP => {
                self.status.byte = self.pull_byte();
            }
            Instruction::ROL => {
                let value = operand.value(self)?;
                let result = value.rotate_left(1);
                operand.write(self, result)?;
                self.status.set_zero(result == 0);
                self.status.set_negative((result & 0b10000000) != 0);
                self.status.set_carry((result & 0b10000000) != 0);
            }
            Instruction::ROR => {
                let value = operand.value(self)?;
                let result = value.rotate_right(1);
                operand.write(self, result)?;
                self.status.set_zero(result == 0);
                self.status.set_negative((result & 0b10000000) != 0);
                self.status.set_carry((result & 1) != 0);
            }
            Instruction::RTI => {
                let old_status = self.pull_byte();
                self.status.byte = (old_status & 0b11001111) | (self.status.byte & 0b00110000);
                let old_pc = self.pull_word();
                self.pc = old_pc;
            }
            Instruction::RTS => {
                self.pc = self.pull_word(); //TODO: Check if it is correct.
                                            // log::debug!("RTS {:04x}", self.pc)
            }
            Instruction::SBC => {
                let rhs = operand.value(self)?;
                let (new, carry) = self.ac.borrowing_sub(rhs, !self.status.carry());
                self.status
                    .set_overflow(((self.ac ^ rhs) & 0x80 != 0) && ((self.ac ^ new) & 0x80) != 0);
                // log::error!("SBC overflow unimplemented!");
                self.ac = new;
                self.status.set_carry(!carry);
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
            }
            Instruction::SEC => self.status.set_carry(true),
            Instruction::SED => self.status.set_decimal(true),
            Instruction::SEI => self.status.set_interrupt_disabled(true),
            Instruction::STA => {
                let addr = operand.address()?;
                self.bus.write(addr as usize, self.ac);
            }
            Instruction::STX => {
                let addr = operand.address()?;
                self.bus.write(addr as usize, self.x);
            }
            Instruction::STY => {
                let addr = operand.address()?;
                self.bus.write(addr as usize, self.y);
            }
            Instruction::TAX => {
                self.x = self.ac;
                self.status.set_zero(self.x == 0);
                self.status.set_negative((self.x & 0b10000000) != 0);
            }
            Instruction::TAY => {
                self.y = self.ac;
                self.status.set_zero(self.y == 0);
                self.status.set_negative((self.y & 0b10000000) != 0);
            }
            Instruction::TSX => {
                self.x = self.stack_pointer;
                self.status.set_zero(self.x == 0);
                self.status.set_negative((self.x & 0b10000000) != 0);
            }
            Instruction::TXA => {
                self.ac = self.x;
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
            }
            Instruction::TXS => {
                self.stack_pointer = self.x;
                self.status.set_zero(self.stack_pointer == 0);
                self.status
                    .set_negative((self.stack_pointer & 0b10000000) != 0);
            }
            Instruction::TYA => {
                self.ac = self.y;
                self.status.set_zero(self.ac == 0);
                self.status.set_negative((self.ac & 0b10000000) != 0);
            }
            _ => return Err(CPUError::UnimplementedInstruction(opcode_data.instruction)),
        }
        self.cycles_left = opcode_data.cycles; // TODO: Model precise cycle behavior
        Ok(())
    }
}
