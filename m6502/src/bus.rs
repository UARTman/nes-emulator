pub trait Bus {
    fn read(&self, addr: usize) -> u8;
    fn write(&mut self, addr: usize, data: u8);

    fn read_word(&self, addr: usize) -> u16 {
        self.read(addr) as u16 | ((self.read(addr + 1) as u16) << 8)
    }

    fn write_word(&mut self, addr: usize, data: u16) {
        self.write(addr, (data & 0xFF) as u8);
        self.write(addr + 1, (data >> 8) as u8)
    }
}

pub struct Ram {
    memory: Vec<u8>,
}

impl Bus for Ram {
    fn read(&self, addr: usize) -> u8 {
        self.memory[addr]
    }

    fn write(&mut self, addr: usize, data: u8) {
        self.memory[addr] = data;
    }
}

impl Ram {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0x10000],
        }
    }

    pub fn of_size(size: usize) -> Self {
        Self {
            memory: vec![0; size],
        }
    }

    pub fn load(&mut self, bytes: &[u8], at: usize) {
        self.memory[at..at + bytes.len()].copy_from_slice(bytes);
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}