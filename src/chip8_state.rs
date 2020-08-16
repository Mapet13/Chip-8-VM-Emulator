pub const MEMORY_SIZE: usize = 0x1000; // 4096
pub const CHIP8_RESERVED_MEMORY_SIZE: usize = 0x200; // 512

pub struct Chip8State {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; 16],
    pub i: u16,
    pub _delay_timer: u8,
    pub _sound_timer: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub stack: [u16; 16],
}