use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

pub struct Canvas {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        let screen_x = (x + (self.width as i32) / 2) as u32;
        let screen_y = (-y + (self.height as i32) / 2) as u32;

        if screen_x < self.width && screen_y < self.height {
            let index = (screen_y * self.width + screen_x) as usize;
            let color = (r as u32) << 16 | (g as u32) << 8 | b as u32;
            self.buffer[index] = color;
        }
    }

    pub fn display(&self) {
        let mut window = Window::new(
            format!("Canvas ({}x{})", self.width, self.height).as_str(),
            self.width as usize,
            self.height as usize,
            WindowOptions::default(),
        )
        .unwrap();

        window.limit_update_rate(Some(Duration::from_micros(16666)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(&self.buffer, self.width as usize, self.height as usize)
                .unwrap();
        }
    }
}
