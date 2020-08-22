use crate::instructions::InstructionSet;
use crate::write_to_memory::write_font_data_to_memory;
use crate::write_to_memory::write_rom_data_to_memory;
use ggez::event::KeyCode;
use rand::Rng;

pub const MEMORY_SIZE: usize = 0x1000; // 4096
pub const CHIP8_RESERVED_MEMORY_SIZE: usize = 0x200; // 512

pub const SCALE: usize = 16;
pub const DISPLAY_SIZE: [usize; 2] = [64, 32];

pub const DEBUG_EXTRA_DISPLAY_SIZE: [f32; 2] = [300.0, 300.0];

pub struct Chip8VM {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; 16],
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub stack: [u16; 16],
    pub pressed_key: Option<u8>,
    pub display_data: [bool; DISPLAY_SIZE[0] * DISPLAY_SIZE[1]],
    pub waiting_for_key_press: bool,
    pub key_index_store: u8,
}

impl Chip8VM {
    pub fn new(rom_data: &[u8]) -> Self {
        let mut vm = Self {
            waiting_for_key_press: false,
            key_index_store: 0x00,
            display_data: [false; DISPLAY_SIZE[0] * DISPLAY_SIZE[1]],
            memory: [0 as u8; MEMORY_SIZE],
            v: [0 as u8; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0 as u16; 16],
            pressed_key: None,
        };

        write_font_data_to_memory(&mut vm.memory);
        write_rom_data_to_memory(&mut vm.memory, rom_data);

        vm
    }

    pub fn handle_keyboard_input(&mut self, keycode: KeyCode) {
        self.pressed_key = match keycode {
            KeyCode::Key1 => Some(0x1),
            KeyCode::Key2 => Some(0x2),
            KeyCode::Key3 => Some(0x3),
            KeyCode::Key4 => Some(0xC),
            KeyCode::Q => Some(0x4),
            KeyCode::W => Some(0x5),
            KeyCode::E => Some(0x6),
            KeyCode::R => Some(0xD),
            KeyCode::A => Some(0x7),
            KeyCode::S => Some(0x8),
            KeyCode::D => Some(0x9),
            KeyCode::F => Some(0xE),
            KeyCode::Z => Some(0xA),
            KeyCode::X => Some(0x0),
            KeyCode::C => Some(0xB),
            KeyCode::V => Some(0xF),
            _ => None,
        };
    }

