use std::fmt;


pub trait MemoryBus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

pub struct GameBoyMem {
    pub rom: [u8; 0x8000],
    pub ram: [u8; 0x8000]
}

impl GameBoyMem {
    pub fn new() -> Self {
        GameBoyMem {
            rom: [0; 0x8000],
            ram: [0; 0x8000]
        }
    }
}

impl MemoryBus for GameBoyMem {
    fn read(&self, addr: u16) -> u8 {

        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0xFFFF => self.ram[(addr - 0x8000) as usize],
            _ => panic!("Adres buiten bereik!")
        }
    }

    fn write(&mut self, addr: u16, val: u8) {

        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize] = val,
            0x8000..=0xFFFF => self.ram[(addr - 0x8000) as usize] = val,
            _ => panic!("Adres buiten bereik!")
        }
    }
}

impl fmt::Display for GameBoyMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for n in 0..=15 {
            write!(f, "{:02X} ", self.read(n))?
        }

        Ok(())
    }
}

pub fn fill<M: MemoryBus>(mem: &mut M, start: u16, val: u8, count: u16) {

    for n in 0..count {
        let input = start + n;
        mem.write(input, val);
    }
}
