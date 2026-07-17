pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![255; width * height * 3],
        }
    }

    pub fn clear(&mut self, red: u8, green: u8, blue: u8) {
        for pixel in self.pixels.chunks_exact_mut(3) {
            pixel[0] = red;
            pixel[1] = green;
            pixel[2] = blue;
        }
    }

    pub fn set_pixel(
        &mut self,
        x: i32,
        y: i32,
        red: u8,
        green: u8,
        blue: u8,
    ) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return;
        }

        let index = (y * self.width + x) * 3;

        self.pixels[index] = red;
        self.pixels[index + 1] = green;
        self.pixels[index + 2] = blue;
    }
}