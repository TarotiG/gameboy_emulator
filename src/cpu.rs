pub enum Instruction {
    Nop,
    Load(u8, u8),
    Jump(u16)
}

pub fn execute_instruction(ins: Instruction) {
    match ins {
        Instruction::Nop => println!("No instruction received"),
        Instruction::Load(x, y) => println!("Load value {} in register {}", x, y),
        Instruction::Jump(x) => println!("Jump to memory address: {}", x)
    }
}

pub enum EmulatorState {
    Running,
    Paused(u8),
    Error(String)
}

pub fn check_state(state: EmulatorState) {
    match state {
        EmulatorState::Running => println!("GameBoy is running!"),
        EmulatorState::Paused(0) => println!("Pause is over, continue!"),
        EmulatorState::Paused(bits) if bits >= 5 => println!("Pause is taking a while, screen will go on stand-by..."),
        EmulatorState::Paused(bits) => println!("Short break of {} seconds", bits),
        EmulatorState::Error(message) => println!("{}", message)
    }
}

pub struct CPU {
    pub registers: Registers,
    pub pc: u16, // program counter
    pub sp: u16 // stack pointer
}

impl CPU {
    fn new() -> Self {
        CPU { registers: Registers::new(), pc: 0x0100, sp: 0xFFFE }
    }

    fn step(&mut self) {
        self.pc += 1
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
}

impl Registers {
    fn new() -> Self {
        Registers { a: 0x00, b: 0x00, c: 0x00, d: 0x00, e: 0x00, f: 0x00, h: 0x00, l: 0x00 }
    }
}

fn set_bit(value: u8, n: u8) -> u8 {
    let mask = 1 << n;
    mask | value
}

fn clear_bit(value: u8, n: u8) -> u8 {
    let mask = 1 << n;
    !mask & value
}

fn toggle_bit(value: u8, n: u8) -> u8 {
    let mask = 1 << n;
    value ^ mask
}
