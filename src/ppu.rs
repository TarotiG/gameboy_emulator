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

pub struct FrameBuffer {
    buffer: Vec<u8>,
}

impl FrameBuffer {
    pub fn new() -> Self {
        let lengte = SCREEN_WIDTH * SCREEN_HEIGHT * 4;

        if lengte % 4 != 0 {
            panic!("Ongeldige lengte!");
        }

        Self {
            buffer: vec![0; lengte as usize],
        }
    }

    /// Kleurt één specifieke pixel in op het scherm.
    pub fn set_pixel(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) {
        let x_usize = x as usize;
        let y_usize = y as usize;

        let index = (y_usize * SCREEN_WIDTH as usize + x_usize) * 4;

        self.buffer[index] = r;         // Rood
        self.buffer[index + 1] = g;     // Groen
        self.buffer[index + 2] = b;     // Blauw
        self.buffer[index + 3] = 255;   // Alpha (255 = volledig ondoorzichtig)
    }

    /// Zet de interne u8 buffer om naar een u32 slice voor de grafische kaart.
    ///
    /// # Safety
    ///
    /// Deze functie is `unsafe` omdat we een ruwe (raw) pointer casten van `u8` naar `u32`
    /// en vervolgens een nieuwe slice maken.
    /// Dit is in dit specifieke geval veilig omdat we in `new()` de garantie afdwingen
    /// dat de totale lengte van de buffer altijd een veelvoud van 4 is. Hierdoor
    /// lezen we nooit voorbij de grenzen van ons gereserveerde geheugen.
    pub unsafe fn as_u32_slice(&self) -> &[u32] {
        let ptr = self.buffer.as_ptr() as *const u32;

        let len = self.buffer.len() / 4;

        std::slice::from_raw_parts(ptr, len)
    }
}
