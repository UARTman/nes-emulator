use rand::Rng;

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

pub struct Snake {
    pub memory: [u8; 0x10000],
    pub pixbuf: [u8; 32 * 32 * 4],
}

impl Snake {
    pub fn new() -> Self {
        let game_code = vec![
            0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9,
            0x02, 0x85, 0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85,
            0x12, 0xa9, 0x0f, 0x85, 0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60,
            0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe, 0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60,
            0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3, 0x06, 0x20, 0x19, 0x07, 0x20, 0x20,
            0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9, 0x77, 0xf0, 0x0d, 0xc9,
            0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60, 0xa9, 0x04,
            0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
            0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04,
            0x85, 0x02, 0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60,
            0x60, 0x20, 0x94, 0x06, 0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d,
            0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07, 0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60,
            0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06, 0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09,
            0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c, 0x35, 0x07, 0x60, 0xa6,
            0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02, 0x4a, 0xb0,
            0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
            0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28,
            0x60, 0xe6, 0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69,
            0x20, 0x85, 0x10, 0xb0, 0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c,
            0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29, 0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35,
            0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60, 0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10,
            0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea, 0xea, 0xca, 0xd0, 0xfb,
            0x60,
        ];
        let mut memory = [0u8; 0x10000];
        memory[0x600..0x600 + game_code.len()].copy_from_slice(&game_code);
        Self {
            memory,
            pixbuf: [60; 32 * 32 * 4],
        }
    }
}

impl Bus for Snake {
    fn read(&self, addr: usize) -> u8 {
        if addr == 0x2 {
            log::debug!("Read from 0x2, result {}", self.memory[addr]);
        }
        if addr == 0xFE {
            return rand::thread_rng().gen();
        }
        // if addr == 0xFF {
        //     log::info!("0xFF read, value {}", self.memory[0xFF]);
        // }
        self.memory[addr]
    }

    fn write(&mut self, addr: usize, data: u8) {
        // log::debug!("Write of 0x{data:02x} at 0x{addr:04x}");
        if addr == 0x2 {
            log::debug!("Write to 0x2 ({data:02x})");
        }
        if (0x200..0x600).contains(&addr) {
            // log::debug!("Write of 0x{data:02x} at 0x{addr:04x}");
            let px = (addr - 0x200) * 4;
            if data == 0 {
                self.pixbuf[px] = 60;
                self.pixbuf[px + 1] = 60;
                self.pixbuf[px + 2] = 60;
                self.pixbuf[px + 3] = 60;
            } else {
                match data {
                    2 | 9 => {
                        self.pixbuf[px] = 100;
                        self.pixbuf[px + 1] = 100;
                        self.pixbuf[px + 2] = 100;
                        self.pixbuf[px + 3] = 100;
                    },
                    3 | 10 => {
                        self.pixbuf[px] = 255;
                        self.pixbuf[px + 1] = 0;
                        self.pixbuf[px + 2] = 0;
                        self.pixbuf[px + 3] = 255;
                    },
                    4 | 11 => {
                        self.pixbuf[px] = 0;
                        self.pixbuf[px + 1] = 255;
                        self.pixbuf[px + 2] = 0;
                        self.pixbuf[px + 3] = 255;
                    },
                    5 | 12 => {
                        self.pixbuf[px] = 0;
                        self.pixbuf[px + 1] = 0;
                        self.pixbuf[px + 2] = 255;
                        self.pixbuf[px + 3] = 255;
                    },
                    6 | 13 => {
                        self.pixbuf[px] = 255;
                        self.pixbuf[px + 1] = 0;
                        self.pixbuf[px + 2] = 255;
                        self.pixbuf[px + 3] = 255;
                    },
                    7 | 14 => {
                        self.pixbuf[px] = 255;
                        self.pixbuf[px + 1] = 255;
                        self.pixbuf[px + 2] = 0;
                        self.pixbuf[px + 3] = 255;
                    },
                    1 => {
                        self.pixbuf[px] = 255;
                        self.pixbuf[px + 1] = 255;
                        self.pixbuf[px + 2] = 255;
                        self.pixbuf[px + 3] = 255;
                    },
                    _ => {
                        self.pixbuf[px] = rand::thread_rng().gen();
                        self.pixbuf[px + 1] = rand::thread_rng().gen();
                        self.pixbuf[px + 2] = rand::thread_rng().gen();
                        self.pixbuf[px + 3] = rand::thread_rng().gen();
                    }
                }
                
            }
        }
        self.memory[addr] = data;
    }
}
