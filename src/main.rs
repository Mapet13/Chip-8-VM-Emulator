use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};

use rand::Rng;

mod gui;
use gui::ImGuiWrapper;

mod chip8_state;
use chip8_state::*;

mod setup;
use setup::*;

mod utils;
use utils::*;

mod instructions;
use instructions::*;

mod fonts_sprites;
use fonts_sprites::FONTS_SPRITES;

fn write_rom_data_to_memory(memory: &mut [u8; MEMORY_SIZE], rom_data: &[u8]) {
    byte_copy(
        rom_data,
        &mut memory[CHIP8_RESERVED_MEMORY_SIZE..MEMORY_SIZE],
    );
}

fn fetch_opcode(memory: &[u8], pc: u16) -> Result<u16, ()> {
    if pc >= MEMORY_SIZE as u16 {
        return Err(());
    }

    Ok((memory[pc as usize] as u16) << 8 | memory[pc as usize + 1] as u16)
}

struct MainState {
    imgui_wrapper: ImGuiWrapper,
    hidpi_factor: f32,
    chip8_state: Chip8State,
    waiting_for_key_press: bool,
    key_index_store: u8,
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32, rom_data: &[u8]) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let mut s = MainState {
            imgui_wrapper,
            hidpi_factor,
            waiting_for_key_press: false,
            key_index_store: 0x00,
            chip8_state: Chip8State {
                memory: [0 as u8; MEMORY_SIZE],
                v: [0 as u8; 16],
                i: 0,
                delay_timer: 0,
                sound_timer: 0,
                program_counter: 0x200,
                stack_pointer: 0,
                stack: [0 as u16; 16],
                chip8_key: None,
            },
        };

        write_font_data_to_memory(&mut s.chip8_state.memory);
        write_rom_data_to_memory(&mut s.chip8_state.memory, rom_data);

        Ok(s)
    }
}

