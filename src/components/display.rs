const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    scale: u32,
    pixels: [[bool; WIDTH]; HEIGHT],
}


impl Display {

    pub fn new(scale: u32) -> Self {
        let pixels: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

        Display { scale, pixels }
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; WIDTH]; HEIGHT];
    }

    pub fn update_display(&mut self) {
        // update sdl2 window to match pixels
    }

    // !!!
    pub fn draw(&mut self) {
        // draw instruction from cpu
    }
}