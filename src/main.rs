use crate::bus::MemoryBus;
use crate::cpu::CPU;

mod cpu;
mod mmu;
mod ppu;
mod bus;
mod cartridge;


fn main() {
    let mut bus = MemoryBus::new();

    bus.write_byte(0x0100, 0x3E); // Instructie: LD A, d8
    bus.write_byte(0x0101, 0x42); // De data: het getal 0x42 (decimaal 66)
    bus.write_byte(0x0102, 0xAF); // Instructie: XOR A

    let mut cpu = CPU::new(bus);

    // Voer instructie 1 uit (LD A, 0x42)
    cpu.step();
    println!("Register A na stap 1: {:#X}", cpu.registers.a);

    // Voer instructie 2 uit (XOR A)
    cpu.step();
    println!("Register A na stap 2: {:#X}", cpu.registers.a);
}