fn write_font_data_to_memory(memory: &mut [u8; MEMORY_SIZE]) {
    for i in 0..FONTS_SPRITES.len() {
        for j in 0..FONTS_SPRITES[i].len() {
            memory[i * FONTS_SPRITES[i].len() + j] = FONTS_SPRITES[i][j];
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
            if self.waiting_for_key_press {
                if let Some(code) = self.chip8_state.chip8_key {
                    self.chip8_state.v[self.key_index_store as usize] = code;
                    self.waiting_for_key_press = false;
                }
            } else {
                let opcode =
                    fetch_opcode(&self.chip8_state.memory, self.chip8_state.program_counter)
                        .unwrap();
                let instruction = decode_opcode(opcode);

                self.execute_instruction(instruction, opcode);
                if opcode != 0 {
                    //println!("[{:04X?}]: {}", opcode, instruction.to_string());
                }

                if let Some(code) = self.chip8_state.chip8_key {
                    println!("Key Pressed: {:02X?}", code);
                }

                if self.chip8_state.delay_timer > 0 {
                    self.chip8_state.delay_timer -= 1;
                }

                if self.chip8_state.sound_timer > 0 {
                    self.chip8_state.sound_timer -= 1;
                }

                self.chip8_state.program_counter += 2;
            }
        }

        //println!("FPS: {}", ggez::timer::fps(ctx));

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Render game stuff
        {}

        // Render game ui
        {
            self.imgui_wrapper
                .render(ctx, self.hidpi_factor, &self.chip8_state);
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.chip8_state.chip8_key = match keycode {
            KeyCode::Key1 => Some(0),
            KeyCode::Key2 => Some(1),
            KeyCode::Key3 => Some(2),
            KeyCode::Key4 => Some(3),
            KeyCode::Q => Some(4),
            KeyCode::W => Some(5),
            KeyCode::E => Some(6),
            KeyCode::R => Some(7),
            KeyCode::A => Some(8),
            KeyCode::S => Some(9),
            KeyCode::D => Some(10),
            KeyCode::F => Some(11),
            KeyCode::Z => Some(12),
            KeyCode::X => Some(13),
            KeyCode::C => Some(14),
            KeyCode::V => Some(15),
            _ => None,
        };

        self.imgui_wrapper.update_key_down(keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.imgui_wrapper.update_key_up(keycode, keymods);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, val: char) {
        self.imgui_wrapper.update_text(val);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.imgui_wrapper.update_scroll(x, y);
    }
}

impl MainState {
    fn execute_instruction(&mut self, instruction: InstructionSet, opcode: u16) {
        match instruction {
            InstructionSet::ClearScreen => {
                // todo
            }
            InstructionSet::ReturnFromSubroutine => {
                self.chip8_state.stack_pointer -= 1;
                self.chip8_state.program_counter =
                    self.chip8_state.stack[self.chip8_state.stack_pointer as usize];
            }
            InstructionSet::JumpToAddress(address) => {
                self.chip8_state.program_counter = address - 2
            }
            InstructionSet::ExecuteSubroutine(address) => {
                self.chip8_state.stack[self.chip8_state.stack_pointer as usize] =
                    self.chip8_state.program_counter;
                self.chip8_state.stack_pointer += 1;
                self.chip8_state.program_counter = address - 2;
            }
            InstructionSet::AddToRegister(index, value) => {
                self.chip8_state.v[index as usize] += value
            }
            InstructionSet::StoreInRegister(index, value) => {
                self.chip8_state.v[index as usize] = value
            }
            InstructionSet::CopyRegisterValueToOtherRegister(x, y) => {
                self.chip8_state.v[x as usize] = self.chip8_state.v[y as usize]
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToValue(index, value) => {
                if self.chip8_state.v[index as usize] == value {
                    self.chip8_state.program_counter += 2;
                }
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToValue(index, value) => {
                if self.chip8_state.v[index as usize] != value {
                    self.chip8_state.program_counter += 2;
                }
            }
            InstructionSet::SkipFollowingIfRegisterIsEqualToOtherRegister(x, y) => {
                if self.chip8_state.v[x as usize] == self.chip8_state.v[y as usize] {
                    self.chip8_state.program_counter += 2;
                }
            }
            InstructionSet::SetVxToVxOrVy(x, y) => {
                self.chip8_state.v[x as usize] |= self.chip8_state.v[y as usize]
            }
            InstructionSet::SetVxToVxAndVy(x, y) => {
                self.chip8_state.v[x as usize] &= self.chip8_state.v[y as usize]
            }
            InstructionSet::SetVxToVxXorVy(x, y) => {
                self.chip8_state.v[x as usize] ^= self.chip8_state.v[y as usize]
            }
            InstructionSet::AddValueOfRegisterVyToRegisterVx(x, y) => {
                let sum =
                    self.chip8_state.v[x as usize] as u16 + self.chip8_state.v[y as usize] as u16;
                self.chip8_state.v[0xF] = if sum > 255 { 1 } else { 0 };
                self.chip8_state.v[x as usize] = (sum & 0x00FF) as u8;
            }
            InstructionSet::SubtractValueOfRegisterVyFromRegisterVx(x, y) => {
                self.chip8_state.v[0xF] =
                    if self.chip8_state.v[x as usize] > self.chip8_state.v[y as usize] {
                        1
                    } else {
                        0
                    };
                self.chip8_state.v[x as usize] -= self.chip8_state.v[y as usize];
            }
            InstructionSet::StoreValueOfRegisterVyShiftedRightOneBitInVx(x, y) => {
                self.chip8_state.v[x as usize] = self.chip8_state.v[y as usize] >> 1;
                self.chip8_state.v[0xF] = self.chip8_state.v[x as usize] & 0x01;
            }
            InstructionSet::SetVxToValueOfVyMinusVx(x, y) => {
                self.chip8_state.v[0xF] =
                    if self.chip8_state.v[y as usize] > self.chip8_state.v[x as usize] {
                        1
                    } else {
                        0
                    };
                self.chip8_state.v[x as usize] =
                    self.chip8_state.v[y as usize] - self.chip8_state.v[x as usize];
            }
            InstructionSet::StoreValueOfRegisterVyShiftedLeftOneBitInVx(x, y) => {
                self.chip8_state.v[x as usize] = self.chip8_state.v[y as usize] << 1;
                self.chip8_state.v[0xF] = (self.chip8_state.v[x as usize] >> 7) & 0x01;
            }
            InstructionSet::SkipFollowingIfRegisterIsNotEqualToOtherRegister(x, y) => {
                if self.chip8_state.v[x as usize] != self.chip8_state.v[y as usize] {
                    self.chip8_state.program_counter += 2;
                }
            }
            InstructionSet::StoreDelayTimerInRegisterVx(index) => {
                self.chip8_state.v[index as usize] = self.chip8_state.delay_timer;
            }
            InstructionSet::WaitForAKeyPress(index) => {
                self.waiting_for_key_press = true;
                self.key_index_store = index;
            }
            InstructionSet::SetDelayTimerToVx(index) => {
                self.chip8_state.delay_timer = self.chip8_state.v[index as usize];
            }
            InstructionSet::SetSoundTimerToVx(index) => {
                self.chip8_state.sound_timer = self.chip8_state.v[index as usize];
            }
            InstructionSet::AddVxToRegisterI(index) => {
                self.chip8_state.i += self.chip8_state.v[index as usize] as u16;
            }
            InstructionSet::SetIToTheMemoryAddressOfSpriteCorrespondingToVx(index) => { // hope it's a correct implementation
                let v = self.chip8_state.v[index as usize] as usize;
                self.chip8_state.i = (FONTS_SPRITES[v].len() *  v) as u16;
            }
            InstructionSet::StoreTheBinaryCodedDecimalEquivalentOfVx(index) => {
                let v = self.chip8_state.v[index as usize];
                let hundreds_digit = v / 100;
                let tens_digit = (v - hundreds_digit*100) / 10;
                let units_digit = (v - hundreds_digit*100) - (tens_digit*10);
                self.chip8_state.memory[self.chip8_state.i as usize] = units_digit;
                self.chip8_state.memory[self.chip8_state.i as usize + 1] = tens_digit;
                self.chip8_state.memory[self.chip8_state.i as usize + 2] = hundreds_digit;
            }
            InstructionSet::StoreValuesOfV0ToVxInclusiveInMemoryStartingAtAddressI(index) => {
                for i in 0..(index+1) {
                    self.chip8_state.memory[self.chip8_state.i as usize + i as usize] = self.chip8_state.v[i as usize];
                } 
                self.chip8_state.i += index as u16 + 1;
            }
            InstructionSet::FillRegistersV0ToVxInclusiveWithMemoryStartingAtAddressI(index) => {
                for i in 0..(index+1) {
                    self.chip8_state.v[i as usize] = self.chip8_state.memory[self.chip8_state.i as usize + i as usize];
                } 
                self.chip8_state.i += index as u16 + 1;
            }
            InstructionSet::DrawSprite(x, y, sprite_data) => {
                //todo
            }
            InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsNotPressed(index) => {
                if let Some(code) = self.chip8_state.chip8_key {
                    if code != self.chip8_state.v[index as usize] {
                        self.chip8_state.program_counter += 2;
                    }
                }
            }
            InstructionSet::SkipFollowingInstructionIfKeyCorrespondingToVxIsPressed(index) => {
                if let Some(code) = self.chip8_state.chip8_key {
                    if code == self.chip8_state.v[index as usize] {
                        self.chip8_state.program_counter += 2;
                    }
                }
            }
            InstructionSet::StoreAddressInRegisterI(address) => {
                self.chip8_state.i = address;
            }
            InstructionSet::SetVxToRandomNumberWithAMaskOf(index, mask) => {
                let mut rng = rand::thread_rng();
                self.chip8_state.v[index as usize] = rng.gen_range(0, 255) & mask;
            }
            _ => {
                if opcode != 0 {
                    println!("[{:04X?}]: {}", opcode, instruction.to_string());
                }
            }
        }
    }
}

fn main() -> ggez::GameResult {
    let matches = setup_cmd_program_arguments();

    let rom_path = get_rom_path(matches).unwrap();
    println!("ROM file path you provided '{}'", rom_path);

    let rom_data = read_file_as_bytes(rom_path.as_str()).unwrap();

    let cb = ggez::ContextBuilder::new("CHIP-8 VM", "ggez")
        .window_setup(conf::WindowSetup::default().title("CHIP-8 VM"))
        .window_mode(conf::WindowMode::default().resizable(false).dimensions(
            (DISPLAY_SIZE[0] * SCALE) as f32 + DEBUG_EXTRA_DISPLAY_SIZE[0],
            (DISPLAY_SIZE[1] * SCALE) as f32 + DEBUG_EXTRA_DISPLAY_SIZE[1],
        ));
    let (ref mut ctx, event_loop) = &mut cb.build()?;

    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;

    let state = &mut MainState::new(ctx, hidpi_factor, &rom_data)?;

    event::run(ctx, event_loop, state)
}
