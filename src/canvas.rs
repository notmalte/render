use std::{ops::Range, time::Duration};

use minifb::{Key, Window, WindowOptions};

use crate::color::Color;

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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn min_x(&self) -> i32 {
        -(self.width as i32) / 2
    }

    pub fn max_x(&self) -> i32 {
        (self.width as i32) / 2
    }

    pub fn min_y(&self) -> i32 {
        -(self.height as i32) / 2
    }

    pub fn max_y(&self) -> i32 {
        (self.height as i32) / 2
    }

    pub fn x_range(&self) -> Range<i32> {
        self.min_x()..self.max_x()
    }

    pub fn y_range(&self) -> Range<i32> {
        self.min_y()..self.max_y()
    }

    pub fn x_to_screen(&self, x: i32) -> u32 {
        (x + (self.width as i32) / 2) as u32
    }

    pub fn y_to_screen(&self, y: i32) -> u32 {
        (-y + (self.height as i32) / 2) as u32
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        let screen_x = self.x_to_screen(x);
        let screen_y = self.y_to_screen(y);

        if screen_x < self.width && screen_y < self.height {
            let index = (screen_y * self.width + screen_x) as usize;
            self.buffer[index] = color.into();
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
