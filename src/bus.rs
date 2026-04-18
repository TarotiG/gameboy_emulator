pub struct MemoryBus {
    pub rom: [u8; 0x8000],
    pub wram: [u8; 0x2000],
    pub hram: [u8; 127],
    pub ie_register: u8
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus { rom: [0; 0x8000], wram: [0; 0x2000], hram: [0; 127], ie_register: 0 }
    }

    pub fn read_byte(&self, address: u16) -> u8 {

        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0x8000..=0x9FFF => 0xFF, // VRAM (Videokaart - nog niet gebouwd)
            0xA000..=0xBFFF => 0xFF, // External RAM (op de cartridge)
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => 0xFF, // Echo RAM (kopie van WRAM, mag je negeren)
            0xFE00..=0xFE9F => 0xFF, // OAM (Sprite geheugen)
            0xFEA0..=0xFEFF => 0xFF, // Onbruikbaar geheugen

            // --- I/O & HRAM ---
            0xFF01 => 0x00,
            0xFF02 => 0x00,
            0xFF44 => 0x90,
            0xFF00..=0xFF7F => 0xFF,
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,

            _ => panic!("Lezen: Adres buiten bereik: {:#06X}", address)
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {

        match address {
            0x0000..=0x7FFF => {}, // NIET NAAR ROM SCHRIJVEN! (Gebruikt voor MBC later)
            0x8000..=0x9FFF => {}, // VRAM
            0xA000..=0xBFFF => {}, // External RAM
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => {}, // Echo RAM
            0xFE00..=0xFE9F => {}, // OAM
            0xFEA0..=0xFEFF => {}, // Onbruikbaar geheugen

            // --- I/O & HRAM ---
            0xFF01 => {},
            0xFF02 => {
                if value == 0x81 {
                    let letter = self.read_byte(0xFF01) as char;
                    print!("{}", letter);
                }
            },
            0xFF44 => {},
            0xFF00..=0xFF7F => {}, // Overige I/O negeren
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,

            _ => panic!("Schrijven: Adres buiten bereik: {:#06X}", address)
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        for (i, &byte) in rom_data.iter().enumerate() {
            if i < self.rom.len() {
                self.rom[i] = byte;
            }
        }
    }
}
