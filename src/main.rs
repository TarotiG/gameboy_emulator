// mod cpu;
// mod mmu;
// mod ppu;

struct FrameBuffer {
    data: Vec<u8>,
    width: usize,
    height: usize
}

impl FrameBuffer {
    fn new(data: Vec<u8>, width: usize, height: usize) -> Self {
        FrameBuffer {data, width, height}
    }

    fn set_pixel(&mut self, x: u8, y: u8, r: u8, g: u8, b: u8) {

    }

    // SAFETY: u8 buffer is aligned voor u32
    // lengte is meervoud van 4
    unsafe fn as_u32_slice(&self) -> &[u32] {

    }
}

fn main() {

}
