// pub enum Instruction {
//     Nop,
//     Load(u8, u8),
//     Jump(u16)
// }
//
// pub fn execute_instruction(ins: Instruction) {
//     match ins {
//         Instruction::Nop => println!("No instruction received"),
//         Instruction::Load(x, y) => println!("Load value {} in register {}", x, y),
//         Instruction::Jump(x) => println!("Jump to memory address: {}", x)
//     }
// }
//
// pub enum EmulatorState {
//     Running,
//     Paused(u8),
//     Error(String)
// }
//
// pub fn check_state(state: EmulatorState) {
//     match state {
//         EmulatorState::Running => println!("GameBoy is running!"),
//         EmulatorState::Paused(0) => println!("Pause is over, continue!"),
//         EmulatorState::Paused(bits) if bits >= 5 => println!("Pause is taking a while, screen will go on stand-by..."),
//         EmulatorState::Paused(bits) => println!("Short break of {} seconds", bits),
//         EmulatorState::Error(message) => println!("{}", message)
//     }
// }
//
use crate::bus::MemoryBus;


pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8

}

impl Registers {
    pub fn new() -> Self {
        Registers { a: 0x00, b: 0x00, c: 0x00, d: 0x00, e: 0x00, f: 0x00, h: 0x00, l: 0x00 }
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8 & 0xFF;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8 & 0xFF;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8 & 0xFF;
    }

    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.f |= 0b1000_0000;
        } else {
            self.f &= 0b0111_1111;
        }
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.f & 0b1000_0000) != 0
    }
}

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    pub ime: bool // Interrupt Master Enable
}

impl CPU {
    pub fn new(bus: MemoryBus) -> Self {
        CPU { registers: Registers::new(), pc: 0x0100, sp: 0xDFFF, bus, ime: false }
    }

    pub fn step(&mut self) {
        let opcode = self.bus.read_byte(self.pc);
        self.pc += 1;

        match opcode {
            0x00 => println!("Niks doen"),
            0x0E => {
                let value = self.bus.read_byte(self.pc);
                self.pc += 1;
                self.registers.c = value;
                self.pc += 1;
            }
            0x11 => {
                let value = self.read_next_u16();
                self.registers.set_de(value);
            },
            0x12 => {
                let value = self.read_next_u16();
                self.registers.set_de(value);
            },
            0x14 => { // VERBETEREN
                if self.registers.d < 255 {
                    self.registers.d += 1;
                } else {
                    self.registers.h = self.registers.h.wrapping_add(1);
                }

                let isAan = self.registers.h & 0b1000_0000 == 0;
                self.registers.set_zero_flag(isAan); // Z flag
                self.registers.f &= 0b1011_1111; // N flag
                self.registers.f |= 0b0010_0000;
            },
            0xAF => { // XOR A
                self.registers.a ^= self.registers.a;
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.f = 0b1000_0000;
            },
            0x21 => {
                let value = self.read_next_u16();
                self.registers.set_hl(value);
            },
            0x3E => {
                self.registers.a = self.bus.read_byte(self.pc);
                self.pc += 1;
            },
            0x47 => {
                self.registers.b = self.registers.a;
            }
            0xC3 => { // JP a16
                self.pc = self.read_next_u16();
            },
            0xCA => { // JP a16
                let next_byte = self.read_next_u16();

                if self.registers.get_zero_flag() {
                    self.pc = next_byte;
                } else {
                    println!("Doe niks");
                }
            },
            0xCB => {
                let cb_opcode = self.bus.read_byte(self.pc);
                self.pc += 1;
                self.execute_cb_prefix(cb_opcode);
            },
            0xCD => { // CALL a16
                let target = self.read_next_u16();
                self.push_u16(self.pc);
                self.pc = target;
            },
            0xC9 => { // RET
                let target = self.pop_u16();
                self.pc = target;
            },
            0xF3 => self.ime = false,
            0xFB => self.ime = true,
            _ => panic!("Onbekende opcode: {:#04X}", opcode)
        };
    }

    fn read_next_u16(&mut self) -> u16 {
        let lo = self.bus.read_byte(self.pc);
        self.pc += 1;

        let hi = self.bus.read_byte(self.pc);
        self.pc += 1;

        (hi as u16) << 8 | (lo as u16)
    }

    fn push_u16(&mut self, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = value as u8;
        self.sp -= 1;

        self.bus.write_byte(self.sp, hi);
        self.sp -= 1;

        self.bus.write_byte(self.sp, lo);
    }

    fn pop_u16(&mut self) -> u16 {
        let lo_value = self.bus.read_byte(self.sp);
        self.sp += 1;

        let hi_value = self.bus.read_byte(self.sp);
        self.sp += 1;

        ((hi_value as u16) << 8) | (lo_value as u16)
    }

    fn execute_cb_prefix(&mut self, cb_opcode: u8) {
        match cb_opcode {
            0x7C => {
                let isAan = self.registers.h & 0b1000_0000 == 0;
                self.registers.set_zero_flag(isAan);
                self.registers.f &= 0b1011_1111;
                self.registers.f |= 0b0010_0000;
            }
            _ => panic!("Onbekende CB prefix opcode: {:#04X}", cb_opcode)
        }
    }
}

// Helper functies
