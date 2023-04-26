use std::{thread, vec};

use image::RgbImage;
use minifb::{Key, KeyRepeat, Window, WindowOptions};

pub struct Draw {
    window_image: Vec<u32>,
}

impl Draw {
    pub fn new() -> Self {
        Self {
            window_image: Vec::new(),
        }
    }

    fn window_update(
        &self,
        window: &mut Window,
        window_buffer: &mut Vec<u32>,
        image_width: usize,
        image_height: usize,
    ) -> minifb::Result<()> {
        // Dキーが押されたら
        if window.is_key_pressed(Key::D, KeyRepeat::No) {}

        window.update_with_buffer(window_buffer, image_width, image_height)?;

        Ok(())
    }

    pub fn setup_window(&self, window_image: RgbImage) -> minifb::Result<()> {
        if cfg!(test) {
            return Ok(());
        }

        let (image_width, image_height) = window_image.dimensions();
        let mut window = Window::new(
            "Esc: exit. D: ",
            image_width as usize,
            image_height as usize,
            WindowOptions {
                topmost: true,
                ..Default::default()
            },
        )
        .unwrap();

        // ~30fps までにリミットする
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600 * 2)));

        let mut window_buffer: Vec<u32> = vec![0; (image_width * image_height) as usize];

        for (buffer, (_, _, pixel)) in window_buffer
            .iter_mut()
            .zip(window_image.enumerate_pixels())
        {
            *buffer = u32::from_be_bytes([0, pixel[0], pixel[1], pixel[2]]);
        }

        while window.is_open() && !window.is_key_down(Key::Escape) {
            self.window_update(
                &mut window,
                &mut window_buffer,
                image_width as usize,
                image_height as usize,
            )?;
        }

        Ok(())
    }
}
