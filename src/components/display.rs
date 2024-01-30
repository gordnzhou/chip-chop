use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::emulator::{WIDTH, HEIGHT};

pub struct Display {
    scale: i32,
    pub pixels: [[bool; WIDTH]; HEIGHT],
    canvas: Canvas<Window>,
}


impl Display {
    pub fn new(window: Window, scale: i32) -> Result<Self, String> {
        let mut canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let pixels: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

        Ok(Display { scale, pixels, canvas })
    }

    pub fn update_display(&mut self) {
        let pixel_size = self.scale as u32;

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let x = j as i32 * self.scale;
                let y = i as i32 * self.scale;
                if self.pixels[i][j] {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                }
                else {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                let _ = self.canvas.fill_rect(Rect::new(x, y, pixel_size, pixel_size));
            }
        }

        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; WIDTH]; HEIGHT];
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y % HEIGHT][x % WIDTH]
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) {
        self.pixels[y % HEIGHT][x % WIDTH] ^= true;
    }
}