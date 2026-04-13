pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const SCREEN_PIXELS: u32 = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct PPU {
    pub framebuffer: [u32; SCREEN_PIXELS as usize]
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            framebuffer: [0; SCREEN_PIXELS as usize]
        }
    }

    pub fn apply_palette(&mut self, raw_pixels: &[u8], palette: impl Fn (u8) -> u32) {

        for (i, &pixel) in raw_pixels.iter().enumerate() {
            if i < self.framebuffer.len() {
                self.framebuffer[i] = palette(pixel);
            }
        }
    }
}