    pub fn execute_instruction(&mut self, instruction: InstructionSet, opcode: u16) {
        match instruction {
            InstructionSet::ClearScreen => {
                for i in 0..self.display_data.len() {
                    self.display_data[i] = false;
                }
            }
            InstructionSet::ReturnFromSubroutine => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
            }
            InstructionSet::JumpToAddress(address) => self.program_counter = address - 2,
            InstructionSet::ExecuteSubroutine(address) => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = address - 2;
            }
            InstructionSet::AddToRegister(index, value) => self.v[index as usize] += value,
            InstructionSet::StoreInRegister(index, value) => self.v[index as usize] = value,
            InstructionSet::CopyVyValueToVx(x, y) => self.v[x as usize] = self.v[y as usize],
            InstructionSet::SkipFollowingIfRegisterIsEqualToValue(index, value) => {
                if self.v[index as usize] == value {
                    self.program_counter += 2;
                }
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(index, value) => {
                if self.v[index as usize] != value {
                    self.program_counter += 2;
                }
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToOtherRegister(x, y) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.program_counter += 2;
                }
            }
            InstructionSet::SetVxToVxOrVy(x, y) => self.v[x as usize] |= self.v[y as usize],
            InstructionSet::SetVxToVxAndVy(x, y) => self.v[x as usize] &= self.v[y as usize],
            InstructionSet::SetVxToVxXorVy(x, y) => self.v[x as usize] ^= self.v[y as usize],
            InstructionSet::AddVyValueToVx(x, y) => {
                let sum = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                self.v[0xF] = if sum > 255 { 1 } else { 0 };
                self.v[x as usize] = (sum & 0x00FF) as u8;
            }
            InstructionSet::SubtractVyValueFromVx(x, y) => {
                self.v[0xF] = if self.v[x as usize] > self.v[y as usize] {
                    1
                } else {
                    0
                };
                self.v[x as usize] = (self.v[x as usize] as i32 - self.v[y as usize] as i32) as u8;
            }
            InstructionSet::StoreVyValueShiftedRightOneBitInVx(x, y) => {
                self.v[x as usize] = self.v[y as usize] >> 1;
                self.v[0xF] = self.v[y as usize] & 0x01;
            }
            InstructionSet::SetVxToValueOfVyMinusVx(x, y) => {
                self.v[0xF] = if self.v[y as usize] > self.v[x as usize] {
                    1
                } else {
                    0
                };
                self.v[x as usize] = (self.v[y as usize] as i32 - self.v[x as usize] as i32) as u8;
            }
            InstructionSet::StoreVyValueShiftedLeftOneBitInVx(x, y) => {
                self.v[x as usize] = self.v[y as usize] << 1;
                self.v[0xF] = (self.v[y as usize] >> 7) & 0x01;
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToOtherRegister(x, y) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.program_counter += 2;
                }
            }
            InstructionSet::StoreDelayTimerInRegisterVx(index) => {
                self.v[index as usize] = self.delay_timer;
            }
            InstructionSet::WaitForAKeyPress(index) => {
                self.waiting_for_key_press = true;
                self.key_index_store = index;
            }
            InstructionSet::SetDelayTimerToVx(index) => {
                self.delay_timer = self.v[index as usize];
            }
            InstructionSet::SetSoundTimerToVx(index) => {
                self.sound_timer = self.v[index as usize];
            }
            InstructionSet::AddVxToRegisterI(index) => {
                self.i += self.v[index as usize] as u16;
            }
            InstructionSet::SetIToTheMemoryAddressOfSpriteCorrespondingToVx(index) => {
                self.i = self.v[index as usize] as u16 * 5;
            }
            InstructionSet::StoreTheBinaryCodedDecimalEquivalentOfVx(index) => {
                let v = self.v[index as usize];
                let hundreds_digit = (v - (v % 100)) / 100;
                let tens_digit = (v % 100 - v % 10) / 10;
                let units_digit = v % 10;
                self.memory[self.i as usize] = hundreds_digit;
                self.memory[self.i as usize + 1] = tens_digit;
                self.memory[self.i as usize + 2] = units_digit;
            }
            InstructionSet::StoreValuesOfV0ToVxInclusiveInMemoryStartingAtAddressI(index) => {
                for i in 0..(index + 1) {
                    self.memory[self.i as usize + i as usize] = self.v[i as usize];
                }
                self.i += index as u16 + 1;
            }
            InstructionSet::FillRegistersV0ToVxInclusiveWithMemoryStartingAtAddressI(index) => {
                for i in 0..(index + 1) {
                    self.v[i as usize] = self.memory[self.i as usize + i as usize];
                }
                self.i += index as u16 + 1;
            }
            InstructionSet::DrawSprite(x, y, sprite_data) => {
                let height = sprite_data;

                self.v[0xF] = 0;
                for i in 0..height {
                    let row = self.memory[self.i as usize + i as usize];
                    for j in 0..8 {
                        let x_pos = match self.v[x as usize] + j {
                            pos if pos >= DISPLAY_SIZE[0] as u8 => pos - DISPLAY_SIZE[0] as u8,
                            pos => pos,
                        };

                        let y_pos = match self.v[y as usize] + i {
                            pos if pos >= DISPLAY_SIZE[1] as u8 => pos - DISPLAY_SIZE[1] as u8,
                            pos => pos,
                        };

                        let index = (y_pos as usize * DISPLAY_SIZE[0]) + x_pos as usize;
                        let value = self.display_data[index] as u8 ^ ((row >> (7 - j)) % 2);
                        if self.display_data[index] == true && value == 0 {
                            self.v[0xF] = 1;
                        }
                        self.display_data[index] = value == 1;
                    }
                }
            }
            InstructionSet::SkipFollowingIfKeyCorrespondingToVxIsNotPressed(index) => {
                if let Some(code) = self.pressed_key {
                    if code != self.v[index as usize] {
                        self.program_counter += 2;
                    }
                }
            }
            InstructionSet::SkipFollowingIfKeyCorrespondingToVxIsPressed(index) => {
                if let Some(code) = self.pressed_key {
                    if code == self.v[index as usize] {
                        self.program_counter += 2;
                    }
                }
            }
            InstructionSet::StoreAddressInRegisterI(address) => {
                self.i = address;
            }
            InstructionSet::SetVxToRandomNumberWithAMaskOf(index, mask) => {
                let mut rng = rand::thread_rng();
                self.v[index as usize] = rng.gen_range(0, 255) & mask;
            }
            InstructionSet::JumpToAddressWithV0Offset(address) => {
                self.program_counter = address + self.v[0x0] as u16 - 0x02;
            }
            _ => {
                if opcode != 0 {
                    println!("[{:04X?}]: {}", opcode, instruction.to_string());
                }
            }
        }
    }
}
