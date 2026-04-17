pub struct MemoryBus {
    pub rom: [u8; 0x8000],
    pub wram: [u8; 0x2000],
    pub serial_data: u8
}

impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus { rom: [0; 0x8000], wram: [0; 0x2000], serial_data: 0 }
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

            // ---- Link Cable Hack
            0xFF01 => {},
            0xFF02 => {
                if value == 0x81 {
                    let letter = self.read_byte(0xFF01) as char;
                    print!("{}", letter);
                }
            },
            _ => panic!("Adres buiten bereik!")
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
