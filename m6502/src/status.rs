use egui::text::LayoutJob;
use egui::{Color32, FontFamily, FontId, TextFormat, Ui};

 macro_rules! flag {
    ($f:ident, $set:ident, $bit:expr) => {
        impl Status {
            pub fn $f(&self) -> bool {
                (self.byte & (1 << $bit)) != 0
            }

            pub fn $set(&mut self, value: bool) {
                self.set_mask(value, 1 << $bit)
            }
        }
    }
 }

 flag!(carry, set_carry, 0);
 flag!(zero, set_zero, 1);
 flag!(interrupt_disabled, set_interrupt_disabled, 2);
 flag!(decimal, set_decimal, 3);
 flag!(break_flag, set_break, 4);
 flag!(_ignored_flag, _set_ignored_flag, 5);
 flag!(overflow, set_overflow, 6);
 flag!(negative, set_negative, 7);

#[derive(Clone)]
pub struct Status {
    pub byte: u8,
}

impl Default for Status {
    fn default() -> Self {
        Self { byte: 1 << 5 }
    }
}

impl Status {
    #[inline]
    fn set_mask(&mut self, value: bool, mask: u8) {
        if value {
            self.byte |= mask
        } else {
            self.byte &= !mask
        }
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
