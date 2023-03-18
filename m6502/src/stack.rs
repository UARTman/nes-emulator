use super::{bus::Bus, CPU};

impl<T: Bus> CPU<T> {
    pub fn push_byte(&mut self, data: u8) {
        self.bus.write(0x0100 + self.stack_pointer as usize, data);
        self.stack_pointer += 1;
    }
    pub fn pull_byte(&mut self) -> u8 {
        self.stack_pointer -= 1;
        self.bus.read(0x0100 + self.stack_pointer as usize)
    }
    pub fn push_word(&mut self, data: u16) {
        self.bus
            .write(0x0100 + self.stack_pointer as usize, (data & 0xFF) as u8);
        self.bus
            .write(0x0100 + self.stack_pointer as usize + 1, (data >> 8) as u8);
        self.stack_pointer += 2;
    }
    pub fn pull_word(&mut self) -> u16 {
        self.stack_pointer -= 2;
        let low = self.bus.read(0x0100 + self.stack_pointer as usize) as u16;
        let high = self.bus.read(0x0100 + self.stack_pointer as usize + 1) as u16;
        low | (high << 8)
    }
}
