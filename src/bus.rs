pub struct MemoryBus {
    pub rom: [u8; 0x8000],
    pub wram: [u8; 0x2000]
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus { rom: [0; 0x8000], wram: [0; 0x2000] }
    }

    pub fn read_byte(&self, address: u16) -> u8 {

        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            _ => 0xFF
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {

        match address {
            0x0000..=0x7FFF => self.rom[address as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            _ => panic!("Adres buiten bereik!")
        }
    }
}
