use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};

mod imgui_wrapper;
use imgui_wrapper::ImGuiWrapper;

mod chip8_state;
use chip8_state::*;

mod setup;
use setup::*;

mod utils;
use utils::*;

mod instructions;
use instructions::*;

const SCALE: usize = 16;
const DISPLAY_SIZE: [usize; 2] = [64, 32];

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
}

impl MainState {
    fn new(mut ctx: &mut Context, hidpi_factor: f32, rom_data: &[u8]) -> GameResult<MainState> {
        let imgui_wrapper = ImGuiWrapper::new(&mut ctx);
        let mut s = MainState {
            imgui_wrapper,
            hidpi_factor,
            chip8_state: Chip8State {
                memory: [0 as u8; MEMORY_SIZE],
                v: [0 as u8; 16],
                i: 0,
                _delay_timer: 0,
                _sound_timer: 0,
                program_counter: 0x200,
                stack_pointer: 0,
                stack: [0 as u16; 16],
            },
        };

        write_rom_data_to_memory(&mut s.chip8_state.memory, rom_data);

        Ok(s)
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let opcode = fetch_opcode(&self.chip8_state.memory, self.chip8_state.program_counter).unwrap();
        let instruction = decode_opcode(opcode);

        match instruction {
            InstructionSet::ClearScreen => {}
            InstructionSet::ReturnFromSubroutine => {
                self.chip8_state.stack_pointer -= 1;
                self.chip8_state.program_counter = self.chip8_state.stack[self.chip8_state.stack_pointer as usize];
            }
            InstructionSet::JumpToAddress(address) => self.chip8_state.program_counter = address - 2,
            InstructionSet::ExecuteSubroutine(address) => {
                self.chip8_state.stack[self.chip8_state.stack_pointer as usize] = self.chip8_state.program_counter;
                self.chip8_state.stack_pointer += 1;
                self.chip8_state.program_counter = address - 2;
            }
            InstructionSet::AddToRegister(index, value) => self.chip8_state.v[index as usize] += value,
            InstructionSet::StoreInRegister(index, value) => self.chip8_state.v[index as usize] = value,
            InstructionSet::AddVxToRegisterI(index) => self.chip8_state.i += self.chip8_state.v[index as usize] as u16,
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
            InstructionSet::SetVxToVxOrVy(x, y) => self.chip8_state.v[x as usize] |= self.chip8_state.v[y as usize],
            InstructionSet::SetVxToVxAndVy(x, y) => self.chip8_state.v[x as usize] &= self.chip8_state.v[y as usize],
            InstructionSet::SetVxToVxXorVy(x, y) => self.chip8_state.v[x as usize] ^= self.chip8_state.v[y as usize],
            InstructionSet::AddValueOfRegisterVyToRegisterVx(x, y) => {
                let sum = self.chip8_state.v[x as usize] as u16 + self.chip8_state.v[y as usize] as u16;
                self.chip8_state.v[0xF] = if sum > 255 { 1 } else { 0 };
                self.chip8_state.v[x as usize] = (sum & 0x00FF) as u8;
            }
            InstructionSet::SubtractValueOfRegisterVyFromRegisterVx(x, y) => {
                self.chip8_state.v[0xF] = if self.chip8_state.v[x as usize] > self.chip8_state.v[y as usize] {
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
                self.chip8_state.v[0xF] = if self.chip8_state.v[y as usize] > self.chip8_state.v[x as usize] {
                    1
                } else {
                    0
                };
                self.chip8_state.v[x as usize] = self.chip8_state.v[y as usize] - self.chip8_state.v[x as usize];
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
            _ => {}
        }

        if opcode != 0 {
            println!("[{:04X?}]: {}", opcode, instruction.to_string());
        }

        self.chip8_state.program_counter += 2;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        // Render game stuff
        {}

        // Render game ui
        {
            self.imgui_wrapper.render(ctx, self.hidpi_factor, &self.chip8_state);
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

fn main() -> ggez::GameResult {
    let matches = setup_cmd_program_arguments();

    let rom_path = get_rom_path(matches).unwrap();
    println!("ROM file path you provided '{}'", rom_path);

    let rom_data =
        read_file_as_bytes(rom_path.as_str(), MEMORY_SIZE - CHIP8_RESERVED_MEMORY_SIZE).unwrap();

    let cb = ggez::ContextBuilder::new("CHIP-8 VM", "ggez")
        .window_setup(conf::WindowSetup::default().title("CHIP-8 VM"))
        .window_mode(
            conf::WindowMode::default().resizable(true), /*.dimensions(750.0, 500.0)*/
        );
    let (ref mut ctx, event_loop) = &mut cb.build()?;

    let hidpi_factor = event_loop.get_primary_monitor().get_hidpi_factor() as f32;
    println!("main hidpi_factor = {}", hidpi_factor);

    let state = &mut MainState::new(ctx, hidpi_factor, &rom_data)?;

    event::run(ctx, event_loop, state)
}
