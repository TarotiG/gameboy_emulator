
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

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = value as u8 & 0xFF;
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

    pub fn set_subtract_flag(&mut self, value: bool) {
        if value {
            self.f |= 0b0100_0000;
        } else {
            self.f &= 0b1011_1111;
        }
    }

    pub fn get_subtract_flag(&self) -> bool {
        (self.f & 0b0100_0000) != 0
    }

    pub fn set_half_carry_flag(&mut self, value: bool) {
        if value {
            self.f |= 0b0010_0000;
        } else {
            self.f &= 0b1101_1111;
        }
    }
    pub fn get_half_carry_flag(&self) -> bool {
        (self.f & 0b0010_0000) != 0
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.f |= 0b0001_0000;
        } else {
            self.f &= 0b1110_1111;
        }
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.f & 0b0001_0000) != 0
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

        // Debug line
        // println!("PC: {:#06X} | Opcode: {:#04X} | A: {:#04X}", self.pc, opcode, self.registers.a);

        self.pc += 1;

        // Debug lines
        // if self.pc == 0x0200 || self.pc == 0x020B || self.pc == 0x020D {
        //     println!("PC: {:#06X} | C: {:#04X} | D: {:#04X} | E: {:#04X} | Z-flag: {}",
        //              self.pc,
        //              self.registers.c,
        //              self.registers.d,
        //              self.registers.e,
        //              self.registers.get_zero_flag()
        //     );
        // }

        match opcode {
            0x00 => {},
            0x01 => {
                let adres = self.read_next_u16();
                self.registers.set_bc(adres);
            },
            0x02 => {
                self.registers.set_bc(self.registers.a as u16);
            },
            0x03 => {
                let bc = self.registers.get_bc();
                self.registers.set_bc(bc.wrapping_add(1));
            },
            0x04 => {
                let hc = (self.registers.b & 0x0F) == 0x0F;
                self.registers.b = self.registers.b.wrapping_add(1);

                // flags
                self.registers.set_zero_flag(self.registers.b == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(hc);
            },
            0x05 => {
                let hc = (self.registers.b & 0x0F) == 0x0F;
                self.registers.b = self.registers.b.wrapping_sub(1);

                // flags
                self.registers.set_zero_flag(self.registers.b == 0);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag(hc);
            },
            0x06 => {
                let value = self.bus.read_byte(self.pc);
                self.registers.b = value;
            },
            0x07 => {
                let vallend_bitje = self.registers.a >> 7;

                self.registers.a = (self.registers.a << 1) | vallend_bitje;

                // flags
                self.registers.set_zero_flag(false);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(vallend_bitje != 0);
            },
            0x0B => {
                let bc = self.registers.get_bc();
                self.registers.set_bc(bc.wrapping_sub(1));
            },
            0x0C => {
                let hc = (self.registers.c & 0x0F) == 0x0F;
                self.registers.c = self.registers.c.wrapping_add(1);

                // flags
                self.registers.set_zero_flag(self.registers.c == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(hc);
            },
            0x0D => {
                let hc = (self.registers.c & 0x0F) == 0x0F;
                self.registers.c = self.registers.c.wrapping_sub(1);

                // flags
                self.registers.set_zero_flag(self.registers.c == 0);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag(hc);
            },
            0x0E => {
                let value = self.bus.read_byte(self.pc);
                self.pc += 1;
                self.registers.c = value;
            },
            0x0F => {
                let vallend_bitje = self.registers.a << 7;

                self.registers.a = (self.registers.a >> 1) | vallend_bitje;

                // flags
                self.registers.set_zero_flag(false);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(vallend_bitje != 0);
            },
            0x10 => { // STOP
                let _dummy_byte = self.bus.read_byte(self.pc);
                self.pc += 1;
                println!("GameBoy in STOP modus...");
            },
            0x11 => {
                let value = self.read_next_u16();
                self.registers.set_de(value);
            },
            0x12 => {
                let adres = self.registers.get_de();
                self.bus.write_byte(adres, self.registers.a);
            },
            0x13 => {
                let de = self.registers.get_de();
                self.registers.set_de(de.wrapping_add(1));
            },
            0x14 => {
                let hc = (self.registers.d & 0x0F) == 0x0F;

                self.registers.d = self.registers.d.wrapping_add(1);

                // flags
                self.registers.set_zero_flag(self.registers.d == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(hc);
            },
            0x17 => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.a & 0x01) != 0;

                self.registers.a = (self.registers.a << 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(false);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x18 => {
                let offset_byte = self.bus.read_byte(self.pc);
                self.pc += 1;

                let offset = offset_byte as i8;
                self.pc = self.pc.wrapping_add_signed(offset as i16);
            },
            0x1B => {
                let de = self.registers.get_de();
                self.registers.set_de(de.wrapping_sub(1));
            },
            0x1C => {
                let hc = (self.registers.e & 0x0F) == 0x0F;

                self.registers.e = self.registers.e.wrapping_add(1);

                // flags
                self.registers.set_zero_flag(self.registers.e == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(hc);
            },
            0x1F => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.a & 0x01) != 0;

                self.registers.a = (self.registers.a >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(false);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            }
            0x20 => {
                let offset_byte = self.bus.read_byte(self.pc);
                self.pc += 1;

                let offset = offset_byte as i8;

                if !self.registers.get_zero_flag() {
                    self.pc = self.pc.wrapping_add_signed(offset as i16);
                }
            },
            0x21 => {
                let value = self.read_next_u16();
                self.registers.set_hl(value);
            },
            0x22 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.a);
                self.registers.set_hl(adres.wrapping_add(1));
            },
            0x23 => {
                let hl = self.registers.get_hl();
                self.registers.set_hl(hl.wrapping_add(1));
            },
            0x25 => {
                let hc = (self.registers.h & 0x0F) == 0x0F;
                self.registers.h = self.registers.h.wrapping_sub(1);

                // flags
                self.registers.set_zero_flag(self.registers.h == 0);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag(hc);
            },
            0x26 => {
                let value = self.bus.read_byte(self.pc);
                self.pc += 1;
                self.registers.h = value;
            },
            0x2A => {
                let adres = self.registers.get_hl();
                self.registers.a = self.bus.read_byte(adres);
                self.registers.set_hl(adres.wrapping_add(1));
            },
            0x2B => {
                let hl = self.registers.get_hl();
                self.registers.set_hl(hl.wrapping_sub(1));
            },
            0x2C => {
                let hc = (self.registers.l & 0x0F) == 0x0F;

                self.registers.l = self.registers.l.wrapping_add(1);

                // flags
                self.registers.set_zero_flag(self.registers.l == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(hc);
            },
            0x2D => {
                let hc = (self.registers.l & 0x0F) == 0x0F;
                self.registers.l = self.registers.l.wrapping_sub(1);

                // flags
                self.registers.set_zero_flag(self.registers.l == 0);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag(hc);
            },
            0x2F => { // CPL
                self.registers.a = !self.registers.a;

                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag(true);
            },
            0x30 => {
                let offset_byte = self.bus.read_byte(self.pc);
                self.pc += 1;

                let offset = offset_byte as i8;

                if !self.registers.get_carry_flag() {
                    self.pc = self.pc.wrapping_add_signed(offset as i16);
                }
            },
            0x31 => {
                let adres = self.read_next_u16();
                self.sp = adres;
            },
            0x32 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.a);
                self.registers.set_hl(adres.wrapping_sub(1));
            },
            0x33 => {
                self.sp = self.sp.wrapping_add(1);
            },
            0x36 => {
                let adres = self.registers.get_hl();
                let value = self.bus.read_byte(adres);
                self.pc += 1;

                self.bus.write_byte(adres, value);
            },
            0x37 => {
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(true);
            },
            0x38 => {
                let offset_byte = self.bus.read_byte(self.pc);
                self.pc += 1;

                let offset = offset_byte as i8;

                if self.registers.get_carry_flag() {
                    self.pc = self.pc.wrapping_add_signed(offset as i16);
                }
            },
            0x3A => {
                let adres = self.registers.get_hl();
                self.registers.a = self.bus.read_byte(adres);
                self.registers.set_hl(adres.wrapping_sub(1));
            },
            0x3B => {
                self.sp = self.sp.wrapping_sub(1);
            },
            0x3C => {
                let hc = (self.registers.a & 0x0F) == 0x0F;
                self.registers.a = self.registers.a.wrapping_add(1);

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(hc);
            },
            0x3D => {
                let hc = (self.registers.a & 0x0F) == 0x0F;
                self.registers.a = self.registers.a.wrapping_sub(1);

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag(hc);
            },
            0x3E => {
                self.registers.a = self.bus.read_byte(self.pc);
                self.pc += 1;
            },
            0x3F => {
                let carry = self.registers.get_carry_flag();

                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(!carry);
            },
            0x40 => self.registers.b = self.registers.b,
            0x41 => self.registers.b = self.registers.c,
            0x42 => self.registers.b = self.registers.d,
            0x43 => self.registers.b = self.registers.e,
            0x44 => self.registers.b = self.registers.h,
            0x45 => self.registers.b = self.registers.l,
            0x46 => {
                let adres = self.registers.get_hl();
                self.registers.b = self.bus.read_byte(adres);
            },
            0x47 => {
                self.registers.b = self.registers.a;
            },
            0x48 => self.registers.c = self.registers.b,
            0x49 => self.registers.c = self.registers.c,
            0x4A => self.registers.c = self.registers.d,
            0x4B => self.registers.c = self.registers.e,
            0x4C => self.registers.c = self.registers.h,
            0x4D => self.registers.c = self.registers.l,
            0x4E => {
                let adres = self.registers.get_hl();
                self.registers.c = self.bus.read_byte(adres);
            },
            0x4F => {
                self.registers.c = self.registers.a;
            },
            0x50 => self.registers.d = self.registers.b,
            0x51 => self.registers.d = self.registers.c,
            0x52 => self.registers.d = self.registers.d,
            0x53 => self.registers.d = self.registers.e,
            0x54 => self.registers.d = self.registers.h,
            0x55 => self.registers.d = self.registers.l,
            0x56 => {
                let adres = self.registers.get_hl();
                self.registers.d = self.bus.read_byte(adres);
            },
            0x57 => {
                self.registers.d = self.registers.a;
            },
            0x58 => self.registers.e = self.registers.b,
            0x59 => self.registers.e = self.registers.c,
            0x5A => self.registers.e = self.registers.d,
            0x5B => self.registers.e = self.registers.e,
            0x5C => self.registers.e = self.registers.h,
            0x5D => self.registers.e = self.registers.l,
            0x5E => {
                let adres = self.registers.get_hl();
                let value = self.bus.read_byte(adres);

                self.registers.e = value;
            },
            0x5F => {
                self.registers.e = self.registers.a;
            },
            0x60 => self.registers.h = self.registers.b,
            0x61 => self.registers.h = self.registers.c,
            0x62 => self.registers.h = self.registers.d,
            0x63 => self.registers.h = self.registers.e,
            0x64 => self.registers.h = self.registers.h,
            0x65 => self.registers.h = self.registers.l,
            0x66 => { let adres = self.registers.get_hl(); self.registers.h = self.bus.read_byte(adres); },
            0x67 => self.registers.h = self.registers.a,
            0x68 => self.registers.l = self.registers.b,
            0x69 => self.registers.l = self.registers.c,
            0x6A => self.registers.l = self.registers.d,
            0x6B => self.registers.l = self.registers.e,
            0x6C => self.registers.l = self.registers.h,
            0x6D => self.registers.l = self.registers.l,
            0x6E => { let adres = self.registers.get_hl(); self.registers.l = self.bus.read_byte(adres); },
            0x6F => self.registers.l = self.registers.a,
            0x70 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.b);
            },
            0x71 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.c);
            },
            0x72 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.d);
            },
            0x73 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.e);
            },
            0x74 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.h);
            },
            0x75 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.l);
            },
            0x76 => { // HALT
                // Later implementeren
            },
            0x77 => {
                let adres = self.registers.get_hl();
                self.bus.write_byte(adres, self.registers.a);
            },
            0x78 => {
                self.registers.a = self.registers.b;
            },
            0x79 => {
                self.registers.a = self.registers.c;
            },
            0x7A => {
                self.registers.a = self.registers.d;
            },
            0x7B => {
                self.registers.a = self.registers.e;
            },
            0x7C => {
                self.registers.a = self.registers.h;
            },
            0x7D => {
                self.registers.a = self.registers.l;
            },
            0x7E => {
                let adres = self.registers.get_hl();
                self.registers.a = self.bus.read_byte(adres);
            },
            0x7F => self.registers.a = self.registers.a,
            0x80 => {
                let a = self.registers.a;
                let b = self.registers.b;
                let add = (a as u16) + (b as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (b & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x81 => {
                let a = self.registers.a;
                let c = self.registers.c;
                let add = (a as u16) + (c as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (c & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x82 => {
                let a = self.registers.a;
                let d = self.registers.d;
                let add = (a as u16) + (d as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (d & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x83 => {
                let a = self.registers.a;
                let e = self.registers.e;
                let add = (a as u16) + (e as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (e & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x84 => {
                let a = self.registers.a;
                let h = self.registers.h;
                let add = (a as u16) + (h as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (h & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x85 => {
                let a = self.registers.a;
                let l = self.registers.l;
                let add = (a as u16) + (l as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (l & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x86 => {
                let adres = self.registers.get_hl();
                let value = self.bus.read_byte(adres);
                let a = self.registers.a;
                let add = (a as u16) + (value as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x87 => {
                let a1 = self.registers.a;
                let a2 = self.registers.a;
                let add = (a1 as u16) + (a2 as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a1 & 0x0F) + (a2 & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0x90 => {
                let v = self.registers.b;
                let a = self.registers.a;
                self.registers.a = a.wrapping_sub(v);

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (v & 0x0F));
                self.registers.set_carry_flag(a < v);
            },
            0x91 => { let v = self.registers.c; let a = self.registers.a; self.registers.a = a.wrapping_sub(v); self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(true); self.registers.set_half_carry_flag((a & 0x0F) < (v & 0x0F)); self.registers.set_carry_flag(a < v); },
            0x92 => { let v = self.registers.d; let a = self.registers.a; self.registers.a = a.wrapping_sub(v); self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(true); self.registers.set_half_carry_flag((a & 0x0F) < (v & 0x0F)); self.registers.set_carry_flag(a < v); },
            0x93 => { let v = self.registers.e; let a = self.registers.a; self.registers.a = a.wrapping_sub(v); self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(true); self.registers.set_half_carry_flag((a & 0x0F) < (v & 0x0F)); self.registers.set_carry_flag(a < v); },
            0x94 => { let v = self.registers.h; let a = self.registers.a; self.registers.a = a.wrapping_sub(v); self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(true); self.registers.set_half_carry_flag((a & 0x0F) < (v & 0x0F)); self.registers.set_carry_flag(a < v); },
            0x95 => { let v = self.registers.l; let a = self.registers.a; self.registers.a = a.wrapping_sub(v); self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(true); self.registers.set_half_carry_flag((a & 0x0F) < (v & 0x0F)); self.registers.set_carry_flag(a < v); },
            0x96 => { let a = self.registers.a; let adres = self.registers.get_hl(); let v = self.bus.read_byte(adres); self.registers.a = a.wrapping_sub(v); self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(true); self.registers.set_half_carry_flag((a & 0x0F) < (v & 0x0F)); self.registers.set_carry_flag(a < v); },
            0x97 => { self.registers.a = 0; self.registers.set_zero_flag(true); self.registers.set_subtract_flag(true); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xA0 => {
                self.registers.b &= self.registers.b;

                self.registers.set_zero_flag(self.registers.b == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA1 => {
                self.registers.c &= self.registers.c;

                self.registers.set_zero_flag(self.registers.c == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA2 => {
                self.registers.d &= self.registers.d;

                self.registers.set_zero_flag(self.registers.d == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA3 => {
                self.registers.e &= self.registers.e;

                self.registers.set_zero_flag(self.registers.e == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA4 => {
                self.registers.h &= self.registers.h;

                self.registers.set_zero_flag(self.registers.h == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA5 => {
                self.registers.l &= self.registers.l;

                self.registers.set_zero_flag(self.registers.l == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA6 => {
                let adres_hl = self.registers.get_hl();
                let value = self.bus.read_byte(adres_hl);

                self.registers.a &= value;

                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA7 => {
                self.registers.a &= self.registers.a;

                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xA8 => { self.registers.a ^= self.registers.b; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xA9 => { self.registers.a ^= self.registers.c; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xAA => { self.registers.a ^= self.registers.d; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xAB => { self.registers.a ^= self.registers.e; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xAC => { self.registers.a ^= self.registers.h; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xAD => { self.registers.a ^= self.registers.l; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xAE => {
                let adres_hl = self.registers.get_hl();
                let value = self.bus.read_byte(adres_hl);

                self.registers.a ^= value;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0xAF => {
                self.registers.a ^= self.registers.a;
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.f = 0b1000_0000;
            },
            0xB0 => {
                self.registers.b |= self.registers.b;

                self.registers.set_zero_flag(self.registers.b == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0xB1 => { self.registers.a |= self.registers.c; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xB2 => { self.registers.a |= self.registers.d; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xB3 => { self.registers.a |= self.registers.e; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xB4 => { self.registers.a |= self.registers.h; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xB5 => { self.registers.a |= self.registers.l; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xB6 => { let adres = self.registers.get_hl(); self.registers.a |= self.bus.read_byte(adres); self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xB7 => { self.registers.a |= self.registers.a; self.registers.set_zero_flag(self.registers.a == 0); self.registers.set_subtract_flag(false); self.registers.set_half_carry_flag(false); self.registers.set_carry_flag(false); },
            0xB8 => {
                let b = self.registers.b;
                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(b == a);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (b & 0x0F));
                self.registers.set_carry_flag(a < b);
            },
            0xB9 => {
                let c = self.registers.c;
                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(c == a);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (c & 0x0F));
                self.registers.set_carry_flag(a < c);
            },
            0xBA => {
                let d = self.registers.d;
                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(d == a);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (d & 0x0F));
                self.registers.set_carry_flag(a < d);
            },
            0xBB => {
                let e = self.registers.e;
                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(e == a);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (e & 0x0F));
                self.registers.set_carry_flag(a < e);
            },
            0xBC => {
                let h = self.registers.h;
                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(h == a);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (h & 0x0F));
                self.registers.set_carry_flag(a < h);
            },
            0xBD => {
                let l = self.registers.l;
                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(l == a);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (l & 0x0F));
                self.registers.set_carry_flag(a < l);
            },
            0xBE => {
                let adres = self.registers.get_hl();
                let value = self.bus.read_byte(adres);
                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(value == a);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
                self.registers.set_carry_flag(a < value);
            },
            0xBF => {
                let a1 = self.registers.a;
                let a2 = self.registers.a;

                // flags
                self.registers.set_zero_flag(a1 == a2);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a1 & 0x0F) < (a2 & 0x0F));
                self.registers.set_carry_flag(a1 < a2);
            },
            0xC0 => {

                if !self.registers.get_zero_flag() {
                    self.pop_u16();
                } else {
                    println!("Doe niks");
                }
            },
            0xC1 => {
                let bc = self.pop_u16();
                self.registers.set_bc(bc);
            },
            0xC3 => { // JP a16
                self.pc = self.read_next_u16();
            },
            0xC4 => { // CALL NZ, a16
                let target = self.read_next_u16();

                if !self.registers.get_zero_flag() {
                    self.push_u16(self.pc);
                    self.pc = target;

                } else {
                    println!("Doe niks");
                }
            },
            0xC5 => {
                let bc = self.registers.get_bc();
                self.push_u16(bc);
            },
            0xC6 => {
                let value = self.bus.read_byte(self.pc);
                self.pc += 1;

                let a = self.registers.a;
                let add = (a as u16) + (value as u16);

                self.registers.a = add as u8;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag((a & 0x0F) + (value & 0x0F) > 0x0F);
                self.registers.set_carry_flag(add > 0xFF);
            },
            0xCA => { // JP a16
                let next_byte = self.read_next_u16();

                if self.registers.get_zero_flag() {
                    self.pc = next_byte;
                } else {
                    println!("Doe niks");
                }
            },
            0xCB => { // CB prefix
                let cb_opcode = self.bus.read_byte(self.pc);
                self.pc += 1;
                self.execute_cb_prefix(cb_opcode);
            },
            0xCC => {
                let target = self.read_next_u16();

                if self.registers.get_carry_flag() {
                    self.push_u16(self.pc);
                    self.pc = target;
                } else {
                    println!("Doe niks");
                }

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
            0xD1 => {
                let de = self.pop_u16();
                self.registers.set_de(de);
            },
            0xD4 => {
                let target = self.read_next_u16();

                if !self.registers.get_carry_flag() {
                    self.push_u16(self.pc);
                    self.pc = target;
                } else {
                    println!("Doe niks");
                }
            },
            0xD5 => {
                let de = self.registers.get_de();
                self.push_u16(de);
            },
            0xD8 => {
                if self.registers.get_carry_flag() {
                    self.pop_u16();
                }
            },
            0xE0 => {
                let offset = self.bus.read_byte(self.pc);
                self.pc += 1;

                let adres = 0xFF00 | (offset as u16);
                self.bus.write_byte(adres, self.registers.a);
            },
            0xE1 => {
                let value = self.pop_u16();
                self.registers.set_hl(value);
            },
            0xE5 => {
                let hl = self.registers.get_hl();
                self.push_u16(hl);
            },
            0xE6 => {
                let value = self.bus.read_byte(self.pc);
                self.pc += 1;

                self.registers.a &= value;
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(true);
                self.registers.set_carry_flag(false);
            },
            0xEA => {
                let adres = self.read_next_u16();
                self.bus.write_byte(adres, self.registers.a);
                self.pc += 1;
            },
            0xEE => {
                let value = self.bus.read_byte(self.pc);
                self.pc += 1;

                self.registers.a ^= value;
                self.registers.set_zero_flag(value == 0);
            },
            0xF0 => {
                let offset = self.bus.read_byte(self.pc);
                self.pc += 1;

                let adres = 0xFF00 | (offset as u16);
                self.registers.a = self.bus.read_byte(adres);
            },
            0xF1 => {
                let af = self.pop_u16();
                self.registers.set_af(af & 0xFFF0);

                // flags
                self.registers.set_zero_flag(self.registers.f == 0);
                self.registers.set_subtract_flag(self.registers.f & 0x08 == 0x08);
                self.registers.set_half_carry_flag(self.registers.f & 0x10 == 0x10);
                self.registers.set_carry_flag(self.registers.f & 0x20 == 0x20);
            },
            0xF3 => self.ime = false,
            0xF5 => { // PUSH AF
                let a = self.registers.a as u16;
                let f = (self.registers.f & 0b1111_0000) as u16;

                let af = (a << 8) | f;
                self.push_u16(af);

            },
            0xFB => self.ime = true,
            0xFE => { // CP d8 -> OVERSCHRIJF REGISTER A NIET
                let value = self.bus.read_byte(self.pc);
                self.pc += 1;

                let a = self.registers.a;

                // flags
                self.registers.set_zero_flag(a == value);
                self.registers.set_subtract_flag(true);
                self.registers.set_half_carry_flag((a & 0x0F) < (value & 0x0F));
                self.registers.set_carry_flag(a < value);
            },
            0xFF => {
                self.push_u16(self.pc);
                self.pc = 0x038;
            },
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
        self.sp = self.sp.wrapping_add(1);

        let hi_value = self.bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);

        ((hi_value as u16) << 8) | (lo_value as u16)
    }

    fn execute_cb_prefix(&mut self, cb_opcode: u8) {
        match cb_opcode {
            0x18 => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.b & 0x01) != 0;

                self.registers.b = (self.registers.b >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(self.registers.b == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x19 => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.c & 0x01) != 0;

                self.registers.c = (self.registers.c >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(self.registers.c == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x1A => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.d & 0x01) != 0;

                self.registers.d = (self.registers.d >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(self.registers.d == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x1B => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.e & 0x01) != 0;

                self.registers.e = (self.registers.e >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(self.registers.e == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x1C => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.h & 0x01) != 0;

                self.registers.h = (self.registers.h >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(self.registers.h == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x1D => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.l & 0x01) != 0;

                self.registers.l = (self.registers.l >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(self.registers.l == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x1E => {
                let adres = self.registers.get_hl();
                let mut value = self.bus.read_byte(adres);

                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (value & 0x01) != 0;

                value = (value >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(value == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x1F => {
                let carry_oud = if self.registers.get_carry_flag() { 0x80 } else { 0x00 };
                let carry_nieuw = (self.registers.a & 0x01) != 0;

                self.registers.a = (self.registers.a >> 1) | carry_oud;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry_nieuw);
            },
            0x30 => {
                self.registers.b = self.registers.b.rotate_left(4);

                self.registers.set_zero_flag(self.registers.b == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x31 => {
                self.registers.c = self.registers.c.rotate_left(4);

                self.registers.set_zero_flag(self.registers.c == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x32 => {
                self.registers.d = self.registers.d.rotate_left(4);

                self.registers.set_zero_flag(self.registers.d == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x33 => {
                self.registers.e = self.registers.e.rotate_left(4);

                self.registers.set_zero_flag(self.registers.e == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x34 => {
                self.registers.h = self.registers.h.rotate_left(4);

                self.registers.set_zero_flag(self.registers.h == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x35 => {
                self.registers.l = self.registers.l.rotate_left(4);

                self.registers.set_zero_flag(self.registers.l == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x36 => {
                let adres = self.registers.get_hl();
                let mut value = self.bus.read_byte(adres);

                value = value.rotate_left(4);

                self.bus.write_byte(adres, value);

                self.registers.set_zero_flag(value == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x37 => {
                self.registers.a = self.registers.a.rotate_left(4);

                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(false);
            },
            0x38 => {
                let carry = (self.registers.b & 0x01) != 0;
                self.registers.b = self.registers.b >> 1;

                // flags
                self.registers.set_zero_flag(self.registers.b == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x39 => {
                let carry = (self.registers.c & 0x01) != 0;
                self.registers.c = self.registers.c >> 1;

                // flags
                self.registers.set_zero_flag(self.registers.c == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x3A => {
                let carry = (self.registers.d & 0x01) != 0;
                self.registers.d = self.registers.d >> 1;

                // flags
                self.registers.set_zero_flag(self.registers.d == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x3B => {
                let carry = (self.registers.e & 0x01) != 0;
                self.registers.e = self.registers.e >> 1;

                // flags
                self.registers.set_zero_flag(self.registers.e == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x3C => {
                let carry = (self.registers.h & 0x01) != 0;
                self.registers.h = self.registers.h >> 1;

                // flags
                self.registers.set_zero_flag(self.registers.h == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x3D => {
                let carry = (self.registers.l & 0x01) != 0;
                self.registers.l = self.registers.l >> 1;

                // flags
                self.registers.set_zero_flag(self.registers.l == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x3E => {
                let adres = self.registers.get_hl();
                let mut value = self.bus.read_byte(adres);

                let carry = (value & 0x01) != 0;

                let value = value >> 1;
                self.bus.write_byte(adres, value);

                // flags
                self.registers.set_zero_flag(value == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x3F => {
                let carry = (self.registers.a & 0x01) != 0;
                self.registers.a = self.registers.a >> 1;

                // flags
                self.registers.set_zero_flag(self.registers.a == 0);
                self.registers.set_subtract_flag(false);
                self.registers.set_half_carry_flag(false);
                self.registers.set_carry_flag(carry);
            },
            0x7C => {
                let is_aan = self.registers.h & 0b1000_0000 == 0;
                self.registers.set_zero_flag(is_aan);
                self.registers.f &= 0b1011_1111;
                self.registers.f |= 0b0010_0000;
            }
            _ => panic!("Onbekende CB prefix opcode: {:#04X}", cb_opcode)
        }
    }
}