use macroquad::prelude::Image;
use snake_game::SnakeCanvas;

#[derive(Clone)]
pub struct MCSnakeCanvas {
    pub image: Image,
}

impl Default for MCSnakeCanvas {
    fn default() -> Self {
        let mut image = Image {
            bytes: vec![],
            width: 32,
            height: 32,
        };
        for _ in 0..32 * 32 {
            image.bytes.push(60);
            image.bytes.push(60);
            image.bytes.push(60);
            image.bytes.push(255);
        }
        image.width = 32;
        image.height = 32;
        Self { image }
    }
}

impl SnakeCanvas for MCSnakeCanvas {
    fn write_pixel(&mut self, at: usize, colors: (u8, u8, u8, u8)) {
        self.image.bytes[at * 4] = colors.0;
        self.image.bytes[at * 4 + 1] = colors.1;
        self.image.bytes[at * 4 + 2] = colors.2;
        self.image.bytes[at * 4 + 3] = colors.3;
    }
}
