pub const MEMORY_SIZE: usize = 0x1000; // 4096
pub const CHIP8_RESERVED_MEMORY_SIZE: usize = 0x200; // 512

pub const SCALE: usize = 16;
pub const DISPLAY_SIZE: [usize; 2] = [64, 32];

pub const DEBUG_EXTRA_DISPLAY_SIZE: [f32; 2] = [300.0, 300.0];

pub struct Chip8State {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; 16],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub stack: [u16; 16],
}
