use std::fs;
use crate::bus::MemoryBus;
use crate::cpu::CPU;

mod cpu;
// mod mmu;
mod ppu;
mod bus;
mod cartridge;


fn main() {
    let mut bus = MemoryBus::new();

    let rom_data = fs::read("06-ld r,r.gb").expect("Kan het ROM bestand niet lezen");

    bus.load_rom(&rom_data);
    let mut cpu = CPU::new(bus);

    println!("GameBoy opgestart, Begonnen met uitvoeren...");

    let mut stappen: u64 = 0;
    'Gameboy: loop {
        cpu.step();
        stappen += 1;

        if stappen % 1000000 == 0 {
            println!("Hartslag: {} miljoen stappen.. PC is nu op {:06X}", stappen/1_000_000, cpu.pc);
        }
    }

    // for i in 0..10 {
    //     cpu.step();
    // }
}
