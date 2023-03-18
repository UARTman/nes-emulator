use egui::text::LayoutJob;
use egui::{Color32, FontFamily, FontId, TextFormat, Ui};

pub struct Status {
    pub byte: u8,
}

impl Default for Status {
    fn default() -> Self {
        Self { byte: 0 }
    }
}

impl Status {
    fn set_mask(&mut self, value: bool, mask: u8) {
        if value {
            self.byte |= mask
        } else {
            self.byte &= !mask
        }
    }

    pub fn carry(&self) -> bool {
        (self.byte & 1) != 0
    }

    pub fn set_carry(&mut self, value: bool) {
        self.set_mask(value, 0b00000001);
    }

    pub fn zero(&self) -> bool {
        (self.byte & 0b10) != 0
    }

    pub fn set_zero(&mut self, value: bool) {
        self.set_mask(value, 0b00000010);
    }

    pub fn interrupt_disabled(&self) -> bool {
        (self.byte & 0b100) != 0
    }

    pub fn set_interrupt_disabled(&mut self, value: bool) {
        self.set_mask(value, 0b00000100);
    }

    pub fn decimal(&self) -> bool {
        (self.byte & 0b1000) != 0
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.set_mask(value, 0b00001000);
    }

    pub fn break_flag(&self) -> bool {
        (self.byte & 0b10000) != 0
    }

    pub fn set_break(&mut self, value: bool) {
        self.set_mask(value, 0b00010000);
    }

    pub fn _ignored_flag(&self) -> bool {
        (self.byte & 0b100000) != 0
    }

    pub fn _set_ignored_flag(&mut self, value: bool) {
        self.set_mask(value, 0b00100000);
    }

    pub fn overflow(&self) -> bool {
        (self.byte & 0b1000000) != 0
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.set_mask(value, 0b01000000);
    }

    pub fn negative(&self) -> bool {
        (self.byte & 0b10000000) != 0
    }

    pub fn set_negative(&mut self, value: bool) {
        self.set_mask(value, 0b10000000);
    }

    pub fn render(&mut self, ui: &mut Ui) {
        let mut layout = LayoutJob::default();
        fn flag_format(x: bool) -> TextFormat {
            let clicked = TextFormat {
                color: Color32::RED,
                font_id: FontId {
                    family: FontFamily::Monospace,
                    ..FontId::default()
                },
                ..TextFormat::default()
            };
            if x {
                clicked
            } else {
                TextFormat {
                    font_id: FontId {
                        family: FontFamily::Monospace,
                        ..FontId::default()
                    },
                    ..TextFormat::default()
                }
            }
        }
        layout.append(
            &format!("Status flag: {:#04X} (", self.byte),
            0.0,
            Default::default(),
        );
        layout.append("N", 0.0, flag_format(self.negative()));
        layout.append("V", 0.0, flag_format(self.overflow()));
        layout.append("-", 0.0, flag_format(self._ignored_flag()));
        layout.append("B", 0.0, flag_format(self.break_flag()));
        layout.append("D", 0.0, flag_format(self.decimal()));
        layout.append("I", 0.0, flag_format(self.interrupt_disabled()));
        layout.append("Z", 0.0, flag_format(self.zero()));
        layout.append("C", 0.0, flag_format(self.carry()));
        layout.append(")", 0.0, Default::default());
        ui.label(layout);
    }
}
