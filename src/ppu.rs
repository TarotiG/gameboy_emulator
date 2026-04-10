pub const SCREEN_WIDTH = 160;
pub const SCREEN_HEIGHT = 144;
pub const SCREEN_PIXELS = SCREEN_WIDTH * SCREEN_HEIGHT;

pub struct PPU {
    pub framebuffer: [u32; SCREEN_PIXELS]
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            framebuffer: [0; SCREEN_PIXELS]
        }
    }

    pub fn apply_palette(&mut self, raw_pixels: &[u8], palette: impl Fn (u8) -> u32) {

        for (i, &pixel) in raw_pixels.iter().enumerate() {
            if i < self.frame_buffer.len() {
                self.frame_buffer[i] = palette(pixel);
            }
    }
}
